use crate::domain::error::AppError;
use crate::infrastructure::database::repositories::history::{HistoryEntry, HistoryRepository};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// An action that can be undone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UndoableAction {
    pub id: String,
    pub description: String,
    pub action_type: String,
    pub undo_data: String, // JSON-serialized undo data
    pub timestamp: String,
}

pub struct HistoryService;

impl HistoryService {
    /// Records an event in the history log.
    pub fn record_event(
        conn_mutex: Arc<Mutex<Connection>>,
        event_type: &str,
        description: &str,
        details: Option<&str>,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        let id = uuid::Uuid::new_v4().to_string();
        HistoryRepository::record(&conn, &id, event_type, description, details)
    }

    /// Gets recent history entries.
    pub fn get_recent_history(
        conn_mutex: Arc<Mutex<Connection>>,
        limit: u32,
    ) -> Result<Vec<HistoryEntry>, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        HistoryRepository::get_recent(&conn, limit)
    }

    /// Gets history entries by type.
    pub fn get_history_by_type(
        conn_mutex: Arc<Mutex<Connection>>,
        event_type: &str,
        limit: u32,
    ) -> Result<Vec<HistoryEntry>, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        HistoryRepository::get_by_type(&conn, event_type, limit)
    }

    /// Clears history.
    pub fn clear_history(conn_mutex: Arc<Mutex<Connection>>) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        HistoryRepository::clear(&conn)
    }
}
