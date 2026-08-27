use crate::app::{App, MessageType};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::io;

/// Czy bufor to kanał IRC (nie Status, nie query)
fn is_channel(name: &str) -> bool {
    name.starts_with('#') || name.starts_with('&') || name.starts_with('+') || name.starts_with('!')
}

/// Mapowanie numerów kolorów mIRC na kolory ratatui
/// Obsługuje standard (0-15) i extended (16-255) kolory
fn mirc_color(n: u16) -> Color {
    match n {
        // Standard mIRC kolory (0-15)
        0 => Color::White,
        1 => Color::Black,
        2 => Color::Blue,
        3 => Color::Green,
        4 => Color::Red,
        5 => Color::Rgb(128, 0, 0),     // dark red / brown
        6 => Color::Magenta,
        7 => Color::Rgb(165, 128, 0),   // orange / dark yellow
        8 => Color::Yellow,
        9 => Color::LightGreen,
        10 => Color::Cyan,              // teal
        11 => Color::LightCyan,
        12 => Color::LightBlue,
        13 => Color::LightMagenta,      // pink
        14 => Color::DarkGray,
        15 => Color::Gray,
        // Extended kolory (16-255) — mapuj na ratatui indexed colors
        16..=255 => Color::Indexed(n as u8),
        _ => Color::White,
    }
}

/// Kolor nicka na podstawie hasha — spójny kolor dla każdego nicka
fn nick_color(nick: &str) -> Color {
    let hash: u32 = nick.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32));
    let colors = [
        Color::Red, Color::Green, Color::Yellow, Color::Blue,
        Color::Magenta, Color::Cyan, Color::LightRed, Color::LightGreen,
        Color::LightYellow, Color::LightBlue, Color::LightMagenta, Color::LightCyan,
    ];
    colors[(hash as usize) % colors.len()]
}

