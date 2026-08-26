use rusqlite::{Connection, Result as SqlResult, params};
use std::path::PathBuf;

/// Persistent storage via SQLCipher (AES-256 encrypted SQLite)
pub struct Storage {
    conn: Connection,
}

impl Storage {
    /// Otwórz lub utwórz zaszyfrowaną bazę danych
    /// passphrase: klucz szyfrowania (AES-256 via SQLCipher)
    pub fn open(path: &str, passphrase: &str) -> SqlResult<Self> {
        let db_path = if path.starts_with("~/") {
            let home = std::env::var("HOME").unwrap_or_default();
            PathBuf::from(home).join(&path[2..])
        } else {
            PathBuf::from(path)
        };

        if let Some(parent) = db_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let conn = Connection::open(&db_path)?;

        // SQLCipher: ustaw klucz szyfrowania
        // To musi być pierwszą operacją po otwarciu połączenia
        conn.execute_batch(&format!("PRAGMA key = '{}';", passphrase.replace('\'', "''")))?;

        let storage = Storage { conn };
        storage.init_tables()?;
        Ok(storage)
    }

    fn init_tables(&self) -> SqlResult<()> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS aliases (
                name TEXT PRIMARY KEY,
                body TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS highlights (
                pattern TEXT PRIMARY KEY,
                color TEXT NOT NULL DEFAULT 'yellow'
            );
            CREATE TABLE IF NOT EXISTS key_bindings (
                key TEXT PRIMARY KEY,
                action TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS servers (
                host TEXT PRIMARY KEY,
                port INTEGER NOT NULL DEFAULT 6697,
                tls INTEGER NOT NULL DEFAULT 1,
                nick TEXT,
                password TEXT,
                nickserv_pass TEXT,
                auto_join TEXT
            );
            CREATE TABLE IF NOT EXISTS notify_list (
                nick TEXT PRIMARY KEY
            );
            CREATE TABLE IF NOT EXISTS ignore_list (
                pattern TEXT PRIMARY KEY,
                flags TEXT NOT NULL DEFAULT 'ALL'
            );
        ")?;
        Ok(())
    }

    // ─── Settings ────────────────────────────────────

    pub fn get_setting(&self, key: &str) -> Option<String> {
        self.conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get(0),
        ).ok()
    }

