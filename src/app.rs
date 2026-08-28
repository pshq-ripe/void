use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::Instant;
use ratatui::style::Color;
use crate::logging::Logger;
use crate::flood::FloodProtection;
use crate::dcc::DccManager;
use crate::storage::Storage;
use crate::scripting::api::LuaHooks;

/// Pojedynczy bufor (okno) — kanał, prywatna rozmowa lub Status.
pub struct Buffer {
    pub name: String,
    pub messages: Vec<StyledMessage>,
    pub nicks: Vec<NickEntry>,
    pub topic: String,
    pub scroll_offset: usize,
    pub unread_count: usize,
    pub has_activity: bool,
    pub new_while_scrolled: usize,  // nowe wiadomości podczas scrollback
    pub charset: String,            // charset for this buffer (empty = use default)
}

/// Wpis na liście nicków z prefixem trybu (@, +, %, ~, &)
#[derive(Clone, Debug)]
pub struct NickEntry {
    pub nick: String,
    pub prefix: String,  // "@" = op, "+" = voice, "%" = halfop, "~" = owner, "&" = admin
    pub account: String,  // IRCv3 extended-join account
    pub realname: String, // IRCv3 extended-join realname
}

impl NickEntry {
    pub fn new(raw: &str) -> Self {
        let prefixes = ['~', '&', '@', '%', '+'];
        let mut prefix = String::new();
        let mut nick = raw.to_string();
        for p in &prefixes {
            if nick.starts_with(*p) {
                prefix.push(nick.remove(0));
            }
        }
        NickEntry { nick, prefix, account: String::new(), realname: String::new() }
    }

    pub fn display(&self) -> String {
        format!("{}{}", self.prefix, self.nick)
    }

    /// Sortowanie: operatorzy pierwsi, potem voice, potem reszta
    pub fn sort_key(&self) -> (u8, String) {
        let rank = match self.prefix.as_str() {
            s if s.contains('~') => 0,
            s if s.contains('&') => 1,
            s if s.contains('@') => 2,
            s if s.contains('%') => 3,
            s if s.contains('+') => 4,
            _ => 5,
        };
        (rank, self.nick.to_lowercase())
    }
}

/// Wiadomość ze znacznikiem czasu i typem
#[derive(Clone)]
pub struct StyledMessage {
    pub timestamp: String,
    pub text: String,
    pub msg_type: MessageType,
}

#[derive(Clone, PartialEq)]
pub enum MessageType {
    Normal,        // <nick> tekst
    Action,        // * nick robi coś
    System,        // -!- komunikat systemowy
    Notice,        // -nick- tekst
    Ctcp,          // CTCP
    ServerReply,   // Odpowiedź serwera (MOTD, numeric)
    Error,         // Błąd
    Highlight,     // Wiadomość zawierająca nasz nick
}

impl Buffer {
    pub fn new(name: &str) -> Self {
        Buffer {
            name: name.to_string(),
            messages: Vec::new(),
            nicks: Vec::new(),
            topic: String::new(),
            scroll_offset: 0,
            unread_count: 0,
            has_activity: false,
            new_while_scrolled: 0,
            charset: String::new(),
        }
    }

    pub fn push_message(&mut self, text: String, msg_type: MessageType, scrollback_limit: usize) {
        let timestamp = chrono::Local::now().format("%H:%M").to_string();
        self.messages.push(StyledMessage {
            timestamp,
            text,
            msg_type,
        });
        // Scrollback indicator — zliczaj nowe wiadomości gdy użytkownik scrolluje
        if self.scroll_offset > 0 {
            self.new_while_scrolled += 1;
        }
        let limit = scrollback_limit.max(100);
        if self.messages.len() > limit {
            self.messages.remove(0);
            if self.scroll_offset > 0 {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
            }
        }
    }

    pub fn add_nick(&mut self, raw: &str) {
        let entry = NickEntry::new(raw);
        if !self.nicks.iter().any(|n| n.nick == entry.nick) {
            self.nicks.push(entry);
            self.sort_nicks();
        }
    }

    pub fn remove_nick(&mut self, nick: &str) {
        self.nicks.retain(|n| n.nick != nick);
    }

    pub fn rename_nick(&mut self, old: &str, new: &str) {
        if let Some(entry) = self.nicks.iter_mut().find(|n| n.nick == old) {
            entry.nick = new.to_string();
            self.sort_nicks();
        }
    }

    pub fn set_nick_info(&mut self, nick: &str, account: &str, realname: &str) {
        if let Some(entry) = self.nicks.iter_mut().find(|n| n.nick == nick) {
            entry.account = account.to_string();
            entry.realname = realname.to_string();
        }
    }

    pub fn set_nick_prefix(&mut self, nick: &str, prefix_char: char, add: bool) {
        if let Some(entry) = self.nicks.iter_mut().find(|n| n.nick == nick) {
            if add {
                if !entry.prefix.contains(prefix_char) {
                    entry.prefix.push(prefix_char);
                }
            } else {
                entry.prefix.retain(|c| c != prefix_char);
            }
            self.sort_nicks();
        }
    }

    pub fn sort_nicks(&mut self) {
        self.nicks.sort_by(|a, b| a.sort_key().cmp(&b.sort_key()));
    }
}

/// Wpis w systemie /NOTIFY
pub struct NotifyEntry {
    pub nick: String,
    pub online: bool,
    pub last_seen: Option<Instant>,
    pub userhost: String,       // user@host when online
    pub channels: Vec<String>,  // channels when online
    pub verified: bool,         // WHOIS verified
    pub action: String,         // action on signon/signoff (e.g., "echo", "beep")
}

/// Wpis w systemie /IGNORE
#[derive(Clone)]
pub struct IgnoreEntry {
    pub pattern: String,
    pub ignore_public: bool,
    pub ignore_private: bool,
    pub ignore_notice: bool,
    pub ignore_ctcp: bool,
    pub ignore_all: bool,
}

/// Wpis w systemie /TIMER
pub struct TimerEntry {
    pub name: String,
    pub interval_ms: u64,
    pub repeat: i32,   // -1 = infinite
    pub command: String,
    pub next_fire: Instant,
    pub remaining: i32,
}

