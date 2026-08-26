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

/// Menedżer DCC
pub struct DccManager {
    pub sessions: Vec<DccSession>,
    next_id: usize,
    pub download_dir: PathBuf,
}

impl DccManager {
    pub fn new(download_dir: &str) -> Self {
        let path = PathBuf::from(shellexpand::tilde(download_dir).to_string());
        DccManager {
            sessions: Vec::new(),
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
}
