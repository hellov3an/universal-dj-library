use crate::domain::error::AppError;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Backup metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub path: String,
    pub created_at: String,
    pub track_count: u64,
    pub playlist_count: u64,
    pub size_bytes: u64,
}

pub struct BackupService;

impl BackupService {
    /// Creates a JSON backup of the library to the specified path.
    pub fn create_backup(
        conn_mutex: Arc<Mutex<Connection>>,
        backup_path: &str,
    ) -> Result<BackupInfo, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;

        let track_count: u64 = conn
            .query_row("SELECT COUNT(*) FROM tracks", [], |row| row.get(0))
            .unwrap_or(0);
        let playlist_count: u64 = conn
            .query_row("SELECT COUNT(*) FROM playlists", [], |row| row.get(0))
            .unwrap_or(0);

        // Export tracks as JSON
        let mut tracks_stmt = conn.prepare("SELECT * FROM tracks")?;
        let tracks: Vec<serde_json::Value> = tracks_stmt
            .query_map([], |row| {
                let id: String = row.get("id")?;
                let title: Option<String> = row.get("title")?;
                let artist: Option<String> = row.get("artist")?;
                let album: Option<String> = row.get("album")?;
                let file_path: String = row.get("file_path")?;
                Ok(serde_json::json!({
                    "id": id,
                    "title": title,
                    "artist": artist,
                    "album": album,
                    "file_path": file_path,
                }))
            })?
            .filter_map(|r| r.ok())
            .collect();

        let backup_data = serde_json::json!({
            "version": "1.0",
            "created_at": chrono::Utc::now().to_rfc3339(),
            "track_count": track_count,
            "playlist_count": playlist_count,
            "tracks": tracks,
        });

        let json_str = serde_json::to_string_pretty(&backup_data)
            .map_err(|e| AppError::Filesystem(e.to_string()))?;

        std::fs::write(backup_path, &json_str)?;

        let size = std::fs::metadata(backup_path)?.len();

        Ok(BackupInfo {
            path: backup_path.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            track_count,
            playlist_count,
            size_bytes: size,
        })
    }

    /// Lists available backups in a directory.
    pub fn list_backups(backup_dir: &str) -> Result<Vec<BackupInfo>, AppError> {
        let dir = Path::new(backup_dir);
        if !dir.exists() {
            return Ok(vec![]);
        }

        let mut backups = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("json") {
                if let Ok(metadata) = std::fs::metadata(&path) {
                    backups.push(BackupInfo {
                        path: path.to_string_lossy().to_string(),
                        created_at: String::new(),
                        track_count: 0,
                        playlist_count: 0,
                        size_bytes: metadata.len(),
                    });
                }
            }
        }
        Ok(backups)
    }
}
