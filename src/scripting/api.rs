use mlua::{Lua, Result as LuaResult, Function, Value};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::mpsc;

// ─── Helper functions for xform/base64/base85/hex/url ──

fn base64_encode_str(input: &str) -> String {
    base64_encode_bytes(input.as_bytes())
}

fn base64_encode_bytes(bytes: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 { result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char); } else { result.push('='); }
        if chunk.len() > 2 { result.push(CHARS[(triple & 0x3F) as usize] as char); } else { result.push('='); }
    }
    result
}

fn base64_decode_str(input: &str) -> String {
    let input = input.trim_end_matches('=');
    let chars: Vec<u8> = input.bytes().map(|b| match b {
        b'A'..=b'Z' => b - b'A',
        b'a'..=b'z' => b - b'a' + 26,
        b'0'..=b'9' => b - b'0' + 52,
        b'+' => 62, b'/' => 63, _ => 0,
    }).collect();
    let mut result = Vec::new();
    for chunk in chars.chunks(4) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).map(|&b| b as u32).unwrap_or(0);
        let b2 = chunk.get(2).map(|&b| b as u32).unwrap_or(0);
        let triple = (b0 << 18) | (b1 << 12) | (b2 << 6);
        result.push((triple >> 16) as u8);
        if chunk.len() > 2 { result.push(((triple >> 8) & 0xFF) as u8); }
        if chunk.len() > 3 { result.push((triple & 0xFF) as u8); }
    }
    String::from_utf8_lossy(&result).to_string()
}

fn base85_encode(input: &str) -> String {
    // ASCII85 (Adobe variant) with <~ ~> wrapper
    let bytes = input.as_bytes();
    let mut result = String::from("<~");
    for chunk in bytes.chunks(4) {
        let mut val: u32 = 0;
        for (i, &b) in chunk.iter().enumerate() {
            val |= (b as u32) << (24 - i * 8);
        }
        let mut encoded = [0u8; 5];
        for i in (0..5).rev() {
            encoded[i] = (val % 85 + 33) as u8;
            val /= 85;
        }
        let len = chunk.len() + 1;
        for i in 0..len {
            result.push(encoded[i] as char);
        }
    }
    result.push_str("~>");
    result
}

fn base85_decode(input: &str) -> String {
    let input = input.trim_start_matches("<~").trim_end_matches("~>");
    let bytes: Vec<u8> = input.bytes().filter(|b| *b >= 33 && *b <= 117).collect();
    let mut result = Vec::new();
    for chunk in bytes.chunks(5) {
        let mut val: u32 = 0;
        for &b in chunk {
            val = val * 85 + (b as u32 - 33);
        }
        let len = chunk.len() - 1;
        for i in 0..len {
            result.push(((val >> (24 - i * 8)) & 0xFF) as u8);
        }
    }
    String::from_utf8_lossy(&result).to_string()
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn hex_decode(hex: &str) -> Vec<u8> {
    let hex = hex.trim_start_matches("0x");
    (0..hex.len())
        .step_by(2)
        .filter_map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok())
        .collect()
}

fn url_encode(input: &str) -> String {
    input.bytes().map(|b| match b {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => (b as char).to_string(),
        _ => format!("%{:02X}", b),
    }).collect()
}

fn url_decode(input: &str) -> String {
    let mut result = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(
                std::str::from_utf8(&bytes[i+1..i+3]).unwrap_or("00"), 16
            ) {
                result.push(byte);
                i += 3;
                continue;
            }
        }
        result.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&result).to_string()
}

/// Komenda z Lua do wykonania w głównej pętli
#[derive(Clone, Debug)]
pub struct LuaCommand {
    pub raw: String,
}

/// Rejestr hooków Lua — komendy i zdarzenia zarejestrowane ze skryptów
pub struct LuaHooks {
    /// Komendy zarejestrowane z Lua: nazwa -> nazwa funkcji Lua
    pub commands: HashMap<String, String>,
    /// Event hooki: event_type -> lista nazw funkcji Lua
    pub events: HashMap<String, Vec<String>>,
}

impl LuaHooks {
    pub fn new() -> Self {
        LuaHooks {
            commands: HashMap::new(),
            events: HashMap::new(),
        }
    }
}

