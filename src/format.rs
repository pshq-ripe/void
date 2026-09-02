/// Format string engine — epic5/LiCe5 style
/// Replaces %T, %N, %C, etc. with actual values
/// Returns styled spans with theme colors

use crate::app::App;
use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

/// Replace format variables and return styled spans
/// Variables: %T=time, %N=nick, %C=channel, %S=server, %W=window,
///            %A=away, %H=host, %M=modes, %L=lag, %#=nickcount, %*=unread
pub fn expand_status_format<'a>(app: &App, template: &str) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    let mut current_text = String::new();
    let mut chars = template.chars().peekable();
    let bg = app.theme_colors.status_bar_bg;

    let flush = |text: &mut String, spans: &mut Vec<Span<'a>>, fg: Color, bg: Color| {
        if !text.is_empty() {
            spans.push(Span::styled(text.clone(), Style::default().fg(fg).bg(bg)));
            text.clear();
        }
    };

    while let Some(c) = chars.next() {
        if c == '%' {
            match chars.next() {
                Some('T') => {
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    let ts_format = app.settings.get("TIMESTAMP_FORMAT");
                    let time = chrono::Local::now().format(ts_format).to_string();
                    spans.push(Span::styled(time, Style::default().fg(app.theme_colors.status_bar_info_fg).bg(bg)));
                }
                Some('N') => {
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    spans.push(Span::styled(
                        format!("{} ", app.server().our_nick),
                        Style::default().fg(app.theme_colors.nick_op_nick).bg(bg).add_modifier(Modifier::BOLD),
                    ));
                }
                Some('C') => {
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    let buf = &app.buffers[app.current_buffer_idx];
                    if buf.name != "(Status)" {
                        spans.push(Span::styled(
                            buf.name.clone(),
                            Style::default().fg(app.theme_colors.input_prompt_fg).bg(bg).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                Some('S') => {
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    spans.push(Span::styled(
                        app.server().host.clone(),
                        Style::default().fg(app.theme_colors.status_bar_info_fg).bg(bg),
                    ));
                }
                Some('W') => {
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    spans.push(Span::styled(
                        format!("[{}]", app.current_buffer_idx),
                        Style::default().fg(app.theme_colors.status_bar_info_fg).bg(bg),
                    ));
                }
                Some('A') => {
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    if app.server().away_message.is_some() {
                        spans.push(Span::styled(
                            "AWAY".to_string(),
                            Style::default().fg(app.theme_colors.msg_action).bg(bg).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                Some('H') => {
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    spans.push(Span::styled(
                        app.server().host.clone(),
                        Style::default().fg(app.theme_colors.status_bar_info_fg).bg(bg),
                    ));
                }
                Some('M') => {
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    if !app.server().user_modes.is_empty() {
                        spans.push(Span::styled(
                            format!("+{}", app.server().user_modes),
                            Style::default().fg(app.theme_colors.status_bar_info_fg).bg(bg),
                        ));
                    }
                }
                Some('#') => {
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    let buf = &app.buffers[app.current_buffer_idx];
                    spans.push(Span::styled(
                        buf.nicks.len().to_string(),
                        Style::default().fg(app.theme_colors.msg_system).bg(bg),
                    ));
                }
                Some('*') => {
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    let buf = &app.buffers[app.current_buffer_idx];
                    if buf.unread_count > 0 {
                        spans.push(Span::styled(
                            buf.unread_count.to_string(),
                            Style::default().fg(app.theme_colors.msg_highlight).bg(bg).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                Some('L') => {
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    let lag = app.server().lag_ms;
                    let lag_color = if lag < 100 { Color::Green }
                        else if lag < 300 { Color::Yellow }
                        else { Color::Red };
                    spans.push(Span::styled(
                        format!("{}ms", lag),
                        Style::default().fg(lag_color).bg(bg),
                    ));
                }
                Some('@') => {
                    // Channel operator status
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    let buf = &app.buffers[app.current_buffer_idx];
                    let my_nick = &app.server().our_nick;
                    let is_op = buf.nicks.iter().any(|n| n.nick == *my_nick && n.prefix.contains('@'));
                    if is_op {
                        spans.push(Span::styled(
                            "@".to_string(),
                            Style::default().fg(app.theme_colors.nick_op).bg(bg).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                Some('+') => {
                    // Voice status
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    let buf = &app.buffers[app.current_buffer_idx];
                    let my_nick = &app.server().our_nick;
                    let is_voice = buf.nicks.iter().any(|n| n.nick == *my_nick && n.prefix.contains('+'));
                    if is_voice {
                        spans.push(Span::styled(
                            "+".to_string(),
                            Style::default().fg(app.theme_colors.nick_voice).bg(bg).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                Some('B') => {
                    // Bell indicator
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    // Bell is handled by terminal, just show indicator
                }
                Some('F') => {
                    // Flags (user modes)
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    if !app.server().user_modes.is_empty() {
                        spans.push(Span::styled(
                            format!("+{}", app.server().user_modes),
                            Style::default().fg(app.theme_colors.status_bar_info_fg).bg(bg),
                        ));
                    }
                }
                Some('Q') => {
                    // Query nick (private message target)
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    let buf = &app.buffers[app.current_buffer_idx];
                    if !buf.name.starts_with('#') && !buf.name.starts_with('&') && buf.name != "(Status)" {
                        spans.push(Span::styled(
                            buf.name.clone(),
                            Style::default().fg(app.theme_colors.msg_notice).bg(bg).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                Some('R') => {
                    // Room/channel name (alias for %C)
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    let buf = &app.buffers[app.current_buffer_idx];
                    if buf.name != "(Status)" {
                        spans.push(Span::styled(
                            buf.name.clone(),
                            Style::default().fg(app.theme_colors.input_prompt_fg).bg(bg).add_modifier(Modifier::BOLD),
                        ));
                    }
                }
                Some('U') => {
                    // User count in current channel
                    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
                    let buf = &app.buffers[app.current_buffer_idx];
                    spans.push(Span::styled(
                        buf.nicks.len().to_string(),
                        Style::default().fg(app.theme_colors.msg_system).bg(bg),
                    ));
                }
                Some('%') => {
                    current_text.push('%');
                }
                Some(other) => {
                    current_text.push('%');
                    current_text.push(other);
                }
                None => {
                    current_text.push('%');
                }
            }
        } else {
            current_text.push(c);
        }
    }

    // Flush remaining text
    flush(&mut current_text, &mut spans, app.theme_colors.status_bar_info_fg, bg);
    spans
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