    pub fn set_setting(&self, key: &str, value: &str) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn get_all_settings(&self) -> Vec<(String, String)> {
        let mut stmt = self.conn.prepare("SELECT key, value FROM settings ORDER BY key").unwrap();
        stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).unwrap().filter_map(|r| r.ok()).collect()
    }

    // ─── Aliases ─────────────────────────────────────

    pub fn get_alias(&self, name: &str) -> Option<String> {
        self.conn.query_row(
            "SELECT body FROM aliases WHERE name = ?1",
            params![name.to_uppercase()],
            |row| row.get(0),
        ).ok()
    }

    pub fn set_alias(&self, name: &str, body: &str) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO aliases (name, body) VALUES (?1, ?2)",
            params![name.to_uppercase(), body],
        )?;
        Ok(())
    }

    pub fn remove_alias(&self, name: &str) -> SqlResult<()> {
        self.conn.execute("DELETE FROM aliases WHERE name = ?1", params![name.to_uppercase()])?;
        Ok(())
    }

    pub fn get_all_aliases(&self) -> Vec<(String, String)> {
        let mut stmt = self.conn.prepare("SELECT name, body FROM aliases ORDER BY name").unwrap();
        stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).unwrap().filter_map(|r| r.ok()).collect()
    }

    // ─── Highlights ──────────────────────────────────

    pub fn add_highlight(&self, pattern: &str, color: &str) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO highlights (pattern, color) VALUES (?1, ?2)",
            params![pattern, color],
        )?;
        Ok(())
    }

    pub fn remove_highlight(&self, pattern: &str) -> SqlResult<()> {
        self.conn.execute("DELETE FROM highlights WHERE pattern = ?1", params![pattern])?;
        Ok(())
    }

    pub fn get_all_highlights(&self) -> Vec<(String, String)> {
        let mut stmt = self.conn.prepare("SELECT pattern, color FROM highlights").unwrap();
        stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).unwrap().filter_map(|r| r.ok()).collect()
    }

    // ─── Key Bindings ────────────────────────────────

    pub fn set_key_binding(&self, key: &str, action: &str) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO key_bindings (key, action) VALUES (?1, ?2)",
            params![key, action],
        )?;
        Ok(())
    }

    pub fn remove_key_binding(&self, key: &str) -> SqlResult<()> {
        self.conn.execute("DELETE FROM key_bindings WHERE key = ?1", params![key])?;
        Ok(())
    }

    pub fn get_all_key_bindings(&self) -> Vec<(String, String)> {
        let mut stmt = self.conn.prepare("SELECT key, action FROM key_bindings").unwrap();
        stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).unwrap().filter_map(|r| r.ok()).collect()
    }

    // ─── Servers ─────────────────────────────────────

    pub fn add_server(&self, host: &str, port: u16, tls: bool, nick: &str, password: &str, nickserv: &str, auto_join: &str) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO servers (host, port, tls, nick, password, nickserv_pass, auto_join) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![host, port, tls as i32, nick, password, nickserv, auto_join],
        )?;
        Ok(())
    }

    pub fn get_server(&self, host: &str) -> Option<(String, u16, bool, String, String, String, String)> {
        self.conn.query_row(
            "SELECT host, port, tls, nick, password, nickserv_pass, auto_join FROM servers WHERE host = ?1",
            params![host],
            |row| Ok((
                row.get::<_, String>(0)?,
                row.get::<_, u16>(1)?,
                row.get::<_, i32>(2)? != 0,
                row.get::<_, String>(3).unwrap_or_default(),
                row.get::<_, String>(4).unwrap_or_default(),
                row.get::<_, String>(5).unwrap_or_default(),
                row.get::<_, String>(6).unwrap_or_default(),
            )),
        ).ok()
    }

    pub fn get_all_servers(&self) -> Vec<(String, u16, bool, String)> {
        let mut stmt = self.conn.prepare("SELECT host, port, tls, nick FROM servers ORDER BY host").unwrap();
        stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, u16>(1)?,
                row.get::<_, i32>(2)? != 0,
                row.get::<_, String>(3).unwrap_or_default(),
            ))
        }).unwrap().filter_map(|r| r.ok()).collect()
    }

    // ─── Notify List ─────────────────────────────────

    pub fn add_notify(&self, nick: &str) -> SqlResult<()> {
        self.conn.execute("INSERT OR IGNORE INTO notify_list (nick) VALUES (?1)", params![nick])?;
        Ok(())
    }

    pub fn remove_notify(&self, nick: &str) -> SqlResult<()> {
        self.conn.execute("DELETE FROM notify_list WHERE nick = ?1", params![nick])?;
        Ok(())
    }

    pub fn get_all_notify(&self) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT nick FROM notify_list ORDER BY nick").unwrap();
        stmt.query_map([], |row| row.get::<_, String>(0)).unwrap().filter_map(|r| r.ok()).collect()
    }

    // ─── Ignore List ─────────────────────────────────

    pub fn add_ignore(&self, pattern: &str, flags: &str) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO ignore_list (pattern, flags) VALUES (?1, ?2)",
            params![pattern, flags],
        )?;
        Ok(())
    }

    pub fn remove_ignore(&self, pattern: &str) -> SqlResult<()> {
        self.conn.execute("DELETE FROM ignore_list WHERE pattern = ?1", params![pattern])?;
        Ok(())
    }

    pub fn get_all_ignore(&self) -> Vec<(String, String)> {
        let mut stmt = self.conn.prepare("SELECT pattern, flags FROM ignore_list").unwrap();
        stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        }).unwrap().filter_map(|r| r.ok()).collect()
    }
}
