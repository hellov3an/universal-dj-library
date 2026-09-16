use crate::domain::error::AppError;
use rusqlite::{params, Connection};

pub struct SettingsRepository;

impl SettingsRepository {
    /// Gets a setting value by key.
    pub fn get(conn: &Connection, key: &str) -> Result<Option<String>, AppError> {
        let result = conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            params![key],
            |row| row.get::<_, String>(0),
        );
        match result {
            Ok(val) => Ok(Some(val)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AppError::Database(e.to_string())),
        }
    }

    /// Sets a setting value (upsert).
    pub fn set(conn: &Connection, key: &str, value: &str) -> Result<(), AppError> {
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    /// Deletes a setting.
    pub fn delete(conn: &Connection, key: &str) -> Result<(), AppError> {
        conn.execute("DELETE FROM settings WHERE key = ?1", params![key])?;
        Ok(())
    }

    /// Gets all settings as key-value pairs.
    pub fn get_all(conn: &Connection) -> Result<Vec<(String, String)>, AppError> {
        let mut stmt = conn.prepare("SELECT key, value FROM settings ORDER BY key")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        let mut settings = Vec::new();
        for s in rows {
            settings.push(s?);
        }
        Ok(settings)
    }
}
