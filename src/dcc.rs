use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::PathBuf;

/// Typ transferu DCC
#[derive(Clone, Debug, PartialEq)]
pub enum DccType {
    Chat,
    Send,
    Get,
}

/// Stan transferu
#[derive(Clone, Debug, PartialEq)]
pub enum DccState {
    Pending,     // Oczekuje na akceptację
    Connecting,  // Łączy się
    Active,      // Aktywny transfer
    Completed,   // Zakończony
    Failed(String),
}

/// Pojedyncza sesja DCC
#[derive(Clone, Debug)]
pub struct DccSession {
    pub id: usize,
    pub dcc_type: DccType,
    pub nick: String,
    pub state: DccState,
    pub filename: Option<String>,
    pub filesize: Option<u64>,
    pub bytes_transferred: u64,
    pub addr: Option<SocketAddr>,
    pub path: Option<PathBuf>,
}

/// Sesja DCC Chat
#[derive(Clone, Debug)]
pub struct DccChatSession {
    pub id: usize,
    pub nick: String,
    pub state: DccState,
    pub addr: Option<SocketAddr>,
    pub messages: Vec<(String, String)>, // (from, text)
}

/// Menedżer DCC
pub struct DccManager {
    pub sessions: Vec<DccSession>,
    pub chat_sessions: Vec<DccChatSession>,
    next_id: usize,
    pub download_dir: PathBuf,
}

impl DccManager {
    pub fn new(download_dir: &str) -> Self {
        let path = PathBuf::from(shellexpand::tilde(download_dir).to_string());
        DccManager {
            sessions: Vec::new(),
            chat_sessions: Vec::new(),
            next_id: 1,
            download_dir: path,
        }
    }

