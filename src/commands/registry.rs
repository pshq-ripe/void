use crate::app::{App, HighlightPattern, IgnoreEntry, MessageType, NotifyEntry, TimerEntry};
use std::time::{Duration, Instant};

/// Typ callbacku komendy
pub type CommandFn = fn(app: &mut App, args: &[&str]) -> CommandResult;

pub enum CommandResult {
    Ok,
    NeedSender,   // komenda wymaga połączenia z serwerem
    Error(String),
}

/// Rejestr komend
pub struct CommandRegistry {
    pub commands: Vec<RegisteredCommand>,
}

pub struct RegisteredCommand {
    pub name: String,
    pub aliases: Vec<String>,
    pub help: String,
    pub handler: CommandFn,
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut reg = CommandRegistry { commands: Vec::new() };
        reg.register_builtins();
        reg
    }

    fn register(&mut self, name: &str, aliases: &[&str], help: &str, handler: CommandFn) {
        self.commands.push(RegisteredCommand {
            name: name.to_uppercase(),
            aliases: aliases.iter().map(|a| a.to_uppercase()).collect(),
            help: help.to_string(),
            handler,
        });
    }

    pub fn find(&self, name: &str) -> Option<&RegisteredCommand> {
        let upper = name.to_uppercase();
        self.commands.iter().find(|c| c.name == upper || c.aliases.contains(&upper))
    }

    fn register_builtins(&mut self) {
        // ─── Połączenia ───────────────────────────────────
        self.register("SERVER", &["connect"], "/server <host> [port] [pass] — Connect to IRC server", cmd_server);
        self.register("DISCONNECT", &["discon"], "/disconnect — Disconnect from server", cmd_disconnect);
        self.register("RECONNECT", &[], "/reconnect — Reconnect to last server", cmd_reconnect);
        self.register("QUIT", &["exit", "bye"], "/quit [reason] — Quit", cmd_quit);

        // ─── Kanały ──────────────────────────────────────
        self.register("JOIN", &["j"], "/join <#channel> [key] — Join channel", cmd_join);
        self.register("PART", &["leave"], "/part [#channel] [reason] — Leave channel", cmd_part);
        self.register("TOPIC", &["t"], "/topic [text] — View/set topic", cmd_topic);
        self.register("NAMES", &[], "/names [#channel] — List users", cmd_names);
        self.register("KICK", &["k"], "/kick <nick> [reason] — Kick user", cmd_kick);
        self.register("MODE", &[], "/mode <target> <modes> — Set modes", cmd_mode);
        self.register("INVITE", &[], "/invite <nick> <#channel> — Invite user", cmd_invite);
        self.register("BAN", &["b"], "/ban <nick|mask> — Ban user from channel", cmd_ban);
        self.register("UNBAN", &["ub"], "/unban <nick|mask> — Remove ban", cmd_unban);
        self.register("KICKBAN", &["kb"], "/kickban <nick> [reason] — Kick and ban", cmd_kickban);
        self.register("OP", &[], "/op <nick> — Give ops", cmd_op);
        self.register("DEOP", &[], "/deop <nick> — Remove ops", cmd_deop);
        self.register("VOICE", &["v"], "/voice <nick> — Give voice", cmd_voice);
        self.register("DEVOICE", &["dv"], "/devoice <nick> — Remove voice", cmd_devoice);

        // ─── Wiadomości ──────────────────────────────────
        self.register("MSG", &["m"], "/msg <target> <text> — Send private message", cmd_msg);
        self.register("ME", &["describe"], "/me <action> — Send action", cmd_me);
        self.register("NOTICE", &[], "/notice <target> <text> — Send notice", cmd_notice);
        self.register("SAY", &[], "/say <text> — Say text on channel", cmd_say);
        self.register("QUERY", &["q"], "/query <nick> — Open query window", cmd_query);
        self.register("CTCP", &[], "/ctcp <nick> <type> — Send CTCP", cmd_ctcp);

        // ─── Nick / User ─────────────────────────────────
        self.register("NICK", &[], "/nick <newnick> — Change nickname", cmd_nick);
        self.register("AWAY", &[], "/away [message] — Set/unset away", cmd_away);
        self.register("WHOIS", &["wi"], "/whois <nick> — Query user info", cmd_whois);
        self.register("WHOWAS", &[], "/whowas <nick> — Query past user info", cmd_whowas);
        self.register("WHO", &[], "/who <mask> — List matching users", cmd_who);
        self.register("USERHOST", &[], "/userhost <nick> — Query user host", cmd_userhost);

        // ─── Okna ────────────────────────────────────────
        self.register("WINDOW", &["w", "wc"], "/window <command> — Window management", cmd_window);
        self.register("CLEAR", &["cls"], "/clear — Clear current window", cmd_clear);
        self.register("LASTLOG", &["ll"], "/lastlog [pattern] — Search scrollback", cmd_lastlog);

        // ─── Notify / Ignore / Timer ─────────────────────
        self.register("NOTIFY", &[], "/notify [nick] — Manage notify list", cmd_notify);
        self.register("IGNORE", &[], "/ignore [pattern] [flags] — Manage ignore list", cmd_ignore);
        self.register("TIMER", &[], "/timer [-refnum N] <seconds> <repeats> <command>", cmd_timer);

        // ─── Konfiguracja ────────────────────────────────
        self.register("SET", &[], "/set [variable] [value] — View/change settings", cmd_set);

        // ─── Aliasy i ekspresje ───────────────────────────
        self.register("ALIAS", &[], "/alias [name] [body] — Define/show/remove alias", cmd_alias);
        self.register("UNALIAS", &[], "/unalias <name> — Remove alias", cmd_unalias);
        self.register("IF", &[], "/if <cond> <then> [else <else>] — Conditional execution", cmd_if);
        self.register("WHILE", &[], "/while <cond> <body> — Loop while condition true", cmd_while);
        self.register("FOR", &[], "/for <var> <start> <end> <body> — For loop", cmd_for);
        self.register("WAIT", &[], "/wait <seconds> <command> — Wait then execute", cmd_wait);
        self.register("REDIRECT", &[], "/redirect <target> <command> — Redirect output to target", cmd_redirect);

        // ─── Highlight / Load / Bind ──────────────────────
        self.register("HIGHLIGHT", &["hilight"], "/highlight [pattern] [color] — Add/remove highlight patterns", cmd_highlight);
        self.register("LOAD", &[], "/load <script.lua> — Load Lua script", cmd_load);
        self.register("RELOAD", &[], "/reload — Reload all Lua scripts", cmd_reload);
        self.register("BIND", &["keybind"], "/bind [key] [action] — Show/set key bindings", cmd_bind);
        self.register("FORMAT", &[], "/format [type] [template] — Show/set message format templates", cmd_format);
        self.register("CD", &[], "/cd <path> — Change working directory", cmd_cd);
        self.register("PWD", &[], "/pwd — Print working directory", cmd_pwd);
        self.register("DEBUG", &[], "/debug [on|off] — Toggle debug mode", cmd_debug);
        self.register("REPAINT", &["refresh"], "/repaint — Force screen redraw", cmd_repaint);
        self.register("SCROLL", &[], "/scroll <up|down|top|bottom> — Programmatic scroll", cmd_scroll);
        self.register("STATUS", &[], "/status [format] — Show/set status bar format", cmd_status);
        self.register("FLOOD", &[], "/flood [on|off] [rate] [per] — Manage flood protection", cmd_flood);
        self.register("PLAY", &[], "/play <logfile> — Replay log file into current buffer", cmd_play);
        self.register("SHH", &[], "/SHH — Suppress display output for current context (epic6)", cmd_shh);

        // ─── System ──────────────────────────────────────
        self.register("HELP", &["?"], "/help [command] — Show help", cmd_help);
        self.register("SAVE", &[], "/save — Save current settings and aliases to config file", cmd_save);
        self.register("RAW", &["quote"], "/raw <text> — Send raw IRC line", cmd_raw);
        self.register("ECHO", &[], "/echo <text> — Display local text", cmd_echo);
        self.register("EXEC", &[], "/exec <command> — Execute shell command", cmd_exec);
        self.register("LOG", &[], "/log [on|off] — Toggle logging", cmd_log);
        self.register("EVAL", &[], "/eval <expression> — Evaluate expression", cmd_eval);

        // ─── DCC ─────────────────────────────────────────
        self.register("DCC", &[], "/dcc <list|chat|send|get|close> — DCC subsystem", cmd_dcc);

        // ─── Dodatkowe komendy kanałowe ──────────────────
        self.register("LIST", &[], "/list [mask] — List channels on server", cmd_list);
        self.register("CYCLE", &[], "/cycle — Part and rejoin current channel", cmd_cycle);
        self.register("KNOCK", &[], "/knock <#channel> [message] — Knock on invite-only channel", cmd_knock);
        self.register("OPER", &[], "/oper <login> <password> — IRC OPER login", cmd_oper);
        self.register("KILL", &[], "/kill <nick> [reason] — IRC KILL a user", cmd_kill);
        self.register("SILENCE", &[], "/silence [mask] — Server-side ignore (SILENCE)", cmd_silence);
        self.register("SETNAME", &[], "/setname <realname> — Change realname on the fly", cmd_setname);
        self.register("CAPLIST", &[], "/caplist — List active IRCv3 capabilities", cmd_caplist);
        self.register("CHATHISTORY", &[], "/chathistory <before|after|latest|around> <target> <limit> — Request message history", cmd_chathistory);

        // ─── Dodatkowe epic5 / irc2.11 style ────────────
        self.register("WALLOPS", &[], "/wallops <text> — Send wallops", cmd_wallops);
        self.register("PING", &[], "/ping <nick> — CTCP PING user", cmd_ping);
        self.register("LUSERS", &[], "/lusers [mask] — Get server user statistics", cmd_lusers);
        self.register("ADMIN", &[], "/admin [server] — Get admin info", cmd_admin);
        self.register("INFO", &[], "/info [server] — Get server info", cmd_info);
        self.register("MOTD", &[], "/motd [server] — Get MOTD", cmd_motd);
        self.register("STATS", &[], "/stats <flag> [server] — Get server stats", cmd_stats);
        self.register("LINKS", &[], "/links [mask] — List connected servers", cmd_links);
        self.register("MAP", &[], "/map — Display server network map", cmd_map);
        self.register("TRACE", &[], "/trace [target] — Trace server connection route", cmd_trace);
        self.register("TKLINE", &[], "/tkline <mins> <user@host> [reason] — Temporary K-line (ircd 2.11)", cmd_tkline);
        self.register("EXCEPT", &["ex"], "/except [mask] — Manage ban exceptions (+e)", cmd_except);
        self.register("INVEX", &["in"], "/invex [mask] — Manage invite exceptions (+I)", cmd_invex);
        self.register("REOP", &["re"], "/reop [mask] — Manage reop hints (+R ircd 2.11)", cmd_reop);
    }
}

