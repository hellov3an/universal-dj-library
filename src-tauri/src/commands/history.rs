use crate::domain::error::AppError;
use crate::state::AppState;
use tauri::State;
use serde::Serialize;

#[derive(Serialize)]
pub struct HistoryEvent {
    pub id: String,
    pub event_type: String,
    pub description: String,
    pub details: Option<String>,
    pub timestamp: String,
}

#[tauri::command]
pub fn get_history(state: State<'_, AppState>) -> Result<Vec<HistoryEvent>, AppError> {
    state.with_db(|conn| {
        let mut stmt = conn.prepare("SELECT id, event_type, description, details, timestamp FROM history ORDER BY timestamp DESC LIMIT 100")?;
        let rows = stmt.query_map([], |row| {
            Ok(HistoryEvent {
                id: row.get(0)?,
                event_type: row.get(1)?,
                description: row.get(2)?,
                details: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })?;

        let mut events = Vec::new();
        for r in rows {
            events.push(r?);
        }
        Ok(events)
    })
}