    /// Dodaj nową sesję (odebraną ofertę)
    pub fn add_pending(&mut self, dcc_type: DccType, nick: &str, filename: Option<&str>, filesize: Option<u64>, addr: Option<SocketAddr>) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.sessions.push(DccSession {
            id,
            dcc_type,
            nick: nick.to_string(),
            state: DccState::Pending,
            filename: filename.map(|s| s.to_string()),
            filesize,
            bytes_transferred: 0,
            addr,
            path: filename.map(|f| self.download_dir.join(f)),
        });
        id
    }

    /// Pobierz sesję po ID
    pub fn get(&self, id: usize) -> Option<&DccSession> {
        self.sessions.iter().find(|s| s.id == id)
    }

    /// Pobierz sesję po ID (mutable)
    pub fn get_mut(&mut self, id: usize) -> Option<&mut DccSession> {
        self.sessions.iter_mut().find(|s| s.id == id)
    }

    /// Lista oczekujących
    pub fn pending(&self) -> Vec<&DccSession> {
        self.sessions.iter().filter(|s| s.state == DccState::Pending).collect()
    }

    /// Lista aktywnych
    pub fn active(&self) -> Vec<&DccSession> {
        self.sessions.iter().filter(|s| s.state == DccState::Active).collect()
    }

    /// Parsuj wiadomość CTCP DCC
    pub fn parse_dcc_request(content: &str) -> Option<(DccType, String, Option<u64>, Option<SocketAddr>)> {
        // Format: DCC <type> <filename> <ip_int> <port> [filesize]
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() < 4 {
            return None;
        }
        let dcc_type = match parts[0].to_uppercase().as_str() {
            "SEND" => DccType::Send,
            "CHAT" => DccType::Chat,
            _ => return None,
        };
        let filename = parts[1].to_string();
        let ip_int = parts[2].parse::<u32>().ok()?;
        let port = parts[3].parse::<u16>().ok()?;
        let filesize = parts.get(4).and_then(|s| s.parse::<u64>().ok());

        // Konwersja IP z formatu integer
        let ip = std::net::Ipv4Addr::from(ip_int);
        let addr = SocketAddr::from((ip, port));

        Some((dcc_type, filename, filesize, Some(addr)))
    }

    /// Akceptuj DCC SEND — połącz się z nadawcą i pobierz plik
    pub fn accept_send(&mut self, id: usize) -> Result<String, String> {
        let session = self.sessions.iter_mut().find(|s| s.id == id)
            .ok_or_else(|| format!("No DCC session with id {}", id))?;

        if session.state != DccState::Pending {
            return Err("Session is not in pending state".into());
        }

        let addr = session.addr.ok_or("No address for DCC session")?;
        let filename = session.filename.as_ref().ok_or("No filename")?.clone();
        let _filesize = session.filesize;
        let filepath = self.download_dir.join(&filename);

        // Utwórz katalog pobierania jeśli nie istnieje
        if let Some(parent) = filepath.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("Cannot create dir: {}", e))?;
        }

        session.state = DccState::Connecting;
        let nick = session.nick.clone();

        // Połącz z nadawcą
        let stream = TcpStream::connect(addr)
            .map_err(|e| format!("DCC connect error: {}", e))?;
        stream.set_read_timeout(Some(std::time::Duration::from_secs(300)))
            .map_err(|e| format!("Set timeout error: {}", e))?;

        // Aktualizuj stan
        let session = self.sessions.iter_mut().find(|s| s.id == id).unwrap();
        session.state = DccState::Active;
        session.path = Some(filepath.clone());

        // Odbierz plik
        let mut file = std::fs::File::create(&filepath)
            .map_err(|e| format!("Cannot create file: {}", e))?;
        let mut total_bytes: u64 = 0;
        let mut buf = [0u8; 8192];
        let mut stream = stream;

        loop {
            match stream.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    file.write_all(&buf[..n]).map_err(|e| format!("Write error: {}", e))?;
                    total_bytes += n as u64;

                    // Wyślij ACK (4 bajty, big-endian)
                    let ack = (total_bytes as u32).to_be_bytes();
                    let _ = stream.write_all(&ack);

                    // Aktualizuj progress
                    if let Some(session) = self.sessions.iter_mut().find(|s| s.id == id) {
                        session.bytes_transferred = total_bytes;
                    }
                }
                Err(e) => {
                    // Aktualizuj stan na failed
                    if let Some(session) = self.sessions.iter_mut().find(|s| s.id == id) {
                        session.state = DccState::Failed(format!("Read error: {}", e));
                    }
                    return Err(format!("DCC read error: {}", e));
                }
            }
        }

        // Transfer zakończony
        if let Some(session) = self.sessions.iter_mut().find(|s| s.id == id) {
            session.state = DccState::Completed;
            session.bytes_transferred = total_bytes;
        }

        Ok(format!("DCC SEND from {} complete: {} ({} bytes)", nick, filename, total_bytes))
    }

    /// Formatuj listę sesji do wyświetlenia
    pub fn format_list(&self) -> Vec<String> {
        self.sessions.iter().map(|s| {
            let type_str = match s.dcc_type {
                DccType::Chat => "CHAT",
                DccType::Send => "SEND",
                DccType::Get => "GET",
            };
            let state_str = match &s.state {
                DccState::Pending => "pending".to_string(),
                DccState::Connecting => "connecting".to_string(),
                DccState::Active => {
                    if let Some(size) = s.filesize {
                        let pct = if size > 0 { (s.bytes_transferred * 100) / size } else { 0 };
                        format!("active ({}%)", pct)
                    } else {
                        "active".to_string()
                    }
                }
                DccState::Completed => "completed".to_string(),
                DccState::Failed(e) => format!("failed: {}", e),
            };
            let file_str = s.filename.as_deref().unwrap_or("-");
            format!("  [{}] {} {} {} ({})", s.id, type_str, s.nick, file_str, state_str)
        }).collect()
    }

    // ─── DCC Chat ────────────────────────────────────

    /// Dodaj nową sesję DCC Chat
    pub fn add_chat(&mut self, nick: &str, addr: SocketAddr) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.chat_sessions.push(DccChatSession {
            id,
            nick: nick.to_string(),
            state: DccState::Pending,
            addr: Some(addr),
            messages: Vec::new(),
        });
        id
    }

    /// Połącz z DCC Chat
    pub fn connect_chat(&mut self, id: usize) -> Result<(), String> {
        let session = self.chat_sessions.iter_mut().find(|s| s.id == id)
            .ok_or("No such DCC chat session")?;
        let addr = session.addr.ok_or("No address for DCC chat")?;
        session.state = DccState::Active;
        // TCP connection would be established here
        // For now, just mark as active
        Ok(())
    }

    /// Wyślij wiadomość DCC Chat
    pub fn send_chat_message(&mut self, id: usize, text: &str) -> Result<(), String> {
        let session = self.chat_sessions.iter_mut().find(|s| s.id == id)
            .ok_or("No such DCC chat session")?;
        if session.state != DccState::Active {
            return Err("DCC chat not active".into());
        }
        session.messages.push(("me".to_string(), text.to_string()));
        // Would send through TCP stream here
        Ok(())
    }

    /// Odbierz wiadomość DCC Chat
    pub fn receive_chat_message(&mut self, id: usize, text: &str) {
        if let Some(session) = self.chat_sessions.iter_mut().find(|s| s.id == id) {
            session.messages.push((session.nick.clone(), text.to_string()));
        }
    }

    /// Zamknij sesję DCC Chat
    pub fn close_chat(&mut self, id: usize) {
        if let Some(session) = self.chat_sessions.iter_mut().find(|s| s.id == id) {
            session.state = DccState::Completed;
        }
    }

    // ─── DCC Resume ──────────────────────────────────

    /// Wznów przerwany transfer DCC SEND
    pub fn resume_send(&mut self, id: usize) -> Result<String, String> {
        let session = self.sessions.iter().find(|s| s.id == id)
            .ok_or("No such DCC session")?;

        if session.dcc_type != DccType::Send {
            return Err("Can only resume DCC SEND".into());
        }

        let filepath = session.path.as_ref().ok_or("No file path")?;
        let addr = session.addr.ok_or("No address")?;
        let filename = session.filename.as_deref().unwrap_or("file");

        // Sprawdź ile już pobrano
        let existing_size = std::fs::metadata(filepath)
            .map(|m| m.len())
            .unwrap_or(0);

        if existing_size == 0 {
            return Err("No partial file to resume".into());
        }

        // Wyślij DCC RESUME
        let port = addr.port();
        Ok(format!("DCC RESUME {} {} {}", filename, port, existing_size))
    }

    /// Lista sesji DCC Chat
    pub fn list_chats(&self) -> Vec<String> {
        self.chat_sessions.iter().map(|s| {
            let state = match &s.state {
                DccState::Pending => "pending",
                DccState::Active => "active",
                DccState::Completed => "closed",
                _ => "unknown",
            };
            format!("  [{}] DCC CHAT {} ({}) {} msgs", s.id, s.nick, state, s.messages.len())
        }).collect()
    }
}