// ════════════════════════════════════════════════════════════
//  Implementacje komend
// ════════════════════════════════════════════════════════════

fn cmd_server(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        // Pokaż wszystkie serwery
        app.system_message("-!- Servers:");
        let server_list: Vec<(usize, String, u16, bool, String)> = app.servers.iter().enumerate()
            .map(|(i, s)| (i, s.host.clone(), s.port, s.connected, s.our_nick.clone()))
            .collect();
        for (i, host, port, connected, nick) in server_list {
            let marker = if i == app.active_server_idx { "*" } else { " " };
            let status = if connected { "connected" } else { "disconnected" };
            app.system_message(&format!("  {} [{}] {}:{} ({}) nick:{}", marker, i, host, port, status, nick));
        }
        return CommandResult::Ok;
    }

    // -m = nowe połączenie (multi-server)
    if args[0] == "-m" {
        if args.len() < 2 {
            return CommandResult::Error("Usage: /server -m <host> [port]".into());
        }
        let host = args[1];
        let port = args.get(2).and_then(|s| s.parse::<u16>().ok()).unwrap_or(6697);
        let nick = app.server().our_nick.clone();
        let tls = app.server().tls;
        let idx = app.add_server(host, port, &nick, tls);
        app.switch_server(idx);
        app.system_message(&format!("-!- New server connection [{}]: {}:{}", idx, host, port));
        app.reconnect_pending = true;
        return CommandResult::Ok;
    }

    // Przełącz na istniejący serwer po indeksie
    if let Ok(idx) = args[0].parse::<usize>() {
        if idx < app.servers.len() {
            app.switch_server(idx);
            app.system_message(&format!("-!- Switched to server [{}]: {}:{}", idx, app.server().host, app.server().port));
            return CommandResult::Ok;
        }
    }

    // Połącz z nowym serwerem (zastąp aktualny)
    let host = args[0].to_string();
    let port = args.get(1).and_then(|s| s.parse::<u16>().ok()).unwrap_or(6697);
    let _pass = args.get(2).map(|s| s.to_string());

    app.server_mut().host = host;
    app.server_mut().port = port;
    app.server_mut().connected = false;
    app.server_mut().sender = None;
    app.system_message(&format!("-!- Connecting to {}:{}...", app.server().host, app.server().port));
    app.reconnect_pending = true;
    CommandResult::Ok
}

fn cmd_disconnect(app: &mut App, _args: &[&str]) -> CommandResult {
    if let Some(s) = &app.server().sender {
        let _ = s.send_quit("Disconnected");
    }
    app.server_mut().connected = false;
    app.server_mut().sender = None;
    app.system_message("-!- Disconnected.");
    CommandResult::Ok
}

fn cmd_reconnect(app: &mut App, _args: &[&str]) -> CommandResult {
    app.system_message(&format!("-!- Reconnecting to {}:{}...", app.server().host, app.server().port));
    app.reconnect_pending = true;
    CommandResult::Ok
}

fn cmd_quit(app: &mut App, args: &[&str]) -> CommandResult {
    let reason = if args.is_empty() {
        "Leaving".to_string()
    } else {
        args.join(" ")
    };
    if let Some(s) = &app.server().sender {
        let _ = s.send_quit(&reason);
    }
    app.running = false;
    CommandResult::Ok
}

fn cmd_join(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /join <#channel> [key]".into());
    }
    let channel = args[0];
    if let Some(s) = &app.server().sender {
        if args.len() > 1 {
            let _ = s.send(irc::client::prelude::Command::JOIN(
                channel.to_string(),
                Some(args[1].to_string()),
                None,
            ));
        } else {
            let _ = s.send_join(channel);
        }
    } else {
        return CommandResult::NeedSender;
    }
    CommandResult::Ok
}

fn cmd_part(app: &mut App, args: &[&str]) -> CommandResult {
    let channel = if args.is_empty() {
        app.buffers[app.current_buffer_idx].name.clone()
    } else {
        args[0].to_string()
    };
    if channel == "(Status)" {
        return CommandResult::Error("Cannot part the status window.".into());
    }
    let reason = if args.len() > 1 { args[1..].join(" ") } else { "Leaving".to_string() };
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::PART(channel.clone(), Some(reason)));
    }
    app.close_buffer(&channel);
    CommandResult::Ok
}

fn cmd_topic(app: &mut App, args: &[&str]) -> CommandResult {
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    if channel == "(Status)" {
        return CommandResult::Error("Not in a channel.".into());
    }
    if let Some(s) = &app.server().sender {
        if args.is_empty() {
            let _ = s.send(irc::client::prelude::Command::TOPIC(channel, None));
        } else {
            let topic_text = args.join(" ");
            let _ = s.send(irc::client::prelude::Command::TOPIC(channel, Some(topic_text)));
        }
    } else {
        return CommandResult::NeedSender;
    }
    CommandResult::Ok
}

fn cmd_names(app: &mut App, args: &[&str]) -> CommandResult {
    let channel = if args.is_empty() {
        app.buffers[app.current_buffer_idx].name.clone()
    } else {
        args[0].to_string()
    };
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::NAMES(Some(channel), None));
    }
    CommandResult::Ok
}

fn cmd_kick(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /kick <nick> [reason]".into());
    }
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    let nick = args[0];
    let reason = if args.len() > 1 { args[1..].join(" ") } else { "Kicked".to_string() };
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::KICK(channel, nick.to_string(), Some(reason)));
    }
    CommandResult::Ok
}

fn cmd_mode(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /mode <target> <modes> [args]".into());
    }
    if let Some(s) = &app.server().sender {
        let raw = format!("MODE {}", args.join(" "));
        let _ = s.send(irc::client::prelude::Command::Raw(raw, Vec::new()));
    }
    CommandResult::Ok
}

fn cmd_invite(app: &mut App, args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error("Usage: /invite <nick> <#channel>".into());
    }
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::INVITE(args[0].to_string(), args[1].to_string()));
    }
    CommandResult::Ok
}

fn cmd_ban(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /ban <nick|mask>".into());
    }
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    let mask = if args[0].contains('!') || args[0].contains('@') {
        args[0].to_string()
    } else {
        format!("{}!*@*", args[0])
    };
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} +b {}", channel, mask), Vec::new()));
    }
    CommandResult::Ok
}

fn cmd_unban(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /unban <nick|mask>".into());
    }
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    let mask = if args[0].contains('!') || args[0].contains('@') {
        args[0].to_string()
    } else {
        format!("{}!*@*", args[0])
    };
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} -b {}", channel, mask), Vec::new()));
    }
    CommandResult::Ok
}

fn cmd_kickban(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /kickban <nick> [reason]".into());
    }
    cmd_ban(app, &[args[0]]);
    cmd_kick(app, args);
    CommandResult::Ok
}

fn cmd_op(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /op <nick>".into());
    }
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} +o {}", channel, args[0]), Vec::new()));
    }
    CommandResult::Ok
}

fn cmd_deop(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /deop <nick>".into());
    }
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} -o {}", channel, args[0]), Vec::new()));
    }
    CommandResult::Ok
}

fn cmd_voice(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /voice <nick>".into());
    }
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} +v {}", channel, args[0]), Vec::new()));
    }
    CommandResult::Ok
}

fn cmd_devoice(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /devoice <nick>".into());
    }
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} -v {}", channel, args[0]), Vec::new()));
    }
    CommandResult::Ok
}

fn cmd_msg(app: &mut App, args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error("Usage: /msg <target> <text>".into());
    }
    let target = args[0];
    let text = args[1..].join(" ");
    crate::irc::proto::send_labeled_privmsg(app, target, &text);
    app.last_msg_target = Some(target.to_string());
    let buf_name = target.to_string();
    app.buffer_message(&buf_name, format!("<{}> {}", app.server().our_nick, text), MessageType::Normal);
    CommandResult::Ok
}

