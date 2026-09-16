use crate::application::transfer_service::{TransferPlan, TransferReport, TransferService};
use crate::domain::error::AppError;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn create_transfer_plan(
    adapter_id: String,
    dest_path: String,
    track_ids: Vec<String>,
    state: State<'_, AppState>,
) -> Result<TransferPlan, AppError> {
    let conn = state.db.clone();
    tokio::task::spawn_blocking(move || {
        TransferService::create_transfer_plan(conn, &adapter_id, &dest_path, &track_ids)
    })
    .await
    .map_err(|e| AppError::Transfer {
        message: e.to_string(),
    })?
}

#[tauri::command]
pub async fn execute_transfer(mut plan: TransferPlan) -> Result<TransferReport, AppError> {
    tokio::task::spawn_blocking(move || TransferService::execute_transfer(&mut plan))
        .await
        .map_err(|e| AppError::Transfer {
            message: e.to_string(),
        })?
}
