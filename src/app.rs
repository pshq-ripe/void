use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use ratatui::style::Color;
use crate::logging::Logger;
use crate::flood::FloodProtection;
use crate::dcc::DccManager;
use crate::storage::Storage;
use crate::scripting::api::LuaHooks;

/// Pojedynczy bufor (okno) — kanał, prywatna rozmowa lub Status.
pub struct Buffer {
    pub name: String,
    pub messages: Vec<StyledMessage>,
    pub nicks: Vec<NickEntry>,
    pub topic: String,
    pub scroll_offset: usize,
    pub unread_count: usize,
    pub has_activity: bool,
    pub new_while_scrolled: usize,  // nowe wiadomości podczas scrollback
}

/// Wpis na liście nicków z prefixem trybu (@, +, %, ~, &)
#[derive(Clone, Debug)]
pub struct NickEntry {
    pub nick: String,
    pub prefix: String,  // "@" = op, "+" = voice, "%" = halfop, "~" = owner, "&" = admin
    pub account: String,  // IRCv3 extended-join account
    pub realname: String, // IRCv3 extended-join realname
}

impl NickEntry {
    pub fn new(raw: &str) -> Self {
        let prefixes = ['~', '&', '@', '%', '+'];
        let mut prefix = String::new();
        let mut nick = raw.to_string();
        for p in &prefixes {
            if nick.starts_with(*p) {
                prefix.push(nick.remove(0));
            }
        }
        NickEntry { nick, prefix, account: String::new(), realname: String::new() }
    }

    pub fn display(&self) -> String {
        format!("{}{}", self.prefix, self.nick)
    }

    /// Sortowanie: operatorzy pierwsi, potem voice, potem reszta
    pub fn sort_key(&self) -> (u8, String) {
        let rank = match self.prefix.as_str() {
            s if s.contains('~') => 0,
            s if s.contains('&') => 1,
            s if s.contains('@') => 2,
            s if s.contains('%') => 3,
            s if s.contains('+') => 4,
            _ => 5,
        };
        (rank, self.nick.to_lowercase())
    }
}

/// Wiadomość ze znacznikiem czasu i typem
#[derive(Clone)]
pub struct StyledMessage {
    pub timestamp: String,
    pub text: String,
    pub msg_type: MessageType,
}

#[derive(Clone, PartialEq)]
pub enum MessageType {
    Normal,        // <nick> tekst
    Action,        // * nick robi coś
    System,        // -!- komunikat systemowy
    Notice,        // -nick- tekst
    Ctcp,          // CTCP
    ServerReply,   // Odpowiedź serwera (MOTD, numeric)
    Error,         // Błąd
    Highlight,     // Wiadomość zawierająca nasz nick
}

impl Buffer {
    pub fn new(name: &str) -> Self {
        Buffer {
            name: name.to_string(),
            messages: Vec::new(),
            nicks: Vec::new(),
            topic: String::new(),
            scroll_offset: 0,
            unread_count: 0,
            has_activity: false,
            new_while_scrolled: 0,
        }
    }

    pub fn push_message(&mut self, text: String, msg_type: MessageType, scrollback_limit: usize) {
        let timestamp = chrono::Local::now().format("%H:%M").to_string();
        self.messages.push(StyledMessage {
            timestamp,
            text,
            msg_type,
        });
        // Scrollback indicator — zliczaj nowe wiadomości gdy użytkownik scrolluje
        if self.scroll_offset > 0 {
            self.new_while_scrolled += 1;
        }
        let limit = scrollback_limit.max(100);
        if self.messages.len() > limit {
            self.messages.remove(0);
            if self.scroll_offset > 0 {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
            }
        }
    }

    pub fn add_nick(&mut self, raw: &str) {
        let entry = NickEntry::new(raw);
        if !self.nicks.iter().any(|n| n.nick == entry.nick) {
            self.nicks.push(entry);
            self.sort_nicks();
        }
    }

    pub fn remove_nick(&mut self, nick: &str) {
        self.nicks.retain(|n| n.nick != nick);
    }

    pub fn rename_nick(&mut self, old: &str, new: &str) {
        if let Some(entry) = self.nicks.iter_mut().find(|n| n.nick == old) {
            entry.nick = new.to_string();
            self.sort_nicks();
        }
    }

