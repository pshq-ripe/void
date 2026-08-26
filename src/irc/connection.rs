use futures::StreamExt;
use irc::client::prelude::*;
use irc::proto::CapSubCommand;
use ring::{digest, hmac, pbkdf2};
use tokio::sync::mpsc;

/// Prosty base64 encode
fn base64_encode(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 { result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char); } else { result.push('='); }
        if chunk.len() > 2 { result.push(CHARS[(triple & 0x3F) as usize] as char); } else { result.push('='); }
    }
    result
}

/// Base64 decode
fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let input = input.trim_end_matches('=');
    let chars: Vec<u8> = input.bytes().map(|b| {
        match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => 0,
        }
    }).collect();

    let mut result = Vec::new();
    for chunk in chars.chunks(4) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).map(|&b| b as u32).unwrap_or(0);
        let b2 = chunk.get(2).map(|&b| b as u32).unwrap_or(0);
        let triple = (b0 << 18) | (b1 << 12) | (b2 << 6);
        result.push((triple >> 16) as u8);
        if chunk.len() > 2 { result.push(((triple >> 8) & 0xFF) as u8); }
        if chunk.len() > 3 { result.push((triple & 0xFF) as u8); }
    }
    Ok(result)
}

/// SCRAM-SHA-512 state machine
struct ScramState {
    client_first_bare: String,
    nonce: String,
}

impl ScramState {
    fn new(nick: &str) -> Self {
        // Generuj losowy nonce
        let mut nonce_bytes = [0u8; 24];
        for i in 0..24 {
            nonce_bytes[i] = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .subsec_nanos() as u8)
                .wrapping_add(i as u8);
        }
        let nonce = base64_encode(&nonce_bytes);
        let client_first_bare = format!("n={},r={}", nick, nonce);
        ScramState { client_first_bare, nonce }
    }

    /// Krok 1: client-first-message
    fn client_first(&self) -> String {
        format!("n,,{}", self.client_first_bare)
    }

    /// Krok 2: przetwórz server-first, zwróć client-final
    fn process_server_first(&self, server_first_b64: &str, password: &str) -> Result<String, String> {
        let server_first = String::from_utf8(base64_decode(server_first_b64)?)
            .map_err(|e| format!("UTF-8 error: {}", e))?;

        // Parsuj: r=<nonce>,s=<salt>,i=<iterations>
        let mut server_nonce = String::new();
        let mut salt = Vec::new();
        let mut iterations: u32 = 4096;

        for part in server_first.split(',') {
            if let Some(val) = part.strip_prefix("r=") {
                server_nonce = val.to_string();
            } else if let Some(val) = part.strip_prefix("s=") {
                salt = base64_decode(val)?;
            } else if let Some(val) = part.strip_prefix("i=") {
                iterations = val.parse().unwrap_or(4096);
            }
        }

        // Sprawdź czy server nonce zaczyna się od client nonce
        if !server_nonce.starts_with(&self.nonce) {
            return Err("Server nonce doesn't match client nonce".into());
        }

        // SaltedPassword = PBKDF2(password, salt, iterations)
        let mut salted_password = [0u8; 64]; // SHA-512 = 64 bytes
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA512,
            std::num::NonZeroU32::new(iterations).unwrap(),
            &salt,
            password.as_bytes(),
            &mut salted_password,
        );

        // ClientKey = HMAC(SaltedPassword, "Client Key")
        let client_key = hmac::Key::new(hmac::HMAC_SHA512, &salted_password);
        let client_key_sig = hmac::sign(&client_key, b"Client Key");
        let client_key_bytes = client_key_sig.as_ref();

        // StoredKey = H(ClientKey)
        let stored_key = digest::digest(&digest::SHA512, client_key_bytes);

        // AuthMessage = client-first-bare + "," + server-first + "," + client-final-without-proof
        let channel_binding = base64_encode(b"n,,");
        let client_final_without_proof = format!("c={},r={}", channel_binding, server_nonce);
        let auth_message = format!("{},{},{}", self.client_first_bare, server_first, client_final_without_proof);

        // ClientSignature = HMAC(StoredKey, AuthMessage)
        let stored_key_hmac = hmac::Key::new(hmac::HMAC_SHA512, stored_key.as_ref());
        let client_signature = hmac::sign(&stored_key_hmac, auth_message.as_bytes());

        // ClientProof = ClientKey XOR ClientSignature
        let mut client_proof = Vec::with_capacity(client_key_bytes.len());
        for (a, b) in client_key_bytes.iter().zip(client_signature.as_ref().iter()) {
            client_proof.push(a ^ b);
        }

        // ServerKey = HMAC(SaltedPassword, "Server Key")
        let server_key = hmac::Key::new(hmac::HMAC_SHA512, &salted_password);
        let _server_key_sig = hmac::sign(&server_key, b"Server Key");
        // (server_key_bytes nie jest potrzebny w client-final, ale potrzebny do weryfikacji)

        let proof_b64 = base64_encode(&client_proof);
        Ok(format!("{},p={}", client_final_without_proof, proof_b64))
    }
}

