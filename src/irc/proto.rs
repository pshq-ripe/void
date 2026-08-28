use std::collections::HashMap;
use crate::app::{App, MessageType, OutputContext};
use irc::client::prelude::*;

/// Wyślij PRIVMSG — z labeled-response tagiem jeśli serwer wspiera
pub fn send_labeled_privmsg(app: &mut App, target: &str, text: &str) {
    let has_labeled = app.server().server_info.tokens.contains_key("LABELED-RESPONSE");
    let sender = app.server().sender.clone();
    if let Some(s) = sender {
        if has_labeled {
            let label = app.next_label();
            app.pending_labels.insert(label.clone(), format!("PRIVMSG {}", target));
            let msg = format!("@label={} PRIVMSG {} :{}", label, target, text);
            let _ = s.send(Command::Raw(msg, Vec::new()));
        } else {
            let _ = s.send_privmsg(target, text);
        }
    }
}

/// Wyślij NOTICE — z labeled-response tagiem jeśli serwer wspiera
pub fn send_labeled_notice(app: &mut App, target: &str, text: &str) {
    let has_labeled = app.server().server_info.tokens.contains_key("LABELED-RESPONSE");
    let sender = app.server().sender.clone();
    if let Some(s) = sender {
        if has_labeled {
            let label = app.next_label();
            app.pending_labels.insert(label.clone(), format!("NOTICE {}", target));
            let msg = format!("@label={} NOTICE {} :{}", label, target, text);
            let _ = s.send(Command::Raw(msg, Vec::new()));
        } else {
            let _ = s.send_notice(target, text);
        }
    }
}