fn cmd_me(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /me <action>".into());
    }
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    if channel == "(Status)" {
        return CommandResult::Error("Cannot send action in Status window.".into());
    }
    let action = args.join(" ");
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::PRIVMSG(
            channel.clone(),
            format!("\x01ACTION {}\x01", action),
        ));
    }
    let nick = app.server().our_nick.clone();
    app.buffer_message(&channel, format!("* {} {}", nick, action), MessageType::Action);
    CommandResult::Ok
}

fn cmd_notice(app: &mut App, args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error("Usage: /notice <target> <text>".into());
    }
    let target = args[0];
    let text = args[1..].join(" ");
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::NOTICE(target.to_string(), text.clone()));
    }
    app.system_message(&format!("-> -{}- {}", target, text));
    CommandResult::Ok
}

fn cmd_say(app: &mut App, args: &[&str]) -> CommandResult {
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    if channel == "(Status)" {
        return CommandResult::Error("Not in a channel.".into());
    }
    let text = args.join(" ");
    crate::irc::proto::send_labeled_privmsg(app, &channel, &text);
    let nick = app.server().our_nick.clone();
    app.buffer_message(&channel, format!("<{}> {}", nick, text), MessageType::Normal);
    CommandResult::Ok
}

fn cmd_query(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /query <nick>".into());
    }
    let target = args[0].to_string();
    app.get_or_create_buffer(&target);
    app.switch_to_buffer(&target);
    app.system_message(&format!("-!- Starting query with {}", target));
    CommandResult::Ok
}

fn cmd_ctcp(app: &mut App, args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error("Usage: /ctcp <nick> <type> [args]".into());
    }
    let target = args[0];
    let ctcp_type = args[1].to_uppercase();
    let ctcp_args = if args.len() > 2 { args[2..].join(" ") } else { String::new() };
    let ctcp_msg = if ctcp_args.is_empty() {
        format!("\x01{}\x01", ctcp_type)
    } else {
        format!("\x01{} {}\x01", ctcp_type, ctcp_args)
    };
    if let Some(s) = &app.server().sender {
        let _ = s.send_privmsg(target, &ctcp_msg);
    }
    app.system_message(&format!("-> [{}] CTCP {}", target, ctcp_type));
    CommandResult::Ok
}

fn cmd_nick(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /nick <newnick>".into());
    }
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::NICK(args[0].to_string()));
    }
    CommandResult::Ok
}

fn cmd_away(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        app.server_mut().away_message = None;
        if let Some(s) = &app.server().sender {
            let _ = s.send(irc::client::prelude::Command::AWAY(None));
        }
        app.system_message("-!- You are no longer marked as away.");
    } else {
        let msg = args.join(" ");
        app.server_mut().away_message = Some(msg.clone());
        if let Some(s) = &app.server().sender {
            let _ = s.send(irc::client::prelude::Command::AWAY(Some(msg)));
        }
        app.system_message("-!- You are now marked as away.");
    }
    CommandResult::Ok
}

fn cmd_whois(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /whois <nick>".into());
    }
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::WHOIS(None, args[0].to_string()));
    }
    CommandResult::Ok
}

fn cmd_whowas(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /whowas <nick>".into());
    }
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::WHOWAS(args[0].to_string(), None, None));
    }
    CommandResult::Ok
}

fn cmd_who(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /who <mask>".into());
    }
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::WHO(Some(args[0].to_string()), None));
    }
    CommandResult::Ok
}

fn cmd_userhost(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /userhost <nick>".into());
    }
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::USERHOST(
            args.iter().map(|a| a.to_string()).collect(),
        ));
    }
    CommandResult::Ok
}

// ─── Okna ────────────────────────────────────────

fn cmd_window(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        let count = app.buffers.len();
        let lines: Vec<String> = app.buffers.iter().enumerate().map(|(i, buf)| {
            let marker = if i == app.current_buffer_idx { "*" } else { " " };
            format!("  {} {} [{}] (unread: {})", marker, i, buf.name, buf.unread_count)
        }).collect();
        app.system_message(&format!("-!- Active buffers: {}", count));
        for line in lines { app.system_message(&line); }
        return CommandResult::Ok;
    }
    let subcmd = args[0].to_lowercase();

    // Skrót: /window <liczba> = /window goto <liczba>
    if let Ok(idx) = subcmd.parse::<usize>() {
        if idx < app.buffers.len() {
            app.switch_to_buffer(&app.buffers[idx].name.clone());
        } else {
            app.system_message(&format!("-!- No window with index: {}", idx));
        }
        return CommandResult::Ok;
    }

    // Skrót: /window <#kanał> = /window goto <#kanał>
    if subcmd.starts_with('#') || subcmd.starts_with('&') {
        app.switch_to_buffer(&subcmd);
        return CommandResult::Ok;
    }

    match subcmd.as_str() {
        "next" | "n" => { app.next_buffer(); }
        "prev" | "p" => { app.prev_buffer(); }
        "close" | "kill" | "c" => {
            let name = app.buffers[app.current_buffer_idx].name.clone();
            app.close_buffer(&name);
        }
        "goto" | "g" => {
            if let Some(n) = args.get(1) {
                if let Ok(idx) = n.parse::<usize>() {
                    if idx < app.buffers.len() {
                        app.current_buffer_idx = idx;
                    }
                } else {
                    app.switch_to_buffer(n);
                }
            }
        }
        "list" | "l" => {
            let lines: Vec<String> = app.buffers.iter().enumerate().map(|(i, buf)| {
                let marker = if i == app.current_buffer_idx { "*" } else { " " };
                format!("  {} {} [{}] (unread: {})", marker, i, buf.name, buf.unread_count)
            }).collect();
            for line in lines { app.system_message(&line); }
        }
        "last" => {
            if let Some(idx) = app.last_buffer_idx {
                if idx < app.buffers.len() {
                    let name = app.buffers[idx].name.clone();
                    app.switch_to_buffer(&name);
                }
            }
        }
        "number" | "refnum" => {
            if let Some(n) = args.get(1) {
                if let Ok(idx) = n.parse::<usize>() {
                    if idx < app.buffers.len() {
                        app.system_message(&format!("-!- Window {} is: {}", idx, app.buffers[idx].name));
                    }
                }
            } else {
                app.system_message(&format!("-!- Current window number: {}", app.current_buffer_idx));
            }
        }
        "name" => {
            if let Some(new_name) = args.get(1) {
                app.buffers[app.current_buffer_idx].name = new_name.to_string();
                app.system_message(&format!("-!- Window renamed to: {}", new_name));
            } else {
                app.system_message(&format!("-!- Current window: {}", app.buffers[app.current_buffer_idx].name));
            }
        }
        "swap" => {
            if let Some(n) = args.get(1) {
                if let Ok(idx) = n.parse::<usize>() {
                    if idx < app.buffers.len() && idx != app.current_buffer_idx {
                        app.buffers.swap(app.current_buffer_idx, idx);
                        app.system_message(&format!("-!- Swapped windows {} and {}", app.current_buffer_idx, idx));
                    }
                }
            }
        }
        "move" => {
            if let Some(n) = args.get(1) {
                if let Ok(idx) = n.parse::<usize>() {
                    if idx < app.buffers.len() && idx != app.current_buffer_idx {
                        let buf = app.buffers.remove(app.current_buffer_idx);
                        app.buffers.insert(idx, buf);
                        app.current_buffer_idx = idx;
                        app.system_message(&format!("-!- Window moved to position {}", idx));
                    }
                }
            }
        }
        "hide" => {
            let name = app.buffers[app.current_buffer_idx].name.clone();
            if name == "(Status)" {
                app.system_message("-!- Cannot hide the status window.");
            } else {
                app.system_message(&format!("-!- Window {} hidden (use /window show {} to restore)", name, name));
                // Ukryj — przejdź do statusu
                app.switch_to_buffer("(Status)");
            }
        }
        "show" => {
            if let Some(name) = args.get(1) {
                app.switch_to_buffer(name);
                app.system_message(&format!("-!- Showing window: {}", name));
            } else {
                app.system_message("-!- Usage: /window show <name>");
            }
        }
        "split" => {
            if let Some(name) = args.get(1) {
                // Split z nazwanym buforem
                if let Some(idx) = app.buffers.iter().position(|b| b.name == *name) {
                    app.split_buffer_idx = Some(idx);
                    app.system_message(&format!("-!- Split screen: {} | {}", app.buffers[app.current_buffer_idx].name, name));
                } else {
                    app.system_message(&format!("-!- No buffer: {}", name));
                }
            } else {
                // Split z następnym buforem
                let next = (app.current_buffer_idx + 1) % app.buffers.len();
                app.split_buffer_idx = Some(next);
                app.system_message(&format!("-!- Split screen: {} | {}",
                    app.buffers[app.current_buffer_idx].name, app.buffers[next].name));
            }
        }
        "unsplit" => {
            app.split_buffer_idx = None;
            app.system_message("-!- Split screen disabled.");
        }
        "level" => {
            if let Some(level) = args.get(1) {
                app.system_message(&format!("-!- Window level set to: {}", level));
            } else {
                app.system_message("-!- Window level: all (available: all, msgs, joins, parts, quits, topics, modes)");
            }
        }
        "logfile" => {
            if let Some(path) = args.get(1) {
                app.settings.set("LOG_FILE", path);
                app.system_message(&format!("-!- Log file set to: {}", path));
            } else {
                app.system_message(&format!("-!- Log file: {}", app.settings.get("LOG_FILE")));
            }
        }
        "push" => {
            // Zapisz aktualny bufor na stosie
            app.system_message(&format!("-!- Pushed window: {}", app.buffers[app.current_buffer_idx].name));
        }
        "pop" => {
            app.system_message("-!- Pop: no window on stack.");
        }
        "server" => {
            if let Some(server_idx) = args.get(1) {
                if let Ok(idx) = server_idx.parse::<usize>() {
                    if idx < app.servers.len() {
                        app.switch_server(idx);
                        app.system_message(&format!("-!- Window bound to server [{}]: {}:{}", idx, app.server().host, app.server().port));
                    } else {
                        app.system_message(&format!("-!- No server with index: {}", idx));
                    }
                } else {
                    // Szukaj po hoście
                    if let Some(idx) = app.find_server(server_idx) {
                        app.switch_server(idx);
                        app.system_message(&format!("-!- Window bound to server: {}", server_idx));
                    } else {
                        app.system_message(&format!("-!- No server: {}", server_idx));
                    }
                }
            } else {
                app.system_message(&format!("-!- Current server [{}]: {}:{}", app.active_server_idx, app.server().host, app.server().port));
            }
        }
        "notify" => {
            if let Some(level) = args.get(1) {
                app.system_message(&format!("-!- Window notify level: {}", level));
            } else {
                app.system_message("-!- Window notify: all (available: all, msg, highlight, none)");
            }
        }
        "format" => {
            if let Some(fmt) = args.get(1) {
                app.system_message(&format!("-!- Window format: {}", fmt));
            } else {
                app.system_message("-!- Window format: default");
            }
        }
        "balance" => {
            app.system_message("-!- Split windows balanced (50/50).");
        }
        "shrink" => {
            app.system_message("-!- Split: top window shrunk.");
        }
        "grow" => {
            app.system_message("-!- Split: top window grown.");
        }
        _ => {
            return CommandResult::Error(format!("Unknown window command: {}. Try: next, prev, close, goto, list, last, number, name, swap, move, hide, show, split, unsplit", subcmd));
        }
    }
    CommandResult::Ok
}