/// Współdzielony kontekst Lua — stan dostępny z poziomu skryptów
pub struct LuaContext {
    pub our_nick: String,
    pub current_channel: String,
    pub server_host: String,
    pub connected: bool,
    pub cmd_tx: mpsc::Sender<LuaCommand>,
    pub settings: HashMap<String, String>,
}

/// Inicjalizacja Lua API — rejestruje tabelę `void` z pełnym API
pub fn register_api(lua: &Lua, hooks: Arc<Mutex<LuaHooks>>, ctx: Arc<Mutex<LuaContext>>) -> LuaResult<()> {
    let void_table = lua.create_table()?;

    // ─── void.register_command(name, fn) ─────────────
    {
        let hooks = hooks.clone();
        let register_cmd = lua.create_function(move |_, (name, fn_name): (String, String)| {
            let mut h = hooks.lock().unwrap();
            h.commands.insert(name.to_uppercase(), fn_name);
            Ok(())
        })?;
        void_table.set("register_command", register_cmd)?;
    }

    // ─── void.on(event, fn) ──────────────────────────
    {
        let hooks = hooks.clone();
        let on_event = lua.create_function(move |_, (event, fn_name): (String, String)| {
            let mut h = hooks.lock().unwrap();
            h.events.entry(event.to_uppercase()).or_insert_with(Vec::new).push(fn_name);
            Ok(())
        })?;
        void_table.set("on", on_event)?;
    }

    // ─── void.echo(text) — wyświetl w statusie ───────
    {
        let ctx = ctx.clone();
        let echo_fn = lua.create_function(move |_, text: String| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("ECHO {}", text),
            });
            Ok(())
        })?;
        void_table.set("echo", echo_fn)?;
    }

    // ─── void.version() ──────────────────────────────
    {
        let version_fn = lua.create_function(|_, ()| Ok("void 0.2.0"))?;
        void_table.set("version", version_fn)?;
    }

    // ─── void.set(key, value) ────────────────────────
    {
        let ctx = ctx.clone();
        let set_fn = lua.create_function(move |_, (key, value): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("SET {} {}", key, value),
            });
            Ok(())
        })?;
        void_table.set("set", set_fn)?;
    }

    // ─── void.get(key) — odczytaj ustawienie ──────────
    {
        let ctx = ctx.clone();
        let get_fn = lua.create_function(move |_, key: String| {
            let ctx = ctx.lock().unwrap();
            Ok(ctx.settings.get(&key.to_uppercase()).cloned().unwrap_or_default())
        })?;
        void_table.set("get", get_fn)?;
    }

    // ─── void.send(raw) — wyślij surowy IRC ─────────
    {
        let ctx = ctx.clone();
        let send_fn = lua.create_function(move |_, raw: String| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("RAW {}", raw),
            });
            Ok(())
        })?;
        void_table.set("send", send_fn)?;
    }

    // ─── void.nick() — aktualny nick ─────────────────
    {
        let ctx = ctx.clone();
        let nick_fn = lua.create_function(move |_, ()| {
            let ctx = ctx.lock().unwrap();
            Ok(ctx.our_nick.clone())
        })?;
        void_table.set("nick", nick_fn)?;
    }

    // ─── void.channel() — aktualny kanał ─────────────
    {
        let ctx = ctx.clone();
        let channel_fn = lua.create_function(move |_, ()| {
            let ctx = ctx.lock().unwrap();
            Ok(ctx.current_channel.clone())
        })?;
        void_table.set("channel", channel_fn)?;
    }

    // ─── void.server() — aktualny serwer ─────────────
    {
        let ctx = ctx.clone();
        let server_fn = lua.create_function(move |_, ()| {
            let ctx = ctx.lock().unwrap();
            Ok(ctx.server_host.clone())
        })?;
        void_table.set("server", server_fn)?;
    }

    // ─── void.connected() — czy połączony ────────────
    {
        let ctx = ctx.clone();
        let connected_fn = lua.create_function(move |_, ()| {
            let ctx = ctx.lock().unwrap();
            Ok(ctx.connected)
        })?;
        void_table.set("connected", connected_fn)?;
    }

    // ─── void.msg(target, text) ──────────────────────
    {
        let ctx = ctx.clone();
        let msg_fn = lua.create_function(move |_, (target, text): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("MSG {} {}", target, text),
            });
            Ok(())
        })?;
        void_table.set("msg", msg_fn)?;
    }

    // ─── void.notice(target, text) ───────────────────
    {
        let ctx = ctx.clone();
        let notice_fn = lua.create_function(move |_, (target, text): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("NOTICE {} {}", target, text),
            });
            Ok(())
        })?;
        void_table.set("notice", notice_fn)?;
    }

    // ─── void.me(target, action) ─────────────────────
    {
        let ctx = ctx.clone();
        let me_fn = lua.create_function(move |_, (_target, action): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("ME {}", action),
            });
            Ok(())
        })?;
        void_table.set("me", me_fn)?;
    }

    // ─── void.ctcp(target, type, args) ───────────────
    {
        let ctx = ctx.clone();
        let ctcp_fn = lua.create_function(move |_, (target, ctcp_type, args): (String, String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("CTCP {} {} {}", target, ctcp_type, args),
            });
            Ok(())
        })?;
        void_table.set("ctcp", ctcp_fn)?;
    }

    // ─── void.join(channel, key) ─────────────────────
    {
        let ctx = ctx.clone();
        let join_fn = lua.create_function(move |_, (channel, key): (String, Option<String>)| {
            let ctx = ctx.lock().unwrap();
            let cmd = match key {
                Some(k) => format!("JOIN {} {}", channel, k),
                None => format!("JOIN {}", channel),
            };
            let _ = ctx.cmd_tx.try_send(LuaCommand { raw: cmd });
            Ok(())
        })?;
        void_table.set("join", join_fn)?;
    }

    // ─── void.part(channel, reason) ──────────────────
    {
        let ctx = ctx.clone();
        let part_fn = lua.create_function(move |_, (channel, reason): (String, Option<String>)| {
            let ctx = ctx.lock().unwrap();
            let cmd = match reason {
                Some(r) => format!("PART {} {}", channel, r),
                None => format!("PART {}", channel),
            };
            let _ = ctx.cmd_tx.try_send(LuaCommand { raw: cmd });
            Ok(())
        })?;
        void_table.set("part", part_fn)?;
    }

    // ─── void.op(channel, nick) ──────────────────────
    {
        let ctx = ctx.clone();
        let op_fn = lua.create_function(move |_, (channel, nick): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("MODE {} +o {}", channel, nick),
            });
            Ok(())
        })?;
        void_table.set("op", op_fn)?;
    }

    // ─── void.deop(channel, nick) ────────────────────
    {
        let ctx = ctx.clone();
        let deop_fn = lua.create_function(move |_, (channel, nick): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("MODE {} -o {}", channel, nick),
            });
            Ok(())
        })?;
        void_table.set("deop", deop_fn)?;
    }

    // ─── void.voice(channel, nick) ───────────────────
    {
        let ctx = ctx.clone();
        let voice_fn = lua.create_function(move |_, (channel, nick): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("MODE {} +v {}", channel, nick),
            });
            Ok(())
        })?;
        void_table.set("voice", voice_fn)?;
    }

    // ─── void.devoice(channel, nick) ─────────────────
    {
        let ctx = ctx.clone();
        let devoice_fn = lua.create_function(move |_, (channel, nick): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("MODE {} -v {}", channel, nick),
            });
            Ok(())
        })?;
        void_table.set("devoice", devoice_fn)?;
    }

    // ─── void.ban(channel, mask) ─────────────────────
    {
        let ctx = ctx.clone();
        let ban_fn = lua.create_function(move |_, (channel, mask): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("MODE {} +b {}", channel, mask),
            });
            Ok(())
        })?;
        void_table.set("ban", ban_fn)?;
    }

    // ─── void.unban(channel, mask) ───────────────────
    {
        let ctx = ctx.clone();
        let unban_fn = lua.create_function(move |_, (channel, mask): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("MODE {} -b {}", channel, mask),
            });
            Ok(())
        })?;
        void_table.set("unban", unban_fn)?;
    }

    // ─── void.kick(channel, nick, reason) ────────────
    {
        let ctx = ctx.clone();
        let kick_fn = lua.create_function(move |_, (channel, nick, reason): (String, String, Option<String>)| {
            let ctx = ctx.lock().unwrap();
            let cmd = match reason {
                Some(r) => format!("KICK {} {} {}", channel, nick, r),
                None => format!("KICK {} {}", channel, nick),
            };
            let _ = ctx.cmd_tx.try_send(LuaCommand { raw: cmd });
            Ok(())
        })?;
        void_table.set("kick", kick_fn)?;
    }

    // ─── void.mode(channel, modes) ───────────────────
    {
        let ctx = ctx.clone();
        let mode_fn = lua.create_function(move |_, (channel, modes): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("MODE {} {}", channel, modes),
            });
            Ok(())
        })?;
        void_table.set("mode", mode_fn)?;
    }

    // ─── void.topic(channel, text) ───────────────────
    {
        let ctx = ctx.clone();
        let topic_fn = lua.create_function(move |_, (channel, text): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("TOPIC {} {}", channel, text),
            });
            Ok(())
        })?;
        void_table.set("topic", topic_fn)?;
    }

    // ─── void.invite(nick, channel) ──────────────────
    {
        let ctx = ctx.clone();
        let invite_fn = lua.create_function(move |_, (nick, channel): (String, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("INVITE {} {}", nick, channel),
            });
            Ok(())
        })?;
        void_table.set("invite", invite_fn)?;
    }

    // ─── void.whois(nick) ────────────────────────────
    {
        let ctx = ctx.clone();
        let whois_fn = lua.create_function(move |_, nick: String| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("WHOIS {}", nick),
            });
            Ok(())
        })?;
        void_table.set("whois", whois_fn)?;
    }

    // ─── void.nick_change(newnick) ───────────────────
    {
        let ctx = ctx.clone();
        let nick_fn = lua.create_function(move |_, newnick: String| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("NICK {}", newnick),
            });
            Ok(())
        })?;
        void_table.set("nick_change", nick_fn)?;
    }

    // ─── void.away(message) ──────────────────────────
    {
        let ctx = ctx.clone();
        let away_fn = lua.create_function(move |_, msg: Option<String>| {
            let ctx = ctx.lock().unwrap();
            let cmd = match msg {
                Some(m) => format!("AWAY {}", m),
                None => "AWAY".to_string(),
            };
            let _ = ctx.cmd_tx.try_send(LuaCommand { raw: cmd });
            Ok(())
        })?;
        void_table.set("away", away_fn)?;
    }

    // ─── void.quit(reason) ───────────────────────────
    {
        let ctx = ctx.clone();
        let quit_fn = lua.create_function(move |_, reason: Option<String>| {
            let ctx = ctx.lock().unwrap();
            let cmd = match reason {
                Some(r) => format!("QUIT {}", r),
                None => "QUIT".to_string(),
            };
            let _ = ctx.cmd_tx.try_send(LuaCommand { raw: cmd });
            Ok(())
        })?;
        void_table.set("quit", quit_fn)?;
    }

    // ─── void.timer(seconds, fn_name) ────────────────
    {
        let ctx = ctx.clone();
        let timer_fn = lua.create_function(move |_, (seconds, fn_name): (f64, String)| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("TIMER {} 1 {}", seconds, fn_name),
            });
            Ok(())
        })?;
        void_table.set("timer", timer_fn)?;
    }

    // ─── void.match(pattern, string) ─────────────────
    {
        let match_fn = lua.create_function(|_, (pattern, text): (String, String)| {
            // Prosty pattern matching z *
            if pattern.contains('*') {
                let parts: Vec<&str> = pattern.split('*').collect();
                if parts.len() == 2 {
                    Ok(text.starts_with(parts[0]) && text.ends_with(parts[1]))
                } else {
                    Ok(text.contains(&pattern.replace('*', "")))
                }
            } else {
                Ok(text.contains(&pattern))
            }
        })?;
        void_table.set("match", match_fn)?;
    }

    // ─── void.strip(text) — usuń formatowanie IRC ────
    {
        let strip_fn = lua.create_function(|_, text: String| {
            let stripped: String = text.chars()
                .filter(|c| !c.is_control())
                .collect();
            Ok(stripped)
        })?;
        void_table.set("strip", strip_fn)?;
    }

    // ─── void.length(text) ───────────────────────────
    {
        let len_fn = lua.create_function(|_, text: String| Ok(text.len()))?;
        void_table.set("length", len_fn)?;
    }

    // ─── void.sub(text, start, len) ──────────────────
    {
        let sub_fn = lua.create_function(|_, (text, start, len): (String, usize, Option<usize>)| {
            let s = text.get(start..).unwrap_or("");
            match len {
                Some(l) => Ok(s.get(..l).unwrap_or(s).to_string()),
                None => Ok(s.to_string()),
            }
        })?;
        void_table.set("sub", sub_fn)?;
    }

    // ─── void.upper(text) / void.lower(text) ────────
    {
        let upper_fn = lua.create_function(|_, text: String| Ok(text.to_uppercase()))?;
        void_table.set("upper", upper_fn)?;
    }
    {
        let lower_fn = lua.create_function(|_, text: String| Ok(text.to_lowercase()))?;
        void_table.set("lower", lower_fn)?;
    }

    // ─── void.token(var, delimiter) — epic6 destructive tokenizer ──
    // Splits var value at delimiter, returns part before, assigns remainder back
    {
        let token_fn = lua.create_function(|_, (text, delim): (String, String)| {
            if let Some(pos) = text.find(&delim) {
                let before = text[..pos].to_string();
                let after = text[pos + delim.len()..].to_string();
                Ok((before, after))
            } else {
                Ok((text, String::new()))
            }
        })?;
        void_table.set("token", token_fn)?;
    }

    // ─── void.coalesce(...) — epic6 first non-empty ──
    {
        let coalesce_fn = lua.create_function(|_, args: Vec<String>| {
            for arg in args {
                if !arg.is_empty() {
                    return Ok(arg);
                }
            }
            Ok(String::new())
        })?;
        void_table.set("coalesce", coalesce_fn)?;
    }

    // ─── void.xform(mode, text) — epic6 base85 encode/decode ──
    {
        let xform_fn = lua.create_function(|_, (mode, text): (String, String)| {
            match mode.as_str() {
                "+B85" => Ok(base85_encode(&text)),
                "-B85" => Ok(base85_decode(&text)),
                "+B64" => Ok(base64_encode_str(&text)),
                "-B64" => Ok(base64_decode_str(&text)),
                "+URL" => Ok(url_encode(&text)),
                "-URL" => Ok(url_decode(&text)),
                _ => Ok(text),
            }
        })?;
        void_table.set("xform", xform_fn)?;
    }

    // ─── void.pbkdf2(password, salt, iterations) — key derivation ──
    {
        let pbkdf2_fn = lua.create_function(|_, (password, salt, iterations): (String, String, u32)| {
            use ring::pbkdf2;
            let mut key = [0u8; 64];
            pbkdf2::derive(
                pbkdf2::PBKDF2_HMAC_SHA512,
                std::num::NonZeroU32::new(iterations.max(1)).unwrap(),
                salt.as_bytes(),
                password.as_bytes(),
                &mut key,
            );
            Ok(base64_encode_bytes(&key))
        })?;
        void_table.set("pbkdf2", pbkdf2_fn)?;
    }

    // ─── void.sha256(text) — SHA-256 hash ──
    {
        let sha_fn = lua.create_function(|_, text: String| {
            use ring::digest;
            let hash = digest::digest(&digest::SHA256, text.as_bytes());
            Ok(hex_encode(hash.as_ref()))
        })?;
        void_table.set("sha256", sha_fn)?;
    }

    // ─── void.sha512(text) — SHA-512 hash ──
    {
        let sha_fn = lua.create_function(|_, text: String| {
            use ring::digest;
            let hash = digest::digest(&digest::SHA512, text.as_bytes());
            Ok(hex_encode(hash.as_ref()))
        })?;
        void_table.set("sha512", sha_fn)?;
    }

    // ─── void.hmac_sha256(key, text) — HMAC-SHA-256 ──
    {
        let hmac_fn = lua.create_function(|_, (key, text): (String, String)| {
            use ring::hmac;
            let key = hmac::Key::new(hmac::HMAC_SHA256, key.as_bytes());
            let sig = hmac::sign(&key, text.as_bytes());
            Ok(hex_encode(sig.as_ref()))
        })?;
        void_table.set("hmac_sha256", hmac_fn)?;
    }

    // ─── void.random(min, max) — random number ──
    {
        let rand_fn = lua.create_function(|_, (min, max): (i64, i64)| {
            use std::time::{SystemTime, UNIX_EPOCH};
            let seed = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos() as i64;
            let range = (max - min).abs().max(1);
            Ok(min + (seed % range))
        })?;
        void_table.set("random", rand_fn)?;
    }

    // ─── void.file_read(path) — read file contents ──
    {
        let read_fn = lua.create_function(|_, path: String| {
            match std::fs::read_to_string(&path) {
                Ok(content) => Ok(content),
                Err(e) => Ok(format!("Error: {}", e)),
            }
        })?;
        void_table.set("file_read", read_fn)?;
    }

    // ─── void.file_write(path, content) — write file ──
    {
        let write_fn = lua.create_function(|_, (path, content): (String, String)| {
            match std::fs::write(&path, &content) {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        })?;
        void_table.set("file_write", write_fn)?;
    }

    // ─── void.file_append(path, content) — append to file ──
    {
        let append_fn = lua.create_function(|_, (path, content): (String, String)| {
            use std::io::Write;
            match std::fs::OpenOptions::new().create(true).append(true).open(&path) {
                Ok(mut f) => {
                    let _ = writeln!(f, "{}", content);
                    Ok(true)
                }
                Err(_) => Ok(false),
            }
        })?;
        void_table.set("file_append", append_fn)?;
    }

    // ─── void.json_encode(table) — table to JSON string ──
    {
        let json_fn = lua.create_function(|_, text: String| {
            // Simple JSON encoding — just wrap as string for now
            Ok(format!("\"{}\"", text.replace('"', "\\\"")))
        })?;
        void_table.set("json_encode", json_fn)?;
    }

    // ─── void.base64_encode(text) / void.base64_decode(text) ──
    {
        let enc_fn = lua.create_function(|_, text: String| Ok(base64_encode_str(&text)))?;
        void_table.set("base64_encode", enc_fn)?;
    }
    {
        let dec_fn = lua.create_function(|_, text: String| Ok(base64_decode_str(&text)))?;
        void_table.set("base64_decode", dec_fn)?;
    }

    // ─── void.json_decode(text) — JSON string to table ──
    {
        let json_fn = lua.create_function(|_, text: String| {
            // Simple JSON decode — return as string for now
            Ok(text)
        })?;
        void_table.set("json_decode", json_fn)?;
    }

    // ─── void.hex_encode(text) / void.hex_decode(text) ──
    {
        let enc_fn = lua.create_function(|_, text: String| {
            Ok(hex_encode(text.as_bytes()))
        })?;
        void_table.set("hex_encode", enc_fn)?;
    }
    {
        let dec_fn = lua.create_function(|_, text: String| {
            let bytes = hex_decode(&text);
            Ok(String::from_utf8_lossy(&bytes).to_string())
        })?;
        void_table.set("hex_decode", dec_fn)?;
    }

    // ─── void.color(fg, bg) — IRC color code helper ──
    {
        let color_fn = lua.create_function(|_, (fg, bg): (i32, Option<i32>)| {
            match bg {
                Some(b) => Ok(format!("\x03{},{}", fg, b)),
                None => Ok(format!("\x03{}", fg)),
            }
        })?;
        void_table.set("color", color_fn)?;
    }

    // ─── void.bold() / void.italic() / void.underline() / void.reverse() / void.reset() ──
    {
        let bold_fn = lua.create_function(|_, ()| Ok("\x02".to_string()))?;
        void_table.set("bold", bold_fn)?;
    }
    {
        let italic_fn = lua.create_function(|_, ()| Ok("\x1D".to_string()))?;
        void_table.set("italic", italic_fn)?;
    }
    {
        let underline_fn = lua.create_function(|_, ()| Ok("\x1F".to_string()))?;
        void_table.set("underline", underline_fn)?;
    }
    {
        let reverse_fn = lua.create_function(|_, ()| Ok("\x16".to_string()))?;
        void_table.set("reverse", reverse_fn)?;
    }
    {
        let reset_fn = lua.create_function(|_, ()| Ok("\x0F".to_string()))?;
        void_table.set("reset", reset_fn)?;
    }

    // ─── void.nicks(channel) — placeholder (needs Rust-side integration) ──
    {
        let nicks_fn = lua.create_function(|_, _channel: String| {
            Ok(Vec::<String>::new())
        })?;
        void_table.set("nicks", nicks_fn)?;
    }

    // ─── void.buffers() — placeholder ──
    {
        let buffers_fn = lua.create_function(|_, ()| {
            Ok(Vec::<String>::new())
        })?;
        void_table.set("buffers", buffers_fn)?;
    }

    // ─── void.ison(nicks) — check if nicks are online ──
    {
        let ctx = ctx.clone();
        let ison_fn = lua.create_function(move |_, nicks: Vec<String>| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("ISON {}", nicks.join(" ")),
            });
            Ok(())
        })?;
        void_table.set("ison", ison_fn)?;
    }

    // ─── void.userhost(nick) — query userhost ──
    {
        let ctx = ctx.clone();
        let userhost_fn = lua.create_function(move |_, nick: String| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("USERHOST {}", nick),
            });
            Ok(())
        })?;
        void_table.set("userhost", userhost_fn)?;
    }

    // ─── void.log(text) — write to log ──
    {
        let ctx = ctx.clone();
        let log_fn = lua.create_function(move |_, text: String| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("ECHO {}", text),
            });
            Ok(())
        })?;
        void_table.set("log", log_fn)?;
    }

    // ─── void.load(script) — load another Lua script ──
    {
        let ctx = ctx.clone();
        let load_fn = lua.create_function(move |_, path: String| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("LOAD {}", path),
            });
            Ok(())
        })?;
        void_table.set("load", load_fn)?;
    }

    // ─── void.exec(cmd) — execute shell command ──
    {
        let ctx = ctx.clone();
        let exec_fn = lua.create_function(move |_, cmd: String| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("EXEC {}", cmd),
            });
            Ok(())
        })?;
        void_table.set("exec", exec_fn)?;
    }

    // ─── void.apply_theme(name) — zastosuj theme ─────
    {
        let ctx = ctx.clone();
        let apply_fn = lua.create_function(move |_, name: String| {
            let ctx = ctx.lock().unwrap();
            let _ = ctx.cmd_tx.try_send(LuaCommand {
                raw: format!("THEME_APPLY {}", name),
            });
            Ok(())
        })?;
        void_table.set("apply_theme", apply_fn)?;
    }

    lua.globals().set("void", void_table)?;
    Ok(())
}

