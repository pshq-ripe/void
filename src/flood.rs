use std::collections::VecDeque;
use std::time::Instant;

/// Ochrona przed floodem — limituje ilość wysyłanych wiadomości na serwer
pub struct FloodProtection {
    pub enabled: bool,
    pub max_messages: usize,
    pub window_secs: u64,
    timestamps: VecDeque<Instant>,
    queue: VecDeque<String>,
}

impl FloodProtection {
    pub fn new(max_messages: usize, window_secs: u64) -> Self {
        FloodProtection {
            enabled: true,
            max_messages,
            window_secs,
            timestamps: VecDeque::new(),
            queue: VecDeque::new(),
        }
    }

    /// Sprawdź, czy można wysłać wiadomość. True = tak, wolna droga.
    pub fn can_send(&mut self) -> bool {
        if !self.enabled {
            return true;
        }
        let now = Instant::now();
        // Usuń stare timestamps sprzed okna
        while let Some(front) = self.timestamps.front() {
            if now.duration_since(*front).as_secs() >= self.window_secs {
                self.timestamps.pop_front();
            } else {
                break;
            }
        }
        self.timestamps.len() < self.max_messages
    }

    /// Zaznacz, że wysłano wiadomość
    pub fn record_send(&mut self) {
        self.timestamps.push_back(Instant::now());
    }

    /// Dodaj do kolejki (jeśli flood)
    pub fn enqueue(&mut self, raw_line: String) {
        self.queue.push_back(raw_line);
    }

    /// Pobierz z kolejki, jeśli można wysłać
    pub fn dequeue(&mut self) -> Option<String> {
        if self.can_send() {
            self.queue.pop_front()
        } else {
            None
        }
    }

    /// Czy jest coś w kolejce?
    pub fn has_queued(&self) -> bool {
        !self.queue.is_empty()
    }
}
