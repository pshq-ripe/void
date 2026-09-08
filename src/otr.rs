/// OTR (Off-the-Record) encryption for private messages
/// Simplified OTR v2-like protocol over IRC PRIVMSG
use ring::agreement::{EphemeralPrivateKey, PublicKey, UnparsedPublicKey, ECDH_P256};
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, AES_256_GCM};
use ring::digest::{SHA256, digest};
use ring::hmac;
use ring::rand::{SecureRandom, SystemRandom};
use std::collections::HashMap;

/// Stan sesji OTR z danym nickiem
#[derive(Clone, Debug)]
pub enum OtrState {
    Plaintext,          // Brak szyfrowania
    AwaitingKey,        // Wysłaliśmy DH pubkey, czekamy na odpowiedź
    Encrypted,          // Sesja szyfrowana
    Finished,           // Sesja zakończona
}

/// Sesja OTR z konkretnym użytkownikiem
pub struct OtrSession {
    pub state: OtrState,
    pub peer_nick: String,
    pub our_private: Option<EphemeralPrivateKey>,
    pub our_public: Option<Vec<u8>>,
    pub shared_secret: Option<Vec<u8>>,
    pub send_key: Option<Vec<u8>>,
    pub recv_key: Option<Vec<u8>>,
    pub send_counter: u64,
    pub recv_counter: u64,
    pub fingerprint: Option<String>,
}

impl OtrSession {
    pub fn new(peer_nick: &str) -> Self {
        OtrSession {
            state: OtrState::Plaintext,
            peer_nick: peer_nick.to_string(),
            our_private: None,
            our_public: None,
            shared_secret: None,
            send_key: None,
            recv_key: None,
            send_counter: 0,
            recv_counter: 0,
            fingerprint: None,
        }
    }

    /// Rozpocznij key exchange — generuj DH keypair
    pub fn initiate(&mut self) -> Result<Vec<u8>, String> {
        let rng = SystemRandom::new();
        let private_key = EphemeralPrivateKey::generate(&ECDH_P256, &rng)
            .map_err(|e| format!("Key gen error: {:?}", e))?;
        let public_key = private_key.compute_public_key()
            .map_err(|e| format!("Pub key error: {:?}", e))?;

        let pub_bytes = public_key.as_ref().to_vec();
        self.our_private = Some(private_key);
        self.our_public = Some(pub_bytes.clone());
        self.state = OtrState::AwaitingKey;

        // OTR message: "?OTR:AAM..." — DH pubkey as base64
        Ok(pub_bytes)
    }

    /// Odbierz DH pubkey od peera i oblicz shared secret
    pub fn receive_key(&mut self, peer_pub_bytes: &[u8]) -> Result<(), String> {
        let peer_pub = UnparsedPublicKey::new(&ECDH_P256, peer_pub_bytes);

        if let Some(ref our_private) = self.our_private {
            // Oblicz shared secret przez ECDH
            let our_private_owned = std::mem::replace(
                &mut self.our_private,
                None
            ).ok_or("No private key")?;

            let shared = ring::agreement::agree_ephemeral(
                our_private_owned,
                &peer_pub,
                |shared| shared.to_vec(),
            ).map_err(|e| format!("ECDH error: {:?}", e))?;

            // Derive send/recv keys z shared secret
            let send_key_material = derive_key(&shared, b"send");
            let recv_key_material = derive_key(&shared, b"recv");

            self.shared_secret = Some(shared);
            self.send_key = Some(send_key_material[..32].to_vec());
            self.recv_key = Some(recv_key_material[..32].to_vec());
            self.state = OtrState::Encrypted;

            // Fingerprint
            let fp = digest(&SHA256, &self.our_public.as_ref().unwrap());
            self.fingerprint = Some(hex::encode(fp.as_ref()));
        } else {
            return Err("No private key — initiate first".into());
        }
        Ok(())
    }

    /// Szyfruj wiadomość
    pub fn encrypt(&mut self, plaintext: &str) -> Result<String, String> {
        let key_bytes = self.send_key.as_ref().ok_or("No send key")?;
        let unbound = UnboundKey::new(&AES_256_GCM, key_bytes)
            .map_err(|e| format!("Key error: {:?}", e))?;
        let less_safe = LessSafeKey::new(unbound);

        // Nonce z counter
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[..8].copy_from_slice(&self.send_counter.to_be_bytes());
        self.send_counter += 1;

        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let mut in_out = plaintext.as_bytes().to_vec();
        less_safe.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| format!("Encrypt error: {:?}", e))?;

        // Base64 encode
        Ok(base64::encode(&in_out))
    }

    /// Deszyfruj wiadomość
    pub fn decrypt(&mut self, ciphertext_b64: &str) -> Result<String, String> {
        let key_bytes = self.recv_key.as_ref().ok_or("No recv key")?;
        let ciphertext = base64::decode(ciphertext_b64)
            .map_err(|e| format!("Base64 error: {}", e))?;

        let unbound = UnboundKey::new(&AES_256_GCM, key_bytes)
            .map_err(|e| format!("Key error: {:?}", e))?;
        let less_safe = LessSafeKey::new(unbound);

        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[..8].copy_from_slice(&self.recv_counter.to_be_bytes());
        self.recv_counter += 1;

        let nonce = Nonce::assume_unique_for_key(nonce_bytes);
        let mut in_out = ciphertext;
        let plaintext = less_safe.open_in_place(nonce, Aad::empty(), &mut in_out)
            .map_err(|e| format!("Decrypt error: {:?}", e))?;

        String::from_utf8(plaintext.to_vec())
            .map_err(|e| format!("UTF-8 error: {}", e))
    }
}

/// Derive key z shared secret
fn derive_key(shared: &[u8], info: &[u8]) -> Vec<u8> {
    let key = hmac::Key::new(hmac::HMAC_SHA256, shared);
    let mut ctx = hmac::Context::with_key(&key);
    ctx.update(info);
    let tag = ctx.sign();
    tag.as_ref().to_vec()
}

/// OTR manager — sesje per nick
pub struct OtrManager {
    pub sessions: HashMap<String, OtrSession>,
}

impl OtrManager {
    pub fn new() -> Self {
        OtrManager {
            sessions: HashMap::new(),
        }
    }

    /// Rozpocznij OTR z nickiem
    pub fn start(&mut self, nick: &str) -> Result<Vec<u8>, String> {
        let mut session = OtrSession::new(nick);
        let key = session.initiate()?;
        self.sessions.insert(nick.to_string(), session);
        Ok(key)
    }

    /// Sprawdź czy wiadomość to OTR
    pub fn is_otr_message(text: &str) -> bool {
        text.starts_with("?OTR:")
    }

    /// Przetwórz wiadomość OTR
    pub fn handle_message(&mut self, from: &str, text: &str) -> Result<Option<String>, String> {
        // Parsuj OTR message
        if text.starts_with("?OTR:AAM,") {
            // DH key exchange
            let key_b64 = &text[9..];
            let key_bytes = base64::decode(key_b64)
                .map_err(|e| format!("OTR key decode error: {}", e))?;

            let session = self.sessions.entry(from.to_string())
                .or_insert_with(|| OtrSession::new(from));
            session.receive_key(&key_bytes)?;
            return Ok(None);
        } else if text.starts_with("?OTR:AAM:") {
            // Encrypted message
            let msg_b64 = &text[9..];
            let session = self.sessions.get_mut(from)
                .ok_or("No OTR session")?;
            let plaintext = session.decrypt(msg_b64)?;
            return Ok(Some(plaintext));
        }

        Ok(None)
    }
}