/// Informacja o połączeniu z jednym serwerem
pub struct ServerConnection {
    pub host: String,
    pub port: u16,
    pub tls: bool,
    pub connected: bool,
    pub our_nick: String,
    pub away_message: Option<String>,
    pub sender: Option<irc::client::Sender>,
    pub user_modes: String,
    pub server_info: ServerInfo,
    pub nick_password: Option<String>,
    pub lag_ms: u64,                    // current lag in milliseconds
    pub lag_ping_sent: Option<std::time::Instant>, // when ping was sent
    pub raw_log: Vec<String>,          // raw IRC protocol log
    pub raw_log_enabled: bool,
    pub netsplit_active: bool,
    pub netsplit_nicks: Vec<String>,   // nicks lost in current netsplit
    pub netsplit_server: String,       // server that split
    pub netsplit_start: Option<std::time::Instant>,
    pub ban_list: Vec<BanEntry>,       // tracked ban list per channel
    pub chatnets: HashMap<String, ChatNet>, // IRC network configs
    pub write_buffer: VecDeque<String>, // outgoing message buffer
    pub massjoin_buffer: Vec<(String, String, String)>, // (nick, channel, host) buffered joins
    pub massjoin_timer: Option<Instant>, // when first join was buffered
    pub nickmatch_cache: HashMap<String, bool>, // pattern -> nick -> matched
    pub pending_redirects: Vec<ServerRedirect>, // pending command redirects
    pub default_charset: String,               // default charset (UTF-8)
    pub bouncer: Option<crate::bouncer::Bouncer>,
}

/// Konfiguracja sieci IRC (chatnet)
#[derive(Clone, Debug)]
pub struct ChatNet {
    pub name: String,
    pub servers: Vec<String>,
    pub default_port: u16,
    pub default_tls: bool,
    pub nickserv_pass: String,
    pub auto_join: Vec<String>,
    pub charset: String,                       // charset for this network
}

/// Redirect tracking — correlate responses to requests
#[derive(Clone, Debug)]
pub struct ServerRedirect {
    pub command: String,      // WHO, MODE, WHOIS, etc.
    pub target: String,       // channel or nick
    pub request_id: String,   // unique identifier
    pub callback: String,     // what to do with the response
}

/// Wpis na liście banów
#[derive(Clone, Debug)]
pub struct BanEntry {
    pub channel: String,
    pub mask: String,
    pub set_by: String,
    pub timestamp: i64,
}

/// Parsowane tokeny ISUPPORT (005)
#[derive(Default)]
pub struct ServerInfo {
    pub network: String,
    pub chantypes: String,       // np. "#&+!"
    pub prefix_modes: String,    // np. "(qaohv)~&@%+"
    pub chanmodes: String,        // np. "beI,k,l,imnpst"
    pub nicklen: Option<usize>,
    pub topiclen: Option<usize>,
    pub channellen: Option<usize>,
    pub modes: Option<usize>,    // max modes per line
    pub tokens: HashMap<String, String>,
}

impl ServerConnection {
    pub fn new(host: &str, port: u16, nick: &str, tls: bool) -> Self {
        ServerConnection {
            host: host.to_string(),
            port,
            tls,
            connected: false,
            our_nick: nick.to_string(),
            away_message: None,
            sender: None,
            user_modes: String::new(),
            server_info: ServerInfo::default(),
            nick_password: None,
            lag_ms: 0,
            lag_ping_sent: None,
            raw_log: Vec::new(),
            raw_log_enabled: false,
            netsplit_active: false,
            netsplit_nicks: Vec::new(),
            netsplit_server: String::new(),
            netsplit_start: None,
            ban_list: Vec::new(),
            chatnets: HashMap::new(),
            write_buffer: VecDeque::new(),
            massjoin_buffer: Vec::new(),
            massjoin_timer: None,
            nickmatch_cache: HashMap::new(),
            pending_redirects: Vec::new(),
            default_charset: "UTF-8".into(),
            bouncer: None,
        }
    }
}

/// Globalny stan konfiguracji (/SET)
pub struct Settings {
    pub map: HashMap<String, String>,
}

impl Settings {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        // Domyślne wartości SET (wzorowane na epic5)
        map.insert("SCROLL_LINES".into(), "1".into());
        map.insert("SCROLLBACK".into(), "500".into());
        map.insert("BEEP_ON_MSG".into(), "OFF".into());
        map.insert("CLOCK_24HOUR".into(), "ON".into());
        map.insert("SHOW_CHANNEL_NAMES".into(), "ON".into());
        map.insert("SHOW_STATUS_ALL".into(), "ON".into());
        map.insert("LOG".into(), "OFF".into());
        map.insert("LOG_FILE".into(), "void.log".into());
        map.insert("FLOOD_PROTECTION".into(), "ON".into());
        map.insert("FLOOD_RATE".into(), "4".into());
        map.insert("FLOOD_RATE_PER".into(), "2".into());
        map.insert("AUTO_RECONNECT".into(), "ON".into());
        map.insert("AUTO_RECONNECT_DELAY".into(), "15".into());
        map.insert("STATUS_FORMAT".into(), " [ $N ] [ $C ] [ $T ] ".into());
        map.insert("INPUT_PROMPT".into(), "> ".into());
        map.insert("SHOW_TIMESTAMPS".into(), "ON".into());
        map.insert("TIMESTAMP_FORMAT".into(), "%H:%M".into());
        map.insert("DCC_DOWNLOAD_DIR".into(), "~/dcc".into());
        map.insert("CTCP_REPLY".into(), "ON".into());
        map.insert("SSL_VERIFY".into(), "OFF".into());
        map.insert("DEBUG".into(), "OFF".into());
        map.insert("MOUSE".into(), "OFF".into());
        map.insert("SHOW_NICKLIST".into(), "ON".into());
        map.insert("SHOW_STATUSBAR".into(), "ON".into());
        map.insert("SHOW_USER_COUNT".into(), "ON".into());
        map.insert("DEFAULT_KICK_REASON".into(), "Requested".into());
        map.insert("DEFAULT_PART_REASON".into(), "Leaving".into());
        map.insert("DEFAULT_QUIT_REASON".into(), "Leaving".into());
        map.insert("CTCP_VERSION".into(), "Void IRC Client v0.3.0 (Rust)".into());
        map.insert("CTCP_USERINFO".into(), "Void IRC Client".into());
        map.insert("CTCP_SOURCE".into(), "https://github.com/pshq-ripe/void".into());
        map.insert("NICK_WIDTH".into(), "18".into());
        Settings { map }
    }

    pub fn get(&self, key: &str) -> &str {
        self.map.get(&key.to_uppercase()).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_uppercase(), value.to_string());
    }

    pub fn get_bool(&self, key: &str) -> bool {
        matches!(self.get(key), "ON" | "1" | "true" | "yes")
    }

    pub fn get_int(&self, key: &str) -> i64 {
        self.get(key).parse().unwrap_or(0)
    }
}