    pub fn set_nick_info(&mut self, nick: &str, account: &str, realname: &str) {
        if let Some(entry) = self.nicks.iter_mut().find(|n| n.nick == nick) {
            entry.account = account.to_string();
            entry.realname = realname.to_string();
        }
    }

    pub fn set_nick_prefix(&mut self, nick: &str, prefix_char: char, add: bool) {
        if let Some(entry) = self.nicks.iter_mut().find(|n| n.nick == nick) {
            if add {
                if !entry.prefix.contains(prefix_char) {
                    entry.prefix.push(prefix_char);
                }
            } else {
                entry.prefix.retain(|c| c != prefix_char);
            }
            self.sort_nicks();
        }
    }

    pub fn sort_nicks(&mut self) {
        self.nicks.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
    }
}

/// Wpis w systemie /NOTIFY
pub struct NotifyEntry {
    pub nick: String,
    pub online: bool,
    pub last_seen: Option<Instant>,
}

/// Wpis w systemie /IGNORE
#[derive(Clone)]
pub struct IgnoreEntry {
    pub pattern: String,
    pub ignore_public: bool,
    pub ignore_private: bool,
    pub ignore_notice: bool,
    pub ignore_ctcp: bool,
    pub ignore_all: bool,
}

/// Wpis w systemie /TIMER
pub struct TimerEntry {
    pub name: String,
    pub interval_ms: u64,
    pub repeat: i32,   // -1 = infinite
    pub command: String,
    pub next_fire: Instant,
    pub remaining: i32,
}

/// Informacja o połączeniu z jednym serwerem
pub struct ServerConnection {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub connected: bool,
    pub our_nick: String,
    pub away_message: Option<String>,
    pub sender: Option<irc::client::Sender>,
    pub user_modes: String,
    pub server_info: ServerInfo,
    pub nick_password: Option<String>,
}

/// Parsowane tokeny ISUPPORT (005)
#[derive(Default)]
pub struct ServerInfo {
    pub network: String,
    pub chantypes: String,       // np. "#&+!"
    pub prefix_modes: String,    // np. "(qaohv)~&@%+"
    pub chanmodes: String,        // np. "beI,k,l,imnpst"
    pub nicklen: Option<usize>,
    pub topiclen: Option<usize>,
    pub channellen: Option<usize>,
    pub modes: Option<usize>,    // max modes per line
    pub tokens: HashMap<String, String>,
}

impl ServerConnection {
    pub fn new(host: &str, port: u16, nick: &str, tls: bool) -> Self {
        ServerConnection {
            host: host.to_string(),
            port,
            tls,
            connected: false,
            our_nick: nick.to_string(),
            away_message: None,
            sender: None,
            user_modes: String::new(),
            server_info: ServerInfo::default(),
            nick_password: None,
        }
    }
}

/// Globalny stan konfiguracji (/SET)
pub struct Settings {
    pub map: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        // Domyślne wartości SET (wzorowane na epic5)
        map.insert("SCROLL_LINES".into(), "1".into());
        map.insert("SCROLLBACK".into(), "500".into());
        map.insert("BEEP_ON_MSG".into(), "OFF".into());
        map.insert("CLOCK_24HOUR".into(), "ON".into());
        map.insert("SHOW_CHANNEL_NAMES".into(), "ON".into());
        map.insert("SHOW_STATUS_ALL".into(), "ON".into());
        map.insert("LOG".into(), "OFF".into());
        map.insert("LOG_FILE".into(), "void.log".into());
        map.insert("FLOOD_PROTECTION".into(), "ON".into());
        map.insert("FLOOD_RATE".into(), "4".into());
        map.insert("FLOOD_RATE_PER".into(), "2".into());
        map.insert("AUTO_RECONNECT".into(), "ON".into());
        map.insert("AUTO_RECONNECT_DELAY".into(), "15".into());
        map.insert("STATUS_FORMAT".into(), " [ $N ] [ $C ] [ $T ] ".into());
        map.insert("INPUT_PROMPT".into(), "> ".into());
        map.insert("SHOW_TIMESTAMPS".into(), "ON".into());
        map.insert("TIMESTAMP_FORMAT".into(), "%H:%M".into());
        map.insert("DCC_DOWNLOAD_DIR".into(), "~/dcc".into());
        map.insert("CTCP_REPLY".into(), "ON".into());
        map.insert("SSL_VERIFY".into(), "OFF".into());
        map.insert("DEBUG".into(), "OFF".into());
        map.insert("NICK_WIDTH".into(), "18".into());
        Settings { map }
    }

    pub fn get(&self, key: &str) -> &str {
        self.map.get(&key.to_uppercase()).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_uppercase(), value.to_string());
    }

    pub fn get_bool(&self, key: &str) -> bool {
        matches!(self.get(key), "ON" | "1" | "true" | "yes")
    }

    pub fn get_int(&self, key: &str) -> i64 {
        self.get(key).parse().unwrap_or(0)
    }
}

