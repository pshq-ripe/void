use anyhow::Result;
use clap::Parser;
use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

use void::app::App;
use void::commands::registry::CommandRegistry;
use void::irc::connection::{self, IrcEvent};
use void::irc::proto::handle_irc_message;
use void::scripting::engine;
use void::ui::input::handle_key;
use void::ui::renderer;

#[derive(Parser, Debug)]
#[command(name = "void")]
#[command(about = "Void — A Lua-scriptable IRC client in Rust (epic5 inspired)")]
#[command(disable_help_flag = false)]
struct Args {
    /// IRC server hostname
    #[arg(short = 'c', long)]
    server: Option<String>,

    /// Nickname
    #[arg(short = 'n', long)]
    nickname: Option<String>,

    /// Channel to auto-join
    #[arg(short = 'j', long)]
    channel: Option<String>,

    /// Bind to vhost
    #[arg(short = 'H', long)]
    vhost: Option<String>,

    /// Server password
    #[arg(short = 'p', long)]
    password: Option<String>,

    /// Port (default: 6697 for TLS)
    #[arg(short = 'P', long, default_value = "6697")]
    port: u16,

    /// Disable TLS
    #[arg(long)]
    no_tls: bool,

    /// NickServ password (auto-identify after connect)
    #[arg(short = 'N', long)]
    nickserv: Option<String>,

    /// SASL credentials (nick:password for PLAIN auth)
    #[arg(long)]
    sasl: Option<String>,

    /// Proxy type (socks5)
    #[arg(long)]
    proxy_type: Option<String>,

    /// Proxy server hostname
    #[arg(long)]
    proxy_server: Option<String>,

    /// Proxy port (default: 1080)
    #[arg(long, default_value = "1080")]
    proxy_port: u16,

    /// Proxy username
    #[arg(long)]
    proxy_user: Option<String>,

    /// Proxy password
    #[arg(long)]
    proxy_pass: Option<String>,

    /// Force IPv6
    #[arg(long)]
    ipv6: bool,

    /// Database encryption passphrase (default: derived from system)
    #[arg(long)]
    db_pass: Option<String>,
}


