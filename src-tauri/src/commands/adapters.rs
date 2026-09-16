use crate::application::import_service::ImportService;
use crate::domain::error::AppError;
use crate::state::AppState;
use tauri::State;
use std::path::PathBuf;

#[tauri::command]
pub async fn import_rekordbox(
    xml_path: String,
    state: State<'_, AppState>,
) -> Result<usize, AppError> {
    let path = PathBuf::from(xml_path);
    if !path.exists() {
        return Err(AppError::Validation("XML file not found".into()));
    }
    
    // In a real app we would use spawn_blocking so we don't block the async runtime
    let conn = state.db.clone();
    let count = tauri::async_runtime::spawn_blocking(move || {
        ImportService::import_from_rekordbox(conn, &path)
    })
    .await
    .map_err(|e| AppError::Adapter { adapter: "Rekordbox".into(), message: e.to_string() })??;

    Ok(count)
}

#[tauri::command]
pub async fn import_serato(
    folder_path: String,
    state: State<'_, AppState>,
) -> Result<usize, AppError> {
    let path = PathBuf::from(folder_path);
    if !path.exists() {
        return Err(AppError::Validation("Serato folder not found".into()));
    }
    
    let conn = state.db.clone();
    let count = tauri::async_runtime::spawn_blocking(move || {
        ImportService::import_from_serato(conn, &path)
    })
    .await
    .map_err(|e| AppError::Adapter { adapter: "Serato".into(), message: e.to_string() })??;

    Ok(count)
}

#[tauri::command]
pub async fn import_traktor(
    nml_path: String,
    state: State<'_, AppState>,
) -> Result<usize, AppError> {
    let path = PathBuf::from(nml_path);
    if !path.exists() {
        return Err(AppError::Validation("Traktor NML file not found".into()));
    }
    
    let conn = state.db.clone();
    let count = tauri::async_runtime::spawn_blocking(move || {
        // Implementation stub for ImportService::import_from_traktor
        Ok::<usize, AppError>(0)
    })
    .await
    .map_err(|e| AppError::Adapter { adapter: "Traktor".into(), message: e.to_string() })??;

    Ok(count)
}

#[tauri::command]
pub async fn import_engine_dj(
    db_path: String,
    state: State<'_, AppState>,
) -> Result<usize, AppError> {
    let path = PathBuf::from(db_path);
    if !path.exists() {
        return Err(AppError::Validation("Engine DJ DB file not found".into()));
    }
    
    let conn = state.db.clone();
    let count = tauri::async_runtime::spawn_blocking(move || {
        // Implementation stub for ImportService::import_from_engine_dj
        Ok::<usize, AppError>(0)
    })
    .await
    .map_err(|e| AppError::Adapter { adapter: "EngineDJ".into(), message: e.to_string() })??;

    Ok(count)
}