/// Centralna struktura stanu aplikacji
pub struct App {
    pub buffers: Vec<Buffer>,
    pub current_buffer_idx: usize,
    pub input_text: String,
    pub input_history: Vec<String>,
    pub input_history_idx: Option<usize>,
    pub input_cursor_pos: usize,
    pub servers: Vec<ServerConnection>,
    pub active_server_idx: usize,
    pub settings: Settings,
    pub notify_list: Vec<NotifyEntry>,
    pub ignore_list: Vec<IgnoreEntry>,
    pub timers: Vec<TimerEntry>,
    pub last_msg_target: Option<String>,
    pub running: bool,
    pub reconnect_pending: bool,
    pub pending_exec: Vec<String>,
    pub last_buffer_idx: Option<usize>,
    pub aliases: HashMap<String, String>,
    pub highlight_patterns: Vec<HighlightPattern>,
    pub key_bindings: HashMap<String, String>,
    pub label_counter: u64,
    pub pending_labels: HashMap<String, String>, // label -> request description
    pub format_templates: HashMap<String, String>,
    pub status_format: String,
    pub event_formats: HashMap<String, String>,
    pub theme_colors: ThemeColors,
    pub split_buffer_idx: Option<usize>, // None = brak split, Some(idx) = drugi bufor
    pub split_scroll_offset: usize,      // niezależny scroll dla split pane
    pub split_horizontal: bool,          // false=vertical(top/bottom), true=horizontal(left/right)
    pub suppress_display: bool,          // /SHH — wycisz wyświetlanie w aktualnym kontekście
    pub output_context: OutputContext,    // aktualny kontekst wyjścia
    pub storage: Option<Storage>,
    pub lua_hooks: Option<Arc<Mutex<LuaHooks>>>,
    pub lua: Option<Arc<mlua::Lua>>,
    pub lua_ctx: Option<Arc<Mutex<crate::scripting::api::LuaContext>>>,
    pub logger: Logger,
    pub flood: FloodProtection,
    pub dcc: DccManager,
}

/// Wzorzec podświetlania
#[derive(Clone)]
pub struct HighlightPattern {
    pub pattern: String,
    pub color: String,
}

/// Kolory theme'a — używane przez renderer
#[derive(Clone, Debug)]
pub struct ThemeColors {
    pub name: String,
    pub desc: String,
    pub is_dark: bool,
    // Status bar
    pub status_bar_bg: Color,
    pub status_bar_fg: Color,
    pub status_bar_active_bg: Color,
    pub status_bar_active_fg: Color,
    pub status_bar_activity_bg: Color,
    pub status_bar_activity_fg: Color,
    pub status_bar_info_fg: Color,
    // Topic bar
    pub topic_bar_bg: Color,
    pub topic_bar_fg: Color,
    // Input
    pub input_bg: Color,
    pub input_fg: Color,
    pub input_prompt_fg: Color,
    // Borders & UI
    pub border: Color,
    pub timestamp: Color,
    pub scroll_indicator_fg: Color,
    pub scroll_indicator_bg: Color,
    // Messages
    pub msg_normal: Color,
    pub msg_action: Color,
    pub msg_system: Color,
    pub msg_notice: Color,
    pub msg_highlight: Color,
    pub msg_error: Color,
    pub msg_server: Color,
    pub msg_ctcp: Color,
    pub msg_url: Color,
    // Nick list
    pub nick_op: Color,
    pub nick_op_nick: Color,
    pub nick_voice: Color,
    pub nick_voice_nick: Color,
    pub nick_halfop: Color,
    pub nick_halfop_nick: Color,
    pub nick_founder: Color,
    pub nick_founder_nick: Color,
    pub nick_admin: Color,
    pub nick_admin_nick: Color,
    pub nick_normal: Color,
    pub nick_normal_prefix: Color,
    pub nick_list_header: Color,
    // Chat background
    pub chat_bg: Color,
    pub nick_list_bg: Color,
    // Dynamic nick palette
    pub nick_colors: Vec<Color>,
}

impl Default for ThemeColors {
    fn default() -> Self {
        ThemeColors {
            name: "Default".into(),
            desc: "Clean dark theme — white/gray".into(),
            is_dark: true,
            status_bar_bg: Color::DarkGray,
            status_bar_fg: Color::White,
            status_bar_active_bg: Color::White,
            status_bar_active_fg: Color::Black,
            status_bar_activity_bg: Color::Gray,
            status_bar_activity_fg: Color::White,
            status_bar_info_fg: Color::White,
            topic_bar_bg: Color::DarkGray,
            topic_bar_fg: Color::White,
            input_bg: Color::Reset,
            input_fg: Color::White,
            input_prompt_fg: Color::Gray,
            border: Color::DarkGray,
            timestamp: Color::DarkGray,
            scroll_indicator_fg: Color::Black,
            scroll_indicator_bg: Color::Yellow,
            msg_normal: Color::White,
            msg_action: Color::Yellow,
            msg_system: Color::Cyan,
            msg_notice: Color::Magenta,
            msg_highlight: Color::White,
            msg_error: Color::LightRed,
            msg_server: Color::DarkGray,
            msg_ctcp: Color::Red,
            msg_url: Color::LightBlue,
            nick_op: Color::Red,
            nick_op_nick: Color::LightRed,
            nick_voice: Color::Yellow,
            nick_voice_nick: Color::LightYellow,
            nick_halfop: Color::Cyan,
            nick_halfop_nick: Color::LightCyan,
            nick_founder: Color::Magenta,
            nick_founder_nick: Color::LightMagenta,
            nick_admin: Color::Red,
            nick_admin_nick: Color::LightRed,
            nick_normal: Color::White,
            nick_normal_prefix: Color::DarkGray,
            nick_list_header: Color::DarkGray,
            chat_bg: Color::Reset,
            nick_list_bg: Color::Reset,
            nick_colors: vec![
                Color::LightRed,
                Color::LightGreen,
                Color::LightYellow,
                Color::Rgb(137, 180, 250), // Blue
                Color::Rgb(203, 166, 247), // Mauve
                Color::Rgb(148, 226, 213), // Teal
                Color::Rgb(250, 179, 135), // Peach
                Color::Rgb(137, 220, 235), // Sky
                Color::Rgb(245, 194, 231), // Pink
                Color::Rgb(180, 190, 254), // Lavender
                Color::Rgb(235, 160, 172), // Maroon
                Color::Rgb(166, 209, 137), // Olive/LightGreen
            ],
        }
    }
}

/// Kontekst wyjścia (epic6 /ON CONTEXT)
#[derive(Clone, Debug)]
pub struct OutputContext {
    pub server: String,
    pub window: String,
    pub sender: String,
    pub target: String,
    pub level: String,
}

impl Default for OutputContext {
    fn default() -> Self {
        OutputContext {
            server: String::new(),
            window: String::new(),
            sender: String::new(),
            target: String::new(),
            level: String::new(),
        }
    }
}

