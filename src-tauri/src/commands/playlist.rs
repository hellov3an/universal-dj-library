use crate::application::playlist_service::PlaylistService;
use crate::domain::error::AppError;
use crate::infrastructure::database::repositories::playlist::PlaylistRow;
use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Deserialize)]
pub struct CreatePlaylistRequest {
    pub name: String,
    pub parent_id: Option<String>,
    pub is_folder: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateSmartPlaylistRequest {
    pub name: String,
    pub parent_id: Option<String>,
    pub rules_json: String,
}

#[tauri::command]
pub async fn get_playlists(state: State<'_, AppState>) -> Result<Vec<PlaylistRow>, AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || PlaylistService::get_all_playlists(conn))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn create_playlist(
    request: CreatePlaylistRequest,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let conn = state.db.clone();
    let name = request.name;
    let parent_id = request.parent_id;
    let is_folder = request.is_folder;
    tokio::task::spawn_blocking(move || {
        PlaylistService::create_playlist(conn, &name, parent_id.as_deref(), is_folder)
    })
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn create_smart_playlist(
    request: CreateSmartPlaylistRequest,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let conn = state.db.clone();
    let name = request.name;
    let parent_id = request.parent_id;
    let rules_json = request.rules_json;
    tokio::task::spawn_blocking(move || {
        PlaylistService::create_smart_playlist(conn, &name, parent_id.as_deref(), &rules_json)
    })
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn rename_playlist(
    id: String,
    name: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || PlaylistService::rename_playlist(conn, &id, &name))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn delete_playlist(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || PlaylistService::delete_playlist(conn, &id))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn add_track_to_playlist(
    playlist_id: String,
    track_id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || {
        PlaylistService::add_track_to_playlist(conn, &playlist_id, &track_id)
    })
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn remove_track_from_playlist(
    playlist_id: String,
    track_id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || {
        PlaylistService::remove_track_from_playlist(conn, &playlist_id, &track_id)
    })
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn get_playlist_track_ids(
    playlist_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || {
        PlaylistService::get_playlist_track_ids(conn, &playlist_id)
    })
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn reorder_playlist(
    playlist_id: String,
    track_ids: Vec<String>,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || {
        PlaylistService::reorder_playlist(conn, &playlist_id, &track_ids)
    })
    .await
    .map_err(|e| AppError::Database(e.to_string()))?
}