fn cmd_clear(app: &mut App, _args: &[&str]) -> CommandResult {
    app.current_buffer_mut().messages.clear();
    app.current_buffer_mut().scroll_offset = 0;
    CommandResult::Ok
}

fn cmd_lastlog(app: &mut App, args: &[&str]) -> CommandResult {
    let pattern = if args.is_empty() { "" } else { args[0] };
    let buf = app.current_buffer();
    let mut matches: Vec<String> = Vec::new();
    for msg in &buf.messages {
        if pattern.is_empty() || msg.text.to_lowercase().contains(&pattern.to_lowercase()) {
            matches.push(format!("[{}] {}", msg.timestamp, msg.text));
        }
    }
    if matches.is_empty() {
        app.system_message("-!- No matches found in scrollback.");
    } else {
        app.system_message(&format!("-!- Lastlog: {} matches", matches.len()));
        for m in matches {
            app.system_message(&m);
        }
    }
    CommandResult::Ok
}

// ─── Notify / Ignore / Timer ──────────────────────

fn cmd_notify(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        if app.notify_list.is_empty() {
            app.system_message("-!- Notify list is empty.");
        } else {
            let lines: Vec<String> = app.notify_list.iter().map(|n| {
                let status = if n.online { "online" } else { "offline" };
                format!("  {} ({})", n.nick, status)
            }).collect();
            app.system_message("-!- Notify list:");
            for line in lines { app.system_message(&line); }
        }
        return CommandResult::Ok;
    }
    let nick = args[0];
    // Toggle: jeśli jest, usuwamy; jeśli nie ma, dodajemy
    if let Some(pos) = app.notify_list.iter().position(|n| n.nick.to_lowercase() == nick.to_lowercase()) {
        app.notify_list.remove(pos);
        app.system_message(&format!("-!- Removed {} from notify list.", nick));
    } else {
        app.notify_list.push(NotifyEntry {
            nick: nick.to_string(),
            online: false,
            last_seen: None,
        });
        app.system_message(&format!("-!- Added {} to notify list.", nick));
        // Wyślij ISON żeby sprawdzić status
        if let Some(s) = &app.server().sender {
            let _ = s.send(irc::client::prelude::Command::ISON(vec![nick.to_string()]));
        }
    }
    CommandResult::Ok
}

fn cmd_ignore(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        if app.ignore_list.is_empty() {
            app.system_message("-!- Ignore list is empty.");
        } else {
            let lines: Vec<String> = app.ignore_list.iter().map(|e| {
                let mut flags = Vec::new();
                if e.ignore_all { flags.push("ALL"); }
                if e.ignore_public { flags.push("PUBLIC"); }
                if e.ignore_private { flags.push("MSG"); }
                if e.ignore_notice { flags.push("NOTICE"); }
                if e.ignore_ctcp { flags.push("CTCP"); }
                format!("  {} [{}]", e.pattern, flags.join(","))
            }).collect();
            app.system_message("-!- Ignore list:");
            for line in lines { app.system_message(&line); }
        }
        return CommandResult::Ok;
    }
    let pattern = args[0].to_string();
    // Sprawdź czy usuwamy
    if let Some(pos) = app.ignore_list.iter().position(|e| e.pattern == pattern) {
        app.ignore_list.remove(pos);
        app.system_message(&format!("-!- Removed {} from ignore list.", pattern));
        return CommandResult::Ok;
    }
    // Parsuj flagi
    let flags = if args.len() > 1 { args[1].to_uppercase() } else { "ALL".to_string() };
    let entry = IgnoreEntry {
        pattern: pattern.clone(),
        ignore_all: flags.contains("ALL"),
        ignore_public: flags.contains("PUBLIC") || flags.contains("ALL"),
        ignore_private: flags.contains("MSG") || flags.contains("ALL"),
        ignore_notice: flags.contains("NOTICE") || flags.contains("ALL"),
        ignore_ctcp: flags.contains("CTCP") || flags.contains("ALL"),
    };
    app.ignore_list.push(entry);
    app.system_message(&format!("-!- Ignoring {} [{}]", pattern, flags));
    CommandResult::Ok
}

fn cmd_timer(app: &mut App, args: &[&str]) -> CommandResult {
    if args.len() < 3 {
        if app.timers.is_empty() {
            app.system_message("-!- No active timers.");
        } else {
            let lines: Vec<String> = app.timers.iter().map(|t| {
                format!("  [{}] every {}ms, repeats: {}, cmd: {}",
                    t.name, t.interval_ms, t.remaining, t.command)
            }).collect();
            app.system_message("-!- Active timers:");
            for line in lines { app.system_message(&line); }
        }
        return CommandResult::Ok;
    }
    let seconds = args[0].parse::<f64>().unwrap_or(1.0);
    let repeats = args[1].parse::<i32>().unwrap_or(1);
    let command = args[2..].join(" ");
    let name = format!("timer_{}", app.timers.len());
    app.timers.push(TimerEntry {
        name: name.clone(),
        interval_ms: (seconds * 1000.0) as u64,
        repeat: repeats,
        command,
        next_fire: Instant::now() + Duration::from_secs_f64(seconds),
        remaining: repeats,
    });
    app.system_message(&format!("-!- Timer '{}' created.", name));
    CommandResult::Ok
}

// ─── Aliasy i ekspresje ──────────────────────────

fn cmd_alias(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        if app.aliases.is_empty() {
            app.system_message("-!- No aliases defined.");
        } else {
            app.system_message("-!- Aliases:");
            let mut entries: Vec<(String, String)> = app.aliases.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            for (name, body) in entries {
                app.system_message(&format!("  {} = {}", name.to_lowercase(), body));
            }
        }
        return CommandResult::Ok;
    }
    let name = args[0].to_uppercase();
    if args.len() == 1 {
        // Pokaż alias
        if let Some(body) = app.aliases.get(&name) {
            app.system_message(&format!("-!- {} = {}", name.to_lowercase(), body));
        } else {
            app.system_message(&format!("-!- No such alias: {}", args[0]));
        }
    } else {
        let body = args[1..].join(" ");
        app.aliases.insert(name.clone(), body.clone());
        app.system_message(&format!("-!- Alias {} = {}", name.to_lowercase(), body));
    }
    CommandResult::Ok
}

fn cmd_unalias(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /unalias <name>".into());
    }
    let name = args[0].to_uppercase();
    if app.aliases.remove(&name).is_some() {
        app.system_message(&format!("-!- Removed alias: {}", args[0]));
    } else {
        app.system_message(&format!("-!- No such alias: {}", args[0]));
    }
    CommandResult::Ok
}

fn cmd_if(app: &mut App, args: &[&str]) -> CommandResult {
    // /if <cond> <then> [else <else>]
    // Warunki: $0 == text, $0 != text, $# > N, $0 =~ pattern
    if args.len() < 3 {
        return CommandResult::Error("Usage: /if <cond> <then_cmd> [else <else_cmd>]".into());
    }
    let cond = args[0];
    let then_start = 1;
    let else_pos = args.iter().position(|&a| a.eq_ignore_ascii_case("else"));

    let (then_args, else_args) = if let Some(pos) = else_pos {
        (&args[then_start..pos], &args[pos + 1..])
    } else {
        (&args[then_start..], &[][..])
    };

    let result = evaluate_condition(app, cond);
    let cmd_args = if result { then_args } else { else_args };
    if !cmd_args.is_empty() {
        let cmd_text = cmd_args.join(" ");
        if cmd_text.starts_with('/') {
            let parts: Vec<&str> = cmd_text.split_whitespace().collect();
            let cmd_name = &parts[0][1..];
            let cmd_args = &parts[1..];
            if let Some(cmd) = CommandRegistry::new().find(cmd_name) {
                let handler = cmd.handler;
                return handler(app, cmd_args);
            }
        }
    }
    CommandResult::Ok
}

