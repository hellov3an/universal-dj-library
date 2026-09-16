use crate::application::tag_service::TagService;
use crate::domain::error::AppError;
use crate::infrastructure::database::repositories::tag::TagRow;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_tags(state: State<'_, AppState>) -> Result<Vec<TagRow>, AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || TagService::get_all_tags(conn))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn create_tag(
    name: String,
    color: Option<String>,
    state: State<'_, AppState>,
) -> Result<String, AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || TagService::create_tag(conn, &name, color.as_deref()))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn rename_tag(
    id: String,
    name: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || TagService::rename_tag(conn, &id, &name))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn update_tag_color(
    id: String,
    color: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || TagService::update_tag_color(conn, &id, color.as_deref()))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn delete_tag(id: String, state: State<'_, AppState>) -> Result<(), AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || TagService::delete_tag(conn, &id))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn assign_tag(
    track_id: String,
    tag_id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || TagService::assign_tag(conn, &track_id, &tag_id))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn remove_tag_from_track(
    track_id: String,
    tag_id: String,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || TagService::remove_tag(conn, &track_id, &tag_id))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn get_tags_for_track(
    track_id: String,
    state: State<'_, AppState>,
) -> Result<Vec<String>, AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || TagService::get_tags_for_track(conn, &track_id))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}

#[tauri::command]
pub async fn batch_assign_tag(
    track_ids: Vec<String>,
    tag_id: String,
    state: State<'_, AppState>,
) -> Result<u64, AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || TagService::batch_assign_tag(conn, &track_ids, &tag_id))
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
}
