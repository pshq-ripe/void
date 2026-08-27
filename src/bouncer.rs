/// IRC Bouncer/Proxy — allows other IRC clients to connect to void
/// void acts as a server, forwarding IRC sessions to connected clients

use std::collections::HashMap;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

/// Connected bouncer client
#[derive(Debug)]
pub struct BouncerClient {
    pub id: usize,
    pub stream: TcpStream,
    pub nick: String,
    pub authenticated: bool,
}

/// Bouncer server state
pub struct Bouncer {
    pub port: u16,
    pub password: String,
    pub running: bool,
    pub clients: HashMap<usize, BouncerClient>,
    next_client_id: usize,
    pub message_log: Vec<String>, // recent messages for replay
}

impl Bouncer {
    pub fn new(port: u16, password: &str) -> Self {
        Bouncer {
            port,
            password: password.to_string(),
            running: false,
            clients: HashMap::new(),
            next_client_id: 1,
            message_log: Vec::new(),
        }
    }

    /// Start the bouncer listener
    pub fn start(&mut self) -> Result<(), String> {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr)
            .map_err(|e| format!("Cannot bind bouncer to {}: {}", addr, e))?;
        self.running = true;
        Ok(())
    }

    /// Stop the bouncer
    pub fn stop(&mut self) {
        self.running = false;
        self.clients.clear();
    }

    /// Add a connected client
    pub fn add_client(&mut self, stream: TcpStream) -> usize {
        let id = self.next_client_id;
        self.next_client_id += 1;
        self.clients.insert(id, BouncerClient {
            id,
            stream,
            nick: String::new(),
            authenticated: false,
        });
        id
    }

    /// Remove a client
    pub fn remove_client(&mut self, id: usize) {
        self.clients.remove(&id);
    }

    /// Send a message to all authenticated clients
    pub fn broadcast_to_clients(&mut self, message: &str) {
        // Log for replay
        self.message_log.push(message.to_string());
        if self.message_log.len() > 500 {
            self.message_log.remove(0);
        }

        // Send to all authenticated clients
        let mut failed = Vec::new();
        for (id, client) in &mut self.clients {
            if client.authenticated {
                if client.stream.write_all(message.as_bytes()).is_err()
                    || client.stream.write_all(b"\r\n").is_err()
                {
                    failed.push(*id);
                }
            }
        }
        for id in failed {
            self.clients.remove(&id);
        }
    }

    /// Replay recent messages to a newly connected client
    pub fn replay_to_client(&mut self, client_id: usize) {
        if let Some(client) = self.clients.get_mut(&client_id) {
            for msg in &self.message_log {
                let _ = client.stream.write_all(msg.as_bytes());
                let _ = client.stream.write_all(b"\r\n");
            }
        }
    }

    /// Get number of connected clients
    pub fn client_count(&self) -> usize {
        self.clients.len()
    }

    /// Get status info
    pub fn status(&self) -> String {
        let clients: Vec<String> = self.clients.values()
            .map(|c| format!("{} ({})", c.id, if c.authenticated { &c.nick } else { "pending" }))
            .collect();
        format!("Bouncer on :{} — {} clients [{}]", self.port, self.clients.len(), clients.join(", "))
    }
}