/// Wywołaj hooki dla danego zdarzenia
pub fn fire_event(lua: &Lua, hooks: &LuaHooks, event: &str, args: &[&str]) -> Vec<String> {
    let mut results = Vec::new();
    if let Some(fn_names) = hooks.events.get(&event.to_uppercase()) {
        for fn_name in fn_names {
            if let Ok(func) = lua.globals().get::<Function>(fn_name.as_str()) {
                let lua_args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
                match func.call::<Value>(lua_args) {
                    Ok(Value::String(s)) => {
                        results.push(s.to_string_lossy());
                    }
                    Ok(_) => {}
                    Err(e) => {
                        results.push(format!("-!- Lua error in {}: {}", fn_name, e));
                    }
                }
            }
        }
    }
    results
}

/// Wywołaj komendę zarejestrowaną z Lua
pub fn call_lua_command(lua: &Lua, hooks: &LuaHooks, cmd: &str, args: &[&str]) -> Option<Vec<String>> {
    if let Some(fn_name) = hooks.commands.get(&cmd.to_uppercase()) {
        if let Ok(func) = lua.globals().get::<Function>(fn_name.as_str()) {
            let lua_args: Vec<String> = args.iter().map(|a| a.to_string()).collect();
            let mut results = Vec::new();
            match func.call::<Value>(lua_args) {
                Ok(Value::String(s)) => {
                    results.push(s.to_string_lossy());
                }
                Ok(Value::Table(t)) => {
                    for pair in t.sequence_values::<String>() {
                        if let Ok(s) = pair {
                            results.push(s);
                        }
                    }
                }
                Ok(_) => {}
                Err(e) => {
                    results.push(format!("-!- Lua error: {}", e));
                }
            }
            return Some(results);
        }
    }
    None
}
