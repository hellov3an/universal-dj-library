use crate::domain::error::AppError;
use crate::domain::track::Track;
use crate::infrastructure::database::repositories::track::TrackRepository;
use crate::infrastructure::filesystem::scanner::LibraryScanner;
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ScanProgress {
    pub scanned_files: u32,
    pub current_file: String,
}

pub struct LibraryService;

impl LibraryService {
    /// Scans a directory, parses metadata, saves to database, and emits progress to frontend.
    pub fn scan_directory(
        conn_mutex: Arc<Mutex<Connection>>,
        app: AppHandle,
        dir_path: &str,
    ) -> Result<u32, AppError> {
        let path = Path::new(dir_path);
        if !path.exists() || !path.is_dir() {
            return Err(AppError::Validation("Invalid directory path".into()));
        }

        let mut count = 0;
        
        let tracks = LibraryScanner::scan_directory(path, |file_path| {
            count += 1;
            // Emit progress every 10 files to avoid spamming IPC
            if count % 10 == 0 {
                let progress = ScanProgress {
                    scanned_files: count,
                    current_file: file_path.file_name().unwrap_or_default().to_string_lossy().to_string(),
                };
                let _ = app.emit("library_scan_progress", progress);
            }
        })?;

        // Final progress
        let _ = app.emit("library_scan_progress", ScanProgress {
            scanned_files: count,
            current_file: "Saving to database...".into(),
        });

        // Save to DB
        let conn = conn_mutex.lock().map_err(|_| AppError::Database(rusqlite::Error::InvalidQuery.to_string()))?;
        
        // Wrap in transaction for performance
        conn.execute_batch("BEGIN TRANSACTION;")?;
        
        for track in &tracks {
            if let Err(e) = TrackRepository::upsert(&conn, track) {
                log::error!("Failed to save track {}: {}", track.file_path, e);
            }
        }
        
        conn.execute_batch("COMMIT;")?;

        Ok(tracks.len() as u32)
    }

    /// Fetches tracks, optionally filtering by a search query
    pub fn get_tracks(
        conn_mutex: Arc<Mutex<Connection>>,
        query: Option<String>,
    ) -> Result<Vec<Track>, AppError> {
        let conn = conn_mutex.lock().map_err(|_| AppError::Database(rusqlite::Error::InvalidQuery.to_string()))?;
        
        if let Some(q) = query {
            let q = q.trim();
            if !q.is_empty() {
                // Check for field-specific search like "artist:Daft"
                if let Some(colon_idx) = q.find(':') {
                    let field = &q[..colon_idx];
                    let value = &q[colon_idx + 1..];
                    let valid_fields = ["artist", "genre", "album", "label", "key", "bpm", "rating", "year"];
                    if valid_fields.contains(&field) {
                        return TrackRepository::search_field(&conn, field, value);
                    }
                }
                // Fall back to general FTS search
                return TrackRepository::search(&conn, q);
            }
        }
        
        TrackRepository::get_all(&conn)
    }
}