/// Centralna struktura stanu aplikacji
pub struct App {
    pub buffers: Vec<Buffer>,
    pub current_buffer_idx: usize,
    pub input_text: String,
    pub input_history: Vec<String>,
    pub input_history_idx: Option<usize>,
    pub input_cursor_pos: usize,
    pub servers: Vec<ServerConnection>,
    pub active_server_idx: usize,
    pub settings: Settings,
    pub notify_list: Vec<NotifyEntry>,
    pub ignore_list: Vec<IgnoreEntry>,
    pub timers: Vec<TimerEntry>,
    pub last_msg_target: Option<String>,
    pub running: bool,
    pub reconnect_pending: bool,
    pub pending_exec: Vec<String>,
    pub last_buffer_idx: Option<usize>,
    pub aliases: HashMap<String, String>,
    pub highlight_patterns: Vec<HighlightPattern>,
    pub key_bindings: HashMap<String, String>,
    pub label_counter: u64,
    pub pending_labels: HashMap<String, String>, // label -> request description
    pub format_templates: HashMap<String, String>,
    pub theme_colors: ThemeColors,
    pub split_buffer_idx: Option<usize>, // None = brak split, Some(idx) = drugi bufor
    pub split_scroll_offset: usize,      // niezależny scroll dla split pane
    pub split_horizontal: bool,          // false=vertical(top/bottom), true=horizontal(left/right)
    pub suppress_display: bool,          // /SHH — wycisz wyświetlanie w aktualnym kontekście
    pub output_context: OutputContext,    // aktualny kontekst wyjścia
    pub storage: Option<Storage>,
    pub lua_hooks: Option<Arc<Mutex<LuaHooks>>>,
    pub lua: Option<Arc<mlua::Lua>>,
    pub lua_ctx: Option<Arc<Mutex<crate::scripting::api::LuaContext>>>,
    pub logger: Logger,
    pub flood: FloodProtection,
    pub dcc: DccManager,
}

/// Wzorzec podświetlania
#[derive(Clone)]
pub struct HighlightPattern {
    pub pattern: String,
    pub color: String,
}

/// Kolory theme'a — używane przez renderer
#[derive(Clone)]
pub struct ThemeColors {
    pub name: String,
    pub status_bar_bg: Color,
    pub status_bar_fg: Color,
    pub topic_bar_bg: Color,
    pub topic_bar_fg: Color,
    pub input_fg: Color,
    pub border: Color,
    pub timestamp: Color,
    pub msg_normal: Color,
    pub msg_action: Color,
    pub msg_system: Color,
    pub msg_notice: Color,
    pub msg_highlight: Color,
    pub msg_error: Color,
    pub msg_server: Color,
    pub msg_ctcp: Color,
    pub nick_op: Color,
    pub nick_voice: Color,
    pub nick_halfop: Color,
    pub nick_founder: Color,
    pub nick_admin: Color,
    pub nick_normal: Color,
}

impl Default for ThemeColors {
    fn default() -> Self {
        ThemeColors {
            name: "Default".into(),
            status_bar_bg: Color::Green,
            status_bar_fg: Color::Black,
            topic_bar_bg: Color::Green,
            topic_bar_fg: Color::Black,
            input_fg: Color::LightGreen,
            border: Color::DarkGray,
            timestamp: Color::DarkGray,
            msg_normal: Color::Green,
            msg_action: Color::Yellow,
            msg_system: Color::Cyan,
            msg_notice: Color::Magenta,
            msg_highlight: Color::White,
            msg_error: Color::LightRed,
            msg_server: Color::DarkGray,
            msg_ctcp: Color::Red,
            nick_op: Color::Red,
            nick_voice: Color::Yellow,
            nick_halfop: Color::Cyan,
            nick_founder: Color::Magenta,
            nick_admin: Color::Red,
            nick_normal: Color::Green,
        }
    }
}