impl App {
    pub fn new(nick: &str, server: &str, port: u16, tls: bool, db_pass: &str) -> Self {
        let settings = Settings::new();
        let log_file = settings.get("LOG_FILE").to_string();
        let flood_rate = settings.get_int("FLOOD_RATE").max(1) as usize;
        let flood_per = settings.get_int("FLOOD_RATE_PER").max(1) as u64;
        let dcc_dir = settings.get("DCC_DOWNLOAD_DIR").to_string();
        let mut app = App {
            buffers: vec![Buffer::new("(Status)")],
            current_buffer_idx: 0,
            input_text: String::new(),
            input_history: Vec::new(),
            input_history_idx: None,
            input_cursor_pos: 0,
            servers: vec![ServerConnection::new(server, port, nick, tls)],
            active_server_idx: 0,
            settings,
            notify_list: Vec::new(),
            ignore_list: Vec::new(),
            timers: Vec::new(),
            last_msg_target: None,
            running: true,
            reconnect_pending: false,
            pending_exec: Vec::new(),
            last_buffer_idx: None,
            aliases: HashMap::new(),
            highlight_patterns: Vec::new(),
            key_bindings: HashMap::new(),
            label_counter: 0,
            pending_labels: HashMap::new(),
            format_templates: {
                let mut m = HashMap::new();
                m.insert("JOIN".into(), "* $0 has joined $1".into());
                m.insert("PART".into(), "* $0 has left $1 ($2)".into());
                m.insert("QUIT".into(), "* $0 has quit IRC ($1)".into());
                m.insert("KICK".into(), "* $0 was kicked from $1 by $2 ($3)".into());
                m.insert("NICK".into(), "* $0 is now known as $1".into());
                m.insert("MODE".into(), "* $0 sets mode: $1".into());
                m.insert("TOPIC".into(), "* $0 set topic to: $1".into());
                m.insert("MSG".into(), "<$0> $1".into());
                m.insert("ACTION".into(), "* $0 $1".into());
                m.insert("NOTICE".into(), "-$0- $1".into());
                m
            },
            status_format: "%T %N%# %@%C%+ %W %A %H%B %F %Q%M".into(),
            event_formats: {
                let mut m = HashMap::new();
                m.insert("join".into(), "* $ch($1): $0 ($2)".into());
                m.insert("part".into(), "* $ch($1): $0 ($2)".into());
                m.insert("quit".into(), "* Signoff $0 ($1)".into());
                m.insert("kick".into(), "* $ch($2): $0 by $1 ($3)".into());
                m.insert("nick".into(), "* $0 is now known as $1".into());
                m.insert("mode".into(), "* $ch($1): \"$2\" by $0".into());
                m.insert("topic".into(), "* $ch($1): $0 changed topic to: $2".into());
                m.insert("msg".into(), "[$0] $1".into());
                m.insert("notice".into(), "-$0- $1".into());
                m.insert("action".into(), "* $0 $1".into());
                m.insert("public".into(), "<$0> $1".into());
                m.insert("invite".into(), "* $0 invites you to $1".into());
                m.insert("ctcp".into(), "* CTCP $1 from $0".into());
                m.insert("nick_signon".into(), "* Signon detected: $0".into());
                m.insert("nick_signoff".into(), "* Signoff detected: $0".into());
                m
            },
            theme_colors: ThemeColors::default(),
            split_buffer_idx: None,
            split_scroll_offset: 0,
            split_horizontal: false,
            suppress_display: false,
            output_context: OutputContext::default(),
            storage: Storage::open("~/.void/void.db", db_pass).ok(),
            lua_hooks: None,
            lua: None,
            lua_ctx: None,
            logger: Logger::new(&log_file),
            flood: FloodProtection::new(flood_rate, flood_per),
            dcc: DccManager::new(&dcc_dir),
        };
        // MOTD — losowy ASCII art logo
        for line in crate::motd::get_motd().lines() {
            if !line.is_empty() {
                app.system_message(line);
            }
        }
        app.load_config();
        app.load_from_db();
        app
    }

    // ─── Multi-server helpers ──────────────────────────

    pub fn server(&self) -> &ServerConnection {
        &self.servers[self.active_server_idx]
    }

    pub fn server_mut(&mut self) -> &mut ServerConnection {
        &mut self.servers[self.active_server_idx]
    }

    pub fn add_server(&mut self, host: &str, port: u16, nick: &str, tls: bool) -> usize {
        let idx = self.servers.len();
        self.servers.push(ServerConnection::new(host, port, nick, tls));
        idx
    }

    pub fn switch_server(&mut self, idx: usize) {
        if idx < self.servers.len() {
            self.active_server_idx = idx;
        }
    }

    /// Znajdź indeks serwera po hoście
    pub fn find_server(&self, host: &str) -> Option<usize> {
        self.servers.iter().position(|s| s.host == host)
    }

    /// Wczytaj konfigurację z ~/.void/void.conf
    fn load_config(&mut self) {
        let config_path = std::env::var("HOME")
            .map(|h| std::path::PathBuf::from(h).join(".void").join("void.conf"))
            .unwrap_or_else(|_| std::path::PathBuf::from(".void/void.conf"));

        let content = match std::fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(_) => return, // Brak pliku — pierwszy start
        };

        let mut section = "";
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') { continue; }
            if line == "[settings]" { section = "settings"; continue; }
            if line == "[aliases]" { section = "aliases"; continue; }