/// Przetwarzanie wiadomości IRC i aktualizacja stanu App
pub fn handle_irc_message(app: &mut App, msg: &Message) {
    let source = msg.source_nickname().unwrap_or("").to_string();

    // Raw log — zapisz surową wiadomość
    if app.server().raw_log_enabled {
        let raw = format!("{}", msg);
        app.server_mut().raw_log.push(raw);
        // Limit do 1000 wpisów
        if app.server_mut().raw_log.len() > 1000 {
            app.server_mut().raw_log.remove(0);
        }
    }

    // IRCv3 server-time: wyciągnij timestamp z message tags
    let _server_time = msg.tags.as_ref().and_then(|tags| {
        tags.iter()
            .find(|t| t.0 == "time")
            .and_then(|t| t.1.as_deref())
    });

    // IRCv3 labeled-response: wyciągnij label z message tags
    let label = msg.tags.as_ref().and_then(|tags| {
        tags.iter()
            .find(|t| t.0 == "label")
            .and_then(|t| t.1.as_deref())
    });
    if let Some(lbl) = label {
        // Sprawdź czy to odpowiedź na nasz request
        if let Some(desc) = app.pending_labels.remove(lbl) {
            app.system_message(&format!("-!- [label:{}] Response to: {}", lbl, desc));
        }
    }

    // Ustaw kontekst wyjścia (epic6 /ON CONTEXT)
    let target = match &msg.command {
        Command::PRIVMSG(t, _) | Command::NOTICE(t, _) | Command::JOIN(t, _, _) | Command::PART(t, _) => t.clone(),
        Command::KICK(t, _, _) => t.clone(),
        _ => "*".to_string(),
    };
    let level = match &msg.command {
        Command::PRIVMSG(_, _) => "MSG",
        Command::NOTICE(_, _) => "NOTICE",
        Command::JOIN(_, _, _) => "JOINS",
        Command::PART(_, _) => "PARTS",
        Command::QUIT(_) => "QUITS",
        Command::NICK(_) => "NICKS",
        Command::KICK(_, _, _) => "KICKS",
        Command::ChannelMODE(_, _) | Command::UserMODE(_, _) => "MODES",
        Command::TOPIC(_, _) => "TOPICS",
        _ => "OTHER",
    };
    app.output_context = OutputContext {
        server: app.server().host.clone(),
        window: app.buffers[app.current_buffer_idx].name.clone(),
        sender: source.clone(),
        target,
        level: level.to_string(),
    };
    // Reset /SHH na每个 nowy kontekst
    app.suppress_display = false;

    match &msg.command {
        // ─── Odpowiedzi serwera (MOTD, Numerics) ─────────

        // RPL_WELCOME (001), RPL_YOURHOST (002), RPL_CREATED (003), RPL_MYINFO (004)
        Command::Response(resp, args) => {
            handle_server_response(app, *resp, args, &source);
        }

        // ─── PRIVMSG ─────────────────────────────────────
        Command::PRIVMSG(target, text) => {
            // Sprawdź ignorowanie
            if app.is_ignored(&source, "PUBLIC") || app.is_ignored(&source, "MSG") {
                return;
            }

            let buf_name = if target == &app.server().our_nick {
                source.clone() // prywatna wiadomość → bufor nicku
            } else {
                target.clone()
            };

            // Obsługa CTCP
            if text.starts_with('\x01') && text.ends_with('\x01') {
                let ctcp_content = &text[1..text.len()-1];
                handle_ctcp(app, &source, &buf_name, ctcp_content, false);
                return;
            }

            app.get_or_create_buffer(&buf_name);
            app.buffer_message(&buf_name, format!("<{}> {}", source, text), MessageType::Normal);
            app.last_msg_target = Some(source.clone());
        }

        // ─── NOTICE ──────────────────────────────────────
        Command::NOTICE(target, text) => {
            if app.is_ignored(&source, "NOTICE") {
                return;
            }

            // Obsługa CTCP reply
            if text.starts_with('\x01') && text.ends_with('\x01') {
                let ctcp_content = &text[1..text.len()-1];
                handle_ctcp(app, &source, "(Status)", ctcp_content, true);
                return;
            }

            let buf_name = if target == &app.server().our_nick || source.is_empty() {
                "(Status)".to_string()
            } else {
                target.clone()
            };
            let display_source = if source.is_empty() { "Server" } else { &source };
            app.buffer_message(&buf_name, format!("-{}- {}", display_source, text), MessageType::Notice);
        }

        // ─── JOIN (extended-join: channel, account, realname) ──
        Command::JOIN(channel, account, realname) => {
            let channel = channel.trim_start_matches(':').to_string();
            if source == app.server().our_nick {
                app.get_or_create_buffer(&channel);
                app.switch_to_buffer(&channel);
                app.system_message(&format!("-!- Now talking in {}", channel));
            }
            if let Some(buf) = app.get_buffer_mut(&channel) {
                buf.add_nick(&source);
                let acct = account.as_deref().unwrap_or("");
                let real = realname.as_deref().unwrap_or("");
                if !acct.is_empty() || !real.is_empty() {
                    buf.set_nick_info(&source, acct, real);
                }
            }

            // Massjoin batching — zbierz rapid JOINs i wyświetl razem
            let now = std::time::Instant::now();
            let is_rapid = app.server().massjoin_timer
                .map(|t| now.duration_since(t).as_millis() < 500)
                .unwrap_or(false);

            if is_rapid || app.server().netsplit_active {
                // Dodaj do bufora
                app.server_mut().massjoin_buffer.push((source.clone(), channel.clone(), String::new()));
                if app.server().massjoin_timer.is_none() {
                    app.server_mut().massjoin_timer = Some(now);
                }
            } else {
                // Flush poprzedni bufor jeśli jest
                if !app.server().massjoin_buffer.is_empty() {
                    let buffered = app.server().massjoin_buffer.clone();
                    app.server_mut().massjoin_buffer.clear();
                    app.server_mut().massjoin_timer = None;
                    // Pogrupuj po kanale
                    let mut by_channel: HashMap<String, Vec<String>> = HashMap::new();
                    for (nick, ch, _) in buffered {
                        by_channel.entry(ch).or_default().push(nick);
                    }
                    for (ch, nicks) in by_channel {
                        let msg = format!("* {} have joined {}", nicks.join(", "), ch);
                        app.buffer_message(&ch, msg, MessageType::System);
                    }
                }

                // Nowy join — wyświetl normalnie
                let join_msg = match (account.as_deref(), realname.as_deref()) {
                    (Some(acct), Some(real)) if !acct.is_empty() && !real.is_empty() => {
                        format!("* {} has joined {} [{} / {}]", source, channel, acct, real)
                    }
                    _ => format!("* {} has joined {}", source, channel),
                };
                app.buffer_message(&channel, join_msg, MessageType::System);
                app.server_mut().massjoin_timer = Some(now);
            }

            // Netsplit recovery detection
            if app.server().netsplit_active {
                if let Some(pos) = app.server_mut().netsplit_nicks.iter().position(|n| n == &source) {
                    app.server_mut().netsplit_nicks.remove(pos);
                    app.buffer_message(&channel, format!("-!- Netsplit recovery: {} returned", source), MessageType::System);
                    if app.server_mut().netsplit_nicks.is_empty() {
                        let duration = app.server().netsplit_start.map(|s| s.elapsed().as_secs()).unwrap_or(0);
                        app.system_message(&format!("-!- Netsplit over (lasted {}s)", duration));
                        app.server_mut().netsplit_active = false;
                        app.server_mut().netsplit_server.clear();
                        app.server_mut().netsplit_start = None;
                    }
                }
            }

            // Sprawdź notify
            if app.is_on_notify(&source) {
                app.system_message(&format!("-!- Notify: {} is online (joined {})", source, channel));
            }
        }

        // ─── PART ────────────────────────────────────────
        Command::PART(channel, reason) => {
            let r = reason.as_deref().unwrap_or("");
            if source == app.server().our_nick {
                app.close_buffer(channel);
            } else {
                if let Some(buf) = app.get_buffer_mut(channel) {
                    buf.remove_nick(&source);
                }
                let reason_str = if r.is_empty() { String::new() } else { format!(" ({})", r) };
                app.buffer_message(channel, format!("* {} has left {}{}", source, channel, reason_str), MessageType::System);
            }
        }

        // ─── QUIT ────────────────────────────────────────
        Command::QUIT(reason) => {
            let r = reason.as_deref().unwrap_or("");
            let reason_str = if r.is_empty() { String::new() } else { format!(" ({})", r) };

            // Netsplit detection — jeśli quit reason zawiera "*.net *.split"
            let is_netsplit = r.contains(".net") && r.contains(".split");
            if is_netsplit {
                if !app.server().netsplit_active {
                    app.server_mut().netsplit_active = true;
                    app.server_mut().netsplit_nicks.clear();
                    app.server_mut().netsplit_server = r.to_string();
                    app.server_mut().netsplit_start = Some(std::time::Instant::now());
                }
                app.server_mut().netsplit_nicks.push(source.clone());
            }

            // Usuń z list nicków we WSZYSTKICH buforach i wyświetl tam info
            let mut affected_buffers = Vec::new();
            for buf in &mut app.buffers {
                if buf.nicks.iter().any(|n| n.nick == source) {
                    buf.remove_nick(&source);
                    affected_buffers.push(buf.name.clone());
                }
            }
            for buf_name in affected_buffers {
                if is_netsplit {
                    app.buffer_message(&buf_name, format!("* {} has quit IRC (netsplit){}", source, reason_str), MessageType::System);
                } else {
                    app.buffer_message(&buf_name, format!("* {} has quit IRC{}", source, reason_str), MessageType::System);
                }
            }
        }

        // ─── NICK ────────────────────────────────────────
        Command::NICK(new_nick) => {
            if source == app.server().our_nick {
                app.server_mut().our_nick = new_nick.clone();
            }
            // Zaktualizuj we WSZYSTKICH buforach
            let mut affected_buffers = Vec::new();
            for buf in &mut app.buffers {
                if buf.nicks.iter().any(|n| n.nick == source) {
                    buf.rename_nick(&source, new_nick);
                    affected_buffers.push(buf.name.clone());
                }
            }
            for buf_name in affected_buffers {
                app.buffer_message(&buf_name, format!("* {} is now known as {}", source, new_nick), MessageType::System);
            }
        }

        // ─── KICK ────────────────────────────────────────
        Command::KICK(channel, nick, reason) => {
            let r = reason.as_deref().unwrap_or("");
            if nick == &app.server().our_nick {
                app.system_message(&format!("-!- You have been kicked from {} by {} ({})", channel, source, r));
                app.close_buffer(channel);
            } else {
                if let Some(buf) = app.get_buffer_mut(channel) {
                    buf.remove_nick(nick);
                }
                app.buffer_message(channel, format!("* {} was kicked from {} by {} ({})", nick, channel, source, r), MessageType::System);
            }
        }

        // ─── TOPIC ───────────────────────────────────────
        Command::TOPIC(channel, topic) => {
            let topic_text = topic.as_deref().unwrap_or("");
            if let Some(buf) = app.get_buffer_mut(channel) {
                buf.topic = topic_text.to_string();
            }
            app.buffer_message(channel, format!("* {} set topic to: {}", source, topic_text), MessageType::System);
        }

        // ─── MODE (channel) ───────────────────────────────
        Command::ChannelMODE(channel, modes) => {
            let mut mode_str = String::new();
            for m in modes {
                let flag = m.flag();
                let arg = m.arg().unwrap_or("");
                mode_str.push_str(&flag);
                if !arg.is_empty() {
                    mode_str.push(' ');
                    mode_str.push_str(arg);
                    // Aktualizuj prefix nicka na liście
                    match m {
                        Mode::Plus(ChannelMode::Oper, _) => {
                            if let Some(buf) = app.get_buffer_mut(channel) {
                                buf.set_nick_prefix(arg, '@', true);
                            }
                        }
                        Mode::Minus(ChannelMode::Oper, _) => {
                            if let Some(buf) = app.get_buffer_mut(channel) {
                                buf.set_nick_prefix(arg, '@', false);
                            }
                        }
                        Mode::Plus(ChannelMode::Voice, _) => {
                            if let Some(buf) = app.get_buffer_mut(channel) {
                                buf.set_nick_prefix(arg, '+', true);
                            }
                        }
                        Mode::Minus(ChannelMode::Voice, _) => {
                            if let Some(buf) = app.get_buffer_mut(channel) {
                                buf.set_nick_prefix(arg, '+', false);
                            }
                        }
                        Mode::Plus(ChannelMode::Halfop, _) => {
                            if let Some(buf) = app.get_buffer_mut(channel) {
                                buf.set_nick_prefix(arg, '%', true);
                            }
                        }
                        Mode::Minus(ChannelMode::Halfop, _) => {
                            if let Some(buf) = app.get_buffer_mut(channel) {
                                buf.set_nick_prefix(arg, '%', false);
                            }
                        }
                        Mode::Plus(ChannelMode::Founder, _) => {
                            if let Some(buf) = app.get_buffer_mut(channel) {
                                buf.set_nick_prefix(arg, '~', true);
                            }
                        }
                        Mode::Minus(ChannelMode::Founder, _) => {
                            if let Some(buf) = app.get_buffer_mut(channel) {
                                buf.set_nick_prefix(arg, '~', false);
                            }
                        }
                        Mode::Plus(ChannelMode::Admin, _) => {
                            if let Some(buf) = app.get_buffer_mut(channel) {
                                buf.set_nick_prefix(arg, '&', true);
                            }
                        }
                        Mode::Minus(ChannelMode::Admin, _) => {
                            if let Some(buf) = app.get_buffer_mut(channel) {
                                buf.set_nick_prefix(arg, '&', false);
                            }
                        }
                        _ => {}
                    }
                }
            }
            app.buffer_message(channel, format!("* {} sets mode: {}", source, mode_str), MessageType::System);
        }

        // ─── MODE (user) ─────────────────────────────────
        Command::UserMODE(_target, modes) => {
            let mut mode_str = String::new();
            for m in modes {
                mode_str.push_str(&m.flag());
            }
            app.server_mut().user_modes = mode_str.clone();
            app.system_message(&format!("-!- User mode: {}", mode_str));
        }

        // ─── PING (auto-reply) ───────────────────────────
        Command::PING(server, _) => {
            if let Some(s) = &app.server().sender {
                let _ = s.send(Command::PONG(server.clone(), None));
            }
        }

        // ─── PONG (lag measurement) ─────────────────────
        Command::PONG(_server, _token) => {
            if let Some(sent) = app.server().lag_ping_sent {
                let lag = sent.elapsed().as_millis() as u64;
                app.server_mut().lag_ms = lag;
                app.server_mut().lag_ping_sent = None;
            }
        }

        // ─── IRCv3 away-notify ───────────────────────────
        Command::AWAY(msg) => {
            if source == app.server().our_nick {
                app.server_mut().away_message = msg.clone();
            }
            if msg.is_some() {
                app.system_message(&format!("-!- {} is now away: {}", source, msg.as_deref().unwrap_or("")));
            } else {
                app.system_message(&format!("-!- {} is back.", source));
            }
        }

        // ─── IRCv3 account-notify ────────────────────────
        Command::ACCOUNT(account) => {
            if account == "*" {
                app.system_message(&format!("-!- {} has logged out.", source));
            } else {
                app.system_message(&format!("-!- {} has logged in as: {}", source, account));
            }
        }

        // ─── IRCv3 chghost ───────────────────────────────
        Command::CHGHOST(new_user, new_host) => {
            app.system_message(&format!("-!- {} has changed host to: {}@{}", source, new_user, new_host));
        }

        // ─── IRCv3 invite-notify ─────────────────────────
        Command::INVITE(nick, channel) => {
            if *nick == app.server().our_nick {
                app.system_message(&format!("-!- {} invites you to {}", source, channel));
            } else {
                app.buffer_message(&channel, format!("-!- {} invited {} to {}", source, nick, channel), MessageType::System);
            }
        }

        // ─── IRCv3 monitor ──────────────────────────────
        Command::MONITOR(cmd, data) => {
            match cmd.as_str() {
                "L" => {
                    if let Some(list) = data {
                        app.system_message(&format!("-!- Monitor list: {}", list));
                    }
                }
                "S" => {
                    if let Some(status) = data {
                        app.system_message(&format!("-!- Monitor status: {}", status));
                    }
                }
                _ => {}
            }
        }

        _ => {}
    }
}

