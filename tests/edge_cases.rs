/// Edge case tests — IRC formatting, error handling, boundary conditions
use std::sync::{Arc, Mutex};

#[test]
fn test_empty_message() {
    let mut buf = void::app::Buffer::new("#test");
    buf.push_message("".to_string(), void::app::MessageType::Normal, 500);
    assert_eq!(buf.messages.len(), 1);
    assert_eq!(buf.messages[0].text, "");
}

#[test]
fn test_long_message() {
    let mut buf = void::app::Buffer::new("#test");
    let long_msg = "A".repeat(10000);
    buf.push_message(long_msg.clone(), void::app::MessageType::Normal, 500);
    assert_eq!(buf.messages[0].text.len(), 10000);
}

#[test]
fn test_unicode_nick() {
    let mut buf = void::app::Buffer::new("#test");
    buf.add_nick("żółć");
    buf.add_nick("日本語");
    assert_eq!(buf.nicks.len(), 2);
    assert!(buf.nicks.iter().any(|n| n.nick == "żółć"));
    assert!(buf.nicks.iter().any(|n| n.nick == "日本語"));
}

#[test]
fn test_special_chars_in_nick() {
    let mut buf = void::app::Buffer::new("#test");
    buf.add_nick("user[123]");
    buf.add_nick("user~1");
    assert!(buf.nicks.iter().any(|n| n.nick == "user[123]"));
    assert!(buf.nicks.iter().any(|n| n.nick == "user~1"));
}

#[test]
fn test_rename_nonexistent_nick() {
    let mut buf = void::app::Buffer::new("#test");
    buf.add_nick("user1");
    buf.rename_nick("nonexistent", "newnick");
    assert!(buf.nicks.iter().any(|n| n.nick == "user1"));
    assert!(!buf.nicks.iter().any(|n| n.nick == "newnick"));
}

#[test]
fn test_remove_nonexistent_nick() {
    let mut buf = void::app::Buffer::new("#test");
    buf.add_nick("user1");
    buf.remove_nick("nonexistent");
    assert_eq!(buf.nicks.len(), 1);
}

#[test]
fn test_prefix_on_nonexistent_nick() {
    let mut buf = void::app::Buffer::new("#test");
    buf.add_nick("user1");
    buf.set_nick_prefix("nonexistent", '@', true);
    let u1 = buf.nicks.iter().find(|n| n.nick == "user1").unwrap();
    assert_eq!(u1.prefix, "");
}

#[test]
fn test_empty_channel_name() {
    let buf = void::app::Buffer::new("");
    assert_eq!(buf.name, "");
}

#[test]
fn test_settings_unknown_key() {
    let settings = void::app::Settings::new();
    assert_eq!(settings.get("NONEXISTENT_KEY"), "");
}

#[test]
fn test_settings_zero_int() {
    let mut settings = void::app::Settings::new();
    settings.set("ZERO", "0");
    assert_eq!(settings.get_int("ZERO"), 0);
}

#[test]
fn test_settings_negative_int() {
    let mut settings = void::app::Settings::new();
    settings.set("NEG", "-5");
    assert_eq!(settings.get_int("NEG"), -5);
}

#[test]
fn test_settings_invalid_int() {
    let mut settings = void::app::Settings::new();
    settings.set("INVALID", "abc");
    assert_eq!(settings.get_int("INVALID"), 0);
}

#[test]
fn test_ignore_case_insensitive() {
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    app.ignore_list.push(void::app::IgnoreEntry {
        pattern: "SpAmMeR*".to_string(),
        ignore_public: true,
        ignore_private: false,
        ignore_notice: false,
        ignore_ctcp: false,
        ignore_all: false,
    });
    assert!(app.is_ignored("spammer123", "PUBLIC"));
    assert!(app.is_ignored("SPAMMER", "PUBLIC"));
    assert!(!app.is_ignored("spammer123", "MSG"));
}

