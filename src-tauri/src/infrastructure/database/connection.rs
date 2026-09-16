use rusqlite::Connection;
use std::fs;
use std::path::Path;

use crate::domain::error::AppError;

/// Initialize a SQLite database connection with optimal settings.
/// Creates the database file and parent directories if they don't exist.
pub fn initialize(db_path: &Path) -> Result<Connection, AppError> {
    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let conn = Connection::open(db_path)?;

    // Configure SQLite for optimal performance
    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA foreign_keys = ON;
        PRAGMA cache_size = -64000;
        PRAGMA busy_timeout = 5000;
        PRAGMA temp_store = MEMORY;
        ",
    )?;

    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize_creates_database() {
        let temp_dir = std::env::temp_dir().join("udl_test_db");
        let db_path = temp_dir.join("test.db");

        // Clean up from any previous test run
        let _ = fs::remove_dir_all(&temp_dir);

        let conn = initialize(&db_path).expect("Failed to initialize database");

        // Verify WAL mode is enabled
        let journal_mode: String = conn
            .pragma_query_value(None, "journal_mode", |row| row.get(0))
            .unwrap();
        assert_eq!(journal_mode.to_lowercase(), "wal");

        // Verify foreign keys are enabled
        let fk: i64 = conn
            .pragma_query_value(None, "foreign_keys", |row| row.get(0))
            .unwrap();
        assert_eq!(fk, 1);

        // Clean up
        drop(conn);
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