/// Parsuj kody formatowania IRC (\x02 bold, \x03 color, \x1D italic, itp.)
/// i zwraca Vec<Span> z nałożonymi stylami
fn parse_irc_formatting(text: &str, base_color: Color) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut current_text = String::new();
    let mut bold = false;
    let mut italic = false;
    let mut underline = false;
    let mut reverse = false;
    let mut strikethrough = false;
    let mut fg_color = base_color;
    let mut bg_color: Option<Color> = None;
    let mut chars = text.chars().peekable();

    let flush = |text: &mut String, spans: &mut Vec<Span<'static>>,
                 bold: bool, italic: bool, underline: bool, reverse: bool,
                 strikethrough: bool, fg: Color, bg: Option<Color>| {
        if !text.is_empty() {
            let mut style = Style::default().fg(fg);
            if let Some(bg) = bg {
                style = style.bg(bg);
            }
            let mut modifiers = Modifier::empty();
            if bold { modifiers |= Modifier::BOLD; }
            if italic { modifiers |= Modifier::ITALIC; }
            if underline { modifiers |= Modifier::UNDERLINED; }
            if reverse { modifiers |= Modifier::REVERSED; }
            if strikethrough { modifiers |= Modifier::CROSSED_OUT; }
            if !modifiers.is_empty() {
                style = style.add_modifier(modifiers);
            }
            spans.push(Span::styled(text.clone(), style));
            text.clear();
        }
    };

    while let Some(c) = chars.next() {
        match c {
            '\x02' => {
                flush(&mut current_text, &mut spans, bold, italic, underline, reverse, strikethrough, fg_color, bg_color);
                bold = !bold;
            }
            '\x03' => {
                flush(&mut current_text, &mut spans, bold, italic, underline, reverse, strikethrough, fg_color, bg_color);
                // Parsuj kolor: \x03FG[,BG] — obsługuje 0-255 (256 colors)
                let mut color_str = String::new();
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() && color_str.len() < 3 {
                        color_str.push(next);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if color_str.is_empty() {
                    // Reset koloru
                    fg_color = base_color;
                    bg_color = None;
                } else {
                    fg_color = mirc_color(color_str.parse().unwrap_or(0));
                    // Sprawdź czy jest przecinek i kolor tła
                    if chars.peek() == Some(&',') {
                        chars.next();
                        let mut bg_str = String::new();
                        while let Some(&next) = chars.peek() {
                            if next.is_ascii_digit() && bg_str.len() < 3 {
                                bg_str.push(next);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        if !bg_str.is_empty() {
                            bg_color = Some(mirc_color(bg_str.parse().unwrap_or(1)));
                        }
                    }
                }
            }
            '\x0F' => {
                // Reset all
                flush(&mut current_text, &mut spans, bold, italic, underline, reverse, strikethrough, fg_color, bg_color);
                bold = false;
                italic = false;
                underline = false;
                reverse = false;
                strikethrough = false;
                fg_color = base_color;
                bg_color = None;
            }
            '\x1D' => {
                flush(&mut current_text, &mut spans, bold, italic, underline, reverse, strikethrough, fg_color, bg_color);
                italic = !italic;
            }
            '\x1F' => {
                flush(&mut current_text, &mut spans, bold, italic, underline, reverse, strikethrough, fg_color, bg_color);
                underline = !underline;
            }
            '\x16' => {
                flush(&mut current_text, &mut spans, bold, italic, underline, reverse, strikethrough, fg_color, bg_color);
                reverse = !reverse;
            }
            '\x1E' => {
                flush(&mut current_text, &mut spans, bold, italic, underline, reverse, strikethrough, fg_color, bg_color);
                strikethrough = !strikethrough;
            }
            _ => {
                current_text.push(c);
            }
        }
    }
    flush(&mut current_text, &mut spans, bold, italic, underline, reverse, strikethrough, fg_color, bg_color);
    if spans.is_empty() {
        spans.push(Span::raw(String::new()));
    }
    highlight_urls(spans)
}

/// Podświetl URL-e w spanach — http/https/ftp/www podkreślone na niebiesko
fn highlight_urls(spans: Vec<Span<'static>>) -> Vec<Span<'static>> {
    let mut result = Vec::new();
    for span in spans {
        let text: &str = span.content.as_ref();
        // Znajdź pozycje URL-i
        let mut last_end = 0;
        let mut found = false;
        for (start, url_start) in find_url_positions(text) {
            found = true;
            if start > last_end {
                result.push(Span::styled(
                    text[last_end..start].to_string(),
                    span.style,
                ));
            }
            result.push(Span::styled(
                text[start..url_start].to_string(),
                span.style.add_modifier(Modifier::UNDERLINED).fg(Color::LightBlue),
            ));
            last_end = url_start;
        }
        if found && last_end < text.len() {
            result.push(Span::styled(
                text[last_end..].to_string(),
                span.style,
            ));
        }
        if !found {
            result.push(span);
        }
    }
    result
}

/// Znajdź pozycje URL-ów w tekście — zwraca (start_punktu, end_punktu)
fn find_url_positions(text: &str) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    let lower = text.to_lowercase();
    let prefixes = ["https://", "http://", "ftp://", "www."];
    for prefix in &prefixes {
        let mut search_from = 0;
        while let Some(pos) = lower[search_from..].find(prefix) {
            let abs_pos = search_from + pos;
            // Znajdź koniec URL (spacja, newline, lub koniec tekstu)
            let end = text[abs_pos..].find(|c: char| c.is_whitespace() || c == '"')
                .map(|p| abs_pos + p)
                .unwrap_or(text.len());
            positions.push((abs_pos, end));
            search_from = end;
        }
    }
    positions.sort_by_key(|p| p.0);
    positions
}

/// Renderuj chat buffer w danym area
fn render_chat(f: &mut ratatui::Frame, area: ratatui::layout::Rect, buf: &crate::app::Buffer, settings: &crate::app::Settings, scroll_override: Option<usize>, theme: &crate::app::ThemeColors) {
    let show_indicator = buf.new_while_scrolled > 0 && scroll_override.unwrap_or(buf.scroll_offset) > 0;
    let chat_height = area.height as usize;
    let total_msgs = buf.messages.len();
    let show_timestamps = settings.get_bool("SHOW_TIMESTAMPS");
    let scroll_offset = scroll_override.unwrap_or(buf.scroll_offset);

    let all_text: Vec<Line> = buf.messages
        .iter()
        .map(|m| {
            let color = match m.msg_type {
                MessageType::Normal => theme.msg_normal,
                MessageType::Action => theme.msg_action,
                MessageType::System => theme.msg_system,
                MessageType::Notice => theme.msg_notice,
                MessageType::Ctcp => theme.msg_ctcp,
                MessageType::ServerReply => theme.msg_server,
                MessageType::Error => theme.msg_error,
                MessageType::Highlight => theme.msg_highlight,
            };

            let msg_spans = if (m.msg_type == MessageType::Normal || m.msg_type == MessageType::Highlight)
                && m.text.starts_with('<')
            {
                if let Some(end) = m.text.find('>') {
                    let nick = &m.text[1..end];
                    let rest = &m.text[end..];
                    let nc = nick_color(nick);
                    // Szukaj prefixu w liście nicków
                    let prefix = buf.nicks.iter()
                        .find(|n| n.nick == nick)
                        .map(|n| n.prefix.as_str())
                        .unwrap_or("");
                    let prefix_color = match prefix {
                        s if s.contains('@') => Color::Red,
                        s if s.contains('+') => Color::Yellow,
                        s if s.contains('%') => Color::Cyan,
                        s if s.contains('~') => Color::Magenta,
                        s if s.contains('&') => Color::Red,
                        _ => Color::DarkGray,
                    };
                    let mut v = vec![
                        Span::styled("<".to_string(), Style::default().fg(color)),
                    ];
                    if !prefix.is_empty() {
                        v.push(Span::styled(prefix.to_string(), Style::default().fg(prefix_color)));
                    }
                    v.push(Span::styled(nick.to_string(), Style::default().fg(nc)));
                    v.extend(parse_irc_formatting(rest, color));
                    v
                } else {
                    parse_irc_formatting(&m.text, color)
                }
            } else if m.msg_type == MessageType::Action && m.text.starts_with("* ") {
                if let Some(space_pos) = m.text[2..].find(' ').map(|p| p + 2) {
                    let nick = &m.text[2..space_pos];
                    let rest = &m.text[space_pos..];
                    let nc = nick_color(nick);
                    let prefix = buf.nicks.iter()
                        .find(|n| n.nick == nick)
                        .map(|n| n.prefix.as_str())
                        .unwrap_or("");
                    let prefix_color = match prefix {
                        s if s.contains('@') => Color::Red,
                        s if s.contains('+') => Color::Yellow,
                        s if s.contains('%') => Color::Cyan,
                        s if s.contains('~') => Color::Magenta,
                        s if s.contains('&') => Color::Red,
                        _ => Color::DarkGray,
                    };
                    let mut v = vec![
                        Span::styled("* ".to_string(), Style::default().fg(color)),
                    ];
                    if !prefix.is_empty() {
                        v.push(Span::styled(prefix.to_string(), Style::default().fg(prefix_color)));
                    }
                    v.push(Span::styled(nick.to_string(), Style::default().fg(nc)));
                    v.extend(parse_irc_formatting(rest, color));
                    v
                } else {
                    parse_irc_formatting(&m.text, color)
                }
            } else {
                parse_irc_formatting(&m.text, color)
            };

            if show_timestamps {
                let mut line_spans = vec![
                    Span::styled(format!("[{}] ", m.timestamp), Style::default().fg(Color::DarkGray)),
                ];
                line_spans.extend(msg_spans);
                Line::from(line_spans)
            } else {
                Line::from(msg_spans)
            }
        })
        .collect();

    let scroll_offset = scroll_offset.min(total_msgs.saturating_sub(1));
    let scroll_row = total_msgs.saturating_sub(scroll_offset).saturating_sub(chat_height);

    let chat_paragraph = Paragraph::new(all_text)
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: false })
        .scroll((scroll_row as u16, 0));
    f.render_widget(chat_paragraph, area);

    // Scrollback indicator — nowe wiadomości podczas scrollowania
    if show_indicator {
        let indicator_text = format!(" [{} new] ", buf.new_while_scrolled);
        let indicator = Paragraph::new(Span::styled(
            indicator_text,
            Style::default().fg(Color::Yellow).bg(Color::DarkGray).add_modifier(Modifier::BOLD),
        ));
        // Wyświetl na dole okna
        let indicator_area = ratatui::layout::Rect {
            x: area.x,
            y: area.y + area.height.saturating_sub(1),
            width: area.width.min(20),
            height: 1,
        };
        f.render_widget(indicator, indicator_area);
    }
}

