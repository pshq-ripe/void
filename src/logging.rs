use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

/// Logger do zapisywania historii kanału/rozmowy do pliku
/// Wspiera per-buffer logging i rotację po rozmiarze
pub struct Logger {
    files: HashMap<String, File>,
    pub enabled: bool,
    pub path: PathBuf,
    pub per_buffer: bool,
    pub max_size_bytes: u64,
}

impl Logger {
    pub fn new(path: &str) -> Self {
        Logger {
            files: HashMap::new(),
            enabled: false,
            path: PathBuf::from(path),
            per_buffer: false,
            max_size_bytes: 10 * 1024 * 1024, // 10 MB domyślnie
        }
    }

    pub fn enable(&mut self) -> Result<(), String> {
        if let Some(parent) = self.path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| format!("Cannot create log dir: {}", e))?;
            }
        }
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.path)
            .map_err(|e| format!("Cannot open log file: {}", e))?;
        self.files.insert("_global".into(), file);
        self.enabled = true;
        Ok(())
    }

    pub fn enable_buffer(&mut self, buffer_name: &str) -> Result<(), String> {
        let safe_name = buffer_name.replace(|c: char| !c.is_alphanumeric() && c != '-', "_");
        let log_path = self.path.parent()
            .unwrap_or(&self.path)
            .join(format!("{}.log", safe_name));

        if let Some(parent) = log_path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent).map_err(|e| format!("Cannot create log dir: {}", e))?;
            }
        }

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .map_err(|e| format!("Cannot open log file: {}", e))?;
        self.files.insert(buffer_name.to_string(), file);
        self.per_buffer = true;
        self.enabled = true;
        Ok(())
    }

    pub fn disable(&mut self) {
        self.files.clear();
        self.enabled = false;
        self.per_buffer = false;
    }

    pub fn disable_buffer(&mut self, buffer_name: &str) {
        self.files.remove(buffer_name);
    }

    /// Rotacja pliku jeśli przekracza max_size_bytes
    fn rotate_if_needed(&mut self, buffer_name: &str) {
        let safe_name = buffer_name.replace(|c: char| !c.is_alphanumeric() && c != '-', "_");
        let log_path = if buffer_name == "_global" {
            self.path.clone()
        } else {
            self.path.parent()
                .unwrap_or(&self.path)
                .join(format!("{}.log", safe_name))
        };

        if let Ok(meta) = std::fs::metadata(&log_path) {
            if meta.len() > self.max_size_bytes {
                let rotated = format!("{}.1", log_path.display());
                let _ = std::fs::rename(&log_path, &rotated);
                // Otwórz nowy plik
                if let Ok(new_file) = OpenOptions::new().create(true).append(true).open(&log_path) {
                    self.files.insert(buffer_name.to_string(), new_file);
                }
            }
        }
    }

    pub fn write_line(&mut self, buffer_name: &str, text: &str) {
        if !self.enabled {
            return;
        }

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");

        // Per-buffer logging
        if self.per_buffer && self.files.contains_key(buffer_name) {
            self.rotate_if_needed(buffer_name);
            if let Some(ref mut file) = self.files.get_mut(buffer_name) {
                let _ = writeln!(file, "[{}] {}", timestamp, text);
            }
        }

        // Global logging
        if self.files.contains_key("_global") {
            self.rotate_if_needed("_global");
            if let Some(ref mut file) = self.files.get_mut("_global") {
                let _ = writeln!(file, "[{}] [{}] {}", timestamp, buffer_name, text);
            }
        }
    }
}