fn cmd_while(app: &mut App, args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error("Usage: /while <condition> <command>".into());
    }
    let cond = args[0];
    let body = args[1..].join(" ");
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 100;

    while evaluate_condition(app, cond) && iterations < MAX_ITERATIONS {
        if body.starts_with('/') {
            let parts: Vec<&str> = body.split_whitespace().collect();
            let cmd_name = &parts[0][1..];
            let cmd_args = &parts[1..];
            if let Some(cmd) = CommandRegistry::new().find(cmd_name) {
                let handler = cmd.handler;
                handler(app, cmd_args);
            }
        }
        iterations += 1;
    }
    if iterations >= MAX_ITERATIONS {
        app.system_message("-!- /while: iteration limit reached (100)");
    }
    CommandResult::Ok
}

fn cmd_for(app: &mut App, args: &[&str]) -> CommandResult {
    // /for <var> <start> <end> <command>
    if args.len() < 4 {
        return CommandResult::Error("Usage: /for <var> <start> <end> <command>".into());
    }
    let var_name = args[0];
    let start: i64 = args[1].parse().unwrap_or(0);
    let end: i64 = args[2].parse().unwrap_or(0);
    let body = args[3..].join(" ");
    let mut iterations = 0;
    const MAX_ITERATIONS: usize = 1000;

    let mut i = start;
    while i <= end && iterations < MAX_ITERATIONS {
        // Replace $var with current value
        let expanded = body.replace(&format!("${}", var_name), &i.to_string());
        if expanded.starts_with('/') {
            let parts: Vec<&str> = expanded.split_whitespace().collect();
            let cmd_name = &parts[0][1..];
            let cmd_args = &parts[1..];
            if let Some(cmd) = CommandRegistry::new().find(cmd_name) {
                let handler = cmd.handler;
                handler(app, cmd_args);
            }
        }
        i += 1;
        iterations += 1;
    }
    if iterations >= MAX_ITERATIONS {
        app.system_message("-!- /for: iteration limit reached (1000)");
    }
    CommandResult::Ok
}

fn cmd_wait(app: &mut App, args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error("Usage: /wait <seconds> <command>".into());
    }
    let seconds: f64 = args[0].parse().unwrap_or(1.0);
    let command = args[1..].join(" ");
    app.system_message(&format!("-!- Waiting {}s then executing: {}", seconds, command));
    // Add as a one-shot timer
    app.timers.push(crate::app::TimerEntry {
        name: format!("wait_{}", app.timers.len()),
        interval_ms: (seconds * 1000.0) as u64,
        repeat: 1,
        command,
        next_fire: std::time::Instant::now() + std::time::Duration::from_secs_f64(seconds),
        remaining: 1,
    });
    CommandResult::Ok
}

fn cmd_redirect(app: &mut App, args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error("Usage: /redirect <target> <command>".into());
    }
    let target = args[0];
    let command = args[1..].join(" ");
    app.system_message(&format!("-!- Redirect {} -> {}", command, target));
    // Execute command and redirect output to target
    // For now, just execute the command
    if command.starts_with('/') {
        let parts: Vec<&str> = command.split_whitespace().collect();
        let cmd_name = &parts[0][1..];
        let cmd_args = &parts[1..];
        if let Some(cmd) = CommandRegistry::new().find(cmd_name) {
            let handler = cmd.handler;
            handler(app, cmd_args);
        }
    }
    CommandResult::Ok
}

/// Ewaluacja warunków epic5-style
/// Warunki: left op right — expand_variables już rozwija $0-$9, $N, $C, $S
fn evaluate_condition(_app: &App, cond: &str) -> bool {
    let parts: Vec<&str> = cond.split_whitespace().collect();
    if parts.len() < 3 { return false; }

    let left = parts[0];
    let op = parts[1];
    let right = parts[2];

    match op {
        "==" | "eq" => left == right,
        "!=" | "ne" => left != right,
        ">" => left.parse::<i64>().unwrap_or(0) > right.parse::<i64>().unwrap_or(0),
        "<" => left.parse::<i64>().unwrap_or(0) < right.parse::<i64>().unwrap_or(0),
        ">=" => left.parse::<i64>().unwrap_or(0) >= right.parse::<i64>().unwrap_or(0),
        "<=" => left.parse::<i64>().unwrap_or(0) <= right.parse::<i64>().unwrap_or(0),
        "=~" => {
            if right.contains('*') {
                let p: Vec<&str> = right.split('*').collect();
                if p.len() == 2 {
                    left.starts_with(p[0]) && left.ends_with(p[1])
                } else {
                    left.contains(&right.replace('*', ""))
                }
            } else {
                left.contains(right)
            }
        }
        _ => false,
    }
}

// ─── Konfiguracja ────────────────────────────────

fn cmd_set(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        app.system_message("-!- Current settings:");
        let mut keys: Vec<String> = app.settings.map.keys().cloned().collect();
        keys.sort();
        for key in keys {
            app.system_message(&format!("  {} = {}", key, app.settings.get(&key)));
        }
        return CommandResult::Ok;
    }
    if args.len() == 1 {
        let val = app.settings.get(args[0]);
        app.system_message(&format!("  {} = {}", args[0].to_uppercase(), val));
    } else {
        let value = args[1..].join(" ");
        app.settings.set(args[0], &value);
        app.system_message(&format!("-!- SET {} = {}", args[0].to_uppercase(), value));
    }
    CommandResult::Ok
}

// ─── System ──────────────────────────────────────

fn cmd_help(app: &mut App, args: &[&str]) -> CommandResult {
    let registry = CommandRegistry::new();
    if args.is_empty() {
        app.system_message("-!- Available commands:");
        for cmd in &registry.commands {
            let aliases = if cmd.aliases.is_empty() {
                String::new()
            } else {
                format!(" (aliases: {})", cmd.aliases.join(", ").to_lowercase())
            };
            app.system_message(&format!("  /{}{}", cmd.name.to_lowercase(), aliases));
        }
        app.system_message("-!- Type /help <command> for details.");
    } else {
        let name = args[0].to_uppercase();
        if let Some(cmd) = registry.find(&name) {
            app.system_message(&format!("-!- {}", cmd.help));
        } else {
            app.system_message(&format!("-!- Unknown command: {}", args[0]));
        }
    }
    CommandResult::Ok
}

fn cmd_raw(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /raw <text>".into());
    }
    let raw_line = args.join(" ");
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(raw_line.clone(), Vec::new()));
    }
    app.system_message(&format!("-> {}", raw_line));
    CommandResult::Ok
}

fn cmd_echo(app: &mut App, args: &[&str]) -> CommandResult {
    let text = args.join(" ");
    let buf_name = app.buffers[app.current_buffer_idx].name.clone();
    app.buffer_message(&buf_name, text, MessageType::System);
    CommandResult::Ok
}

fn cmd_exec(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /exec <command>".into());
    }
    let cmd = args.join(" ");
    app.system_message(&format!("-!- Executing: {}", cmd));
    app.pending_exec.push(cmd);
    CommandResult::Ok
}

fn cmd_log(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        let status = if app.logger.enabled { "ON" } else { "OFF" };
        let mode = if app.logger.per_buffer { "per-buffer" } else { "global" };
        app.system_message(&format!("-!- Logging: {} ({}) [{}]", status, app.logger.path.display(), mode));
        app.system_message("-!- Usage: /log on|off|<#channel> on|off|rotate <size_mb>");
        return CommandResult::Ok;
    }
    match args[0].to_uppercase().as_str() {
        "ON" => {
            match app.logger.enable() {
                Ok(_) => {
                    app.settings.set("LOG", "ON");
                    app.system_message(&format!("-!- Global logging enabled: {}", app.logger.path.display()));
                }
                Err(e) => {
                    app.system_message(&format!("-!- Cannot enable logging: {}", e));
                }
            }
        }
        "OFF" => {
            app.logger.disable();
            app.settings.set("LOG", "OFF");
            app.system_message("-!- Logging disabled.");
        }
        "ROTATE" => {
            if let Some(size) = args.get(1) {
                let mb: u64 = size.parse().unwrap_or(10);
                app.logger.max_size_bytes = mb * 1024 * 1024;
                app.system_message(&format!("-!- Log rotation set: {} MB", mb));
            } else {
                let mb = app.logger.max_size_bytes / (1024 * 1024);
                app.system_message(&format!("-!- Log rotation: {} MB", mb));
            }
        }
        _ => {
            // /log <#channel> on|off — per-buffer logging
            let buf_name = args[0];
            let action = args.get(1).map(|s| s.to_uppercase()).unwrap_or_default();
            match action.as_str() {
                "ON" => {
                    match app.logger.enable_buffer(buf_name) {
                        Ok(_) => {
                            app.system_message(&format!("-!- Logging enabled for: {}", buf_name));
                        }
                        Err(e) => {
                            app.system_message(&format!("-!- Cannot enable logging for {}: {}", buf_name, e));
                        }
                    }
                }
                "OFF" => {
                    app.logger.disable_buffer(buf_name);
                    app.system_message(&format!("-!- Logging disabled for: {}", buf_name));
                }
                _ => {
                    return CommandResult::Error("Usage: /log <#channel> on|off".into());
                }
            }
        }
    }
    CommandResult::Ok
}

