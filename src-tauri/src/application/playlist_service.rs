use crate::domain::error::AppError;
use crate::infrastructure::database::repositories::playlist::PlaylistRepository;
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub struct PlaylistService;

impl PlaylistService {
    /// Creates a new playlist.
    pub fn create_playlist(
        conn_mutex: Arc<Mutex<Connection>>,
        name: &str,
        parent_id: Option<&str>,
        is_folder: bool,
    ) -> Result<String, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        let id = uuid::Uuid::new_v4().to_string();
        PlaylistRepository::create(&conn, &id, name, parent_id, is_folder, false, None)?;
        Ok(id)
    }

    /// Creates a new smart playlist.
    pub fn create_smart_playlist(
        conn_mutex: Arc<Mutex<Connection>>,
        name: &str,
        parent_id: Option<&str>,
        rules_json: &str,
    ) -> Result<String, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        let id = uuid::Uuid::new_v4().to_string();
        PlaylistRepository::create(&conn, &id, name, parent_id, false, true, Some(rules_json))?;
        Ok(id)
    }

    /// Renames a playlist.
    pub fn rename_playlist(
        conn_mutex: Arc<Mutex<Connection>>,
        id: &str,
        name: &str,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        PlaylistRepository::rename(&conn, id, name)
    }

    /// Deletes a playlist.
    pub fn delete_playlist(
        conn_mutex: Arc<Mutex<Connection>>,
        id: &str,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        PlaylistRepository::delete(&conn, id)
    }

    /// Gets all playlists.
    pub fn get_all_playlists(
        conn_mutex: Arc<Mutex<Connection>>,
    ) -> Result<Vec<crate::infrastructure::database::repositories::playlist::PlaylistRow>, AppError>
    {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        PlaylistRepository::get_all(&conn)
    }

    /// Adds a track to a playlist.
    pub fn add_track_to_playlist(
        conn_mutex: Arc<Mutex<Connection>>,
        playlist_id: &str,
        track_id: &str,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        let item_id = uuid::Uuid::new_v4().to_string();
        PlaylistRepository::add_track(&conn, &item_id, playlist_id, track_id)
    }

    /// Removes a track from a playlist.
    pub fn remove_track_from_playlist(
        conn_mutex: Arc<Mutex<Connection>>,
        playlist_id: &str,
        track_id: &str,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        PlaylistRepository::remove_track(&conn, playlist_id, track_id)
    }

    /// Gets track IDs in a playlist.
    pub fn get_playlist_track_ids(
        conn_mutex: Arc<Mutex<Connection>>,
        playlist_id: &str,
    ) -> Result<Vec<String>, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        PlaylistRepository::get_track_ids(&conn, playlist_id)
    }

    /// Reorders tracks in a playlist.
    pub fn reorder_playlist(
        conn_mutex: Arc<Mutex<Connection>>,
        playlist_id: &str,
        track_ids: &[String],
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        PlaylistRepository::reorder_tracks(&conn, playlist_id, track_ids)
    }

    /// Updates smart rules.
    pub fn update_smart_rules(
        conn_mutex: Arc<Mutex<Connection>>,
        playlist_id: &str,
        rules_json: &str,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        PlaylistRepository::update_smart_rules(&conn, playlist_id, rules_json)
    }
}
