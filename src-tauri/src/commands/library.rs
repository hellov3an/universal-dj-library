use crate::application::library_service::LibraryService;
use crate::domain::error::AppError;
use crate::domain::track::Track;
use crate::infrastructure::database::repositories::track::{TrackMetadataUpdate, TrackRepository};
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryStats {
    pub track_count: u64,
    pub playlist_count: u64,
    pub tag_count: u64,
    pub total_duration_ms: u64,
    pub total_file_size: u64,
}

#[tauri::command]
pub async fn get_library_stats(state: State<'_, AppState>) -> Result<LibraryStats, AppError> {
    state.with_db(|conn| {
        let track_count: u64 = conn.query_row("SELECT COUNT(*) FROM tracks", [], |row| row.get(0)).unwrap_or(0);
        let playlist_count: u64 = conn.query_row("SELECT COUNT(*) FROM playlists WHERE is_folder = 0", [], |row| row.get(0)).unwrap_or(0);
        let tag_count: u64 = conn.query_row("SELECT COUNT(*) FROM tags", [], |row| row.get(0)).unwrap_or(0);
        
        let total_duration_ms: u64 = conn.query_row("SELECT COALESCE(SUM(duration_ms), 0) FROM tracks", [], |row| row.get(0)).unwrap_or(0);
        let total_file_size: u64 = conn.query_row("SELECT COALESCE(SUM(file_size), 0) FROM tracks", [], |row| row.get(0)).unwrap_or(0);

        Ok(LibraryStats {
            track_count,
            playlist_count,
            tag_count,
            total_duration_ms,
            total_file_size,
        })
    })
}

#[tauri::command]
pub async fn scan_directory(
    path: String,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<u32, AppError> {
    let conn_mutex = state.db.clone();
    
    tokio::task::spawn_blocking(move || {
        LibraryService::scan_directory(conn_mutex, app, &path)
    })
    .await
    .map_err(|e| AppError::Filesystem(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()).to_string()))?
}

#[tauri::command]
pub async fn get_tracks(
    query: Option<String>,
    state: State<'_, AppState>,
) -> Result<Vec<Track>, AppError> {
    let conn_mutex = state.db.clone();
    
    tokio::task::spawn_blocking(move || {
        LibraryService::get_tracks(conn_mutex, query)
    })
    .await
    .map_err(|_| AppError::Database(rusqlite::Error::InvalidQuery.to_string()))?
}

#[tauri::command]
pub async fn get_track_by_id(
    id: String,
    state: State<'_, AppState>,
) -> Result<Track, AppError> {
    let conn_mutex = state.db.clone();
    tokio::task::spawn_blocking(move || {
        let uuid = uuid::Uuid::parse_str(&id)?;
        let conn = conn_mutex.lock().map_err(|_| AppError::Database("Lock failed".into()))?;
        TrackRepository::get_by_id(&conn, &uuid)
    })
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn update_track(
    id: String,
    update: TrackMetadataUpdate,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let conn_mutex = state.db.clone();
    tokio::task::spawn_blocking(move || {
        let uuid = uuid::Uuid::parse_str(&id)?;
        let conn = conn_mutex.lock().map_err(|_| AppError::Database("Lock failed".into()))?;
        TrackRepository::update_metadata(&conn, &uuid, &update)
    })
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn update_tracks_bulk(
    ids: Vec<String>,
    update: TrackMetadataUpdate,
    state: State<'_, AppState>,
) -> Result<u32, AppError> {
    let conn_mutex = state.db.clone();
    tokio::task::spawn_blocking(move || {
        let conn = conn_mutex.lock().map_err(|_| AppError::Database("Lock failed".into()))?;
        let mut count = 0u32;
        for id_str in &ids {
            if let Ok(uuid) = uuid::Uuid::parse_str(id_str) {
                if TrackRepository::update_metadata(&conn, &uuid, &update).is_ok() {
                    count += 1;
                }
            }
        }
        Ok(count)
    })
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn delete_tracks(
    ids: Vec<String>,
    state: State<'_, AppState>,
) -> Result<u64, AppError> {
    let conn_mutex = state.db.clone();
    tokio::task::spawn_blocking(move || {
        let conn = conn_mutex.lock().map_err(|_| AppError::Database("Lock failed".into()))?;
        let uuids: Vec<uuid::Uuid> = ids
            .iter()
            .filter_map(|s| uuid::Uuid::parse_str(s).ok())
            .collect();
        TrackRepository::delete_tracks(&conn, &uuids)
    })
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn get_app_version(app: AppHandle) -> Result<String, AppError> {
    Ok(app.package_info().version.to_string())
}
