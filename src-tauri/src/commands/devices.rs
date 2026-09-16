use crate::domain::error::AppError;
use crate::infrastructure::devices::manager::{DeviceManager, Device};

#[tauri::command]
pub async fn get_devices() -> Result<Vec<Device>, AppError> {
    Ok(DeviceManager::get_devices())
}