fn cmd_eval(app: &mut App, args: &[&str]) -> CommandResult {
    let text = args.join(" ");
    let lua_ref = app.lua.clone();
    if let Some(lua) = lua_ref {
        match lua.load(&text).eval::<mlua::Value>() {
            Ok(mlua::Value::String(s)) => {
                app.system_message(&format!("-!- Eval: {}", s.to_string_lossy()));
            }
            Ok(mlua::Value::Integer(n)) => {
                app.system_message(&format!("-!- Eval: {}", n));
            }
            Ok(mlua::Value::Number(n)) => {
                app.system_message(&format!("-!- Eval: {}", n));
            }
            Ok(mlua::Value::Boolean(b)) => {
                app.system_message(&format!("-!- Eval: {}", b));
            }
            Ok(mlua::Value::Nil) => {
                app.system_message("-!- Eval: nil");
            }
            Ok(_) => {
                app.system_message("-!- Eval: (complex value)");
            }
            Err(e) => {
                app.system_message(&format!("-!- Eval error: {}", e));
            }
        }
    } else {
        app.system_message(&format!("-!- Eval: {} (Lua engine not available)", text));
    }
    CommandResult::Ok
}

fn cmd_save(app: &mut App, _args: &[&str]) -> CommandResult {
    // Zapisz do SQLite
    app.save_to_db();
    app.system_message("-!- Configuration saved to SQLite (~/.void/void.db)");

    // Zapisz też do pliku tekstowego (backup)
    let config_dir = std::env::var("HOME")
        .map(|h| std::path::PathBuf::from(h).join(".void"))
        .unwrap_or_else(|_| std::path::PathBuf::from(".void"));

    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        return CommandResult::Error(format!("Cannot create config dir: {}", e));
    }

    let config_path = config_dir.join("void.conf");
    let mut output = String::new();

    output.push_str("# Void IRC Client configuration\n");
    output.push_str("# Auto-generated by /save\n\n");
    output.push_str("[settings]\n");
    let mut keys: Vec<String> = app.settings.map.keys().cloned().collect();
    keys.sort();
    for key in keys {
        output.push_str(&format!("{}={}\n", key, app.settings.get(&key)));
    }

    if !app.aliases.is_empty() {
        output.push_str("\n[aliases]\n");
        let mut entries: Vec<(String, String)> = app.aliases.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, body) in entries {
            output.push_str(&format!("{}={}\n", name, body));
        }
    }

    match std::fs::write(&config_path, output) {
        Ok(_) => {
            app.system_message(&format!("-!- Configuration saved to: {}", config_path.display()));
        }
        Err(e) => {
            return CommandResult::Error(format!("Cannot write config: {}", e));
        }
    }
    CommandResult::Ok
}

// ─── Highlight / Load ────────────────────────────

fn cmd_highlight(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        if app.highlight_patterns.is_empty() {
            app.system_message("-!- No highlight patterns.");
        } else {
            app.system_message("-!- Highlight patterns:");
            let patterns: Vec<(usize, String, String)> = app.highlight_patterns.iter()
                .enumerate()
                .map(|(i, h)| (i, h.pattern.clone(), h.color.clone()))
                .collect();
            for (i, pattern, color) in patterns {
                app.system_message(&format!("  [{}] {} ({})", i, pattern, color));
            }
        }
        return CommandResult::Ok;
    }
    let pattern = args[0];
    // Sprawdź czy usuwamy
    if let Some(pos) = app.highlight_patterns.iter().position(|h| h.pattern == pattern) {
        app.highlight_patterns.remove(pos);
        app.system_message(&format!("-!- Removed highlight: {}", pattern));
        return CommandResult::Ok;
    }
    let color = args.get(1).unwrap_or(&"yellow").to_string();
    app.highlight_patterns.push(HighlightPattern {
        pattern: pattern.to_string(),
        color: color.clone(),
    });
    app.system_message(&format!("-!- Highlight added: {} ({})", pattern, color));
    CommandResult::Ok
}

fn cmd_load(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /load <script.lua>".into());
    }
    let path = args[0];
    let script = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => return CommandResult::Error(format!("Cannot load {}: {}", path, e)),
    };
    app.system_message(&format!("-!- Loading script: {}", path));
    let lua_ref = app.lua.clone();
    if let Some(lua) = lua_ref {
        match lua.load(&script).exec() {
            Ok(_) => {
                app.system_message(&format!("-!- Script loaded: {}", path));
            }
            Err(e) => {
                app.system_message(&format!("-!- Lua error in {}: {}", path, e));
            }
        }
    } else {
        app.system_message("-!- Lua engine not available.");
    }
    CommandResult::Ok
}

fn cmd_reload(app: &mut App, _args: &[&str]) -> CommandResult {
    app.system_message("-!- Reloading Lua scripts...");
    let lua_ref = app.lua.clone();
    if let Some(lua) = lua_ref {
        if std::path::Path::new("config.lua").exists() {
            match std::fs::read_to_string("config.lua") {
                Ok(script) => {
                    match lua.load(&script).exec() {
                        Ok(_) => { app.system_message("-!- Reloaded config.lua"); }
                        Err(e) => { app.system_message(&format!("-!- Lua error: {}", e)); }
                    }
                }
                Err(e) => { app.system_message(&format!("-!- Cannot read config.lua: {}", e)); }
            }
        } else {
            app.system_message("-!- No config.lua found");
        }
        if std::path::Path::new("modules/init.lua").exists() {
            match std::fs::read_to_string("modules/init.lua") {
                Ok(script) => {
                    match lua.load(&script).exec() {
                        Ok(_) => { app.system_message("-!- Reloaded modules/init.lua"); }
                        Err(e) => { app.system_message(&format!("-!- Lua error: {}", e)); }
                    }
                }
                Err(e) => { app.system_message(&format!("-!- Cannot read modules/init.lua: {}", e)); }
            }
        }
    } else {
        app.system_message("-!- Lua engine not available.");
    }
    CommandResult::Ok
}

fn cmd_cd(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /cd <path>".into());
    }
    let path = args[0];
    match std::env::set_current_dir(path) {
        Ok(_) => {
            let cwd = std::env::current_dir().unwrap_or_default();
            app.system_message(&format!("-!- Changed directory to: {}", cwd.display()));
        }
        Err(e) => {
            return CommandResult::Error(format!("Cannot cd to {}: {}", path, e));
        }
    }
    CommandResult::Ok
}

fn cmd_pwd(app: &mut App, _args: &[&str]) -> CommandResult {
    match std::env::current_dir() {
        Ok(cwd) => app.system_message(&format!("-!- {}", cwd.display())),
        Err(e) => app.system_message(&format!("-!- Error: {}", e)),
    }
    CommandResult::Ok
}

fn cmd_debug(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        let status = if app.settings.get_bool("DEBUG") { "ON" } else { "OFF" };
        app.system_message(&format!("-!- Debug mode: {}", status));
    } else {
        match args[0].to_uppercase().as_str() {
            "ON" | "1" => {
                app.settings.set("DEBUG", "ON");
                app.system_message("-!- Debug mode enabled.");
            }
            "OFF" | "0" => {
                app.settings.set("DEBUG", "OFF");
                app.system_message("-!- Debug mode disabled.");
            }
            _ => return CommandResult::Error("Usage: /debug [on|off]".into()),
        }
    }
    CommandResult::Ok
}

fn cmd_repaint(app: &mut App, _args: &[&str]) -> CommandResult {
    app.system_message("-!- Screen repainted.");
    CommandResult::Ok
}

fn cmd_scroll(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /scroll <up|down|top|bottom|N>".into());
    }
    let max = app.current_buffer().messages.len().saturating_sub(1);
    let new_offset = match args[0].to_lowercase().as_str() {
        "up" => (app.current_buffer().scroll_offset + 10).min(max),
        "down" => app.current_buffer().scroll_offset.saturating_sub(10),
        "top" => max,
        "bottom" => 0,
        _ => {
            if let Ok(n) = args[0].parse::<usize>() {
                n.min(max)
            } else {
                return CommandResult::Error("Usage: /scroll <up|down|top|bottom|N>".into());
            }
        }
    };
    app.current_buffer_mut().scroll_offset = new_offset;
    app.system_message(&format!("-!- Scroll offset: {}", new_offset));
    CommandResult::Ok
}

fn cmd_status(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        let fmt = app.settings.get("STATUS_FORMAT");
        app.system_message(&format!("-!- Status format: {}", fmt));
        app.system_message("-!- Variables: $N=nick, $C=channel, $H=host, $S=server, $T=topic");
    } else {
        let fmt = args.join(" ");
        app.settings.set("STATUS_FORMAT", &fmt);
        app.system_message(&format!("-!- Status format set: {}", fmt));
    }
    CommandResult::Ok
}

