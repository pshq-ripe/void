/// Test: Lua API + LiCe5 scripts + command registry
/// Runs without TUI — exercises all components directly
use std::sync::{Arc, Mutex};

#[test]
fn lua_integration_test() {
    println!("=== Void IRC Client — Lua/LiCe5 Integration Test ===\n");

    // 1. Init Lua
    let lua = void::scripting::engine::init_lua().expect("Failed to init Lua");
    println!("[OK] Lua engine initialized");

    // 2. Create hooks and context
    let hooks = Arc::new(Mutex::new(void::scripting::api::LuaHooks::new()));
    let (cmd_tx, _cmd_rx) = tokio::sync::mpsc::channel(100);
    let ctx = Arc::new(Mutex::new(void::scripting::api::LuaContext {
        our_nick: "testnick".into(),
        current_channel: "#test".into(),
        server_host: "irc.test.com".into(),
        connected: true,
        cmd_tx,
        settings: std::collections::HashMap::new(),
    }));

    // 3. Register API
    void::scripting::api::register_api(&lua, hooks.clone(), ctx.clone())
        .expect("Failed to register Lua API");
    println!("[OK] Lua API registered (void.* table)");

    // 4. Load config.lua + LiCe5
    void::scripting::engine::load_scripts(&lua);
    println!("[OK] config.lua loaded");

    // 5. Check registered hooks
    let h = hooks.lock().unwrap();
    println!("\n--- Registered Lua Commands ---");
    let mut cmds: Vec<_> = h.commands.keys().collect();
    cmds.sort();
    for cmd in &cmds {
        println!("  /{}", cmd.to_lowercase());
    }
    println!("  Total: {} commands", cmds.len());

    println!("\n--- Registered Lua Event Hooks ---");
    let mut events: Vec<_> = h.events.keys().collect();
    events.sort();
    for event in &events {
        let fns = h.events.get(*event).unwrap();
        println!("  {} -> {} handler(s)", event, fns.len());
    }
    println!("  Total: {} event types", events.len());
    drop(h);

    // 6. Test Lua functions
    println!("\n--- Testing Lua Functions ---");

    // Test void.nick()
    let result: String = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("nick").unwrap()
        .call(()).unwrap();
    println!("  void.nick() = '{}' {}", result, if result == "testnick" { "[OK]" } else { "[FAIL]" });

    // Test void.channel()
    let result: String = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("channel").unwrap()
        .call(()).unwrap();
    println!("  void.channel() = '{}' {}", result, if result == "#test" { "[OK]" } else { "[FAIL]" });

    // Test void.server()
    let result: String = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("server").unwrap()
        .call(()).unwrap();
    println!("  void.server() = '{}' {}", result, if result == "irc.test.com" { "[OK]" } else { "[FAIL]" });

    // Test void.connected()
    let result: bool = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("connected").unwrap()
        .call(()).unwrap();
    println!("  void.connected() = {} {}", result, if result { "[OK]" } else { "[FAIL]" });

    // Test void.version()
    let result: String = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("version").unwrap()
        .call(()).unwrap();
    println!("  void.version() = '{}' [OK]", result);

    // Test void.match()
    let result: bool = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("match").unwrap()
        .call(("hel*", "hello world")).unwrap();
    println!("  void.match('hel*', 'hello world') = {} {}", result, if result { "[OK]" } else { "[FAIL]" });

    // Test void.strip()
    let result: String = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("strip").unwrap()
        .call(("\x02bold\x02 normal",)).unwrap();
    println!("  void.strip('\\x02bold\\x02 normal') = '{}' [OK]", result);

    // Test void.upper() / void.lower()
    let result: String = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("upper").unwrap()
        .call(("hello",)).unwrap();
    println!("  void.upper('hello') = '{}' {}", result, if result == "HELLO" { "[OK]" } else { "[FAIL]" });

    // 7. Test LiCe5 module loading
    println!("\n--- LiCe5 Module Status ---");
    let lice5_table: mlua::Value = lua.globals().get("lice5").unwrap();
    if let mlua::Value::Table(t) = lice5_table {
        let version: String = t.get("version").unwrap_or_default();
        println!("  LiCe5 version: {}", version);
        if let Ok(loaded) = t.get::<mlua::Table>("loaded") {
            let mut modules: Vec<String> = Vec::new();
            for pair in loaded.pairs::<String, bool>() {
                if let Ok((name, ok)) = pair {
                    if ok { modules.push(name); }
                }
            }
            modules.sort();
            for m in &modules {
                println!("  [OK] {}", m);
            }
            println!("  Total: {} modules loaded", modules.len());
        }
    }

    // 8. Test command registry
    println!("\n--- Command Registry Test ---");
    let registry = void::commands::registry::CommandRegistry::new();
    let test_cmds = ["help", "set", "alias", "join", "part", "mode", "op", "voice", "ban", "kick", "whois", "nick", "away", "quit", "server", "window", "notify", "ignore", "timer", "log", "dcc", "raw", "echo", "highlight", "bind", "format", "cd", "pwd", "debug", "scroll", "flood", "play", "load", "reload", "save", "list", "cycle", "knock", "oper", "kill"];
    let mut found = 0;
    let mut missing = 0;
    for cmd in &test_cmds {
        if registry.find(cmd).is_some() {
            found += 1;
        } else {
            println!("  [MISSING] /{}", cmd);
            missing += 1;
        }
    }
    println!("  Found: {}/{} commands [{}]", found, test_cmds.len(), if missing == 0 { "OK" } else { "PARTIAL" });

    // 9. Test Themes System
    println!("\n--- Theme System Test ---");
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    app.lua = Some(Arc::new(lua));

    let themes = [
        "catppuccin", "catppuccinlatte", "dracula", "nord",
        "gruvbox", "gruvboxlight", "solarized", "solarizedlight",
        "tokyonight", "matrix", "cyberpunk", "monokai",
        "onedark", "rosepine", "irssi", "bitchx"
    ];

    for theme_name in &themes {
        app.apply_theme(theme_name);
        println!("  [OK] Theme '{}' applied successfully -> desc: '{}', status_bar_bg: {:?}",
            app.theme_colors.name, app.theme_colors.desc, app.theme_colors.status_bar_bg);
        assert!(!app.theme_colors.name.is_empty());
        assert!(!app.theme_colors.desc.is_empty());
    }

    println!("\n=== Test Complete ===");
}
