use rusqlite::{Connection, Result as SqlResult, params};
use std::path::PathBuf;

/// Persistent storage via SQLCipher (AES-256 encrypted SQLite)
pub struct Storage {
    conn: Connection,
}

impl Storage {
    /// Otwórz lub utwórz zaszyfrowaną bazę danych
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

    // Helper: prepare and query, returning Vec<T>
    fn query_vec<F, T>(&self, sql: &str, map_fn: F) -> Vec<T>
    where
        F: Fn(&rusqlite::Row) -> rusqlite::Result<T>,
    {
        let mut stmt = match self.conn.prepare(sql) {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };
        let rows = match stmt.query_map([], |row| map_fn(row)) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        rows.filter_map(|r| r.ok()).collect()
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
        self.query_vec(
            "SELECT key, value FROM settings ORDER BY key",
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
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
        self.query_vec(
            "SELECT name, body FROM aliases ORDER BY name",
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
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
        self.query_vec(
            "SELECT pattern, color FROM highlights",
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
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
        self.query_vec(
            "SELECT key, action FROM key_bindings",
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
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
        self.query_vec(
            "SELECT host, port, tls, nick FROM servers ORDER BY host",
            |row| Ok((
                row.get::<_, String>(0)?,
                row.get::<_, u16>(1)?,
                row.get::<_, i32>(2)? != 0,
                row.get::<_, String>(3).unwrap_or_default(),
            )),
        )
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
        self.query_vec(
            "SELECT nick FROM notify_list ORDER BY nick",
            |row| row.get::<_, String>(0),
        )
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
        self.query_vec(
            "SELECT pattern, flags FROM ignore_list",
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
        )
    }

    // ─── Session save/restore ─────────────────────────

    pub fn init_session_table(&self) -> SqlResult<()> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS session_buffers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                server_host TEXT NOT NULL,
                channel TEXT,
                auto_join INTEGER DEFAULT 0
            );
        ")?;
        Ok(())
    }

    pub fn save_session_buffer(&self, name: &str, server_host: &str, channel: &str, auto_join: bool) -> SqlResult<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO session_buffers (name, server_host, channel, auto_join) VALUES (?1, ?2, ?3, ?4)",
            params![name, server_host, channel, auto_join as i32],
        )?;
        Ok(())
    }

    pub fn get_session_buffers(&self) -> Vec<(String, String, String, bool)> {
        self.query_vec(
            "SELECT name, server_host, channel, auto_join FROM session_buffers ORDER BY id",
            |row| Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i32>(3)? != 0,
            )),
        )
    }

    pub fn clear_session_buffers(&self) -> SqlResult<()> {
        self.conn.execute("DELETE FROM session_buffers", [])?;
        Ok(())
    }

    // ─── Window layout persistence ───────────────────

    pub fn init_layout_table(&self) -> SqlResult<()> {
        self.conn.execute_batch("
            CREATE TABLE IF NOT EXISTS window_layout (
                idx INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                split_idx INTEGER,
                split_horizontal INTEGER DEFAULT 0
            );
        ")?;
        Ok(())
    }

    pub fn save_window_layout(&self, windows: &[(String, Option<usize>, bool)]) -> SqlResult<()> {
        self.conn.execute("DELETE FROM window_layout", [])?;
        for (i, (name, split_idx, horizontal)) in windows.iter().enumerate() {
            self.conn.execute(
                "INSERT INTO window_layout (idx, name, split_idx, split_horizontal) VALUES (?1, ?2, ?3, ?4)",
                params![i as i32, name, split_idx.map(|s| s as i32), *horizontal as i32],
            )?;
        }
        Ok(())
    }

    pub fn get_window_layout(&self) -> Vec<(String, Option<usize>, bool)> {
        self.query_vec(
            "SELECT name, split_idx, split_horizontal FROM window_layout ORDER BY idx",
            |row| Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<i32>>(1)?.map(|s| s as usize),
                row.get::<_, i32>(2)? != 0,
            )),
        )
    }
}