/// Kontekst wyjścia (epic6 /ON CONTEXT)
#[derive(Clone, Debug)]
pub struct OutputContext {
    pub server: String,
    pub window: String,
    pub sender: String,
    pub target: String,
    pub level: String,
}

impl Default for OutputContext {
    fn default() -> Self {
        OutputContext {
            server: String::new(),
            window: String::new(),
            sender: String::new(),
            target: String::new(),
            level: String::new(),
        }
    }
}

impl App {
    pub fn new(nick: &str, server: &str, port: u16, tls: bool, db_pass: &str) -> Self {
        let settings = Settings::new();
        let log_file = settings.get("LOG_FILE").to_string();
        let flood_rate = settings.get_int("FLOOD_RATE").max(1) as usize;
        let flood_per = settings.get_int("FLOOD_RATE_PER").max(1) as u64;
        let dcc_dir = settings.get("DCC_DOWNLOAD_DIR").to_string();
        let mut app = App {
            buffers: vec![Buffer::new("(Status)")],
            current_buffer_idx: 0,
            input_text: String::new(),
            input_history: Vec::new(),
            input_history_idx: None,
            input_cursor_pos: 0,
            servers: vec![ServerConnection::new(server, port, nick, tls)],
            active_server_idx: 0,
            settings,
            notify_list: Vec::new(),
            ignore_list: Vec::new(),
            timers: Vec::new(),
            last_msg_target: None,
            running: true,
            reconnect_pending: false,
            pending_exec: Vec::new(),
            last_buffer_idx: None,
            aliases: HashMap::new(),
            highlight_patterns: Vec::new(),
            key_bindings: HashMap::new(),
            label_counter: 0,
            pending_labels: HashMap::new(),
            format_templates: {
                let mut m = HashMap::new();
                m.insert("JOIN".into(), "* $0 has joined $1".into());
                m.insert("PART".into(), "* $0 has left $1 ($2)".into());
                m.insert("QUIT".into(), "* $0 has quit IRC ($1)".into());
                m.insert("KICK".into(), "* $0 was kicked from $1 by $2 ($3)".into());
                m.insert("NICK".into(), "* $0 is now known as $1".into());
                m.insert("MODE".into(), "* $0 sets mode: $1".into());
                m.insert("TOPIC".into(), "* $0 set topic to: $1".into());
                m.insert("MSG".into(), "<$0> $1".into());
                m.insert("ACTION".into(), "* $0 $1".into());
                m.insert("NOTICE".into(), "-$0- $1".into());
                m
            },
            theme_colors: ThemeColors::default(),
            split_buffer_idx: None,
            split_scroll_offset: 0,
            split_horizontal: false,
            suppress_display: false,
            output_context: OutputContext::default(),
            storage: Storage::open("~/.void/void.db", db_pass).ok(),
            lua_hooks: None,
            lua: None,
            lua_ctx: None,
            logger: Logger::new(&log_file),
            flood: FloodProtection::new(flood_rate, flood_per),
            dcc: DccManager::new(&dcc_dir),
        };
        // MOTD — losowy ASCII art logo
        for line in crate::motd::get_motd().lines() {
            if !line.is_empty() {
                app.system_message(line);
            }
        }
        app.load_config();
        app.load_from_db();
        app
    }

    // ─── Multi-server helpers ──────────────────────────

    pub fn server(&self) -> &ServerConnection {
        &self.servers[self.active_server_idx]
    }

    pub fn server_mut(&mut self) -> &mut ServerConnection {
        &mut self.servers[self.active_server_idx]
    }

    pub fn add_server(&mut self, host: &str, port: u16, nick: &str, tls: bool) -> usize {
        let idx = self.servers.len();
        self.servers.push(ServerConnection::new(host, port, nick, tls));
        idx
    }

    pub fn switch_server(&mut self, idx: usize) {
        if idx < self.servers.len() {
            self.active_server_idx = idx;
        }
    }

    /// Znajdź indeks serwera po hoście
    pub fn find_server(&self, host: &str) -> Option<usize> {
        self.servers.iter().position(|s| s.host == host)
    }