            if let Some((key, value)) = line.split_once('=') {
                match section {
                    "settings" => {
                        self.settings.set(key.trim(), value.trim());
                    }
                    "aliases" => {
                        self.aliases.insert(key.trim().to_uppercase(), value.trim().to_string());
                    }
                    _ => {}
                }
            }
        }
        self.system_message(&format!("-!- Loaded config from: {}", config_path.display()));
    }

    // ─── Zarządzanie buforami ──────────────────────────────

    pub fn current_buffer(&self) -> &Buffer {
        &self.buffers[self.current_buffer_idx]
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer_idx]
    }

    pub fn get_buffer(&self, name: &str) -> Option<&Buffer> {
        self.buffers.iter().find(|b| b.name == name)
    }

    pub fn get_buffer_mut(&mut self, name: &str) -> Option<&mut Buffer> {
        self.buffers.iter_mut().find(|b| b.name == name)
    }

    pub fn get_or_create_buffer(&mut self, name: &str) -> &mut Buffer {
        if !self.buffers.iter().any(|b| b.name == name) {
            self.buffers.push(Buffer::new(name));
        }
        self.buffers.iter_mut().find(|b| b.name == name).unwrap()
    }

    pub fn switch_to_buffer(&mut self, name: &str) {
        if let Some(idx) = self.buffers.iter().position(|b| b.name == name) {
            self.last_buffer_idx = Some(self.current_buffer_idx);
            self.current_buffer_idx = idx;
            self.buffers[idx].unread_count = 0;
            self.buffers[idx].has_activity = false;
            self.sync_lua_context();
        }
    }

    pub fn next_buffer(&mut self) {
        if !self.buffers.is_empty() {
            self.current_buffer_idx = (self.current_buffer_idx + 1) % self.buffers.len();
            self.buffers[self.current_buffer_idx].unread_count = 0;
            self.buffers[self.current_buffer_idx].has_activity = false;
            self.sync_lua_context();
        }
    }

    pub fn prev_buffer(&mut self) {
        if !self.buffers.is_empty() {
            self.current_buffer_idx = if self.current_buffer_idx == 0 {
                self.buffers.len() - 1
            } else {
                self.current_buffer_idx - 1
            };
            self.buffers[self.current_buffer_idx].unread_count = 0;
            self.buffers[self.current_buffer_idx].has_activity = false;
            self.sync_lua_context();
        }
    }

    /// Synchronizuj LuaContext z aktualnym stanem App
    pub fn sync_lua_context(&self) {
        if let Some(ref ctx) = self.lua_ctx {
            if let Ok(mut c) = ctx.lock() {
                c.our_nick = self.server().our_nick.clone();
                c.current_channel = self.buffers[self.current_buffer_idx].name.clone();
                c.server_host = self.server().host.clone();
                c.connected = self.server().connected;
                c.settings = self.settings.map.clone();
            }
        }
    }

    pub fn close_buffer(&mut self, name: &str) {
        if name == "(Status)" { return; }
        self.buffers.retain(|b| b.name != name);
        if self.current_buffer_idx >= self.buffers.len() {
            self.current_buffer_idx = self.buffers.len().saturating_sub(1);
        }
    }

    // ─── Wiadomości ──────────────────────────────────────

    pub fn system_message(&mut self, text: &str) {
        self.logger.write_line("(Status)", text);
        let limit = self.settings.get_int("SCROLLBACK").max(100) as usize;
        let buf = self.get_or_create_buffer("(Status)");
        buf.push_message(text.to_string(), MessageType::System, limit);
    }

    pub fn buffer_message(&mut self, buffer: &str, text: String, msg_type: MessageType) {
        self.logger.write_line(buffer, &text);

        // /SHH — wycisz wyświetlanie w aktualnym kontekście
        if self.suppress_display {
            self.suppress_display = false;
            return;
        }

        let limit = self.settings.get_int("SCROLLBACK").max(100) as usize;
        let current_name = self.buffers[self.current_buffer_idx].name.clone();

        // Sprawdź highlight — wiadomość zawiera nasz nick
        let effective_type = if msg_type == MessageType::Normal || msg_type == MessageType::Action {
            let nick_lower = self.server().our_nick.to_lowercase();
            let text_lower = text.to_lowercase();
            if !nick_lower.is_empty() && text_lower.contains(&nick_lower) {
                MessageType::Highlight
            } else {
                msg_type
            }
        } else {
            msg_type
        };

        let buf = self.get_or_create_buffer(buffer);
        buf.push_message(text, effective_type.clone(), limit);
        if buf.name != current_name {
            buf.unread_count += 1;
            buf.has_activity = true;
            // Beep na highlight
            if effective_type == MessageType::Highlight && self.settings.get_bool("BEEP_ON_MSG") {
                eprint!("\x07");
            }
        }
    }

    // ─── Input history ──────────────────────────────────

    pub fn push_input_history(&mut self) {
        if !self.input_text.is_empty() {
            self.input_history.push(self.input_text.clone());
            if self.input_history.len() > 100 {
                self.input_history.remove(0);
            }
        }
        self.input_history_idx = None;
    }

    pub fn history_prev(&mut self) {
        if self.input_history.is_empty() { return; }
        match self.input_history_idx {
            None => {
                self.input_history_idx = Some(self.input_history.len() - 1);
            }
            Some(idx) if idx > 0 => {
                self.input_history_idx = Some(idx - 1);
            }
            _ => return,
        }
        if let Some(idx) = self.input_history_idx {
            self.input_text = self.input_history[idx].clone();
            self.input_cursor_pos = self.input_text.len();
        }
    }

    pub fn history_next(&mut self) {
        match self.input_history_idx {
            Some(idx) if idx < self.input_history.len() - 1 => {
                self.input_history_idx = Some(idx + 1);
                self.input_text = self.input_history[idx + 1].clone();
                self.input_cursor_pos = self.input_text.len();
            }
            Some(_) => {
                self.input_history_idx = None;
                self.input_text.clear();
                self.input_cursor_pos = 0;
            }
            None => {}
        }
    }

    // ─── Ignore ──────────────────────────────────────

    pub fn is_ignored(&self, nick: &str, msg_type: &str) -> bool {
        for entry in &self.ignore_list {
            let pattern_lower = entry.pattern.to_lowercase();
            let nick_lower = nick.to_lowercase();
            let matches = if pattern_lower.contains('*') {
                let parts: Vec<&str> = pattern_lower.split('*').collect();
                if parts.len() == 2 {
                    nick_lower.starts_with(parts[0]) && nick_lower.ends_with(parts[1])
                } else {
                    nick_lower.contains(&pattern_lower.replace('*', ""))
                }
            } else {
                nick_lower == pattern_lower
            };

            if matches {
                if entry.ignore_all { return true; }
                match msg_type {
                    "PUBLIC" if entry.ignore_public => return true,
                    "MSG" if entry.ignore_private => return true,
                    "NOTICE" if entry.ignore_notice => return true,
                    "CTCP" if entry.ignore_ctcp => return true,
                    _ => {}
                }
            }
        }
        false
    }

    // ─── Notify ──────────────────────────────────────

    pub fn is_on_notify(&self, nick: &str) -> bool {
        self.notify_list.iter().any(|n| n.nick.to_lowercase() == nick.to_lowercase())
    }

    // ─── Aliasy i ekspansja zmiennych ──────────────────

    /// Rozwiń zmienne epic5-style w tekście aliasu
    /// $0-$9 = argumenty, $* = wszystkie, $N = nick, $C = kanał, $S = serwer
    pub fn expand_variables(&self, template: &str, args: &[&str]) -> String {
        let mut result = String::with_capacity(template.len());
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
                    Some('N') => result.push_str(&self.server().our_nick),
                    Some('C') => {
                        let buf = &self.buffers[self.current_buffer_idx];
                        if buf.name != "(Status)" { result.push_str(&buf.name); }
                    }
                    Some('S') => result.push_str(&self.server().host),
                    Some('$') => result.push('$'),
                    Some(other) => {
                        result.push('$');
                        result.push(other);
                    }
                    None => result.push('$'),
                }
            } else {
                result.push(c);
            }
        }
        result
    }

    /// Cached nick-pattern matching (irssi nickmatch-cache style)
    pub fn nick_matches_pattern(&mut self, nick: &str, pattern: &str) -> bool {
        let cache_key = format!("{}:{}", nick, pattern);
        if let Some(&result) = self.server_mut().nickmatch_cache.get(&cache_key) {
            return result;
        }
        let result = self.match_pattern(nick, pattern);
        self.server_mut().nickmatch_cache.insert(cache_key, result);
        // Limit cache size
        if self.server_mut().nickmatch_cache.len() > 1000 {
            self.server_mut().nickmatch_cache.clear();
        }
        result
    }

    /// Dodaj redirect tracking dla komendy
    pub fn track_redirect(&mut self, command: &str, target: &str, callback: &str) {
        let request_id = format!("{}_{}", command, self.server_mut().pending_redirects.len());
        self.server_mut().pending_redirects.push(ServerRedirect {
            command: command.to_string(),
            target: target.to_string(),
            request_id,
            callback: callback.to_string(),
        });
    }

    /// Znajdź i usuń pasujący redirect
    pub fn find_redirect(&mut self, command: &str, target: &str) -> Option<ServerRedirect> {
        if let Some(pos) = self.server_mut().pending_redirects.iter().position(|r| {
            r.command == command && r.target == target
        }) {
            Some(self.server_mut().pending_redirects.remove(pos))
        } else {
            None
        }
    }

    fn match_pattern(&self, text: &str, pattern: &str) -> bool {
        let text_lower = text.to_lowercase();
        let pattern_lower = pattern.to_lowercase();
        if pattern_lower.contains('*') {
            let parts: Vec<&str> = pattern_lower.split('*').collect();
            if parts.len() == 2 {
                text_lower.starts_with(parts[0]) && text_lower.ends_with(parts[1])
            } else {
                text_lower.contains(&pattern_lower.replace('*', ""))
            }
        } else {
            text_lower.contains(&pattern_lower)
        }
    }

    /// Sprawdź czy komenda to alias i rozwiń. Zwraca Some(rozwinięty_tekst) lub None.
    pub fn resolve_alias(&self, cmd_name: &str, args: &[&str]) -> Option<String> {
        let upper = cmd_name.to_uppercase();
        self.aliases.get(&upper).map(|template| {
            self.expand_variables(template, args)
        })
    }

    /// Generuj unikalny label dla labeled-response
    pub fn next_label(&mut self) -> String {
        self.label_counter += 1;
        format!("v{}", self.label_counter)
    }

    /// Zastosuj theme z Lua
    pub fn apply_theme(&mut self, theme_name: &str) {
        let parse_color_str = |s: &str, default_color: Color| -> Color {
            let clean = s.trim().to_lowercase();
            if clean.is_empty() {
                return default_color;
            }

            if clean == "default" || clean == "none" || clean == "reset" || clean == "transparent" {
                return Color::Reset;
            }

            // Hex format #rgb, #rrggbb, 0xrrggbb
            if let Some(hex_str) = clean.strip_prefix('#').or_else(|| clean.strip_prefix("0x")) {
                if hex_str.len() == 3 {
                    let r = u8::from_str_radix(&hex_str[0..1].repeat(2), 16);
                    let g = u8::from_str_radix(&hex_str[1..2].repeat(2), 16);
                    let b = u8::from_str_radix(&hex_str[2..3].repeat(2), 16);
                    if let (Ok(r), Ok(g), Ok(b)) = (r, g, b) {
                        return Color::Rgb(r, g, b);
                    }
                } else if hex_str.len() == 6 {
                    let r = u8::from_str_radix(&hex_str[0..2], 16);
                    let g = u8::from_str_radix(&hex_str[2..4], 16);
                    let b = u8::from_str_radix(&hex_str[4..6], 16);
                    if let (Ok(r), Ok(g), Ok(b)) = (r, g, b) {
                        return Color::Rgb(r, g, b);
                    }
                }
            }

            // rgb(r, g, b)
            if clean.starts_with("rgb(") && clean.ends_with(')') {
                let inner = &clean[4..clean.len() - 1];
                let parts: Vec<&str> = inner.split(',').map(|p| p.trim()).collect();
                if parts.len() == 3 {
                    if let (Ok(r), Ok(g), Ok(b)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>(), parts[2].parse::<u8>()) {
                        return Color::Rgb(r, g, b);
                    }
                }
            }

            // Indexed / ANSI 256
            let idx_str = clean.strip_prefix("idx:").or_else(|| clean.strip_prefix("ansi:")).unwrap_or(&clean);
            if let Ok(idx) = idx_str.parse::<u8>() {
                return Color::Indexed(idx);
            }

            // Named colors (normalized)
            let normalized = clean.replace(['_', '-'], "");
            match normalized.as_str() {
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "yellow" => Color::Yellow,
                "blue" => Color::Blue,
                "magenta" | "purple" => Color::Magenta,
                "cyan" | "teal" => Color::Cyan,
                "white" => Color::White,
                "gray" | "grey" | "darkgray" => Color::DarkGray,
                "lightgray" | "lightgrey" | "silver" => Color::Gray,
                "lightred" | "brightred" => Color::LightRed,
                "lightgreen" | "brightgreen" | "lime" => Color::LightGreen,
                "lightyellow" | "brightyellow" => Color::LightYellow,
                "lightblue" | "brightblue" => Color::LightBlue,
                "lightmagenta" | "brightmagenta" | "pink" => Color::LightMagenta,
                "lightcyan" | "brightcyan" => Color::LightCyan,

                // Extended named dark and accent colors
                "darkred" => Color::Rgb(139, 0, 0),
                "darkgreen" => Color::Rgb(0, 100, 0),
                "darkblue" | "navy" => Color::Rgb(0, 0, 139),
                "darkmagenta" => Color::Rgb(139, 0, 139),
                "darkcyan" => Color::Rgb(0, 139, 139),
                "darkyellow" | "olive" => Color::Rgb(139, 139, 0),
                "orange" => Color::Rgb(255, 140, 0),
                "peach" => Color::Rgb(250, 179, 135),
                "violet" | "lavender" => Color::Rgb(180, 190, 254),
                "gold" => Color::Rgb(255, 215, 0),
                "brown" => Color::Rgb(165, 42, 42),
                _ => default_color,
            }
        };

        let parse_lua_val = |val: &mlua::Value, default_color: Color| -> Color {
            match val {
                mlua::Value::String(s) => {
                    if let Ok(s_str) = s.to_str() {
                        parse_color_str(&s_str, default_color)
                    } else {
                        default_color
                    }
                }
                mlua::Value::Integer(n) if *n >= 0 && *n <= 255 => Color::Indexed(*n as u8),
                mlua::Value::Table(tbl) => {
                    let r = tbl.get::<u8>("r").or_else(|_| tbl.get::<u8>(1)).ok();
                    let g = tbl.get::<u8>("g").or_else(|_| tbl.get::<u8>(2)).ok();
                    let b = tbl.get::<u8>("b").or_else(|_| tbl.get::<u8>(3)).ok();
                    if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                        Color::Rgb(r, g, b)
                    } else {
                        default_color
                    }
                }
                _ => default_color,
            }
        };

        // Czytaj theme z Lua global table
        if let Some(ref lua) = self.lua {
            if let Ok(themes_table) = lua.globals().get::<mlua::Table>("void_themes") {
                if let Ok(theme_table) = themes_table.get::<mlua::Table>(theme_name.to_lowercase()) {
                    if let Ok(desc) = theme_table.get::<String>("desc") {
                        self.theme_colors.desc = desc;
                    }
                    if let Ok(is_dark) = theme_table.get::<bool>("is_dark") {
                        self.theme_colors.is_dark = is_dark;
                    }

                    // Czytaj ui colors
                    if let Ok(ui) = theme_table.get::<mlua::Table>("ui") {
                        if let Ok(v) = ui.get::<mlua::Value>("status_bar_bg") { self.theme_colors.status_bar_bg = parse_lua_val(&v, self.theme_colors.status_bar_bg); }
                        if let Ok(v) = ui.get::<mlua::Value>("status_bar_fg") { self.theme_colors.status_bar_fg = parse_lua_val(&v, self.theme_colors.status_bar_fg); }
                        if let Ok(v) = ui.get::<mlua::Value>("status_bar_active_bg") { self.theme_colors.status_bar_active_bg = parse_lua_val(&v, self.theme_colors.status_bar_active_bg); }
                        if let Ok(v) = ui.get::<mlua::Value>("status_bar_active_fg") { self.theme_colors.status_bar_active_fg = parse_lua_val(&v, self.theme_colors.status_bar_active_fg); }
                        if let Ok(v) = ui.get::<mlua::Value>("status_bar_activity_bg") { self.theme_colors.status_bar_activity_bg = parse_lua_val(&v, self.theme_colors.status_bar_activity_bg); }
                        if let Ok(v) = ui.get::<mlua::Value>("status_bar_activity_fg") { self.theme_colors.status_bar_activity_fg = parse_lua_val(&v, self.theme_colors.status_bar_activity_fg); }
                        if let Ok(v) = ui.get::<mlua::Value>("status_bar_info_fg") { self.theme_colors.status_bar_info_fg = parse_lua_val(&v, self.theme_colors.status_bar_info_fg); }
                        if let Ok(v) = ui.get::<mlua::Value>("topic_bar_bg") { self.theme_colors.topic_bar_bg = parse_lua_val(&v, self.theme_colors.topic_bar_bg); }
                        if let Ok(v) = ui.get::<mlua::Value>("topic_bar_fg") { self.theme_colors.topic_bar_fg = parse_lua_val(&v, self.theme_colors.topic_bar_fg); }
                        if let Ok(v) = ui.get::<mlua::Value>("input_bg") { self.theme_colors.input_bg = parse_lua_val(&v, self.theme_colors.input_bg); }
                        if let Ok(v) = ui.get::<mlua::Value>("input_fg") { self.theme_colors.input_fg = parse_lua_val(&v, self.theme_colors.input_fg); }
                        if let Ok(v) = ui.get::<mlua::Value>("input_prompt_fg") { self.theme_colors.input_prompt_fg = parse_lua_val(&v, self.theme_colors.input_prompt_fg); }
                        if let Ok(v) = ui.get::<mlua::Value>("border") { self.theme_colors.border = parse_lua_val(&v, self.theme_colors.border); }
                        if let Ok(v) = ui.get::<mlua::Value>("timestamp") { self.theme_colors.timestamp = parse_lua_val(&v, self.theme_colors.timestamp); }
                        if let Ok(v) = ui.get::<mlua::Value>("scroll_indicator_fg") { self.theme_colors.scroll_indicator_fg = parse_lua_val(&v, self.theme_colors.scroll_indicator_fg); }
                        if let Ok(v) = ui.get::<mlua::Value>("scroll_indicator_bg") { self.theme_colors.scroll_indicator_bg = parse_lua_val(&v, self.theme_colors.scroll_indicator_bg); }
                        if let Ok(v) = ui.get::<mlua::Value>("chat_bg") { self.theme_colors.chat_bg = parse_lua_val(&v, self.theme_colors.chat_bg); }
                        if let Ok(v) = ui.get::<mlua::Value>("nick_list_bg") { self.theme_colors.nick_list_bg = parse_lua_val(&v, self.theme_colors.nick_list_bg); }
                    }

                    // Czytaj message colors
                    if let Ok(msgs) = theme_table.get::<mlua::Table>("messages") {
                        if let Ok(v) = msgs.get::<mlua::Value>("normal") { self.theme_colors.msg_normal = parse_lua_val(&v, self.theme_colors.msg_normal); }
                        if let Ok(v) = msgs.get::<mlua::Value>("action") { self.theme_colors.msg_action = parse_lua_val(&v, self.theme_colors.msg_action); }
                        if let Ok(v) = msgs.get::<mlua::Value>("system") { self.theme_colors.msg_system = parse_lua_val(&v, self.theme_colors.msg_system); }
                        if let Ok(v) = msgs.get::<mlua::Value>("notice") { self.theme_colors.msg_notice = parse_lua_val(&v, self.theme_colors.msg_notice); }
                        if let Ok(v) = msgs.get::<mlua::Value>("highlight") { self.theme_colors.msg_highlight = parse_lua_val(&v, self.theme_colors.msg_highlight); }
                        if let Ok(v) = msgs.get::<mlua::Value>("error") { self.theme_colors.msg_error = parse_lua_val(&v, self.theme_colors.msg_error); }
                        if let Ok(v) = msgs.get::<mlua::Value>("server") { self.theme_colors.msg_server = parse_lua_val(&v, self.theme_colors.msg_server); }
                        if let Ok(v) = msgs.get::<mlua::Value>("ctcp") { self.theme_colors.msg_ctcp = parse_lua_val(&v, self.theme_colors.msg_ctcp); }
                        if let Ok(v) = msgs.get::<mlua::Value>("url") { self.theme_colors.msg_url = parse_lua_val(&v, self.theme_colors.msg_url); }
                    }

                    // Czytaj nick colors
                    if let Ok(nicks) = theme_table.get::<mlua::Table>("nicks") {
                        if let Ok(v) = nicks.get::<mlua::Value>("op") { self.theme_colors.nick_op = parse_lua_val(&v, self.theme_colors.nick_op); }
                        if let Ok(v) = nicks.get::<mlua::Value>("op_nick") { self.theme_colors.nick_op_nick = parse_lua_val(&v, self.theme_colors.nick_op_nick); }
                        if let Ok(v) = nicks.get::<mlua::Value>("voice") { self.theme_colors.nick_voice = parse_lua_val(&v, self.theme_colors.nick_voice); }
                        if let Ok(v) = nicks.get::<mlua::Value>("voice_nick") { self.theme_colors.nick_voice_nick = parse_lua_val(&v, self.theme_colors.nick_voice_nick); }
                        if let Ok(v) = nicks.get::<mlua::Value>("halfop") { self.theme_colors.nick_halfop = parse_lua_val(&v, self.theme_colors.nick_halfop); }
                        if let Ok(v) = nicks.get::<mlua::Value>("halfop_nick") { self.theme_colors.nick_halfop_nick = parse_lua_val(&v, self.theme_colors.nick_halfop_nick); }
                        if let Ok(v) = nicks.get::<mlua::Value>("founder") { self.theme_colors.nick_founder = parse_lua_val(&v, self.theme_colors.nick_founder); }
                        if let Ok(v) = nicks.get::<mlua::Value>("founder_nick") { self.theme_colors.nick_founder_nick = parse_lua_val(&v, self.theme_colors.nick_founder_nick); }
                        if let Ok(v) = nicks.get::<mlua::Value>("admin") { self.theme_colors.nick_admin = parse_lua_val(&v, self.theme_colors.nick_admin); }
                        if let Ok(v) = nicks.get::<mlua::Value>("admin_nick") { self.theme_colors.nick_admin_nick = parse_lua_val(&v, self.theme_colors.nick_admin_nick); }
                        if let Ok(v) = nicks.get::<mlua::Value>("normal") { self.theme_colors.nick_normal = parse_lua_val(&v, self.theme_colors.nick_normal); }
                        if let Ok(v) = nicks.get::<mlua::Value>("normal_prefix") { self.theme_colors.nick_normal_prefix = parse_lua_val(&v, self.theme_colors.nick_normal_prefix); }
                        if let Ok(v) = nicks.get::<mlua::Value>("header") { self.theme_colors.nick_list_header = parse_lua_val(&v, self.theme_colors.nick_list_header); }
                    }

                    // Czytaj nick_colors (dynamic nick palette)
                    if let Ok(palette) = theme_table.get::<mlua::Table>("nick_colors") {
                        let mut colors = Vec::new();
                        for val in palette.sequence_values::<mlua::Value>() {
                            if let Ok(v) = val {
                                colors.push(parse_lua_val(&v, Color::White));
                            }
                        }
                        if !colors.is_empty() {
                            self.theme_colors.nick_colors = colors;
                        }
                    }

                    if let Ok(name) = theme_table.get::<String>("name") {
                        self.theme_colors.name = name;
                    } else {
                        self.theme_colors.name = theme_name.to_string();
                    }
                }
            }
        }
    }

    /// Rozwiń szablon formatu wiadomości (np. "JOIN" → "* $0 has joined $1")
    pub fn format_message(&self, fmt_type: &str, args: &[&str]) -> Option<String> {
        self.format_templates.get(&fmt_type.to_uppercase()).map(|template| {
            self.expand_variables(template, args)
        })
    }

    // ─── SQLite persistence ──────────────────────────

    /// Załaduj dane z SQLite do App
    pub fn load_from_db(&mut self) {
        let storage = match &self.storage {
            Some(s) => s,
            None => return,
        };

        // Załaduj ustawienia
        for (key, value) in storage.get_all_settings() {
            self.settings.set(&key, &value);
        }

        // Załaduj aliasy
        for (name, body) in storage.get_all_aliases() {
            self.aliases.insert(name, body);
        }

        // Załaduj highlight patterns
        for (pattern, color) in storage.get_all_highlights() {
            self.highlight_patterns.push(HighlightPattern { pattern, color });
        }

        // Załaduj key bindings
        for (key, action) in storage.get_all_key_bindings() {
            self.key_bindings.insert(key, action);
        }

        // Załaduj notify list
        for nick in storage.get_all_notify() {
            self.notify_list.push(crate::app::NotifyEntry {
                nick,
                online: false,
                last_seen: None,
                userhost: String::new(),
                channels: Vec::new(),
                verified: false,
                action: "echo".into(),
            });
        }

        // Załaduj ignore list
        for (pattern, flags) in storage.get_all_ignore() {
            self.ignore_list.push(crate::app::IgnoreEntry {
                pattern: pattern.clone(),
                ignore_all: flags.contains("ALL"),
                ignore_public: flags.contains("PUBLIC") || flags.contains("ALL"),
                ignore_private: flags.contains("MSG") || flags.contains("ALL"),
                ignore_notice: flags.contains("NOTICE") || flags.contains("ALL"),
                ignore_ctcp: flags.contains("CTCP") || flags.contains("ALL"),
            });
        }
    }

    /// Zapisz aktualny stan do SQLite
    pub fn save_to_db(&self) {
        let storage = match &self.storage {
            Some(s) => s,
            None => return,
        };

        // Zapisz ustawienia
        for (key, value) in &self.settings.map {
            let _ = storage.set_setting(key, value);
        }

        // Zapisz aliasy
        for (name, body) in &self.aliases {
            let _ = storage.set_alias(name, body);
        }

        // Zapisz highlight patterns
        for h in &self.highlight_patterns {
            let _ = storage.add_highlight(&h.pattern, &h.color);
        }

        // Zapisz key bindings
        for (key, action) in &self.key_bindings {
            let _ = storage.set_key_binding(key, action);
        }

        // Zapisz notify list
        for n in &self.notify_list {
            let _ = storage.add_notify(&n.nick);
        }

        // Zapisz ignore list
        for entry in &self.ignore_list {
            let mut flags = Vec::new();
            if entry.ignore_all { flags.push("ALL"); }
            if entry.ignore_public { flags.push("PUBLIC"); }
            if entry.ignore_private { flags.push("MSG"); }
            if entry.ignore_notice { flags.push("NOTICE"); }
            if entry.ignore_ctcp { flags.push("CTCP"); }
            let _ = storage.add_ignore(&entry.pattern, &flags.join(","));
        }

        // Zapisz sesję (bufory + serwery)
        let _ = storage.init_session_table();
        let _ = storage.clear_session_buffers();
        for buf in &self.buffers {
            let is_channel = buf.name.starts_with('#') || buf.name.starts_with('&');
            let _ = storage.save_session_buffer(
                &buf.name,
                &self.server().host,
                if is_channel { &buf.name } else { "" },
                is_channel,
            );
        }

        // Zapisz layout okien
        let _ = storage.init_layout_table();
        let layout: Vec<(String, Option<usize>, bool)> = self.buffers.iter().enumerate().map(|(i, buf)| {
            let split_idx = if self.split_buffer_idx == Some(i) { Some(i) } else { None };
            (buf.name.clone(), split_idx, self.split_horizontal)
        }).collect();
        let _ = storage.save_window_layout(&layout);
    }

    /// Przywróć sesję z SQLite — dołącz do zapisanych kanałów
    pub fn restore_session(&self) -> Vec<String> {
        let storage = match &self.storage {
            Some(s) => s,
            None => return Vec::new(),
        };
        let _ = storage.init_session_table();
        storage.get_session_buffers()
            .iter()
            .filter(|(_, _, ch, auto)| *auto && !ch.is_empty())
            .map(|(_, _, ch, _)| ch.clone())
            .collect()
    }
}
