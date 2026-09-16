use crate::domain::error::AppError;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub id: String,
    pub event_type: String,
    pub description: String,
    pub details: Option<String>,
    pub timestamp: String,
}

pub struct HistoryRepository;

impl HistoryRepository {
    /// Records a history entry.
    pub fn record(
        conn: &Connection,
        id: &str,
        event_type: &str,
        description: &str,
        details: Option<&str>,
    ) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "INSERT INTO history (id, event_type, description, details, timestamp) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, event_type, description, details, now],
        )?;
        Ok(())
    }

    /// Gets recent history entries.
    pub fn get_recent(conn: &Connection, limit: u32) -> Result<Vec<HistoryEntry>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT * FROM history ORDER BY timestamp DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |row| {
            Ok(HistoryEntry {
                id: row.get("id")?,
                event_type: row.get("event_type")?,
                description: row.get("description")?,
                details: row.get("details")?,
                timestamp: row.get("timestamp")?,
            })
        })?;
        let mut entries = Vec::new();
        for e in rows {
            entries.push(e?);
        }
        Ok(entries)
    }

    /// Gets history entries by event type.
    pub fn get_by_type(conn: &Connection, event_type: &str, limit: u32) -> Result<Vec<HistoryEntry>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT * FROM history WHERE event_type = ?1 ORDER BY timestamp DESC LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![event_type, limit], |row| {
            Ok(HistoryEntry {
                id: row.get("id")?,
                event_type: row.get("event_type")?,
                description: row.get("description")?,
                details: row.get("details")?,
                timestamp: row.get("timestamp")?,
            })
        })?;
        let mut entries = Vec::new();
        for e in rows {
            entries.push(e?);
        }
        Ok(entries)
    }

    /// Clears all history.
    pub fn clear(conn: &Connection) -> Result<(), AppError> {
        conn.execute("DELETE FROM history", [])?;
        Ok(())
    }
}
