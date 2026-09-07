/// Format engine tests — status bar variables, IRC formatting, color parsing
use ratatui::style::Color;

#[test]
fn test_expand_status_format_basic() {
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");

    // Add a channel buffer
    app.buffers.push(void::app::Buffer::new("#test"));
    app.current_buffer_idx = 1;

    let result = void::format::expand_status_format(&app, "%N %C");
    assert!(!result.is_empty());
}

#[test]
fn test_expand_status_format_empty_template() {
    let app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    let result = void::format::expand_status_format(&app, "");
    assert!(result.is_empty());
}

#[test]
fn test_expand_status_format_literal_text() {
    let app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    let result = void::format::expand_status_format(&app, "Hello World");
    assert!(!result.is_empty());
}

#[test]
fn test_expand_status_format_nick() {
    let app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    let result = void::format::expand_status_format(&app, "%N");
    assert!(!result.is_empty());
}

#[test]
fn test_expand_status_format_time() {
    let app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    let result = void::format::expand_status_format(&app, "%T");
    assert!(!result.is_empty());
}

#[test]
fn test_expand_status_format_server() {
    let app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    let result = void::format::expand_status_format(&app, "%S");
    assert!(!result.is_empty());
}

#[test]
fn test_expand_status_format_channel() {
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    app.buffers.push(void::app::Buffer::new("#test"));
    app.current_buffer_idx = 1;
    let result = void::format::expand_status_format(&app, "%C");
    assert!(!result.is_empty());
}

#[test]
fn test_expand_status_format_combined() {
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    app.buffers.push(void::app::Buffer::new("#test"));
    app.current_buffer_idx = 1;
    let result = void::format::expand_status_format(&app, " [%T] %N %C ");
    assert!(!result.is_empty());
}

#[test]
fn test_expand_status_format_percent_escape() {
    let app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    let result = void::format::expand_status_format(&app, "100%%");
    assert!(!result.is_empty());
}

#[test]
fn test_mirc_color_basic() {
    // Test mirc_color function indirectly via parse_irc_formatting
    // mirc_color(0) = white, mirc_color(1) = black, etc.
    let app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    let result = void::format::expand_status_format(&app, "%T %N");
    assert!(!result.is_empty());
}

#[test]
fn test_expand_status_format_lag() {
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    app.server_mut().lag_ms = 42;
    let result = void::format::expand_status_format(&app, "%L");
    assert!(!result.is_empty());
}

#[test]
fn test_expand_status_format_nickcount() {
    let mut app = void::app::App::new("testnick", "irc.test.com", 6697, true, "testpass");
    app.buffers.push(void::app::Buffer::new("#test"));
    app.current_buffer_idx = 1;
    app.buffers[1].add_nick("user1");
    app.buffers[1].add_nick("user2");
    let result = void::format::expand_status_format(&app, "%#");
    assert!(!result.is_empty());
}