    /// Wczytaj konfigurację z ~/.void/void.conf
    fn load_config(&mut self) {
        let config_path = std::env::var("HOME")
            .map(|h| std::path::PathBuf::from(h).join(".void").join("void.conf"))
            .unwrap_or_else(|_| std::path::PathBuf::from(".void/void.conf"));

        let content = match std::fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(_) => return, // Brak pliku — pierwszy start
        };

        let mut section = "";
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if line == "[settings]" { section = "settings"; continue; }
            if line == "[aliases]" { section = "aliases"; continue; }

            if let Some((key, value)) = line.split_once('=') {
                match section {
                    "settings" => {
                        self.settings.set(key.trim(), value.trim());
                    }
                    "aliases" => {
                        self.aliases.insert(key.trim().to_uppercase(), value.trim().to_string());
                    }
                    _ => {}
                }
            }
        }
        self.system_message(&format!("-!- Loaded config from: {}", config_path.display()));
    }

    // ─── Zarządzanie buforami ──────────────────────────────

    pub fn current_buffer(&self) -> &Buffer {
        &self.buffers[self.current_buffer_idx]
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer_idx]
    }

    pub fn get_buffer(&self, name: &str) -> Option<&Buffer> {
        self.buffers.iter().find(|b| b.name == name)
    }

    pub fn get_buffer_mut(&mut self, name: &str) -> Option<&mut Buffer> {
        self.buffers.iter_mut().find(|b| b.name == name)
    }

    pub fn get_or_create_buffer(&mut self, name: &str) -> &mut Buffer {
        if !self.buffers.iter().any(|b| b.name == name) {
            self.buffers.push(Buffer::new(name));
        }
        self.buffers.iter_mut().find(|b| b.name == name).unwrap()
    }

    pub fn switch_to_buffer(&mut self, name: &str) {
        if let Some(idx) = self.buffers.iter().position(|b| b.name == name) {
            self.last_buffer_idx = Some(self.current_buffer_idx);
            self.current_buffer_idx = idx;
            self.buffers[idx].unread_count = 0;
            self.buffers[idx].has_activity = false;
            self.sync_lua_context();
        }
    }

    pub fn next_buffer(&mut self) {
        if !self.buffers.is_empty() {
            self.current_buffer_idx = (self.current_buffer_idx + 1) % self.buffers.len();
            self.buffers[self.current_buffer_idx].unread_count = 0;
            self.buffers[self.current_buffer_idx].has_activity = false;
            self.sync_lua_context();
        }
    }

    pub fn prev_buffer(&mut self) {
        if !self.buffers.is_empty() {
            self.current_buffer_idx = if self.current_buffer_idx == 0 {
                self.buffers.len() - 1
            } else {
                self.current_buffer_idx - 1
            };
            self.buffers[self.current_buffer_idx].unread_count = 0;
            self.buffers[self.current_buffer_idx].has_activity = false;
            self.sync_lua_context();
        }
    }

    /// Synchronizuj LuaContext z aktualnym stanem App
    pub fn sync_lua_context(&self) {
        if let Some(ref ctx) = self.lua_ctx {
            if let Ok(mut c) = ctx.lock() {
                c.our_nick = self.server().our_nick.clone();
                c.current_channel = self.buffers[self.current_buffer_idx].name.clone();
                c.server_host = self.server().host.clone();
                c.connected = self.server().connected;
                c.settings = self.settings.map.clone();
            }
        }
    }

    pub fn close_buffer(&mut self, name: &str) {
        if name == "(Status)" { return; }
        self.buffers.retain(|b| b.name != name);
        if self.current_buffer_idx >= self.buffers.len() {
            self.current_buffer_idx = self.buffers.len().saturating_sub(1);
        }
    }

    // ─── Wiadomości ──────────────────────────────────────

    pub fn system_message(&mut self, text: &str) {
        self.logger.write_line("(Status)", text);
        let limit = self.settings.get_int("SCROLLBACK").max(100) as usize;
        let buf = self.get_or_create_buffer("(Status)");
        buf.push_message(text.to_string(), MessageType::System, limit);
    }

    pub fn buffer_message(&mut self, buffer: &str, text: String, msg_type: MessageType) {
        self.logger.write_line(buffer, &text);

        // /SHH — wycisz wyświetlanie w aktualnym kontekście
        if self.suppress_display {
            self.suppress_display = false;
            return;
        }

        let limit = self.settings.get_int("SCROLLBACK").max(100) as usize;
        let current_name = self.buffers[self.current_buffer_idx].name.clone();

        // Sprawdź highlight — wiadomość zawiera nasz nick
        let effective_type = if msg_type == MessageType::Normal || msg_type == MessageType::Action {
            let nick_lower = self.server().our_nick.to_lowercase();
            let text_lower = text.to_lowercase();
            if !nick_lower.is_empty() && text_lower.contains(&nick_lower) {
                MessageType::Highlight
            } else {
                msg_type
            }
        } else {
            msg_type
        };

        let buf = self.get_or_create_buffer(buffer);
        buf.push_message(text, effective_type.clone(), limit);
        if buf.name != current_name {
            buf.unread_count += 1;
            buf.has_activity = true;
            // Beep na highlight
            if effective_type == MessageType::Highlight && self.settings.get_bool("BEEP_ON_MSG") {
                eprint!("\x07");
            }
        }
    }

    // ─── Input history ──────────────────────────────────

    pub fn push_input_history(&mut self) {
        if !self.input_text.is_empty() {
            self.input_history.push(self.input_text.clone());
            if self.input_history.len() > 100 {
                self.input_history.remove(0);
            }
        }
        self.input_history_idx = None;
    }

    pub fn history_prev(&mut self) {
        if self.input_history.is_empty() { return; }
        match self.input_history_idx {
            None => {
                self.input_history_idx = Some(self.input_history.len() - 1);
            }
            Some(idx) if idx > 0 => {
                self.input_history_idx = Some(idx - 1);
            }
            _ => return,
        }
        if let Some(idx) = self.input_history_idx {
            self.input_text = self.input_history[idx].clone();
            self.input_cursor_pos = self.input_text.len();
        }
    }

    pub fn history_next(&mut self) {
        match self.input_history_idx {
            Some(idx) if idx < self.input_history.len() - 1 => {
                self.input_history_idx = Some(idx + 1);
                self.input_text = self.input_history[idx + 1].clone();
                self.input_cursor_pos = self.input_text.len();
            }
            Some(_) => {
                self.input_history_idx = None;
                self.input_text.clear();
                self.input_cursor_pos = 0;
            }
            None => {}
        }
    }

    // ─── Ignore ──────────────────────────────────────

    pub fn is_ignored(&self, nick: &str, msg_type: &str) -> bool {
        for entry in &self.ignore_list {
            let pattern_lower = entry.pattern.to_lowercase();
            let nick_lower = nick.to_lowercase();
            let matches = if pattern_lower.contains('*') {
                let parts: Vec<&str> = pattern_lower.split('*').collect();
                if parts.len() == 2 {
                    nick_lower.starts_with(parts[0]) && nick_lower.ends_with(parts[1])
                } else {
                    nick_lower.contains(&pattern_lower.replace('*', ""))
                }
            } else {
                nick_lower == pattern_lower
            };

            if matches {
                if entry.ignore_all { return true; }
                match msg_type {
                    "PUBLIC" if entry.ignore_public => return true,
                    "MSG" if entry.ignore_private => return true,
                    "NOTICE" if entry.ignore_notice => return true,
                    "CTCP" if entry.ignore_ctcp => return true,
                    _ => {}
                }
            }
        }
        false
    }

    // ─── Notify ──────────────────────────────────────

    pub fn is_on_notify(&self, nick: &str) -> bool {
        self.notify_list.iter().any(|n| n.nick.to_lowercase() == nick.to_lowercase())
    }

    // ─── Aliasy i ekspansja zmiennych ──────────────────

    /// Rozwiń zmienne epic5-style w tekście aliasu
    /// $0-$9 = argumenty, $* = wszystkie, $N = nick, $C = kanał, $S = serwer
    pub fn expand_variables(&self, template: &str, args: &[&str]) -> String {
        let mut result = String::with_capacity(template.len());
        let mut chars = template.chars().peekable();
        while let Some(c) = chars.next() {
            if c == '$' {
                match chars.next() {
                    Some('*') => {
                        result.push_str(&args.join(" "));
                    }
                    Some(d @ '0'..='9') => {
                        let idx = (d as usize) - ('0' as usize);
                        if idx < args.len() {
                            result.push_str(args[idx]);
                        }
                    }
                    Some('N') => result.push_str(&self.server().our_nick),
                    Some('C') => {
                        let buf = &self.buffers[self.current_buffer_idx];
                        if buf.name != "(Status)" { result.push_str(&buf.name); }
                    }
                    Some('S') => result.push_str(&self.server().host),
                    Some('$') => result.push('$'),
                    Some(other) => {
                        result.push('$');
                        result.push(other);
                    }
                    None => result.push('$'),
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    /// Sprawdź czy komenda to alias i rozwiń. Zwraca Some(rozwinięty_tekst) lub None.
    pub fn resolve_alias(&self, cmd_name: &str, args: &[&str]) -> Option<String> {
        let upper = cmd_name.to_uppercase();
        self.aliases.get(&upper).map(|template| {
            self.expand_variables(template, args)
        })
    }

    /// Generuj unikalny label dla labeled-response
    pub fn next_label(&mut self) -> String {
        self.label_counter += 1;
        format!("v{}", self.label_counter)
    }

    /// Zastosuj theme z Lua
    pub fn apply_theme(&mut self, theme_name: &str) {
        // Mapuj nazwy kolorów na ratatui Color
        let parse_color = |s: &str| -> Color {
            match s.to_lowercase().as_str() {
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "yellow" => Color::Yellow,
                "blue" => Color::Blue,
                "magenta" | "purple" => Color::Magenta,
                "cyan" => Color::Cyan,
                "white" => Color::White,
                "dark_gray" | "darkgray" | "gray" => Color::DarkGray,
                "light_red" | "lightred" => Color::LightRed,
                "light_green" | "lightgreen" => Color::LightGreen,
                "light_yellow" | "lightyellow" => Color::LightYellow,
                "light_blue" | "lightblue" => Color::LightBlue,
                "light_magenta" | "lightmagenta" => Color::LightMagenta,
                "light_cyan" | "lightcyan" => Color::LightCyan,
                _ => Color::White,
            }
        };

        // Czytaj theme z Lua global table
        if let Some(ref lua) = self.lua {
            if let Ok(themes_table) = lua.globals().get::<mlua::Table>("void_themes") {
                if let Ok(theme_table) = themes_table.get::<mlua::Table>(theme_name.to_lowercase()) {
                    // Czytaj ui colors
                    if let Ok(ui) = theme_table.get::<mlua::Table>("ui") {
                        if let Ok(v) = ui.get::<String>("status_bar_bg") { self.theme_colors.status_bar_bg = parse_color(&v); }
                        if let Ok(v) = ui.get::<String>("status_bar_fg") { self.theme_colors.status_bar_fg = parse_color(&v); }
                        if let Ok(v) = ui.get::<String>("topic_bar_bg") { self.theme_colors.topic_bar_bg = parse_color(&v); }
                        if let Ok(v) = ui.get::<String>("topic_bar_fg") { self.theme_colors.topic_bar_fg = parse_color(&v); }
                        if let Ok(v) = ui.get::<String>("input_fg") { self.theme_colors.input_fg = parse_color(&v); }
                        if let Ok(v) = ui.get::<String>("border") { self.theme_colors.border = parse_color(&v); }
                        if let Ok(v) = ui.get::<String>("timestamp") { self.theme_colors.timestamp = parse_color(&v); }
                    }
                    // Czytaj message colors
                    if let Ok(msgs) = theme_table.get::<mlua::Table>("messages") {
                        if let Ok(v) = msgs.get::<String>("normal") { self.theme_colors.msg_normal = parse_color(&v); }
                        if let Ok(v) = msgs.get::<String>("action") { self.theme_colors.msg_action = parse_color(&v); }
                        if let Ok(v) = msgs.get::<String>("system") { self.theme_colors.msg_system = parse_color(&v); }
                        if let Ok(v) = msgs.get::<String>("notice") { self.theme_colors.msg_notice = parse_color(&v); }
                        if let Ok(v) = msgs.get::<String>("highlight") { self.theme_colors.msg_highlight = parse_color(&v); }
                        if let Ok(v) = msgs.get::<String>("error") { self.theme_colors.msg_error = parse_color(&v); }
                        if let Ok(v) = msgs.get::<String>("server") { self.theme_colors.msg_server = parse_color(&v); }
                        if let Ok(v) = msgs.get::<String>("ctcp") { self.theme_colors.msg_ctcp = parse_color(&v); }
                    }
                    // Czytaj nick colors
                    if let Ok(nicks) = theme_table.get::<mlua::Table>("nicks") {
                        if let Ok(v) = nicks.get::<String>("op") { self.theme_colors.nick_op = parse_color(&v); }
                        if let Ok(v) = nicks.get::<String>("voice") { self.theme_colors.nick_voice = parse_color(&v); }
                        if let Ok(v) = nicks.get::<String>("halfop") { self.theme_colors.nick_halfop = parse_color(&v); }
                        if let Ok(v) = nicks.get::<String>("founder") { self.theme_colors.nick_founder = parse_color(&v); }
                        if let Ok(v) = nicks.get::<String>("admin") { self.theme_colors.nick_admin = parse_color(&v); }
                        if let Ok(v) = nicks.get::<String>("normal") { self.theme_colors.nick_normal = parse_color(&v); }
                    }
                    self.theme_colors.name = theme_name.to_string();
                }
            }
        }
    }

    /// Rozwiń szablon formatu wiadomości (np. "JOIN" → "* $0 has joined $1")
    pub fn format_message(&self, fmt_type: &str, args: &[&str]) -> Option<String> {
        self.format_templates.get(&fmt_type.to_uppercase()).map(|template| {
            self.expand_variables(template, args)
        })
    }

    // ─── SQLite persistence ──────────────────────────

    /// Załaduj dane z SQLite do App
    pub fn load_from_db(&mut self) {
        let storage = match &self.storage {
            Some(s) => s,
            None => return,
        };

        // Załaduj ustawienia
        for (key, value) in storage.get_all_settings() {
            self.settings.set(&key, &value);
        }

        // Załaduj aliasy
        for (name, body) in storage.get_all_aliases() {
            self.aliases.insert(name, body);
        }

        // Załaduj highlight patterns
        for (pattern, color) in storage.get_all_highlights() {
            self.highlight_patterns.push(HighlightPattern { pattern, color });
        }

        // Załaduj key bindings
        for (key, action) in storage.get_all_key_bindings() {
            self.key_bindings.insert(key, action);
        }

        // Załaduj notify list
        for nick in storage.get_all_notify() {
            self.notify_list.push(crate::app::NotifyEntry {
                nick,
                online: false,
                last_seen: None,
            });
        }

        // Załaduj ignore list
        for (pattern, flags) in storage.get_all_ignore() {
            self.ignore_list.push(crate::app::IgnoreEntry {
                pattern: pattern.clone(),
                ignore_all: flags.contains("ALL"),
                ignore_public: flags.contains("PUBLIC") || flags.contains("ALL"),
                ignore_private: flags.contains("MSG") || flags.contains("ALL"),
                ignore_notice: flags.contains("NOTICE") || flags.contains("ALL"),
                ignore_ctcp: flags.contains("CTCP") || flags.contains("ALL"),
            });
        }
    }

    /// Zapisz aktualny stan do SQLite
    pub fn save_to_db(&self) {
        let storage = match &self.storage {
            Some(s) => s,
            None => return,
        };

        // Zapisz ustawienia
        for (key, value) in &self.settings.map {
            let _ = storage.set_setting(key, value);
        }

        // Zapisz aliasy
        for (name, body) in &self.aliases {
            let _ = storage.set_alias(name, body);
        }

        // Zapisz highlight patterns
        for h in &self.highlight_patterns {
            let _ = storage.add_highlight(&h.pattern, &h.color);
        }

        // Zapisz key bindings
        for (key, action) in &self.key_bindings {
            let _ = storage.set_key_binding(key, action);
        }

        // Zapisz notify list
        for n in &self.notify_list {
            let _ = storage.add_notify(&n.nick);
        }

        // Zapisz ignore list
        for entry in &self.ignore_list {
            let mut flags = Vec::new();
            if entry.ignore_all { flags.push("ALL"); }
            if entry.ignore_public { flags.push("PUBLIC"); }
            if entry.ignore_private { flags.push("MSG"); }
            if entry.ignore_notice { flags.push("NOTICE"); }
            if entry.ignore_ctcp { flags.push("CTCP"); }
            let _ = storage.add_ignore(&entry.pattern, &flags.join(","));
        }
    }
}
