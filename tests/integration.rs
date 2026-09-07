/// Integration tests for IRC protocol, settings persistence, theme application
use std::sync::{Arc, Mutex};

#[test]
fn test_settings_persistence() {
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");

    // Set values
    app.settings.set("MY_TEST_KEY", "test_value");
    app.settings.set("SCROLL_LINES", "10");

    // Verify
    assert_eq!(app.settings.get("MY_TEST_KEY"), "test_value");
    assert_eq!(app.settings.get_int("SCROLL_LINES"), 10);
}

#[test]
fn test_buffer_management() {
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");

    // Initial state — Status buffer only
    assert_eq!(app.buffers.len(), 1);
    assert_eq!(app.buffers[0].name, "(Status)");

    // Add buffer
    app.buffers.push(void::app::Buffer::new("#test"));
    assert_eq!(app.buffers.len(), 2);

    // Switch buffer
    app.current_buffer_idx = 1;
    assert_eq!(app.current_buffer().name, "#test");
}

#[test]
fn test_nick_management() {
    let mut buf = void::app::Buffer::new("#test");

    // Add nicks with prefixes
    buf.add_nick("user1");
    buf.add_nick("user2");
    buf.add_nick("user3");

    // Set prefixes
    buf.set_nick_prefix("user1", '@', true);
    buf.set_nick_prefix("user2", '+', true);

    // Verify
    let u1 = buf.nicks.iter().find(|n| n.nick == "user1").unwrap();
    assert_eq!(u1.prefix, "@");
    let u2 = buf.nicks.iter().find(|n| n.nick == "user2").unwrap();
    assert_eq!(u2.prefix, "+");
    let u3 = buf.nicks.iter().find(|n| n.nick == "user3").unwrap();
    assert_eq!(u3.prefix, "");

    // Rename
    buf.rename_nick("user1", "opnick");
    assert!(buf.nicks.iter().any(|n| n.nick == "opnick"));
    assert!(!buf.nicks.iter().any(|n| n.nick == "user1"));

    // Remove
    buf.remove_nick("user3");
    assert_eq!(buf.nicks.len(), 2);
}

#[test]
fn test_ignore_list() {
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");

    // Add ignore — simple wildcard pattern
    app.ignore_list.push(void::app::IgnoreEntry {
        pattern: "spammer*".to_string(),
        ignore_public: true,
        ignore_private: true,
        ignore_notice: true,
        ignore_ctcp: true,
        ignore_all: true,
    });

    // Check
    assert!(app.is_ignored("spammer", "MSG"));
    assert!(app.is_ignored("spammer", "PUBLIC"));
    assert!(!app.is_ignored("gooduser", "MSG"));
}

#[test]
fn test_notify_list() {
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");

    // Add notify
    app.notify_list.push(void::app::NotifyEntry {
        nick: "friend".to_string(),
        online: false,
        last_seen: None,
        userhost: String::new(),
        channels: Vec::new(),
        verified: false,
        action: String::new(),
    });

    assert_eq!(app.notify_list.len(), 1);
    assert_eq!(app.notify_list[0].nick, "friend");
}

#[test]
fn test_theme_format_engine() {
    let app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");

    // Test format variables
    let format = "%T %N %C %S";
    let result = void::format::expand_status_format(&app, format);
    assert!(!result.is_empty());
}

#[test]
fn test_window_levels() {
    let mut buf = void::app::Buffer::new("#test");
    assert_eq!(buf.level, void::app::WindowLevel::All);

    buf.level = void::app::WindowLevel::Msgs;
    assert_eq!(buf.level, void::app::WindowLevel::Msgs);

    buf.level = void::app::WindowLevel::Joins;
    assert_eq!(buf.level, void::app::WindowLevel::Joins);
}

#[test]
fn test_command_registry_aliases() {
    let registry = void::commands::registry::CommandRegistry::new();

    // Test aliases
    assert!(registry.find("j").is_some());      // /join
    assert!(registry.find("q").is_some());      // /quit
    assert!(registry.find("dc").is_some());     // /disconnect
    assert!(registry.find("wi").is_some());     // /whois
    assert!(registry.find("wii").is_some());    // /whois2
    assert!(registry.find("kb").is_some());     // /kickban
    assert!(registry.find("v").is_some());      // /voice
    assert!(registry.find("serv").is_some());   // /server
}

#[test]
fn test_lua_api_nicks_and_buffers() {
    let lua = void::scripting::engine::init_lua().expect("Failed to init Lua");
    let hooks = Arc::new(Mutex::new(void::scripting::api::LuaHooks::new()));
    let (cmd_tx, _cmd_rx) = tokio::sync::mpsc::channel(100);

    let mut nicks_map = std::collections::HashMap::new();
    nicks_map.insert("#test".to_string(), vec!["user1".to_string(), "user2".to_string()]);

    let ctx = Arc::new(Mutex::new(void::scripting::api::LuaContext {
        our_nick: "testnick".into(),
        current_channel: "#test".into(),
        server_host: "irc.test.com".into(),
        connected: true,
        cmd_tx,
        settings: std::collections::HashMap::new(),
        nicks_by_channel: nicks_map,
        buffer_names: vec!["(Status)".into(), "#test".into()],
    }));

    void::scripting::api::register_api(&lua, hooks.clone(), ctx.clone())
        .expect("Failed to register Lua API");

    // Test void.nicks("#test")
    let result: Vec<String> = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("nicks").unwrap()
        .call(("#test",)).unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&"user1".to_string()));
    assert!(result.contains(&"user2".to_string()));

    // Test void.buffers()
    let result: Vec<String> = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("buffers").unwrap()
        .call(()).unwrap();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&"(Status)".to_string()));
    assert!(result.contains(&"#test".to_string()));
}

#[test]
fn test_lua_void_get_set() {
    let lua = void::scripting::engine::init_lua().expect("Failed to init Lua");
    let hooks = Arc::new(Mutex::new(void::scripting::api::LuaHooks::new()));
    let (cmd_tx, _cmd_rx) = tokio::sync::mpsc::channel(100);

    let mut settings = std::collections::HashMap::new();
    settings.insert("SCROLL_LINES".to_string(), "5".to_string());

    let ctx = Arc::new(Mutex::new(void::scripting::api::LuaContext {
        our_nick: "testnick".into(),
        current_channel: "#test".into(),
        server_host: "irc.test.com".into(),
        connected: true,
        cmd_tx,
        settings,
        nicks_by_channel: std::collections::HashMap::new(),
        buffer_names: Vec::new(),
    }));

    void::scripting::api::register_api(&lua, hooks.clone(), ctx.clone())
        .expect("Failed to register Lua API");

    // Test void.get("SCROLL_LINES")
    let result: String = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("get").unwrap()
        .call(("SCROLL_LINES",)).unwrap();
    assert_eq!(result, "5");
}