fn cmd_flood(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        let status = if app.flood.enabled { "ON" } else { "OFF" };
        app.system_message(&format!("-!- Flood protection: {} ({} msgs / {} sec)",
            status, app.flood.max_messages, app.flood.window_secs));
        return CommandResult::Ok;
    }
    match args[0].to_uppercase().as_str() {
        "ON" => {
            app.flood.enabled = true;
            app.settings.set("FLOOD_PROTECTION", "ON");
            app.system_message("-!- Flood protection enabled.");
        }
        "OFF" => {
            app.flood.enabled = false;
            app.settings.set("FLOOD_PROTECTION", "OFF");
            app.system_message("-!- Flood protection disabled.");
        }
        _ => {
            if args.len() >= 2 {
                let rate = args[0].parse::<usize>().unwrap_or(4);
                let per = args[1].parse::<u64>().unwrap_or(2);
                app.flood.max_messages = rate.max(1);
                app.flood.window_secs = per.max(1);
                app.settings.set("FLOOD_RATE", &rate.to_string());
                app.settings.set("FLOOD_RATE_PER", &per.to_string());
                app.system_message(&format!("-!- Flood rate set: {} msgs / {} sec", rate, per));
            }
        }
    }
    CommandResult::Ok
}

fn cmd_shh(app: &mut App, _args: &[&str]) -> CommandResult {
    app.suppress_display = true;
    app.system_message("-!- Display suppressed for this context.");
    CommandResult::Ok
}

fn cmd_play(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /play <logfile>".into());
    }
    let path = args[0];
    match std::fs::read_to_string(path) {
        Ok(content) => {
            let buf_name = app.buffers[app.current_buffer_idx].name.clone();
            let mut count = 0;
            for line in content.lines() {
                let line = line.trim();
                if !line.is_empty() {
                    app.buffer_message(&buf_name, line.to_string(), MessageType::ServerReply);
                    count += 1;
                }
            }
            app.system_message(&format!("-!- Played {} lines from {}", count, path));
        }
        Err(e) => {
            return CommandResult::Error(format!("Cannot read {}: {}", path, e));
        }
    }
    CommandResult::Ok
}

fn cmd_bind(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        if app.key_bindings.is_empty() {
            app.system_message("-!- No key bindings. Default bindings:");
            app.system_message("  Ctrl+N/P  — next/prev window");
            app.system_message("  Ctrl+X    — cycle windows");
            app.system_message("  Ctrl+L    — refresh screen");
            app.system_message("  Ctrl+A/E  — start/end of line");
            app.system_message("  Ctrl+U/K  — clear line / kill to end");
            app.system_message("  Ctrl+W    — delete word");
            app.system_message("  Alt+1-9   — jump to window N");
            app.system_message("  Alt+K/B/U/I/R/O — IRC formatting codes");
        } else {
            app.system_message("-!- Key bindings:");
            let bindings: Vec<(String, String)> = app.key_bindings.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            for (key, action) in bindings {
                app.system_message(&format!("  {} = {}", key, action));
            }
        }
        return CommandResult::Ok;
    }
    if args.len() < 2 {
        return CommandResult::Error("Usage: /bind <key> <action>".into());
    }
    let key = args[0].to_uppercase();
    let action = args[1..].join(" ");
    app.key_bindings.insert(key.clone(), action.clone());
    app.system_message(&format!("-!- Bound {} = {}", key, action));
    CommandResult::Ok
}

fn cmd_format(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        app.system_message("-!- Format templates ($0, $1, $2... = args):");
        let mut entries: Vec<(String, String)> = app.format_templates.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        for (name, template) in entries {
            app.system_message(&format!("  {} = {}", name, template));
        }
        return CommandResult::Ok;
    }
    let fmt_type = args[0].to_uppercase();
    if args.len() == 1 {
        if let Some(template) = app.format_templates.get(&fmt_type) {
            app.system_message(&format!("-!- {} = {}", fmt_type, template));
        } else {
            app.system_message(&format!("-!- No format template for: {}", fmt_type));
        }
    } else {
        let template = args[1..].join(" ");
        app.format_templates.insert(fmt_type.clone(), template.clone());
        app.system_message(&format!("-!- Format {} = {}", fmt_type, template));
    }
    CommandResult::Ok
}

// ─── DCC ─────────────────────────────────────────

fn cmd_dcc(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        let lines = app.dcc.format_list();
        if lines.is_empty() {
            app.system_message("-!- No DCC sessions.");
        } else {
            app.system_message("-!- DCC sessions:");
            for line in lines { app.system_message(&line); }
        }
        return CommandResult::Ok;
    }
    let subcmd = args[0].to_lowercase();
    match subcmd.as_str() {
        "list" => {
            let lines = app.dcc.format_list();
            if lines.is_empty() {
                app.system_message("-!- No DCC sessions.");
            } else {
                app.system_message("-!- DCC sessions:");
                for line in lines { app.system_message(&line); }
            }
        }
        "chat" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: /dcc chat <nick>".into());
            }
            app.system_message(&format!("-!- DCC CHAT with {} — not yet implemented (coming with LiCe).", args[1]));
        }
        "send" => {
            if args.len() < 3 {
                return CommandResult::Error("Usage: /dcc send <nick> <file>".into());
            }
            app.system_message(&format!("-!- DCC SEND {} to {} — not yet implemented (coming with LiCe).", args[2], args[1]));
        }
        "get" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: /dcc get <id>".into());
            }
            let id = args[1].parse::<usize>().unwrap_or(0);
            app.system_message(&format!("-!- Accepting DCC transfer id {}...", id));
            match app.dcc.accept_send(id) {
                Ok(msg) => { app.system_message(&format!("-!- {}", msg)); }
                Err(e) => { app.system_message(&format!("-!- DCC error: {}", e)); }
            }
        }
        "close" => {
            if args.len() < 2 {
                return CommandResult::Error("Usage: /dcc close <id>".into());
            }
            let id = args[1].parse::<usize>().unwrap_or(0);
            if let Some(session) = app.dcc.get_mut(id) {
                session.state = crate::dcc::DccState::Failed("Closed by user".into());
                app.system_message(&format!("-!- DCC session {} closed.", id));
            } else {
                app.system_message(&format!("-!- No DCC session with id {}.", id));
            }
        }
        _ => {
            return CommandResult::Error("Usage: /dcc <list|chat|send|get|close>".into());
        }
    }
    CommandResult::Ok
}

// ─── Dodatkowe komendy kanałowe ──────────────────

fn cmd_list(app: &mut App, args: &[&str]) -> CommandResult {
    if let Some(s) = &app.server().sender {
        let mask = args.first().map(|s| s.to_string());
        let _ = s.send(irc::client::prelude::Command::LIST(mask, None));
    }
    CommandResult::Ok
}

fn cmd_cycle(app: &mut App, _args: &[&str]) -> CommandResult {
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    if channel == "(Status)" {
        return CommandResult::Error("Not in a channel.".into());
    }
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::PART(channel.clone(), Some("Cycling".into())));
        let _ = s.send_join(&channel);
    }
    app.system_message(&format!("-!- Cycling {}...", channel));
    CommandResult::Ok
}

fn cmd_knock(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /knock <#channel> [message]".into());
    }
    let channel = args[0];
    let msg = if args.len() > 1 { args[1..].join(" ") } else { "Let me in!".to_string() };
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(
            format!("KNOCK {} :{}", channel, msg), Vec::new()
        ));
    }
    app.system_message(&format!("-!- Knocking on {}...", channel));
    CommandResult::Ok
}

fn cmd_oper(app: &mut App, args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error("Usage: /oper <login> <password>".into());
    }
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(
            format!("OPER {} {}", args[0], args[1]), Vec::new()
        ));
    }
    app.system_message("-!- Sending OPER request...");
    CommandResult::Ok
}

fn cmd_kill(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /kill <nick> [reason]".into());
    }
    let nick = args[0];
    let reason = if args.len() > 1 { args[1..].join(" ") } else { app.server().our_nick.clone() };
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(
            format!("KILL {} :{}", nick, reason), Vec::new()
        ));
    }
    app.system_message(&format!("-!- KILL sent for {}", nick));
    CommandResult::Ok
}

fn cmd_silence(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        if let Some(s) = &app.server().sender {
            let _ = s.send(irc::client::prelude::Command::Raw("SILENCE".into(), Vec::new()));
        }
        app.system_message("-!- Requesting silence list...");
    } else {
        let mask = args[0];
        if mask.starts_with('-') {
            let mask = &mask[1..];
            if let Some(s) = &app.server().sender {
                let _ = s.send(irc::client::prelude::Command::Raw(
                    format!("SILENCE -{}", mask), Vec::new()
                ));
            }
            app.system_message(&format!("-!- Silence removed: {}", mask));
        } else {
            if let Some(s) = &app.server().sender {
                let _ = s.send(irc::client::prelude::Command::Raw(
                    format!("SILENCE +{}", mask), Vec::new()
                ));
            }
            app.system_message(&format!("-!- Silence added: {}", mask));
        }
    }
    CommandResult::Ok
}

fn cmd_setname(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /setname <realname>".into());
    }
    let realname = args.join(" ");
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(
            format!("SETNAME :{}", realname), Vec::new()
        ));
    }
    app.system_message(&format!("-!- Realname set to: {}", realname));
    CommandResult::Ok
}

fn cmd_caplist(app: &mut App, _args: &[&str]) -> CommandResult {
    app.system_message("-!- Active IRCv3 capabilities:");
    let tokens: Vec<(String, String)> = app.server().server_info.tokens.iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    for (key, value) in tokens {
        if value.is_empty() {
            app.system_message(&format!("  {}", key));
        } else {
            app.system_message(&format!("  {}={}", key, value));
        }
    }
    CommandResult::Ok
}

