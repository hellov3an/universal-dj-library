use crate::domain::error::AppError;
use crate::infrastructure::database::repositories::track::{DuplicateGroup, TrackRepository};
use crate::domain::track::Track;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Summary of library health.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub total_tracks: u64,
    pub unanalyzed_tracks: u64,
    pub missing_file_count: u64,
    pub duplicate_hash_groups: Vec<DuplicateGroup>,
    pub duplicate_metadata_groups: Vec<DuplicateGroup>,
    pub tracks_without_metadata: u64,
}

pub struct HealthService;

impl HealthService {
    /// Generates a full library health report.
    pub fn get_health_report(
        conn_mutex: Arc<Mutex<Connection>>,
    ) -> Result<HealthReport, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;

        let total_tracks: u64 = conn
            .query_row("SELECT COUNT(*) FROM tracks", [], |row| row.get(0))
            .unwrap_or(0);

        let unanalyzed_tracks = TrackRepository::get_unanalyzed_count(&conn)?;
        let missing_files = TrackRepository::get_missing_files(&conn)?;
        let duplicate_hash_groups = TrackRepository::get_duplicates_by_hash(&conn)?;
        let duplicate_metadata_groups = TrackRepository::get_duplicates_by_metadata(&conn)?;
        let tracks_without_metadata = TrackRepository::get_tracks_without_metadata(&conn)?;

        Ok(HealthReport {
            total_tracks,
            unanalyzed_tracks,
            missing_file_count: missing_files.len() as u64,
            duplicate_hash_groups,
            duplicate_metadata_groups,
            tracks_without_metadata: tracks_without_metadata.len() as u64,
        })
    }

    /// Gets tracks with missing files.
    pub fn get_missing_file_tracks(
        conn_mutex: Arc<Mutex<Connection>>,
    ) -> Result<Vec<Track>, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        TrackRepository::get_missing_files(&conn)
    }

    /// Relinks a track to a new path.
    pub fn relink_track(
        conn_mutex: Arc<Mutex<Connection>>,
        track_id: &str,
        new_path: &str,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        let uuid = uuid::Uuid::parse_str(track_id)?;
        TrackRepository::relink_track(&conn, &uuid, new_path)
    }
}