/// Obsługa odpowiedzi numerycznych serwera
fn handle_server_response(app: &mut App, resp: Response, args: &[String], _source: &str) {
    let text = if args.len() > 1 {
        args[1..].join(" ")
    } else {
        args.join(" ")
    };

    match resp {
        // ─── Powitanie serwera i ISUPPORT ─────────────
        Response::RPL_WELCOME => {
            app.system_message(&text);
            // Auto-identify z NickServ jeśli hasło jest skonfigurowane
            if let Some(ref pass) = app.server().nick_password {
                if let Some(s) = &app.server().sender {
                    let _ = s.send_privmsg("NickServ", &format!("IDENTIFY {}", pass));
                    app.system_message("-!- Sent IDENTIFY to NickServ.");
                }
            }
        }
        Response::RPL_YOURHOST | Response::RPL_CREATED => {
            app.system_message(&text);
        }
        Response::RPL_MYINFO => {
            app.system_message(&format!("-!- Server info: {}", text));
        }
        Response::RPL_ISUPPORT => {
            if args.len() >= 2 {
                // Ostatni arg to ":are supported by this server" — pomiń
                let tokens = &args[1..args.len().saturating_sub(1)];
                for token in tokens {
                    if let Some((key, value)) = token.split_once('=') {
                        let key_upper = key.to_uppercase();
                        match key_upper.as_str() {
                            "NETWORK" => app.server_mut().server_info.network = value.to_string(),
                            "CHANTYPES" => app.server_mut().server_info.chantypes = value.to_string(),
                            "PREFIX" => app.server_mut().server_info.prefix_modes = value.to_string(),
                            "CHANMODES" => app.server_mut().server_info.chanmodes = value.to_string(),
                            "NICKLEN" => app.server_mut().server_info.nicklen = value.parse().ok(),
                            "TOPICLEN" => app.server_mut().server_info.topiclen = value.parse().ok(),
                            "CHANNELLEN" => app.server_mut().server_info.channellen = value.parse().ok(),
                            "MODES" => app.server_mut().server_info.modes = value.parse().ok(),
                            _ => {}
                        }
                        app.server_mut().server_info.tokens.insert(key_upper, value.to_string());
                    } else {
                        app.server_mut().server_info.tokens.insert(token.to_uppercase(), String::new());
                    }
                }
                let params = tokens.join(" ");
                app.system_message(&format!("-!- ISUPPORT: {}", params));
            }
        }

        // ─── ircd 2.11 Specifics (042, 043) ─────────
        _ if resp as u16 == 42 => {
            if args.len() >= 3 {
                app.system_message(&format!("-!- Your unique ID (UID): {}", args[2]));
            }
        }
        _ if resp as u16 == 43 => {
            if args.len() >= 3 {
                app.system_message(&format!("-!- Nick collision: nick forced to UID {}", args[2]));
                app.server_mut().our_nick = args[2].clone();
            }
        }

        // ─── MOTD ────────────────────────────────────
        Response::RPL_MOTDSTART | Response::RPL_MOTD => {
            app.system_message(&text);
        }
        Response::RPL_ENDOFMOTD => {
            app.system_message(&text);
            app.system_message("-!- End of MOTD.");
        }

        // ─── NAMES odpowiedź (353) ───────────────────
        Response::RPL_NAMREPLY => {
            if args.len() >= 4 {
                let channel = &args[2];
                if let Some(nick_list_str) = args.last() {
                    let buf = app.get_or_create_buffer(channel);
                    for n in nick_list_str.split_whitespace() {
                        buf.add_nick(n);
                    }
                }
            }
        }
        Response::RPL_ENDOFNAMES => {
            // Cicha odpowiedź — lista nicków już załadowana
        }

        // ─── TOPIC odpowiedź (332, 333) ─────────────
        Response::RPL_TOPIC => {
            if args.len() >= 3 {
                let channel = &args[1];
                let topic_text = &args[2];
                if let Some(buf) = app.get_buffer_mut(channel) {
                    buf.topic = topic_text.clone();
                }
                app.buffer_message(channel, format!("-!- Topic for {}: {}", channel, topic_text), MessageType::ServerReply);
            }
        }
        Response::RPL_TOPICWHOTIME => {
            if args.len() >= 4 {
                let channel = &args[1];
                let set_by = &args[2];
                if let Ok(ts) = args[3].parse::<i64>() {
                    let date_str = chrono::DateTime::from_timestamp(ts, 0)
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_else(|| args[3].clone());
                    app.buffer_message(channel, format!("-!- Topic set by {} on {}", set_by, date_str), MessageType::ServerReply);
                } else {
                    app.buffer_message(channel, format!("-!- Topic set by {}", set_by), MessageType::ServerReply);
                }
            }
        }

        // ─── WHOIS ──────────────────────────────────
        Response::RPL_WHOISUSER => {
            if args.len() >= 6 {
                app.system_message(&format!("-!- {} ({}@{}) : {}", args[1], args[2], args[3], args[5]));
            }
        }
        Response::RPL_WHOISCHANNELS => {
            if args.len() >= 3 {
                app.system_message(&format!("-!- {} is on: {}", args[1], args[2]));
            }
        }
        Response::RPL_WHOISSERVER => {
            if args.len() >= 4 {
                app.system_message(&format!("-!- {} using server: {} ({})", args[1], args[2], args[3]));
            }
        }
        Response::RPL_WHOISIDLE => {
            if args.len() >= 4 {
                let idle_secs = args[2].parse::<u64>().unwrap_or(0);
                let idle_min = idle_secs / 60;
                let idle_s = idle_secs % 60;
                if args.len() >= 5 {
                    if let Ok(ts) = args[3].parse::<i64>() {
                        let signon = chrono::DateTime::from_timestamp(ts, 0)
                            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                            .unwrap_or_else(|| args[3].clone());
                        app.system_message(&format!("-!- {} idle {}m {}s, signed on: {}", args[1], idle_min, idle_s, signon));
                    } else {
                        app.system_message(&format!("-!- {} idle {}m {}s", args[1], idle_min, idle_s));
                    }
                } else {
                    app.system_message(&format!("-!- {} idle {}m {}s", args[1], idle_min, idle_s));
                }
            }
        }
        Response::RPL_ENDOFWHOIS => {
            app.system_message("-!- End of /WHOIS.");
        }

        // ─── Lista Banów/Except/Invite/Reop (+b, +e, +I, +R) ───
        Response::RPL_BANLIST => {
            if args.len() >= 3 {
                let channel = &args[1];
                let mask = &args[2];
                let set_by = if args.len() >= 4 { args[3].clone() } else { String::new() };
                let timestamp = if args.len() >= 5 { args[4].parse::<i64>().unwrap_or(0) } else { 0 };
                // Track ban in list
                app.server_mut().ban_list.push(crate::app::BanEntry {
                    channel: channel.clone(),
                    mask: mask.clone(),
                    set_by: set_by.clone(),
                    timestamp,
                });
                let set_by_str = if set_by.is_empty() { String::new() } else { format!(" by {}", set_by) };
                app.buffer_message(channel, format!("-!- Ban: {}{}", mask, set_by_str), MessageType::ServerReply);
            }
        }
        Response::RPL_ENDOFBANLIST => {
            if args.len() >= 2 {
                let channel = &args[1];
                let count = app.server_mut().ban_list.iter().filter(|b| b.channel == *channel).count();
                app.buffer_message(channel, format!("-!- End of Channel Ban List ({} bans).", count), MessageType::ServerReply);
            }
        }
        Response::RPL_EXCEPTLIST => {
            if args.len() >= 3 {
                let channel = &args[1];
                let mask = &args[2];
                let set_by = if args.len() >= 4 { format!(" by {}", args[3]) } else { String::new() };
                app.buffer_message(channel, format!("-!- Ban Exception (+e): {}{}", mask, set_by), MessageType::ServerReply);
            }
        }
        Response::RPL_ENDOFEXCEPTLIST => {
            if args.len() >= 2 {
                app.buffer_message(&args[1], "-!- End of Channel Exception List.".to_string(), MessageType::ServerReply);
            }
        }
        Response::RPL_INVITELIST => {
            if args.len() >= 3 {
                let channel = &args[1];
                let mask = &args[2];
                let set_by = if args.len() >= 4 { format!(" by {}", args[3]) } else { String::new() };
                app.buffer_message(channel, format!("-!- Invite Exception (+I): {}{}", mask, set_by), MessageType::ServerReply);
            }
        }
        Response::RPL_ENDOFINVITELIST => {
            if args.len() >= 2 {
                app.buffer_message(&args[1], "-!- End of Channel Invite List.".to_string(), MessageType::ServerReply);
            }
        }
        _ if resp as u16 == 344 => {
            if args.len() >= 3 {
                let channel = &args[1];
                let mask = &args[2];
                let set_by = if args.len() >= 4 { format!(" by {}", args[3]) } else { String::new() };
                app.buffer_message(channel, format!("-!- Reop Hint (+R): {}{}", mask, set_by), MessageType::ServerReply);
            }
        }
        _ if resp as u16 == 345 => {
            if args.len() >= 2 {
                app.buffer_message(&args[1], "-!- End of Channel Reop List.".to_string(), MessageType::ServerReply);
            }
        }

        // ─── AWAY ────────────────────────────────────
        Response::RPL_AWAY => {
            if args.len() >= 3 {
                app.system_message(&format!("-!- {} is away: {}", args[1], args[2]));
            }
        }
        Response::RPL_UNAWAY => {
            app.system_message("-!- You are no longer marked as away.");
        }
        Response::RPL_NOWAWAY => {
            app.system_message("-!- You are now marked as away.");
        }

        // ─── ISON (notify check) ─────────────────────
        Response::RPL_ISON => {
            if args.len() >= 2 {
                let online_nicks: Vec<&str> = args[1].split_whitespace().collect();
                let mut messages = Vec::new();
                let mut whois_nicks = Vec::new();
                for notify in &mut app.notify_list {
                    let was_online = notify.online;
                    notify.online = online_nicks.iter().any(|n| n.eq_ignore_ascii_case(&notify.nick));
                    if notify.online && !was_online {
                        notify.last_seen = Some(std::time::Instant::now());
                        messages.push(format!("-!- Notify: {} is now ONLINE", notify.nick));
                        whois_nicks.push(notify.nick.clone());
                    } else if !notify.online && was_online {
                        notify.verified = false;
                        notify.userhost.clear();
                        notify.channels.clear();
                        messages.push(format!("-!- Notify: {} is now OFFLINE", notify.nick));
                    }
                }
                for msg in messages {
                    app.system_message(&msg);
                }
                // WHOIS verification for newly online nicks
                for nick in whois_nicks {
                    if let Some(s) = &app.server().sender {
                        let _ = s.send(irc::client::prelude::Command::WHOIS(None, nick));
                    }
                }
            }
        }

        // ─── WHO odpowiedź ───────────────────────────
        Response::RPL_WHOREPLY => {
            // Format: <nick> <user>@<host> <server> <flags> <realname>
            if args.len() >= 7 {
                let _channel = &args[1];
                let user = &args[2];
                let host = &args[3];
                let server = &args[4];
                let nick = &args[5];
                let flags = &args[6];
                let realname = if args.len() >= 8 {
                    args[7..].join(" ")
                } else {
                    String::new()
                };
                // Usuń hopcount z realname jeśli jest
                let realname_clean = realname.split_once(' ').map(|(_, r)| r).unwrap_or(&realname);
                app.system_message(&format!("-!- {} {}@{} ({}) [{}] {}",
                    nick, user, host, server, flags, realname_clean));
            } else {
                app.system_message(&format!("-!- {}", text));
            }
        }
        Response::RPL_ENDOFWHO => {
            app.system_message("-!- End of /WHO.");
        }

        // ─── WHOX (354) — extended WHO response ─────────
        _ if resp as u16 == 354 => {
            // Format: 354 <nick> <channel> <user> <host> <server> <nick> <flags> <account> <realname>
            if args.len() >= 8 {
                let channel = &args[1];
                let user = &args[2];
                let host = &args[3];
                let nick = &args[5];
                let flags = &args[6];
                let account = &args[7];
                let realname = args.get(8).map(|s| s.as_str()).unwrap_or("");
                app.system_message(&format!("-!- {} {}@{} ({}) [{}] acct:{} {}",
                    nick, user, host, channel, flags, account, realname));
            } else {
                app.system_message(&format!("-!- WHOX: {}", text));
            }
        }

        // ─── Błędy ──────────────────────────────────
        Response::ERR_NOSUCHNICK => {
            if args.len() >= 2 {
                app.system_message(&format!("-!- {}: No such nick/channel.", args[1]));
            }
        }
        Response::ERR_NOSUCHCHANNEL => {
            if args.len() >= 2 {
                app.system_message(&format!("-!- {}: No such channel.", args[1]));
            }
        }
        Response::ERR_CANNOTSENDTOCHAN => {
            if args.len() >= 2 {
                app.system_message(&format!("-!- Cannot send to channel {}.", args[1]));
            }
        }
        Response::ERR_NICKNAMEINUSE => {
            if args.len() >= 2 {
                app.system_message(&format!("-!- Nickname {} is already in use.", args[1]));
                // Dodaj podkreślnik i spróbuj ponownie
                let new_nick = format!("{}_", app.server().our_nick);
                if let Some(s) = &app.server().sender {
                    let _ = s.send(Command::NICK(new_nick.clone()));
                }
                app.server_mut().our_nick = new_nick;
            }
        }
        Response::ERR_CHANOPRIVSNEEDED => {
            if args.len() >= 2 {
                app.system_message(&format!("-!- {}: You need channel operator privileges.", args[1]));
            }
        }
        Response::ERR_NOTONCHANNEL => {
            if args.len() >= 2 {
                app.system_message(&format!("-!- {}: You're not on that channel.", args[1]));
            }
        }
        Response::ERR_BANNEDFROMCHAN => {
            if args.len() >= 2 {
                app.system_message(&format!("-!- {}: You are banned from this channel.", args[1]));
            }
        }

        // ─── MONITOR (730-733) ──────────────────────────
        _ if resp as u16 == 730 => {
            // RPL_MONONLINE — monitorowane nicki są online
            if args.len() >= 2 {
                let nicks_str = args[1].trim_start_matches(':');
                let mut messages = Vec::new();
                for nick in nicks_str.split(',') {
                    let nick = nick.trim();
                    for notify in &mut app.notify_list {
                        if notify.nick.eq_ignore_ascii_case(nick) && !notify.online {
                            notify.online = true;
                            messages.push(format!("-!- Monitor: {} is now ONLINE", nick));
                        }
                    }
                }
                for msg in messages { app.system_message(&msg); }
            }
        }
        _ if resp as u16 == 731 => {
            // RPL_MONOFFLINE — monitorowane nicki są offline
            if args.len() >= 2 {
                let nicks_str = args[1].trim_start_matches(':');
                let mut messages = Vec::new();
                for nick in nicks_str.split(',') {
                    let nick = nick.trim();
                    for notify in &mut app.notify_list {
                        if notify.nick.eq_ignore_ascii_case(nick) && notify.online {
                            notify.online = false;
                            messages.push(format!("-!- Monitor: {} is now OFFLINE", nick));
                        }
                    }
                }
                for msg in messages { app.system_message(&msg); }
            }
        }
        _ if resp as u16 == 732 => {
            // RPL_MONLIST — lista monitorowanych nicków
            if args.len() >= 2 {
                let nicks_str = args[1].trim_start_matches(':');
                app.system_message(&format!("-!- Monitor list: {}", nicks_str));
            }
        }
        _ if resp as u16 == 733 => {
            // RPL_ENDOFMONLIST
            app.system_message("-!- End of monitor list.");
        }

        // ─── STARTTLS (670) ─────────────────────────────
        _ if resp as u16 == 670 => {
            app.system_message("-!- STARTTLS: Server supports TLS upgrade.");
            app.system_message("-!- Note: Reconnect with TLS enabled for secure connection.");
        }

        // ─── Nierozpoznane (wypisz jako raw) ─────────
        _ => {
            // Wyswietl odpowiedzi serwera w oknie statusu
            if !text.is_empty() {
                app.system_message(&text);
            }
        }
    }
}