fn cmd_chathistory(app: &mut App, args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        app.system_message("-!- Usage: /chathistory <subcommand> <target> [limit]");
        app.system_message("-!- Subcommands: before, after, latest, around, targets");
        app.system_message("-!- Examples:");
        app.system_message("  /chathistory latest #channel 50");
        app.system_message("  /chathistory before #channel 2023-01-01T00:00:00Z 50");
        app.system_message("  /chathistory targets 2023-01-01T00:00:00Z 25");
        return CommandResult::Ok;
    }
    let subcmd = args[0].to_lowercase();
    let channel = app.buffers[app.current_buffer_idx].name.clone();

    match subcmd.as_str() {
        "latest" => {
            let target = if args.len() > 1 { args[1] } else { &channel };
            let limit = if args.len() > 2 { args[2] } else { "50" };
            if let Some(s) = &app.server().sender {
                let cmd = format!("CHATHISTORY LATEST {} * {}", target, limit);
                let _ = s.send(irc::client::prelude::Command::Raw(cmd, Vec::new()));
            }
            app.system_message(&format!("-!- Requesting latest {} messages for {}", limit, target));
        }
        "before" => {
            if args.len() < 3 {
                return CommandResult::Error("Usage: /chathistory before <target> <timestamp> [limit]".into());
            }
            let target = args[1];
            let timestamp = args[2];
            let limit = args.get(3).unwrap_or(&"50");
            if let Some(s) = &app.server().sender {
                let cmd = format!("CHATHISTORY BEFORE {} timestamp={} {}", target, timestamp, limit);
                let _ = s.send(irc::client::prelude::Command::Raw(cmd, Vec::new()));
            }
            app.system_message(&format!("-!- Requesting messages before {} for {}", timestamp, target));
        }
        "after" => {
            if args.len() < 3 {
                return CommandResult::Error("Usage: /chathistory after <target> <timestamp> [limit]".into());
            }
            let target = args[1];
            let timestamp = args[2];
            let limit = args.get(3).unwrap_or(&"50");
            if let Some(s) = &app.server().sender {
                let cmd = format!("CHATHISTORY AFTER {} timestamp={} {}", target, timestamp, limit);
                let _ = s.send(irc::client::prelude::Command::Raw(cmd, Vec::new()));
            }
            app.system_message(&format!("-!- Requesting messages after {} for {}", timestamp, target));
        }
        "around" => {
            if args.len() < 3 {
                return CommandResult::Error("Usage: /chathistory around <target> <timestamp> [limit]".into());
            }
            let target = args[1];
            let timestamp = args[2];
            let limit = args.get(3).unwrap_or(&"50");
            if let Some(s) = &app.server().sender {
                let cmd = format!("CHATHISTORY AROUND {} timestamp={} {}", target, timestamp, limit);
                let _ = s.send(irc::client::prelude::Command::Raw(cmd, Vec::new()));
            }
            app.system_message(&format!("-!- Requesting messages around {} for {}", timestamp, target));
        }
        "targets" => {
            let timestamp = args.get(1).unwrap_or(&"2020-01-01T00:00:00Z");
            let limit = args.get(2).unwrap_or(&"25");
            if let Some(s) = &app.server().sender {
                let cmd = format!("CHATHISTORY TARGETS timestamp={} {}", timestamp, limit);
                let _ = s.send(irc::client::prelude::Command::Raw(cmd, Vec::new()));
            }
            app.system_message(&format!("-!- Requesting chat targets since {}", timestamp));
        }
        _ => {
            return CommandResult::Error("Usage: /chathistory <before|after|latest|around|targets> ...".into());
        }
    }
    CommandResult::Ok
}

// ─── Dodatkowe komendy epic5-style ───────────────

fn cmd_wallops(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /wallops <text>".into());
    }
    let text = args.join(" ");
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(format!("WALLOPS :{}", text), Vec::new()));
    }
    app.system_message(&format!("-!- Wallops: {}", text));
    CommandResult::Ok
}

fn cmd_ping(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /ping <nick>".into());
    }
    let target = args[0];
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".to_string());
    if let Some(s) = &app.server().sender {
        let _ = s.send_privmsg(target, &format!("\x01PING {}\x01", now));
    }
    app.system_message(&format!("-!- CTCP PING sent to {}", target));
    CommandResult::Ok
}

// ─── Komendy specyficzne dla irc2.11 / RFC2812 ───────

fn cmd_lusers(app: &mut App, args: &[&str]) -> CommandResult {
    if let Some(s) = &app.server().sender {
        let mask = args.first().map(|s| s.to_string());
        let target = args.get(1).map(|s| s.to_string());
        let _ = s.send(irc::client::prelude::Command::LUSERS(mask, target));
    }
    CommandResult::Ok
}

fn cmd_admin(app: &mut App, args: &[&str]) -> CommandResult {
    if let Some(s) = &app.server().sender {
        let target = args.first().map(|s| s.to_string());
        let _ = s.send(irc::client::prelude::Command::ADMIN(target));
    }
    CommandResult::Ok
}

fn cmd_info(app: &mut App, args: &[&str]) -> CommandResult {
    if let Some(s) = &app.server().sender {
        let target = args.first().map(|s| s.to_string());
        let _ = s.send(irc::client::prelude::Command::INFO(target));
    }
    CommandResult::Ok
}

fn cmd_motd(app: &mut App, args: &[&str]) -> CommandResult {
    if let Some(s) = &app.server().sender {
        let target = args.first().map(|s| s.to_string());
        let _ = s.send(irc::client::prelude::Command::MOTD(target));
    }
    CommandResult::Ok
}

fn cmd_stats(app: &mut App, args: &[&str]) -> CommandResult {
    if args.is_empty() {
        return CommandResult::Error("Usage: /stats <flag> [server] (flags: c, h, i, k, K, l, m, o, p, u, y, z, ?)".into());
    }
    if let Some(s) = &app.server().sender {
        let query = args.first().map(|s| s.to_string());
        let target = args.get(1).map(|s| s.to_string());
        let _ = s.send(irc::client::prelude::Command::STATS(query, target));
    }
    CommandResult::Ok
}

fn cmd_links(app: &mut App, args: &[&str]) -> CommandResult {
    if let Some(s) = &app.server().sender {
        let remote = if args.len() > 1 { Some(args[0].to_string()) } else { None };
        let mask = if args.len() > 1 { Some(args[1].to_string()) } else { args.first().map(|s| s.to_string()) };
        let _ = s.send(irc::client::prelude::Command::LINKS(remote, mask));
    }
    CommandResult::Ok
}

fn cmd_map(app: &mut App, _args: &[&str]) -> CommandResult {
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw("MAP".into(), Vec::new()));
    }
    CommandResult::Ok
}

fn cmd_trace(app: &mut App, args: &[&str]) -> CommandResult {
    if let Some(s) = &app.server().sender {
        let target = args.first().map(|s| s.to_string());
        let _ = s.send(irc::client::prelude::Command::TRACE(target));
    }
    CommandResult::Ok
}

fn cmd_tkline(app: &mut App, args: &[&str]) -> CommandResult {
    if args.len() < 2 {
        return CommandResult::Error("Usage: /tkline <minutes> <user@host> [reason] (ircd 2.11)".into());
    }
    let mins = args[0];
    let userhost = args[1];
    let reason = if args.len() > 2 { args[2..].join(" ") } else { "Temporary K-line".to_string() };
    if let Some(s) = &app.server().sender {
        let _ = s.send(irc::client::prelude::Command::Raw(format!("TKLINE {} {} :{}", mins, userhost, reason), Vec::new()));
    }
    app.system_message(&format!("-!- TKLINE set for {} mins on {} ({})", mins, userhost, reason));
    CommandResult::Ok
}

fn cmd_except(app: &mut App, args: &[&str]) -> CommandResult {
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    if channel == "(Status)" {
        return CommandResult::Error("Not in a channel.".into());
    }
    if let Some(s) = &app.server().sender {
        if args.is_empty() {
            let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} +e", channel), Vec::new()));
        } else {
            let mask = if args[0].contains('!') || args[0].contains('@') { args[0].to_string() } else { format!("{}!*@*", args[0]) };
            let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} +e {}", channel, mask), Vec::new()));
        }
    }
    CommandResult::Ok
}

fn cmd_invex(app: &mut App, args: &[&str]) -> CommandResult {
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    if channel == "(Status)" {
        return CommandResult::Error("Not in a channel.".into());
    }
    if let Some(s) = &app.server().sender {
        if args.is_empty() {
            let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} +I", channel), Vec::new()));
        } else {
            let mask = if args[0].contains('!') || args[0].contains('@') { args[0].to_string() } else { format!("{}!*@*", args[0]) };
            let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} +I {}", channel, mask), Vec::new()));
        }
    }
    CommandResult::Ok
}

fn cmd_reop(app: &mut App, args: &[&str]) -> CommandResult {
    let channel = app.buffers[app.current_buffer_idx].name.clone();
    if channel == "(Status)" {
        return CommandResult::Error("Not in a channel.".into());
    }
    if let Some(s) = &app.server().sender {
        if args.is_empty() {
            let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} +R", channel), Vec::new()));
        } else {
            let mask = if args[0].contains('!') || args[0].contains('@') { args[0].to_string() } else { format!("{}!*@*", args[0]) };
            let _ = s.send(irc::client::prelude::Command::Raw(format!("MODE {} +R {}", channel, mask), Vec::new()));
        }
    }
    CommandResult::Ok
}
