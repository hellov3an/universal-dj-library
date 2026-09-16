use serde::{Deserialize, Serialize};

/// Typed error hierarchy for the application.
/// Each variant maps to a subsystem with contextual information.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Filesystem error: {0}")]
    Filesystem(String),

    #[error("Device error: {message}")]
    Device {
        message: String,
        device_id: Option<String>,
    },

    #[error("Adapter error [{adapter}]: {message}")]
    Adapter { adapter: String, message: String },

    #[error("Transfer error: {message}")]
    Transfer { message: String },

    #[error("Audio analysis error: {0}")]
    Audio(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Permission denied: {0}")]
    Permission(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Operation cancelled")]
    Cancelled,
}

/// Serializable error for IPC transport to the frontend.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppErrorPayload {
    pub code: String,
    pub message: String,
    pub details: Option<String>,
}

impl From<&AppError> for AppErrorPayload {
    fn from(err: &AppError) -> Self {
        let code = match err {
            AppError::Database(_) => "DATABASE",
            AppError::Filesystem(_) => "FILESYSTEM",
            AppError::Device { .. } => "DEVICE",
            AppError::Adapter { .. } => "ADAPTER",
            AppError::Transfer { .. } => "TRANSFER",
            AppError::Audio(_) => "AUDIO",
            AppError::Validation(_) => "VALIDATION",
            AppError::Permission(_) => "PERMISSION",
            AppError::NotFound(_) => "NOT_FOUND",
            AppError::Cancelled => "CANCELLED",
        };
        AppErrorPayload {
            code: code.to_string(),
            message: err.to_string(),
            details: None,
        }
    }
}

// Allow AppError to be returned from Tauri commands
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let payload = AppErrorPayload::from(self);
        payload.serialize(serializer)
    }
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Filesystem(err.to_string())
    }
}

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        AppError::Validation(err.to_string())
    }
}
