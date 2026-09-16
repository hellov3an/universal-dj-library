use crate::domain::error::AppError;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Mixxx uses a SQLite database (mixxxdb.sqlite).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixxxTrack {
    pub id: i64,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<String>,
    pub comment: Option<String>,
    pub bpm: Option<f64>,
    pub key: Option<String>,
    pub duration: Option<f64>,
    pub bitrate: Option<i32>,
    pub location: String,
    pub rating: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixxxDatabase {
    pub tracks: Vec<MixxxTrack>,
}

pub struct MixxxAdapter;

impl MixxxAdapter {
    /// Parses a Mixxx SQLite database.
    pub fn parse_database(db_path: &Path) -> Result<MixxxDatabase, AppError> {
        if !db_path.exists() {
            return Err(AppError::NotFound("Mixxx database not found".into()));
        }

        let conn = rusqlite::Connection::open_with_flags(
            db_path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
        )
        .map_err(|e| AppError::Database(format!("Failed to open Mixxx db: {}", e)))?;

        let mut tracks = Vec::new();
        if let Ok(mut stmt) = conn.prepare(
            "SELECT l.id, l.title, l.artist, l.album, l.genre, l.year, l.comment,
                    l.bpm, l.key, l.duration, l.bitrate, t.location, l.rating
             FROM library l
             JOIN track_locations t ON l.location = t.id",
        ) {
            if let Ok(rows) = stmt.query_map([], |row| {
                Ok(MixxxTrack {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    artist: row.get(2)?,
                    album: row.get(3)?,
                    genre: row.get(4)?,
                    year: row.get(5)?,
                    comment: row.get(6)?,
                    bpm: row.get(7)?,
                    key: row.get(8)?,
                    duration: row.get(9)?,
                    bitrate: row.get(10)?,
                    location: row.get(11)?,
                    rating: row.get(12)?,
                })
            }) {
                for track in rows {
                    if let Ok(t) = track {
                        tracks.push(t);
                    }
                }
            }
        }

        Ok(MixxxDatabase { tracks })
    }

    /// Detects if a Mixxx database exists at the standard location.
    pub fn detect(root: &Path) -> bool {
        let db_path = root.join("mixxxdb.sqlite");
        db_path.exists()
    }
}
