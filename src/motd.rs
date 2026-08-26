/// ASCII art MOTD logos for Void IRC Client
/// Randomly selected on startup

const LOGOS: &[&str] = &[
    // Standard FIGlet: big
    r#"
 ██╗   ██╗ ██████╗ ██╗██████╗ 
 ██║   ██║██╔═══██╗██║██╔══██╗
 ██║   ██║██║   ██║██║██║  ██║
 ╚██╗ ██╔╝██║   ██║██║██║  ██║
  ╚████╔╝ ╚██████╔╝██║██████╔╝
   ╚═══╝   ╚═════╝ ╚═╝╚═════╝ 
"#,
    // Slant
    r#"
    __  _______  ______
   / / / /_  __/ ____  /
  / / / / / /  / /  / /
 / /_/ / / /  / /__/ /
 \____/ /_/  \______/
"#,
    // Banner
    r#"
||     ||  _____   ____   _____ 
||     || ||   || ||   | ||   ||
||  _  || ||___|| ||   | ||___||
||_| |_|| ||      ||   | ||     
|__   __| ||      ||___| ||     
   |_|    ||      |____| ||     
"#,
    // 3-D
    r#"
 ___    ___ _________  ______  
\  \  /  //  _______/ / ____ \ 
 \  \/  / |  |__     | |    | |
  \    /  |  __|    | |    | |
   \  /   |  |____  | |____| |
    \/    |_______|  \______/
"#,
    // ANSI Shadow
    r#"
██╗   ██╗ ██████╗ ██╗██████╗ 
██║   ██║██╔═══██╗██║██╔══██╗
██║   ██║██║   ██║██║██║  ██║
╚██╗ ██╔╝██║   ██║██║██║  ██║
 ╚████╔╝ ╚██████╔╝██║██████╔╝
  ╚═══╝   ╚═════╝ ╚═╝╚═════╝ 
"#,
    // Small
    r#"
 _   _
| | | |_   _ _ __ ___   __ _ _ __
| | | | | | | '_ ` _ \ / _` | '__|
| |_| | |_| | | | | | | (_| | |
 \___/ \__,_|_| |_| |_|\__,_|_|
"#,
];

const WELCOME_LINES: &[&str] = &[
    "Welcome to Void IRC Client v0.1.0",
    "Inspired by epic5 + LiCe5 | Rust + Lua",
    "Type /help for commands | /load lice5/init.lua for LiCe5",
    "",
];

/// Get a random logo + welcome message
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
