/// ASCII art MOTD logos for Void IRC Client
/// All logos clearly spell "VOID" — randomly selected on startup

const LOGOS: &[&str] = &[
    // ANSI Shadow (block letters)
    r#"
 ██╗   ██╗  ██████╗  ██╗  ██╗ ██████╗ 
 ██║   ██║ ██╔═══██╗ ██║  ██║ ██╔══██╗
 ██║   ██║ ██║   ██║ ██║  ██║ ██║  ██║
 ╚██╗ ██╔╝ ██║   ██║ ██║  ██║ ██║  ██║
  ╚████╔╝  ╚██████╔╝ ╚█████╔╝ ██║  ██║
   ╚═══╝    ╚═════╝   ╚════╝  ╚═╝  ╚═╝
"#,
    // Big (FIGlet standard)
    r#"
__     __  ____  _____  ____  
\ \   / / / __ \|  __ \|  _ \ 
 \ \_/ / | |  | | |  | | | | |
  \   /  | |  | | |  | | | | |
   | |   | |__| | |__| | |_| |
   |_|    \____/|_____/|____/ 
"#,
    // Slant
    r#"
    __     ______   ______   ______
   / /    / ____/  / ____/  / ____/
  / /    / /      / /      / /     
 / /___ / /___   / /___   / /___   
/_____/ \_____/  \_____/  \_____/  
"#,
    // Banner
    r#"
||    ||   _____   ___    ___   ____  
||    ||  |  __ | |   |  |   | |   | 
||    ||  | |  | | |   |  |   | |   | 
|| /\ ||  | |  | | |   |  |   | |   | 
|||  |||  | |__| | |___|  |___| |___| 
|_/    \|  |_____| |____| |____|_____| 
"#,
    // 3-D
    r#"
 ___      ___   ____   _____ 
\  \    /  /  / __ \ | ____|
 \  \  /  /  | |  | || |__  
  \  \/  /   | |  | ||  __| 
   \    /    | |__| || |___ 
    \__/      \____/ |_____|
"#,
    // Small (clean)
    r#"
 _    ___  ___  ____  
| |  / _ \/ _ \|  _ \ 
| | | | | | | | | | |
| |__| |_| | |_| |_| |
|____|\___/ \___/|____/
"#,
    // Block
    r#"
 ##  ##   ####   ##  ##  ####  
 ##  ##  ##  ##  ### ##  ##  ## 
 ##  ##  ##  ##  ######  ##  ## 
  ####   ##  ##  ## ###  ##  ## 
   ##     ####   ##  ##  ####  
"#,
    // Isometric 3D
    r#"
  ___      ___   ____   _____  
 |   \    |   | |    | |     | 
 |    \   |   | |    | |     | 
 |  |\ \  |   | |    | |     | 
 |  | \ \ |   | |    | |     | 
 |  |  \ \|   | |    | |     | 
 |__|   \_____| |____| |_____| 
"#,
];

const WELCOME_LINES: &[&str] = &[
    "Void IRC Client v0.1.0 — epic5/epic6 inspired, Rust + Lua",
    "Type /help for commands | /load lice5/init.lua for LiCe5 scripts",
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