pub fn draw(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &App) -> anyhow::Result<()> {
    terminal.draw(|f| {
        let area = f.area();
        let buf = app.current_buffer();
        let show_nicks = is_channel(&buf.name);

        // Główny podział pionowy: [Topic] | [Chat(+Nicks)] | [StatusBar] | [Input]
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Length(1),  // Topic bar
                Constraint::Min(5),    // Chat + Nicks
                Constraint::Length(1), // Status bar
                Constraint::Length(3), // Input
            ])
            .split(area);

        // ─── Topic bar ──────────────────────────────────
        let topic_text = if buf.topic.is_empty() {
            format!(" {} ", buf.name)
        } else {
            format!(" {} — {} ", buf.name, buf.topic)
        };
        let topic_bar = Paragraph::new(Span::styled(
            topic_text,
            Style::default().fg(app.theme_colors.topic_bar_fg).bg(app.theme_colors.topic_bar_bg),
        ))
        .style(Style::default().bg(app.theme_colors.topic_bar_bg));
        f.render_widget(topic_bar, main_chunks[0]);

        // ─── Split screen: podziel chat na dwa buforów ────
        let split_areas = if let Some(split_idx) = app.split_buffer_idx {
            let direction = if app.split_horizontal {
                Direction::Horizontal
            } else {
                Direction::Vertical
            };
            let areas = Layout::default()
                .direction(direction)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(main_chunks[1]);
            Some((areas, split_idx))
        } else {
            None
        };

        // ─── Chat + Nicks (podział poziomy) ──────────────
        let primary_area = if let Some((ref areas, _)) = split_areas {
            areas[0]
        } else {
            main_chunks[1]
        };
        let nick_width = app.settings.get_int("NICK_WIDTH").max(12).min(40) as u16;
        let chat_area = if show_nicks {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(20),
                    Constraint::Length(nick_width),
                ])
                .split(primary_area)
        } else {
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(20)])
                .split(primary_area)
        };

        // ─── Chat window (primary buffer) ────────────────
        render_chat(f, chat_area[0], buf, &app.settings, None, &app.theme_colors);

        // ─── Split screen: drugi bufor ───────────────────
        if let Some((ref areas, split_idx)) = split_areas {
            if split_idx < app.buffers.len() {
                let split_buf = &app.buffers[split_idx];
                let split_area = areas[1];
                // Separator
                let sep_direction = if app.split_horizontal {
                    Direction::Horizontal
                } else {
                    Direction::Vertical
                };
                let separator = Paragraph::new(Span::styled(
                    format!(" {} ", split_buf.name),
                    Style::default().fg(Color::DarkGray).bg(Color::Black),
                ));
                let sep_chunks = Layout::default()
                    .direction(sep_direction)
                    .constraints([Constraint::Length(1), Constraint::Min(1)])
                    .split(split_area);
                f.render_widget(separator, sep_chunks[0]);
                render_chat(f, sep_chunks[1], split_buf, &app.settings, Some(app.split_scroll_offset), &app.theme_colors);
            }
        }

        // ─── Nicks list (tylko na kanałach) ─────────────
        if show_nicks {
            let nicks_text: Vec<Line> = buf
                .nicks
                .iter()
                .map(|n| {
                    let (prefix_color, nick_color) = match n.prefix.as_str() {
                        s if s.contains('@') => (app.theme_colors.nick_op, app.theme_colors.nick_op_nick),
                        s if s.contains('+') => (app.theme_colors.nick_voice, app.theme_colors.nick_voice_nick),
                        s if s.contains('%') => (app.theme_colors.nick_halfop, app.theme_colors.nick_halfop_nick),
                        s if s.contains('~') => (app.theme_colors.nick_founder, app.theme_colors.nick_founder_nick),
                        s if s.contains('&') => (app.theme_colors.nick_admin, app.theme_colors.nick_admin_nick),
                        _ => (app.theme_colors.nick_normal_prefix, app.theme_colors.nick_normal),
                    };
                    Line::from(vec![
                        Span::styled(&n.prefix, Style::default().fg(prefix_color)),
                        Span::styled(&n.nick, Style::default().fg(nick_color)),
                    ])
                })
                .collect();
            let nicks_paragraph = Paragraph::new(nicks_text).block(
                Block::default()
                    .borders(Borders::LEFT)
                    .border_style(Style::default().fg(app.theme_colors.border)),
            );
            f.render_widget(nicks_paragraph, chat_area[1]);
        }

        // ─── Status bar ─────────────────────────────────
        // Oblicz szerokości etykiet i przewiń żeby aktywny bufor był widoczny
        let bar_width = main_chunks[2].width as usize;
        let labels: Vec<String> = app.buffers.iter().enumerate().map(|(i, b)| {
            let is_chan = is_channel(&b.name);
            if b.unread_count > 0 && i != app.current_buffer_idx {
                if is_chan { format!(" {}({})({}) ", b.name, b.nicks.len(), b.unread_count) }
                else { format!(" {}({}) ", b.name, b.unread_count) }
            } else if is_chan { format!(" {}({}) ", b.name, b.nicks.len()) }
            else { format!(" {} ", b.name) }
        }).collect();

        // Znajdź offset scrollowania — aktywny bufor musi być widoczny
        let mut tab_offset = 0usize;
        // Sprawdź czy wszystkie się mieszają
        let total_width: usize = labels.iter().map(|l| l.len()).sum();
        if total_width > bar_width.saturating_sub(20) {
            // Nie mieszczą się — przewiń żeby aktywny był widoczny
            let mut width_before_active = 0usize;
            for i in 0..app.current_buffer_idx {
                width_before_active += labels[i].len();
            }
            // Przewiń żeby aktywny bufor był w środku
            let target_pos = bar_width / 3;
            if width_before_active > target_pos {
                tab_offset = width_before_active - target_pos;
            }
        }

        let mut buf_spans = vec![];
        let mut x = 0usize;
        for (i, b) in app.buffers.iter().enumerate() {
            let label = &labels[i];
            let label_width = label.len();
            // Sprawdź czy ten tab jest widoczny
            if x + label_width < tab_offset {
                x += label_width;
                continue;
            }
            if x >= tab_offset + bar_width.saturating_sub(20) {
                break;
            }
            x += label_width;

            let is_active = i == app.current_buffer_idx;
            if is_active {
                buf_spans.push(Span::styled(
                    label.clone(),
                    Style::default()
                        .fg(app.theme_colors.status_bar_active_fg)
                        .bg(app.theme_colors.status_bar_active_bg)
                        .add_modifier(Modifier::BOLD),
                ));
            } else if b.has_activity {
                buf_spans.push(Span::styled(
                    label.clone(),
                    Style::default()
                        .fg(app.theme_colors.status_bar_activity_fg)
                        .bg(app.theme_colors.status_bar_activity_bg),
                ));
            } else {
                buf_spans.push(Span::styled(
                    label.clone(),
                    Style::default()
                        .fg(app.theme_colors.status_bar_fg)
                        .bg(app.theme_colors.status_bar_bg),
                ));
            }
        }

        // Separator
        buf_spans.push(Span::styled(
            "│",
            Style::default().fg(app.theme_colors.border).bg(app.theme_colors.status_bar_bg),
        ));

        // Connection status
        let conn_icon = if app.server().connected { "●" } else { "○" };
        let conn_color = if app.server().connected { Color::LightGreen } else { Color::LightRed };
        buf_spans.push(Span::styled(
            format!(" {} ", conn_icon),
            Style::default().fg(conn_color).bg(app.theme_colors.status_bar_bg),
        ));

        // Server name
        buf_spans.push(Span::styled(
            format!("{} ", app.server().host),
            Style::default().fg(app.theme_colors.status_bar_info_fg).bg(app.theme_colors.status_bar_bg),
        ));

        // Nick
        buf_spans.push(Span::styled(
            format!("{} ", app.server().our_nick),
            Style::default().fg(app.theme_colors.status_bar_info_fg).bg(app.theme_colors.status_bar_bg).add_modifier(Modifier::BOLD),
        ));

        // Away indicator
        if app.server().away_message.is_some() {
            buf_spans.push(Span::styled(
                "AWAY ",
                Style::default().fg(Color::Yellow).bg(app.theme_colors.status_bar_bg).add_modifier(Modifier::BOLD),
            ));
        }

        // User modes
        if !app.server().user_modes.is_empty() {
            buf_spans.push(Span::styled(
                format!("+{} ", app.server().user_modes),
                Style::default().fg(Color::DarkGray).bg(app.theme_colors.status_bar_bg),
            ));
        }

        // Separator
        buf_spans.push(Span::styled(
            "│",
            Style::default().fg(app.theme_colors.border).bg(app.theme_colors.status_bar_bg),
        ));

        // SASL status
        if app.server().nick_password.is_some() {
            buf_spans.push(Span::styled(
                " SASL ",
                Style::default().fg(Color::LightGreen).bg(app.theme_colors.status_bar_bg),
            ));
        }

        // Scroll indicator
        let scroll_offset = app.buffers[app.current_buffer_idx].scroll_offset;
        if scroll_offset > 0 {
            buf_spans.push(Span::styled(
                format!(" ↑{} ", scroll_offset),
                Style::default().fg(Color::Yellow).bg(app.theme_colors.status_bar_bg).add_modifier(Modifier::BOLD),
            ));
        }

        let status_bar = Paragraph::new(Line::from(buf_spans))
            .style(Style::default().bg(app.theme_colors.status_bar_bg));
        f.render_widget(status_bar, main_chunks[2]);

        // ─── Input line ─────────────────────────────────
        let prompt = app.settings.get("INPUT_PROMPT").to_string();
        let cursor_pos = app.input_cursor_pos;
        let display_input = format!("{}{}", prompt, app.input_text);
        let input_block = Paragraph::new(Span::styled(
            &display_input,
            Style::default().fg(app.theme_colors.input_fg),
        ))
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::DarkGray)),
        );
        f.render_widget(input_block, main_chunks[3]);

        // Pozycja kursora w linii wejścia
        let cursor_x = main_chunks[3].x + prompt.len() as u16 + cursor_pos as u16;
        let cursor_y = main_chunks[3].y + 1; // +1 bo border TOP
        f.set_cursor_position((cursor_x, cursor_y));
    })?;
    Ok(())
}
