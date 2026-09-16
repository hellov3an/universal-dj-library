use crate::domain::error::AppError;
use crate::infrastructure::database::repositories::tag::{TagRepository, TagRow};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};

pub struct TagService;

impl TagService {
    /// Creates a new tag.
    pub fn create_tag(
        conn_mutex: Arc<Mutex<Connection>>,
        name: &str,
        color: Option<&str>,
    ) -> Result<String, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        let id = uuid::Uuid::new_v4().to_string();
        TagRepository::create(&conn, &id, name, color)?;
        Ok(id)
    }

    /// Renames a tag.
    pub fn rename_tag(
        conn_mutex: Arc<Mutex<Connection>>,
        id: &str,
        name: &str,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        TagRepository::rename(&conn, id, name)
    }

    /// Updates tag color.
    pub fn update_tag_color(
        conn_mutex: Arc<Mutex<Connection>>,
        id: &str,
        color: Option<&str>,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        TagRepository::update_color(&conn, id, color)
    }

    /// Deletes a tag.
    pub fn delete_tag(
        conn_mutex: Arc<Mutex<Connection>>,
        id: &str,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        TagRepository::delete(&conn, id)
    }

    /// Gets all tags.
    pub fn get_all_tags(
        conn_mutex: Arc<Mutex<Connection>>,
    ) -> Result<Vec<TagRow>, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        TagRepository::get_all(&conn)
    }

    /// Assigns a tag to a track.
    pub fn assign_tag(
        conn_mutex: Arc<Mutex<Connection>>,
        track_id: &str,
        tag_id: &str,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        TagRepository::assign_to_track(&conn, track_id, tag_id)
    }

    /// Removes a tag from a track.
    pub fn remove_tag(
        conn_mutex: Arc<Mutex<Connection>>,
        track_id: &str,
        tag_id: &str,
    ) -> Result<(), AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        TagRepository::remove_from_track(&conn, track_id, tag_id)
    }

    /// Gets tags for a track.
    pub fn get_tags_for_track(
        conn_mutex: Arc<Mutex<Connection>>,
        track_id: &str,
    ) -> Result<Vec<String>, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        TagRepository::get_tags_for_track(&conn, track_id)
    }

    /// Batch assigns a tag to multiple tracks.
    pub fn batch_assign_tag(
        conn_mutex: Arc<Mutex<Connection>>,
        track_ids: &[String],
        tag_id: &str,
    ) -> Result<u64, AppError> {
        let conn = conn_mutex
            .lock()
            .map_err(|_| AppError::Database("Failed to lock database".into()))?;
        TagRepository::batch_assign(&conn, track_ids, tag_id)
    }
}
