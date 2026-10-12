//! SQLite connection setup: `open()`, `open_in_memory()` and pragmas.
//! Must not import `tauri`.

mod migrations;

use std::fs;
use std::path::Path;

use rusqlite::Connection;

use crate::error::AppResult;

/// Opens (or creates) the database file at `path`, creating its parent
/// directory if needed, then applies pragmas and runs pending migrations.
pub fn open(path: &Path) -> AppResult<Connection> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    init(Connection::open(path)?)
}

/// A migrated in-memory database with the same pragmas as [`open`].
#[cfg(test)]
pub fn open_in_memory() -> AppResult<Connection> {
    init(Connection::open_in_memory()?)
}

fn init(mut conn: Connection) -> AppResult<Connection> {
    // SQLite leaves foreign keys off by default, and the setting can't be
    // changed inside a transaction, so it must be set before migrating.
    conn.pragma_update(None, "foreign_keys", "ON")?;
    // Returns the resulting mode, which is "memory" for in-memory databases.
    conn.pragma_update_and_check(None, "journal_mode", "WAL", |row| row.get::<_, String>(0))?;
    migrations::MIGRATIONS.to_latest(&mut conn)?;
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn foreign_keys(conn: &Connection) -> i64 {
        conn.pragma_query_value(None, "foreign_keys", |row| row.get(0))
            .unwrap()
    }

    fn journal_mode(conn: &Connection) -> String {
        conn.pragma_query_value(None, "journal_mode", |row| row.get(0))
            .unwrap()
    }

    #[test]
    fn in_memory_connection_enforces_foreign_keys() {
        let conn = open_in_memory().unwrap();
        assert_eq!(foreign_keys(&conn), 1);
    }

    #[test]
    fn file_connection_enforces_foreign_keys_and_uses_wal() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested").join("finance.db");

        let conn = open(&path).unwrap();

        assert!(path.exists(), "open() should create missing parent dirs");
        assert_eq!(foreign_keys(&conn), 1);
        assert_eq!(journal_mode(&conn), "wal");
    }
}
