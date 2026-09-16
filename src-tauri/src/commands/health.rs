use crate::application::health_service::{HealthReport, HealthService};
use crate::domain::error::AppError;
use crate::domain::track::Track;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_health_report(state: State<'_, AppState>) -> Result<HealthReport, AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || HealthService::get_health_report(conn))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn get_missing_file_tracks(
    state: State<'_, AppState>,
) -> Result<Vec<Track>, AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || HealthService::get_missing_file_tracks(conn))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn relink_track(
    track_id: String,
    new_path: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || HealthService::relink_track(conn, &track_id, &new_path))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}
