use crate::app::App;
use crate::commands::registry::{CommandRegistry, CommandResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Obsługa wciśnięcia klawisza
pub fn handle_key(app: &mut App, key: KeyEvent, registry: &CommandRegistry) -> bool {
    // Ctrl+C — wyjście
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('c') => {
                app.running = false;
                return true;
            }
            KeyCode::Char('n') => {
                app.next_buffer();
                return false;
            }
            KeyCode::Char('p') => {
                app.prev_buffer();
                return false;
            }
            KeyCode::Char('x') => {
                // Ctrl+X — cycle windows (epic5 default)
                app.next_buffer();
                return false;
            }
            KeyCode::Char('l') => {
                let buf = app.current_buffer_mut();
                buf.scroll_offset = 0;
                buf.new_while_scrolled = 0;
                return false;
            }
            KeyCode::Char('a') => {
                app.input_cursor_pos = 0;
                return false;
            }
            KeyCode::Char('e') => {
                app.input_cursor_pos = app.input_text.len();
                return false;
            }
            KeyCode::Char('u') => {
                app.input_text.clear();
                app.input_cursor_pos = 0;
                return false;
            }
            KeyCode::Char('k') => {
                app.input_text.truncate(app.input_cursor_pos);
                return false;
            }
            KeyCode::Char('w') => {
                let text = app.input_text.clone();
                let trimmed = text.trim_end();
                if let Some(pos) = trimmed.rfind(' ') {
                    app.input_text = text[..pos].to_string();
                } else {
                    app.input_text.clear();
                }
                app.input_cursor_pos = app.input_text.len();
                return false;
            }
            KeyCode::Char('r') => {
                // Ctrl+R — reverse search w historii
                // Szukaj ostatniej komendy zawierającej aktualny tekst
                let search = app.input_text.to_lowercase();
                if !search.is_empty() {
                    for entry in app.input_history.iter().rev() {
                        if entry.to_lowercase().contains(&search) {
                            app.input_text = entry.clone();
                            app.input_cursor_pos = app.input_text.len();
                            return false;
                        }
                    }
                } else if !app.input_history.is_empty() {
                    // Pusta linia — pokaż ostatnią komendę
                    let last = app.input_history.last().unwrap().clone();
                    app.input_text = last;
                    app.input_cursor_pos = app.input_text.len();
                }
                return false;
            }
            _ => {}
        }
    }

    // Alt+key — IRC formatting codes i skoki okien
    if key.modifiers.contains(KeyModifiers::ALT) {
        match key.code {
            // Alt+1..Alt+9 — skok do okna po numerze
            KeyCode::Char(c @ '1'..='9') => {
                let idx = (c as usize) - ('1' as usize);
                if idx < app.buffers.len() {
                    app.current_buffer_idx = idx;
                    app.buffers[idx].unread_count = 0;
                    app.buffers[idx].has_activity = false;
                }
                return false;
            }
            // Alt+K — wstaw kod koloru mIRC (\x03)
            KeyCode::Char('k') => {
                app.input_text.push('\x03');
                app.input_cursor_pos = app.input_text.len();
                return false;
            }
            // Alt+U — wstaw underline (\x1F)
            KeyCode::Char('u') => {
                app.input_text.push('\x1F');
                app.input_cursor_pos = app.input_text.len();
                return false;
            }
            // Alt+I — wstaw italic (\x1D)
            KeyCode::Char('i') => {
                app.input_text.push('\x1D');
                app.input_cursor_pos = app.input_text.len();
                return false;
            }
            // Alt+R — wstaw reverse (\x16)
            KeyCode::Char('r') => {
                app.input_text.push('\x16');
                app.input_cursor_pos = app.input_text.len();
                return false;
            }
            // Alt+O — wstaw reset (\x0F)
            KeyCode::Char('o') => {
                app.input_text.push('\x0F');
                app.input_cursor_pos = app.input_text.len();
                return false;
            }
            // Alt+B — skok słowem w lewo
            KeyCode::Char('b') => {
                let text = &app.input_text[..app.input_cursor_pos];
                let trimmed = text.trim_end();
                if let Some(pos) = trimmed.rfind(' ') {
                    app.input_cursor_pos = pos;
                } else {
                    app.input_cursor_pos = 0;
                }
                return false;
            }
            // Alt+F — skok słowem w prawo
            KeyCode::Char('f') => {
                if let Some(pos) = app.input_text[app.input_cursor_pos..].find(' ') {
                    app.input_cursor_pos = (app.input_cursor_pos + pos + 1).min(app.input_text.len());
                } else {
                    app.input_cursor_pos = app.input_text.len();
                }
                return false;
            }
            // Alt+D — usuń słowo w prawo
            KeyCode::Char('d') => {
                if app.input_cursor_pos < app.input_text.len() {
                    let rest = &app.input_text[app.input_cursor_pos..];
                    let end = rest.find(' ').map(|p| p + 1).unwrap_or(rest.len());
                    app.input_text = format!("{}{}",
                        &app.input_text[..app.input_cursor_pos],
                        &app.input_text[app.input_cursor_pos + end..]);
                }
                return false;
            }
            _ => {}
        }
    }

    match key.code {
        KeyCode::Char(c) => {
            app.input_text.push(c);
            app.input_cursor_pos = app.input_text.len();
        }
        KeyCode::Backspace => {
            if !app.input_text.is_empty() && app.input_cursor_pos > 0 {
                app.input_text.pop();
                app.input_cursor_pos = app.input_text.len();
            }
        }
        KeyCode::Delete => {
            if app.input_cursor_pos < app.input_text.len() {
                app.input_text.remove(app.input_cursor_pos);
            }
        }
        KeyCode::Left => {
            app.input_cursor_pos = app.input_cursor_pos.saturating_sub(1);
        }
        KeyCode::Right => {
            if app.input_cursor_pos < app.input_text.len() {
                app.input_cursor_pos += 1;
            }
        }
        KeyCode::Home => {
            app.input_cursor_pos = 0;
        }
        KeyCode::End => {
            app.input_cursor_pos = app.input_text.len();
        }
        KeyCode::Up => {
            app.history_prev();
        }
        KeyCode::Down => {
            app.history_next();
        }
        KeyCode::PageUp => {
            let scroll_lines = app.settings.get_int("SCROLL_LINES").max(1) as usize;
            let buf = &mut app.buffers[app.current_buffer_idx];
            let max_scroll = buf.messages.len().saturating_sub(1);
            buf.scroll_offset = (buf.scroll_offset + scroll_lines * 10).min(max_scroll.max(1));
        }
        KeyCode::PageDown => {
            let scroll_lines = app.settings.get_int("SCROLL_LINES").max(1) as usize;
            let buf = &mut app.buffers[app.current_buffer_idx];
            buf.scroll_offset = buf.scroll_offset.saturating_sub(scroll_lines * 10);
            if buf.scroll_offset == 0 {
                buf.new_while_scrolled = 0;
            }
        }
        KeyCode::Tab => {
            // Autouzupełnianie nicków
            attempt_nick_completion(app);
        }
        KeyCode::Enter => {
            if !app.input_text.is_empty() {
                app.push_input_history();

                if app.input_text.starts_with('/') {
                    let input_clone = app.input_text.clone();
                    let parts: Vec<&str> = input_clone.split_whitespace().collect();
                    let cmd_name = &parts[0][1..]; // strip '/'
                    let args = &parts[1..];

                    if let Some(cmd) = registry.find(cmd_name) {
                        let handler = cmd.handler;
                        match handler(app, args) {
                            CommandResult::Ok => {}
                            CommandResult::NeedSender => {
                                app.system_message("-!- Not connected to server.");
                            }
                            CommandResult::Error(e) => {
                                app.system_message(&format!("-!- Error: {}", e));
                            }
                        }
                    } else if let Some(expanded) = app.resolve_alias(cmd_name, args) {
                        // Rozwiń alias i wykonaj
                        execute_expanded(app, &expanded, registry);
                    } else if let Some(ref hooks) = app.lua_hooks {
                        // Sprawdź komendy Lua
                        if let Some(ref lua) = app.lua {
                            let h = hooks.lock().unwrap();
                            let results = crate::scripting::api::call_lua_command(lua, &h, cmd_name, args);
                            drop(h);
                            if let Some(results) = results {
                                for r in results {
                                    app.system_message(&r);
                                }
                            } else {
                                app.system_message(&format!("-!- Unknown command: /{}", cmd_name));
                            }
                        } else {
                            app.system_message(&format!("-!- Unknown command: /{}", cmd_name));
                        }
                    } else {
                        app.system_message(&format!("-!- Unknown command: /{}", cmd_name));
                    }
                } else {
                    // Wysyłanie zwykłej wiadomości z message breaking
                    let buf_name = app.buffers[app.current_buffer_idx].name.clone();
                    if buf_name == "(Status)" {
                        app.system_message("-!- Cannot send text in Status window. Use /join #channel");
                    } else {
                        let text = app.input_text.clone();
                        let nick = app.server().our_nick.clone();
                        // IRC limit: 512 bajtów total, ~490 na tekst (reszta to prefix)
                        let max_len = 490;
                        let parts = break_message(&text, max_len);
                        for part in &parts {
                            crate::irc::proto::send_labeled_privmsg(app, &buf_name, part);
                        }
                        if parts.len() == 1 {
                            app.buffer_message(
                                &buf_name,
                                format!("<{}> {}", nick, text),
                                crate::app::MessageType::Normal,
                            );
                        } else {
                            for (i, part) in parts.iter().enumerate() {
                                app.buffer_message(
                                    &buf_name,
                                    format!("<{}> [{}/{}] {}", nick, i + 1, parts.len(), part),
                                    crate::app::MessageType::Normal,
                                );
                            }
                        }
                    }
                }
                app.input_text.clear();
                app.input_cursor_pos = 0;
            }
        }
        _ => {}
    }
    false
}

