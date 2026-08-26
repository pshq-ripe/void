/// ASCII art MOTD logos for Void IRC Client
/// All logos clearly spell "VOID" — randomly selected on startup
/// Generated with figlet + toilet

const LOGOS: &[&str] = &[
    // bigmono12 — block letters
    r#"
 ██▒  ▒██   ░████░    ██████   █████▒
 ██▓  ▓██   ██████    ██████   ███████
 ▒██  ██▒  ▒██  ██▒     ██     ██  ▒██▒
 ▒██  ██▒  ██▒  ▒██     ██     ██   ▒██
  ██ ░██   ██    ██     ██     ██   ░██
  ██▒▒██   ██    ██     ██     ██    ██
  ▒████▒   ██    ██     ██     ██   ░██
  ░████░   ██▒  ▒██     ██     ██   ▒██
   ████    ▒██  ██▒     ██     ██  ▒██▒
   ████     ██████    ██████   ███████
    ██      ░████░    ██████   █████▒
"#,
    // bigascii12 — ASCII block
    r#"
 ##:  :##   .####.    ######   #####:
 ##    ##   ######    ######   #######
 :##  ##:  :##  ##:     ##     ##  :##:
 :##  ##:  ##:  :##     ##     ##   :##
  ## .##   ##    ##     ##     ##   .##
  ##::##   ##    ##     ##     ##    ##
  :####:   ##    ##     ##     ##   .##
  .####.   ##:  :##     ##     ##   :##
   ####    :##  ##:     ##     ##  :##:
   ####     ######    ######   #######
    ##      .####.    ######   #####: 
"#,
    // future — clean Unicode
    r#"
╻ ╻┏━┓╻╺┳┓
┃┏┛┃ ┃┃ ┃┃
┗┛ ┗━┛╹╺┻┛
"#,
    // emboss2 — double lines
    r#"
║ ║╔═║╝╔═
║ ║║ ║║║ ║
 ╝ ══╝╝══
"#,
    // letter — simple ASCII
    r#"
V   V  OOO  III DDDD
V   V O   O  I  D   D
V   V O   O  I  D   D
 V V  O   O  I  D   D
  V    OOO  III DDDD
"#,
    // pagga — block dots
    r#"
░█░█░█▀█░▀█▀░█▀▄
░▀▄▀░█░█░░█░░█░█
░░▀░░▀▀▀░▀▀▀░▀▀░
"#,
    // smblock — small block
    r#"
▌ ▌▞▀▖▜▘▛▀▖
▚▗▘▌ ▌▐ ▌ ▌
▝▞ ▌ ▌▐ ▌ ▌
 ▘ ▝▀ ▀▘▀▀
"#,
    // smbraille — braille dots
    r#"
 ⡇⢸ ⡎⢱ ⡇ ⡏⢱
 ⠸⠃ ⠣⠜ ⠇ ⠧⠜
"#,
];

const WELCOME_LINES: &[&str] = &[
    "Void IRC Client v0.1.0 — epic5/epic6 inspired, Rust + Lua",
    "Type /help for commands | /load modules/init.lua for LiCe5 scripts",
    "",
];

/// Get a random VOID logo + welcome message
pub fn get_motd() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos() as usize;
    let logo = LOGOS[seed % LOGOS.len()];

    let mut output = String::new();
    output.push_str(logo);
    for line in WELCOME_LINES {
        output.push_str(line);
        output.push('\n');
    }
    output
}