#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Lua
    let lua = engine::init_lua()?;

    // Lua API (void.register_command, void.on, etc.)
    let lua_hooks = std::sync::Arc::new(std::sync::Mutex::new(
        void::scripting::api::LuaHooks::new()
    ));
    let (lua_cmd_tx, mut lua_cmd_rx) = mpsc::channel::<void::scripting::api::LuaCommand>(100);
    let lua_ctx = std::sync::Arc::new(std::sync::Mutex::new(
        void::scripting::api::LuaContext {
            our_nick: String::new(),
            current_channel: String::new(),
            server_host: String::new(),
            connected: false,
            cmd_tx: lua_cmd_tx,
            settings: std::collections::HashMap::new(),
        }
    ));
    void::scripting::api::register_api(&lua, lua_hooks.clone(), lua_ctx.clone())?;

    // Załaduj config.lua i skrypty PO register_api (żeby void.* było dostępne)
    engine::load_scripts(&lua);

    // Konfiguracja: Lua < CLI
    let mut nick = engine::get_config_string(&lua, "config", "nickname")
        .unwrap_or_else(|| "void_user".into());
    let mut server = engine::get_config_string(&lua, "config", "server")
        .unwrap_or_else(|| "irc.libera.chat".into());
    let mut channel = engine::get_config_vec(&lua, "config", "channels")
        .and_then(|v| v.into_iter().next());
    let port = args.port;
    let tls = !args.no_tls;

    if let Some(s) = args.server { server = s; }
    if let Some(n) = args.nickname { nick = n; }
    if let Some(c) = args.channel { channel = Some(c); }

    // Database passphrase — z CLI lub wygenerowany z systemu
    let db_pass = args.db_pass.clone().unwrap_or_else(|| {
        let hostname = hostname::get().map(|h| h.to_string_lossy().to_string()).unwrap_or_default();
        let username = std::env::var("USER").unwrap_or_default();
        format!("void-{}-{}-salt2026", hostname, username)
    });

    // App state
    let mut app = App::new(&nick, &server, port, tls, &db_pass);
    app.server_mut().nick_password = args.nickserv.clone();
    app.lua_hooks = Some(lua_hooks.clone());
    app.lua_ctx = Some(lua_ctx.clone());
    let lua_arc = Arc::new(lua);
    app.lua = Some(lua_arc.clone());
    let lua = &*lua_arc;

    // Załaduj skonfigurowany theme lub bieżący z Lua
    if let Some(theme_name) = engine::get_config_string(lua, "config", "theme") {
        app.apply_theme(&theme_name);
    } else if let Ok(themes_tbl) = lua.globals().get::<mlua::Table>("void_themes") {
        if let Ok(current) = themes_tbl.get::<String>("current") {
            app.apply_theme(&current);
        }
    }

    let registry = CommandRegistry::new();

    // Kanały komunikacji
    let (irc_tx, mut irc_rx) = mpsc::channel::<IrcEvent>(200);
    let (term_tx, mut term_rx) = mpsc::channel::<event::KeyEvent>(200);
    let (resize_tx, mut resize_rx) = mpsc::channel::<()>(10);
    let (mouse_tx, mut mouse_rx) = mpsc::channel::<event::MouseEvent>(50);
    let (paste_tx, mut paste_rx) = mpsc::channel::<String>(20);

    // Mouse capture — tylko jeśli MOUSE ON w settings
    let mouse_enabled = app.settings.get_bool("MOUSE");
    if mouse_enabled {
        execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;
    }

    // ─── Terminal input task ─────────────────────────
    tokio::spawn(async move {
        loop {
            if event::poll(Duration::from_millis(50)).unwrap_or(false) {
                match event::read() {
                    Ok(Event::Key(key)) => {
                        if term_tx.send(key).await.is_err() {
                            break;
                        }
                    }
                    Ok(Event::Resize(_, _)) => {
                        let _ = resize_tx.send(()).await;
                    }
                    Ok(Event::Mouse(mouse)) => {
                        let _ = mouse_tx.send(mouse).await;
                    }
                    Ok(Event::Paste(text)) => {
                        let _ = paste_tx.send(text).await;
                    }
                    _ => {}
                }
            }
        }
    });

    // ─── IRC connection task ─────────────────────────
    // Odpal połączenie, jeśli mamy serwer
    let proxy_config = connection::ProxyConfig {
        proxy_type: args.proxy_type.clone(),
        server: args.proxy_server.clone(),
        port: args.proxy_port,
        username: args.proxy_user.clone(),
        password: args.proxy_pass.clone(),
    };
    let ipv6_mode = args.ipv6;

    let mut conn_handle: Option<JoinHandle<()>> = {
        let tx = irc_tx.clone();
        let host = app.server().host.clone();
        let port = app.server().port;
        let nickname = app.server().our_nick.clone();
        let use_tls = app.server().tls;
        let password = args.password.clone();
        let sasl = args.sasl.clone();
        let proxy = proxy_config.clone();
        let ipv6 = ipv6_mode;
        Some(tokio::spawn(async move {
            connection::spawn_connection(host, port, nickname, use_tls, password, sasl, false, proxy, ipv6, args.vhost.clone(), tx).await;
        }))
    };

    // ─── TUI init ────────────────────────────────────
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // ─── Main loop ───────────────────────────────────
    let mut auto_join_channel = channel;
    let mut timer_tick = tokio::time::interval(Duration::from_secs(1));
    let mut notify_tick = tokio::time::interval(Duration::from_secs(60));
    let mut save_tick = tokio::time::interval(Duration::from_secs(300)); // auto-save co 5 min
    let mut lag_tick = tokio::time::interval(Duration::from_secs(30)); // lag ping co 30s

    while app.running {
        // Draw
        renderer::draw(&mut terminal, &app)?;

        // Non-blocking poll z kanałów + timer tick + redraw timeout
        tokio::select! {
            // Redraw co 100ms nawet bez eventów (fix: mode lag)
            _ = tokio::time::sleep(Duration::from_millis(100)) => {
                // Timeout — redraw
            }
            Some(_) = resize_rx.recv() => {
                // Terminal resize — wymuś redraw
                let _ = terminal.clear();
            }
            Some(pasted) = paste_rx.recv() => {
                // Bracketed paste — wstaw tekst do linii wejścia
                // Jeśli paste zawiera nowe linie, wykonaj każdą jako komendę
                let lines: Vec<&str> = pasted.lines().collect();
                if lines.len() > 1 {
                    for line in lines {
                        let line = line.trim();
                        if !line.is_empty() {
                            app.input_text = line.to_string();
                            // Symuluj Enter
                            if line.starts_with('/') {
                                let parts: Vec<&str> = line.split_whitespace().collect();
                                let cmd_name = &parts[0][1..];
                                let args = &parts[1..];
                                if let Some(cmd) = registry.find(cmd_name) {
                                    let handler = cmd.handler;
                                    handler(&mut app, args);
                                }
                            } else {
                                let buf_name = app.buffers[app.current_buffer_idx].name.clone();
                                if buf_name != "(Status)" {
                                    if let Some(s) = &app.server().sender {
                                        let _ = s.send_privmsg(&buf_name, line);
                                    }
                                    app.buffer_message(&buf_name, format!("<{}> {}", app.server().our_nick, line), void::app::MessageType::Normal);
                                }
                            }
                        }
                    }
                    app.input_text.clear();
                    app.input_cursor_pos = 0;
                } else {
                    // Pojedyncza linia — wstaw do input
                    app.input_text.push_str(&pasted);
                    app.input_cursor_pos = app.input_text.len();
                }
            }
            Some(lua_cmd) = lua_cmd_rx.recv() => {
                // Komenda z Lua — wykonaj jak wpisaną z klawiatury
                let cmd_text = lua_cmd.raw.clone();
                if cmd_text.starts_with('/') {
                    let parts: Vec<&str> = cmd_text.split_whitespace().collect();
                    let cmd_name = &parts[0][1..];
                    let cmd_args = &parts[1..];
                    if let Some(cmd) = registry.find(cmd_name) {
                        let handler = cmd.handler;
                        handler(&mut app, cmd_args);
                    }
                } else if cmd_text.starts_with("RAW ") {
                    // Surowa komenda IRC
                    let raw = &cmd_text[4..];
                    if let Some(s) = &app.server().sender {
                        let _ = s.send(irc::client::prelude::Command::Raw(raw.to_string(), Vec::new()));
                    }
                } else if cmd_text.starts_with("ECHO ") {
                    let text = &cmd_text[5..];
                    app.system_message(text);
                } else if cmd_text.starts_with("SET ") {
                    let rest = &cmd_text[4..];
                    if let Some((key, value)) = rest.split_once(' ') {
                        app.settings.set(key, value);
                    }
                } else if cmd_text.starts_with("THEME_APPLY ") {
                    let theme_name = &cmd_text[12..];
                    app.apply_theme(theme_name);
                    app.system_message(&format!("-!- Theme applied: {}", theme_name));
                }
            }
            Some(mouse) = mouse_rx.recv() => {
                use crossterm::event::{MouseEventKind, MouseButton};
                match mouse.kind {
                    MouseEventKind::ScrollUp => {
                        // Scroll w split pane jeśli mysz w dolnej połowie
                        if let Some(split_idx) = app.split_buffer_idx {
                            let term_height = terminal.size().map(|s| s.height).unwrap_or(24);
                            if mouse.row > term_height / 2 {
                                let max = app.buffers[split_idx].messages.len().saturating_sub(1);
                                app.split_scroll_offset = (app.split_scroll_offset + 3).min(max);
                            } else {
                                let buf = &mut app.buffers[app.current_buffer_idx];
                                let max = buf.messages.len().saturating_sub(1);
                                buf.scroll_offset = (buf.scroll_offset + 3).min(max);
                            }
                        } else {
                            let buf = &mut app.buffers[app.current_buffer_idx];
                            let max = buf.messages.len().saturating_sub(1);
                            buf.scroll_offset = (buf.scroll_offset + 3).min(max);
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if app.split_buffer_idx.is_some() {
                            let term_height = terminal.size().map(|s| s.height).unwrap_or(24);
                            if mouse.row > term_height / 2 {
                                app.split_scroll_offset = app.split_scroll_offset.saturating_sub(3);
                            } else {
                                let buf = &mut app.buffers[app.current_buffer_idx];
                                buf.scroll_offset = buf.scroll_offset.saturating_sub(3);
                            }
                        } else {
                            let buf = &mut app.buffers[app.current_buffer_idx];
                            buf.scroll_offset = buf.scroll_offset.saturating_sub(3);
                        }
                    }
                    MouseEventKind::Down(MouseButton::Left) => {
                        // Klik na status bar (y=0) — przejdź do bufora
                        if mouse.row == 0 {
                            // Oblicz który bufor został kliknięty
                            let mut x = 0u16;
                            for (i, b) in app.buffers.iter().enumerate() {
                                let label_len = b.name.len() as u16 + 3; // " name "
                                if mouse.column >= x && mouse.column < x + label_len {
                                    app.current_buffer_idx = i;
                                    app.buffers[i].unread_count = 0;
                                    app.buffers[i].has_activity = false;
                                    break;
                                }
                                x += label_len;
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ = timer_tick.tick() => {
                // Sprawdź timery
                let now = std::time::Instant::now();
                let mut commands_to_run = Vec::new();
                for timer in &mut app.timers {
                    if timer.remaining != 0 && now >= timer.next_fire {
                        commands_to_run.push(timer.command.clone());
                        if timer.remaining > 0 {
                            timer.remaining -= 1;
                        }
                        timer.next_fire = now + Duration::from_millis(timer.interval_ms);
                    }
                }
                // Usuń wyczerpane timery
                app.timers.retain(|t| t.remaining != 0);
                // Wykonaj komendy timerów
                for cmd in commands_to_run {
                    if cmd.starts_with('/') {
                        let parts: Vec<&str> = cmd.split_whitespace().collect();
                        let cmd_name = &parts[0][1..];
                        let args = &parts[1..];
                        if let Some(cmd) = registry.find(cmd_name) {
                            let handler = cmd.handler;
                            handler(&mut app, args);
                        }
                    } else {
                        let buf_name = app.buffers[app.current_buffer_idx].name.clone();
                        if buf_name != "(Status)" {
                            if let Some(s) = &app.server().sender {
                                let _ = s.send_privmsg(&buf_name, &cmd);
                            }
                            app.buffer_message(&buf_name, format!("<{}> {}", app.server().our_nick, cmd), void::app::MessageType::Normal);
                        }
                    }
                }
            }
            _ = save_tick.tick() => {
                // Auto-save do SQLite co 5 minut
                app.save_to_db();
            }
            _ = lag_tick.tick() => {
                // Lag measurement — wyślij PING i zmierz czas odpowiedzi
                if app.server().connected {
                    let sender = app.server().sender.clone();
                    if let Some(s) = sender {
                        let now = std::time::Instant::now();
                        app.server_mut().lag_ping_sent = Some(now);
                        let ts = now.elapsed().as_millis().to_string();
                        let _ = s.send(irc::client::prelude::Command::Raw(
                            format!("PING :{}", ts), Vec::new()
                        ));
                    }
                }
            }
            _ = notify_tick.tick() => {
                // Cykliczny polling notify — MONITOR (IRCv3) lub ISON (fallback)
                if !app.notify_list.is_empty() && app.server().connected {
                    if let Some(s) = &app.server().sender {
                        let nicks: Vec<String> = app.notify_list.iter().map(|n| n.nick.clone()).collect();
                        // Sprawdź czy serwer wspiera MONITOR
                        let has_monitor = app.server().server_info.tokens.contains_key("MONITOR");
                        if has_monitor {
                            let nick_list = nicks.join(",");
                            let _ = s.send(irc::client::prelude::Command::Raw(
                                format!("MONITOR + {}", nick_list), Vec::new()
                            ));
                        } else {
                            let _ = s.send(irc::client::prelude::Command::ISON(nicks));
                        }
                    }
                }
            }
            // Sprawdź pending exec
            _ = async {
                if app.pending_exec.is_empty() {
                    futures::future::pending::<()>().await;
                }
            } => {
                for cmd in app.pending_exec.drain(..) {
                    let tx = irc_tx.clone();
                    std::thread::spawn(move || {
                        let output = std::process::Command::new("sh")
                            .arg("-c")
                            .arg(&cmd)
                            .output();
                        let mut lines = Vec::new();
                        match output {
                            Ok(out) => {
                                let stdout = String::from_utf8_lossy(&out.stdout);
                                let stderr = String::from_utf8_lossy(&out.stderr);
                                for line in stdout.lines() {
                                    lines.push(format!("  {}", line));
                                }
                                for line in stderr.lines() {
                                    lines.push(format!("  [err] {}", line));
                                }
                                if lines.is_empty() {
                                    lines.push("  (no output)".into());
                                }
                            }
                            Err(e) => {
                                lines.push(format!("-!- Exec error: {}", e));
                            }
                        }
                        let _ = tx.blocking_send(IrcEvent::ExecOutput(lines));
                    });
                }
            }
            Some(key) = term_rx.recv() => {
                handle_key(&mut app, key, &registry);
                
                // Sprawdź, czy /server zlecił reconnect
                if app.reconnect_pending {
                    app.reconnect_pending = false;
                    if let Some(h) = conn_handle.take() {
                        h.abort();
                    }
                    let tx = irc_tx.clone();
                    let host = app.server().host.clone();
                    let port = app.server().port;
                    let nickname = app.server().our_nick.clone();
                    let use_tls = app.server().tls;
                    conn_handle = Some(tokio::spawn(async move {
                        connection::spawn_connection(host, port, nickname, use_tls, None, None, false, connection::ProxyConfig::default(), false, None, tx).await;
                    }));
                }
            }
            Some(irc_event) = irc_rx.recv() => {
                match irc_event {
                    IrcEvent::Connected(sender) => {
                        app.server_mut().connected = true;
                        app.server_mut().sender = Some(sender.clone());
                        app.system_message(&format!("-!- Connected to {}:{}", app.server().host, app.server().port));
                        // Sync Lua context
                        {
                            let mut ctx = lua_ctx.lock().unwrap();
                            ctx.connected = true;
                            ctx.our_nick = app.server().our_nick.clone();
                            ctx.server_host = app.server().host.clone();
                        }
                        // Auto-join kanału
                        if let Some(ch) = auto_join_channel.take() {
                            let _ = sender.send_join(&ch);
                        }
                        // Restore session — dołącz do zapisanych kanałów
                        let saved_channels = app.restore_session();
                        for ch in saved_channels {
                            app.system_message(&format!("-!- Restoring session: joining {}", ch));
                            let _ = sender.send_join(&ch);
                        }
                    }
                    IrcEvent::Disconnected => {
                        app.server_mut().connected = false;
                        app.server_mut().sender = None;
                        app.system_message("-!- Disconnected from server.");
                        {
                            let mut ctx = lua_ctx.lock().unwrap();
                            ctx.connected = false;
                        }

                        // Auto-reconnect
                        if app.settings.get_bool("AUTO_RECONNECT") && app.running {
                            let delay = app.settings.get_int("AUTO_RECONNECT_DELAY").max(5) as u64;
                            app.system_message(&format!("-!- Reconnecting in {} seconds...", delay));
                            let tx = irc_tx.clone();
                            let host = app.server().host.clone();
                            let port = app.server().port;
                            let nickname = app.server().our_nick.clone();
                            let use_tls = app.server().tls;
                            conn_handle = Some(tokio::spawn(async move {
                                tokio::time::sleep(Duration::from_secs(delay)).await;
                                connection::spawn_connection(host, port, nickname, use_tls, None, None, false, connection::ProxyConfig::default(), false, None, tx).await;
                            }));
                        }
                    }
                    IrcEvent::Message(msg) => {
                        // Odpal hooki Lua dla tego zdarzenia
                        {
                            let hooks = lua_hooks.lock().unwrap();
                            let event_name = format!("{:?}", msg.command);
                            let event_type = event_name.split('(').next().unwrap_or(&event_name);
                            let source = msg.source_nickname().unwrap_or("");
                            let results = void::scripting::api::fire_event(&lua, &hooks, event_type, &[source]);
                            drop(hooks);
                            for result in results {
                                app.system_message(&result);
                            }
                        }
                        handle_irc_message(&mut app, &msg);
                    }
                    IrcEvent::Status(msg) => {
                        app.system_message(&msg);
                    }
                    IrcEvent::ExecOutput(lines) => {
                        for line in lines {
                            app.system_message(&line);
                        }
                    }
                    IrcEvent::CapEvent(sub, data) => {
                        // Odpal hooki Lua dla CAP events
                        let hooks = lua_hooks.lock().unwrap();
                        let results = void::scripting::api::fire_event(
                            &lua, &hooks, "CAP",
                            &[&sub, &data],
                        );
                        drop(hooks);
                        for result in results {
                            app.system_message(&result);
                        }
                    }
                    IrcEvent::Error(e) => {
                        app.system_message(&format!("-!- IRC Error: {}", e));
                    }
                }
            }
        }
    }

    // ─── Zapisz stan do SQLite (z timeoutem) ──────────
    let _ = std::thread::spawn(|| {
        app.save_to_db();
    }).join();

    // ─── Cleanup (ignoruj błędy) ─────────────────────
    let _ = execute!(std::io::stdout(), crossterm::event::DisableMouseCapture);
    let _ = disable_raw_mode();
    let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen);
    let _ = terminal.show_cursor();

    // Wymuś zakończenie procesu
    std::process::exit(0);
}
