/// Format string engine — epic5/LiCe5 style
/// Replaces %T, %N, %C, etc. with actual values
/// Supports IRC color codes in format strings

use crate::app::App;

/// Replace format variables in a template string
/// Variables: %T=time, %N=nick, %C=channel, %S=server, %W=window,
///            %A=away, %H=host, %M=modes, %L=lag, %=pad, %>right-align
pub fn expand_status_format(app: &App, template: &str) -> String {
    let mut result = String::new();
    let mut chars = template.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('T') => {
                    // Time
                    result.push_str(&chrono::Local::now().format("%H:%M").to_string());
                }
                Some('N') => {
                    // Nick
                    result.push_str(&app.server().our_nick);
                }
                Some('C') => {
                    // Channel
                    let buf = &app.buffers[app.current_buffer_idx];
                    if buf.name != "(Status)" {
                        result.push_str(&buf.name);
                    }
                }
                Some('S') => {
                    // Server
                    result.push_str(&app.server().host);
                }
                Some('W') => {
                    // Window number
                    result.push_str(&app.current_buffer_idx.to_string());
                }
                Some('A') => {
                    // Away indicator
                    if app.server().away_message.is_some() {
                        result.push_str("AWAY");
                    }
                }
                Some('H') => {
                    // Host
                    result.push_str(&app.server().host);
                }
                Some('M') => {
                    // User modes
                    if !app.server().user_modes.is_empty() {
                        result.push('+');
                        result.push_str(&app.server().user_modes);
                    }
                }
                Some('L') => {
                    // Lag (placeholder)
                    result.push_str("0");
                }
                Some('#') => {
                    // Nick count in current channel
                    let buf = &app.buffers[app.current_buffer_idx];
                    result.push_str(&buf.nicks.len().to_string());
                }
                Some('*') => {
                    // Unread count
                    let buf = &app.buffers[app.current_buffer_idx];
                    if buf.unread_count > 0 {
                        result.push_str(&buf.unread_count.to_string());
                    }
                }
                Some('=') => {
                    // Pad to fill remaining space (handled by renderer)
                    result.push_str(" ");
                }
                Some('>') => {
                    // Right-align marker (handled by renderer)
                    result.push_str(" ");
                }
                Some('%') => {
                    result.push('%');
                }
                Some(other) => {
                    result.push('%');
                    result.push(other);
                }
                None => {
                    result.push('%');
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Expand event format templates
/// Variables: $0=nick, $1=channel/text, $2=host/text, $3=extra
pub fn expand_event_format(template: &str, args: &[&str]) -> String {
    let mut result = String::new();
    let mut chars = template.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            match chars.next() {
                Some('*') => {
                    // All args joined
                    result.push_str(&args.join(" "));
                }
                Some(d @ '0'..='9') => {
                    let idx = (d as usize) - ('0' as usize);
                    if idx < args.len() {
                        result.push_str(args[idx]);
                    }
                }
                Some('$') => {
                    result.push('$');
                }
                Some(other) => {
                    result.push('$');
                    result.push(other);
                }
                None => {
                    result.push('$');
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Strip IRC formatting codes from text
pub fn strip_formatting(text: &str) -> String {
    text.chars()
        .filter(|c| !c.is_control())
        .collect()
}

/// Convert IRC color codes to ratatui-compatible format
/// This is a placeholder — actual conversion happens in the renderer
pub fn prepare_format_string(template: &str) -> String {
    template.to_string()
}
