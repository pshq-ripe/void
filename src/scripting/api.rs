use mlua::{Lua, Result as LuaResult, Function, Value};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::mpsc;

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
        let version_fn = lua.create_function(|_, ()| Ok("void 0.1.0"))?;
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

    // ─── void.get(key) — placeholder ─────────────────
    {
        let get_fn = lua.create_function(|_, _key: String| Ok(String::new()))?;
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