/// Wykonaj rozwinięty alias — obsługuje wielokrotne komendy rozdzielone ;
fn execute_expanded(app: &mut App, expanded: &str, registry: &CommandRegistry) {
    // Podziel po ; ale nie wewnątrz cytatów
    let commands: Vec<&str> = expanded.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    for cmd_text in commands {
        if cmd_text.starts_with('/') {
            let parts: Vec<&str> = cmd_text.split_whitespace().collect();
            let cmd_name = &parts[0][1..];
            let args = &parts[1..];
            if let Some(cmd) = registry.find(cmd_name) {
                let handler = cmd.handler;
                match handler(app, args) {
                    CommandResult::Ok => {}
                    CommandResult::NeedSender => {
                        app.system_message("-!- Not connected to server.");
                    }
                    CommandResult::Error(e) => {
                        app.system_message(&format!("-!- Error: {}", e));
                    }
                }
            } else if let Some(next_expanded) = app.resolve_alias(cmd_name, args) {
                execute_expanded(app, &next_expanded, registry);
            } else {
                app.system_message(&format!("-!- Unknown command: /{}", cmd_name));
            }
        } else {
            // Zwykły tekst — wyślij na aktualny kanał
            let buf_name = app.buffers[app.current_buffer_idx].name.clone();
            if buf_name != "(Status)" {
                if let Some(s) = &app.server().sender {
                    let _ = s.send_privmsg(&buf_name, cmd_text);
                }
                app.buffer_message(&buf_name, format!("<{}> {}", app.server().our_nick, cmd_text), crate::app::MessageType::Normal);
            }
        }
    }
}