/// Obsługa CTCP
fn handle_ctcp(app: &mut App, source: &str, buf_name: &str, content: &str, is_reply: bool) {
    if app.is_ignored(source, "CTCP") {
        return;
    }

    let parts: Vec<&str> = content.splitn(2, ' ').collect();
    let ctcp_type = parts[0].to_uppercase();
    let ctcp_args = if parts.len() > 1 { parts[1] } else { "" };

    if is_reply {
        // Odpowiedź na nasze CTCP
        app.system_message(&format!("-!- CTCP {} reply from {}: {}", ctcp_type, source, ctcp_args));
        return;
    }

    match ctcp_type.as_str() {
        "ACTION" => {
            app.buffer_message(buf_name, format!("* {} {}", source, ctcp_args), MessageType::Action);
        }
        "VERSION" => {
            if app.settings.get_bool("CTCP_REPLY") {
                let version = app.settings.get("CTCP_VERSION");
                if let Some(s) = &app.server().sender {
                    let _ = s.send(Command::NOTICE(
                        source.to_string(),
                        format!("\x01VERSION {}\x01", version),
                    ));
                }
            }
            app.system_message(&format!("-!- CTCP VERSION from {}", source));
        }
        "PING" => {
            if app.settings.get_bool("CTCP_REPLY") {
                if let Some(s) = &app.server().sender {
                    let _ = s.send(Command::NOTICE(
                        source.to_string(),
                        format!("\x01PING {}\x01", ctcp_args),
                    ));
                }
            }
            app.system_message(&format!("-!- CTCP PING from {}", source));
        }
        "TIME" => {
            if app.settings.get_bool("CTCP_REPLY") {
                let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                if let Some(s) = &app.server().sender {
                    let _ = s.send(Command::NOTICE(
                        source.to_string(),
                        format!("\x01TIME {}\x01", now),
                    ));
                }
            }
            app.system_message(&format!("-!- CTCP TIME from {}", source));
        }
        "CLIENTINFO" => {
            if app.settings.get_bool("CTCP_REPLY") {
                if let Some(s) = &app.server().sender {
                    let _ = s.send(Command::NOTICE(
                        source.to_string(),
                        format!("\x01CLIENTINFO ACTION VERSION PING TIME CLIENTINFO DCC\x01"),
                    ));
                }
            }
            app.system_message(&format!("-!- CTCP CLIENTINFO from {}", source));
        }
        "DCC" => {
            if let Some((dcc_type, filename, filesize, addr)) = crate::dcc::DccManager::parse_dcc_request(ctcp_args) {
                let type_str = match dcc_type {
                    crate::dcc::DccType::Chat => "CHAT",
                    crate::dcc::DccType::Send => "SEND",
                    crate::dcc::DccType::Get => "GET",
                };
                let size_str = filesize.map(|s| format!(" ({} bytes)", s)).unwrap_or_default();
                let id = app.dcc.add_pending(dcc_type, source, Some(&filename), filesize, addr);
                app.system_message(&format!("-!- DCC {} request from {} — file: {}{} — /dcc get {} to accept",
                    type_str, source, filename, size_str, id));
            } else {
                app.system_message(&format!("-!- Unknown DCC request from {}: {}", source, ctcp_args));
            }
        }
        _ => {
            app.system_message(&format!("-!- Unknown CTCP {} from {}: {}", ctcp_type, source, ctcp_args));
        }
    }
}