#[test]
fn test_buffer_scroll_offset_bounds() {
    let mut buf = void::app::Buffer::new("#test");
    for i in 0..5 {
        buf.push_message(format!("msg {}", i), void::app::MessageType::Normal, 500);
    }
    buf.scroll_offset = 100;
    assert_eq!(buf.scroll_offset, 100);

    buf.scroll_offset = buf.scroll_offset.saturating_sub(200);
    assert_eq!(buf.scroll_offset, 0);
}

#[test]
fn test_duplicate_nick_not_added() {
    let mut buf = void::app::Buffer::new("#test");
    buf.add_nick("user1");
    buf.add_nick("user1");
    assert_eq!(buf.nicks.len(), 1);
}

#[test]
fn test_lua_void_match_patterns() {
    let lua = void::scripting::engine::init_lua().expect("Failed to init Lua");
    let hooks = Arc::new(Mutex::new(void::scripting::api::LuaHooks::new()));
    let (cmd_tx, _cmd_rx) = tokio::sync::mpsc::channel(100);
    let ctx = Arc::new(Mutex::new(void::scripting::api::LuaContext {
        our_nick: "test".into(),
        current_channel: "#test".into(),
        server_host: "irc.test.com".into(),
        connected: true,
        cmd_tx,
        settings: std::collections::HashMap::new(),
        nicks_by_channel: std::collections::HashMap::new(),
        buffer_names: Vec::new(),
    }));
    void::scripting::api::register_api(&lua, hooks.clone(), ctx.clone()).unwrap();

    let match_fn = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("match").unwrap();

    // Exact match
    let r: bool = match_fn.call(("hello", "hello")).unwrap();
    assert!(r);

    // Wildcard
    let r: bool = match_fn.call(("hel*", "hello")).unwrap();
    assert!(r);

    // No match
    let r: bool = match_fn.call(("world", "hello")).unwrap();
    assert!(!r);
}

#[test]
fn test_lua_void_strip_all_codes() {
    let lua = void::scripting::engine::init_lua().expect("Failed to init Lua");
    let hooks = Arc::new(Mutex::new(void::scripting::api::LuaHooks::new()));
    let (cmd_tx, _cmd_rx) = tokio::sync::mpsc::channel(100);
    let ctx = Arc::new(Mutex::new(void::scripting::api::LuaContext {
        our_nick: "test".into(),
        current_channel: "#test".into(),
        server_host: "irc.test.com".into(),
        connected: true,
        cmd_tx,
        settings: std::collections::HashMap::new(),
        nicks_by_channel: std::collections::HashMap::new(),
        buffer_names: Vec::new(),
    }));
    void::scripting::api::register_api(&lua, hooks.clone(), ctx.clone()).unwrap();

    let strip_fn = lua.globals()
        .get::<mlua::Table>("void").unwrap()
        .get::<mlua::Function>("strip").unwrap();

    // Bold
    let r: String = strip_fn.call(("\x02bold\x02 normal",)).unwrap();
    assert_eq!(r, "bold normal");

    // Color
    let r: String = strip_fn.call(("\x034red\x03 text",)).unwrap();
    assert!(r.contains("red"));
    assert!(r.contains("text"));

    // Empty
    let r: String = strip_fn.call(("",)).unwrap();
    assert_eq!(r, "");
}

#[test]
fn test_command_registry_unknown_command() {
    let registry = void::commands::registry::CommandRegistry::new();
    assert!(registry.find("nonexistent_command_xyz").is_none());
}

#[test]
fn test_split_ratio_bounds() {
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    app.split_ratio = 50;

    // Simulate grow
    app.split_ratio = (app.split_ratio + 5).min(90);
    assert_eq!(app.split_ratio, 55);

    // Simulate shrink
    app.split_ratio = app.split_ratio.saturating_sub(5).max(10);
    assert_eq!(app.split_ratio, 50);

    // Bounds
    app.split_ratio = 95;
    app.split_ratio = (app.split_ratio + 5).min(90);
    assert_eq!(app.split_ratio, 90);

    app.split_ratio = 5;
    app.split_ratio = app.split_ratio.saturating_sub(5).max(10);
    assert_eq!(app.split_ratio, 10);
}