/// Tab completion — dopasuj nick z aktualnego bufora
fn attempt_nick_completion(app: &mut App) {
    let input = app.input_text.clone();
    let last_word_start = input.rfind(' ').map(|i| i + 1).unwrap_or(0);
    let partial = &input[last_word_start..];
    if partial.is_empty() {
        return;
    }

    let buf = &app.buffers[app.current_buffer_idx];
    let matches: Vec<&str> = buf
        .nicks
        .iter()
        .filter(|n| n.nick.to_lowercase().starts_with(&partial.to_lowercase()))
        .map(|n| n.nick.as_str())
        .collect();

    if matches.len() == 1 {
        let completion = if last_word_start == 0 {
            format!("{}: ", matches[0]) // Na początku linii dodaj dwukropek
        } else {
            format!("{} ", matches[0])
        };
        app.input_text = format!("{}{}", &input[..last_word_start], completion);
        app.input_cursor_pos = app.input_text.len();
    } else if matches.len() > 1 {
        app.system_message(&format!("-!- Completions: {}", matches.join(", ")));
    }
}

/// Podziel wiadomość na części po max_len bajtów (epic6 style)
/// Dzieli na granicach słów, nie łamiąc słów w środku
fn break_message(text: &str, max_len: usize) -> Vec<String> {
    if text.len() <= max_len {
        return vec![text.to_string()];
    }

    let mut parts = Vec::new();
    let mut remaining = text;

    while !remaining.is_empty() {
        if remaining.len() <= max_len {
            parts.push(remaining.to_string());
            break;
        }

        // Znajdź ostatnią spację w zakresie max_len
        let chunk = &remaining[..max_len];
        let split_at = chunk.rfind(' ').unwrap_or(max_len);

        parts.push(remaining[..split_at].to_string());
        remaining = remaining[split_at..].trim_start();
    }

    parts
}
