use std::sync::{Arc, Mutex};
use rusqlite::Connection;
use tauri::AppHandle;
use crate::application::analysis_service::AnalysisManager;

/// Application state shared across all Tauri commands.
/// The database connection is wrapped in a Mutex for thread-safe access.
pub struct AppState {
    pub db: Arc<Mutex<Connection>>,
    pub analysis_manager: AnalysisManager,
}

impl AppState {
    pub fn new(db: Connection, app: AppHandle) -> Self {
        let db_arc = Arc::new(Mutex::new(db));
        let analysis_manager = AnalysisManager::new(db_arc.clone(), app);
        
        AppState { 
            db: db_arc,
            analysis_manager,
        }
    }

    /// Execute a closure with a reference to the database connection.
    pub fn with_db<F, R>(&self, f: F) -> Result<R, crate::domain::error::AppError>
    where
        F: FnOnce(&Connection) -> Result<R, crate::domain::error::AppError>,
    {
        let conn = self.db.lock().map_err(|e| {
            crate::domain::error::AppError::Database(format!(
                "Failed to acquire database lock: {}",
                e
            ))
        })?;
        f(&conn)
    }
}
