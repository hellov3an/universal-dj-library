use crate::domain::error::AppError;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Engine DJ uses a SQLite database (m.db) for its library.
/// This adapter reads from that database.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineDjTrack {
    pub id: i64,
    pub path: String,
    pub filename: String,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub bpm: Option<f64>,
    pub musical_key: Option<String>,
    pub duration_ms: Option<i64>,
    pub bitrate: Option<i32>,
    pub year: Option<i32>,
    pub comment: Option<String>,
    pub rating: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineDjPlaylist {
    pub id: i64,
    pub title: String,
    pub parent_id: Option<i64>,
    pub is_folder: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineDjDatabase {
    pub tracks: Vec<EngineDjTrack>,
    pub playlists: Vec<EngineDjPlaylist>,
}

pub struct EngineDjAdapter;

impl EngineDjAdapter {
    /// Parses an Engine DJ database (m.db SQLite file).
    pub fn parse_database(db_path: &Path) -> Result<EngineDjDatabase, AppError> {
        if !db_path.exists() {
            return Err(AppError::NotFound("Engine DJ database not found".into()));
        }

        let conn = rusqlite::Connection::open_with_flags(
            db_path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
        )
        .map_err(|e| AppError::Database(format!("Failed to open Engine DJ db: {}", e)))?;

        // Read tracks
        let mut tracks = Vec::new();
        if let Ok(mut stmt) = conn.prepare(
            "SELECT id, path, filename, title, artist, album, genre, bpmAnalyzed, 
                    tonality, length, bitrate, year, comment, rating 
             FROM Track",
        ) {
            if let Ok(rows) = stmt.query_map([], |row| {
                Ok(EngineDjTrack {
                    id: row.get(0)?,
                    path: row.get(1)?,
                    filename: row.get(2)?,
                    title: row.get(3)?,
                    artist: row.get(4)?,
                    album: row.get(5)?,
                    genre: row.get(6)?,
                    bpm: row.get(7)?,
                    musical_key: row.get(8)?,
                    duration_ms: row.get(9)?,
                    bitrate: row.get(10)?,
                    year: row.get(11)?,
                    comment: row.get(12)?,
                    rating: row.get(13)?,
                })
            }) {
                for track in rows {
                    if let Ok(t) = track {
                        tracks.push(t);
                    }
                }
            }
        }

        // Read playlists
        let mut playlists = Vec::new();
        if let Ok(mut stmt) = conn.prepare(
            "SELECT id, title, parentListId, isPreparedList FROM Playlist",
        ) {
            if let Ok(rows) = stmt.query_map([], |row| {
                Ok(EngineDjPlaylist {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    parent_id: row.get(2)?,
                    is_folder: false,
                })
            }) {
                for pl in rows {
                    if let Ok(p) = pl {
                        playlists.push(p);
                    }
                }
            }
        }

        Ok(EngineDjDatabase { tracks, playlists })
    }

    /// Detects if an Engine DJ database exists at the given root path.
    pub fn detect(root: &Path) -> bool {
        // Engine DJ stores its database in ENGINE_LIBRARY folder
        let db_path = root.join("Engine Library").join("m.db");
        db_path.exists()
    }
}
