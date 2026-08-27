/// ASCII art MOTD — fixed logo from motd.txt

const LOGO: &str = r#"
 ██▒   █▓ ▒█████   ██▓▓█████▄ 
▓██░   █▒▒██▒  ██▒▓██▒▒██▀ ██▌
 ▓██  █▒░▒██░  ██▒▒██▒░██   █▌
  ▒██ █░░▒██   ██░░██░░▓█▄   ▌
   ▒▀█░  ░ ████▓▒░░██░░▒████▓ 
   ░ ▐░  ░ ▒░▒░▒░ ░▓   ▒▒▓  ▒ 
   ░ ░░    ░ ▒ ▒░  ▒ ░ ░ ▒  ▒ 
     ░░  ░ ░ ░ ▒   ▒ ░ ░ ░  ░ 
      ░      ░ ░   ░     ░    
     ░                 ░      
"#;

const WELCOME_LINES: &[&str] = &[
    "Void IRC Client v0.3.0 — epic5/epic6 inspired, Rust + Lua",
    "Type /help for commands | /load modules/init.lua for LiCe5 scripts",
    "",
];

/// Get the VOID logo + welcome message
pub fn get_motd() -> String {
    let mut output = String::new();
    output.push_str(LOGO);
    for line in WELCOME_LINES {
        output.push_str(line);
        output.push('\n');
    }
    output
}