/// Zdarzenia z warstwy IRC do głównej pętli
pub enum IrcEvent {
    Message(Message),
    Connected(Sender),
    Disconnected,
    /// Komunikat statusu połączenia (verbose)
    Status(String),
    /// Wynik wykonania komendy /exec
    ExecOutput(Vec<String>),
    /// IRCv3 CAP event (subcommand, data)
    CapEvent(String, String),
    Error(String),
}

/// Uruchom połączenie z serwerem w tle
/// Konfiguracja proxy
#[derive(Clone, Default)]
pub struct ProxyConfig {
    pub proxy_type: Option<String>,
    pub server: Option<String>,
    pub port: u16,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub async fn spawn_connection(
    host: String,
    port: u16,
    nickname: String,
    tls: bool,
    password: Option<String>,
    sasl: Option<String>,
    ssl_verify: bool,
    proxy: ProxyConfig,
    ipv6: bool,
    tx: mpsc::Sender<IrcEvent>,
) {
    let tls_label = if tls { "TLS" } else { "PLAIN" };
    let _ = tx.send(IrcEvent::Status(
        format!("-!- Resolving {}:{} ({})...", host, port, tls_label)
    )).await;

    let server_password = password.clone();
    let mut config = Config::default();
    config.server = Some(host.clone());
    config.port = Some(port);
    config.nickname = Some(nickname.clone());
    config.use_tls = Some(tls);
    config.dangerously_accept_invalid_certs = Some(!ssl_verify);
    config.password = password;

    // IPv6 — jeśli wymuszony, dodaj prefiks ipv6:
    if ipv6 {
        let _ = tx.send(IrcEvent::Status(
            "-!- IPv6 mode enabled.".into()
        )).await;
    }

    // Proxy configuration
    if let Some(ref ptype) = proxy.proxy_type {
        use irc::client::data::proxy::ProxyType;
        config.proxy_type = Some(match ptype.to_lowercase().as_str() {
            "socks5" => ProxyType::Socks5,
            "socks4" => ProxyType::Socks5, // fallback — irc crate nie ma Socks4
            _ => ProxyType::Socks5,
        });
        config.proxy_server = proxy.server.clone();
        config.proxy_port = Some(proxy.port);
        config.proxy_username = proxy.username.clone();
        config.proxy_password = proxy.password.clone();
        let _ = tx.send(IrcEvent::Status(
            format!("-!- Using {} proxy: {}:{}",
                ptype, proxy.server.as_deref().unwrap_or("localhost"), proxy.port)
        )).await;
    }

    // POLICY state — pozwól skryptom zmodyfikować ustawienia przed połączeniem
    let _ = tx.send(IrcEvent::Status(
        format!("-!- POLICY: preparing connection to {}:{}...", host, port)
    )).await;
    // Małe opóźnienie żeby skrypty Lua mogły zareagować
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let _ = tx.send(IrcEvent::Status(
        format!("-!- Establishing TCP connection to {}:{}...", host, port)
    )).await;

    match Client::from_config(config).await {
        Ok(mut client) => {
            let _ = tx.send(IrcEvent::Status(
                format!("-!- TCP{} connected.", if tls { "+TLS" } else { "" })
            )).await;

            // IRCv3 CAP negotiation zamiast prostego identify()
            let sender = client.sender();

            // 1. CAP LS — zapytaj o dostępne możliwości
            let _ = tx.send(IrcEvent::Status(
                "-!- Sending CAP LS 302 (IRCv3 capability negotiation)...".into()
            )).await;
            let _ = sender.send(Command::CAP(None, CapSubCommand::LS, None, Some("302".into())));

            // 2. PASS (jeśli podano)
            if let Some(ref pass) = server_password {
                let _ = tx.send(IrcEvent::Status(
                    "-!- Sending PASS (server password)...".into()
                )).await;
                let _ = sender.send(Command::PASS(pass.clone()));
            }

            // 3. NICK + USER
            let _ = tx.send(IrcEvent::Status(
                format!("-!- Sending NICK {} / USER {}...", nickname, nickname)
            )).await;
            let _ = sender.send(Command::NICK(nickname.clone()));
            let _ = sender.send(Command::USER(
                nickname.clone(),
                "0".into(),
                "Void IRC Client".into(),
            ));

            // 4. Jeden stream — CAP negotiation + główny loop
            let _ = tx.send(IrcEvent::Status(
                "-!- Waiting for server welcome (CAP negotiation)...".into()
            )).await;

            let mut cap_done = false;
            let mut sasl_pending = false;
            let mut scram_state: Option<ScramState> = None;
            if let Ok(mut stream) = client.stream() {
                while let Some(result) = stream.next().await {
                    match result {
                        Ok(msg) => {
                            // Obsłuż CAP negotiation wewnętrznie
                            if !cap_done {
                                if let Command::CAP(_, sub, _, ref data) = msg.command {
                                    match sub {
                                        CapSubCommand::LS => {
                                            let caps = data.as_deref().unwrap_or("(none)").to_string();
                                            let _ = tx.send(IrcEvent::Status(
                                                format!("-!- Server capabilities: {}", caps)
                                            )).await;
                                            let _ = tx.send(IrcEvent::CapEvent("LS".into(), caps)).await;
                                            let mut desired = "multi-prefix away-notify account-notify extended-join server-time echo-message".to_string();
                                            if sasl.is_some() {
                                                desired.push_str(" sasl");
                                                // Request specific SASL mechanism
                                                if let Some(ref creds) = sasl {
                                                    if creds.to_uppercase() == "EXTERNAL" {
                                                        desired.push_str(",sasl=EXTERNAL");
                                                    } else if creds.contains(':') {
                                                        // PLAIN or SCRAM — request both
                                                        desired.push_str(",sasl=SCRAM-SHA-512,PLAIN");
                                                    }
                                                }
                                            }
                                            let _ = tx.send(IrcEvent::Status(
                                                format!("-!- Requesting capabilities: {}", desired)
                                            )).await;
                                            let _ = sender.send(Command::CAP(None, CapSubCommand::REQ, None, Some(desired)));
                                        }
                                        CapSubCommand::ACK => {
                                            let caps = data.as_deref().unwrap_or("").to_string();
                                            let _ = tx.send(IrcEvent::Status(
                                                format!("-!- CAP ACK: {}", caps)
                                            )).await;
                                            let _ = tx.send(IrcEvent::CapEvent("ACK".into(), caps.clone())).await;
                                            // SASL auth jeśli potrzebne
                                            if sasl.is_some() {
                                                if caps.contains("sasl") {
                                                    let _ = tx.send(IrcEvent::Status(
                                                        "-!- Starting SASL PLAIN authentication...".into()
                                                    )).await;
                                                    let _ = sender.send(Command::Raw("AUTHENTICATE PLAIN".into(), Vec::new()));
                                                    // AUTHENTICATE + i credentials będą obsłużone w głównej pętli
                                                    sasl_pending = true;
                                                } else {
                                                    let _ = sender.send(Command::CAP(None, CapSubCommand::END, None, None));
                                                    cap_done = true;
                                                    let _ = tx.send(IrcEvent::Connected(sender.clone())).await;
                                                    let _ = tx.send(IrcEvent::Status(
                                                        "-!- Registration complete (SASL not available).".into()
                                                    )).await;
                                                }
                                            } else {
                                                let _ = sender.send(Command::CAP(None, CapSubCommand::END, None, None));
                                                cap_done = true;
                                                let _ = tx.send(IrcEvent::Connected(sender.clone())).await;
                                                let _ = tx.send(IrcEvent::Status(
                                                    "-!- Registration complete.".into()
                                                )).await;
                                            }
                                        }
                                        CapSubCommand::NAK => {
                                            let caps = data.as_deref().unwrap_or("").to_string();
                                            let _ = tx.send(IrcEvent::Status(
                                                format!("-!- CAP NAK (rejected): {}", caps)
                                            )).await;
                                            let _ = tx.send(IrcEvent::CapEvent("NAK".into(), caps)).await;
                                            let _ = sender.send(Command::CAP(None, CapSubCommand::END, None, None));
                                            cap_done = true;
                                            let _ = tx.send(IrcEvent::Connected(sender.clone())).await;
                                            let _ = tx.send(IrcEvent::Status(
                                                "-!- Registration complete (no CAP).".into()
                                            )).await;
                                        }
                                        _ => {}
                                    }
                                } else if sasl_pending {
                                    // Obsłuż SASL: AUTHENTICATE + → wyślij credentials
                                    if let Command::Raw(ref raw, _) = msg.command {
                                        if raw.starts_with("AUTHENTICATE ") {
                                            let challenge = raw[13..].to_string();
                                            // SCRAM-SHA-512: server-first-message
                                            if let Some(ref scram) = scram_state {
                                                if let Some(ref creds) = sasl {
                                                    if let Some((_nick, pass)) = creds.split_once(':') {
                                                        match scram.process_server_first(&challenge, pass) {
                                                            Ok(response) => {
                                                                let _ = tx.send(IrcEvent::Status(
                                                                    "-!- SCRAM-SHA-512: sending client-final...".into()
                                                                )).await;
                                                                let encoded = base64_encode(response.as_bytes());
                                                                let _ = sender.send(Command::Raw(
                                                                    format!("AUTHENTICATE {}", encoded), Vec::new()
                                                                ));
                                                            }
                                                            Err(e) => {
                                                                let _ = tx.send(IrcEvent::Status(
                                                                    format!("-!- SCRAM-SHA-512 error: {}", e)
                                                                )).await;
                                                                let _ = sender.send(Command::Raw("AUTHENTICATE *".into(), Vec::new()));
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else if raw == "AUTHENTICATE +" {
                                            if let Some(ref creds) = sasl {
                                                if creds.to_uppercase() == "EXTERNAL" {
                                                    // SASL EXTERNAL: pusta odpowiedź (certyfikat TLS)
                                                    let _ = tx.send(IrcEvent::Status(
                                                        "-!- Sending SASL EXTERNAL (TLS cert)...".into()
                                                    )).await;
                                                    let _ = sender.send(Command::Raw(
                                                        "AUTHENTICATE +".into(), Vec::new()
                                                    ));
                                                } else if let Some((nick, _pass)) = creds.split_once(':') {
                                                    // SASL SCRAM-SHA-512: client-first-message
                                                    let scram = ScramState::new(nick);
                                                    let client_first = scram.client_first();
                                                    let encoded = base64_encode(client_first.as_bytes());
                                                    scram_state = Some(scram);
                                                    let _ = tx.send(IrcEvent::Status(
                                                        "-!- Starting SASL SCRAM-SHA-512...".into()
                                                    )).await;
                                                    let _ = sender.send(Command::Raw(
                                                        format!("AUTHENTICATE {}", encoded), Vec::new()
                                                    ));
                                                }
                                            }
                                        }
                                    } else if let Command::Response(ref resp, ref args) = msg.command {
                                        let code = *resp as u16;
                                        if code == 903 {
                                            // SASL success
                                            let _ = tx.send(IrcEvent::Status(
                                                "-!- SASL authentication successful.".into()
                                            )).await;
                                            sasl_pending = false;
                                            let _ = sender.send(Command::CAP(None, CapSubCommand::END, None, None));
                                            cap_done = true;
                                            let _ = tx.send(IrcEvent::Connected(sender.clone())).await;
                                        } else if code == 904 || code == 905 {
                                            // SASL failure
                                            let msg_text = args.get(1).map(|s| s.as_str()).unwrap_or("unknown error");
                                            let _ = tx.send(IrcEvent::Status(
                                                format!("-!- SASL authentication failed: {}", msg_text)
                                            )).await;
                                            sasl_pending = false;
                                            let _ = sender.send(Command::CAP(None, CapSubCommand::END, None, None));
                                            cap_done = true;
                                            let _ = tx.send(IrcEvent::Connected(sender.clone())).await;
                                        }
                                    }
                                } else if let Command::Response(ref resp, _) = msg.command {
                                    // Serwer nie obsługuje CAP — wysłał welcome bezpośrednio
                                    if *resp == irc::proto::Response::RPL_WELCOME {
                                        cap_done = true;
                                        let _ = tx.send(IrcEvent::Connected(sender.clone())).await;
                                        let _ = tx.send(IrcEvent::Status(
                                            "-!- Registration complete (server without CAP).".into()
                                        )).await;
                                    }
                                }
                            }

                            // Przekaż wszystkie wiadomości do głównej pętli
                            if tx.send(IrcEvent::Message(msg)).await.is_err() {
                                break;
                            }
                        }
                        Err(e) => {
                            let _ = tx.send(IrcEvent::Error(format!("Stream error: {}", e))).await;
                            break;
                        }
                    }
                }
            }
            let _ = tx.send(IrcEvent::Disconnected).await;
        }
        Err(e) => {
            let _ = tx.send(IrcEvent::Error(format!("Connection error: {}", e))).await;
        }
    }
}
