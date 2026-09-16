use crate::domain::error::AppError;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistRow {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub is_folder: bool,
    pub is_smart: bool,
    pub smart_rules: Option<String>,
    pub sort_order: i32,
    pub track_count: u64,
    pub date_created: String,
    pub date_modified: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistItemRow {
    pub id: String,
    pub playlist_id: String,
    pub track_id: String,
    pub sort_order: i32,
    pub date_added: String,
}

pub struct PlaylistRepository;

impl PlaylistRepository {
    /// Creates a new playlist.
    pub fn create(
        conn: &Connection,
        id: &str,
        name: &str,
        parent_id: Option<&str>,
        is_folder: bool,
        is_smart: bool,
        smart_rules: Option<&str>,
    ) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        let max_order: i32 = conn
            .query_row(
                "SELECT COALESCE(MAX(sort_order), 0) FROM playlists WHERE parent_id IS ?1",
                params![parent_id],
                |row| row.get(0),
            )
            .unwrap_or(0);

        conn.execute(
            "INSERT INTO playlists (id, name, parent_id, is_folder, is_smart, smart_rules, sort_order, date_created, date_modified)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                id,
                name,
                parent_id,
                is_folder as i32,
                is_smart as i32,
                smart_rules,
                max_order + 1,
                now,
                now,
            ],
        )?;
        Ok(())
    }

    /// Renames a playlist.
    pub fn rename(conn: &Connection, id: &str, name: &str) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE playlists SET name = ?1, date_modified = ?2 WHERE id = ?3",
            params![name, now, id],
        )?;
        Ok(())
    }

    /// Deletes a playlist and all its items (cascades via FK).
    pub fn delete(conn: &Connection, id: &str) -> Result<(), AppError> {
        conn.execute("DELETE FROM playlists WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Gets all playlists (flat list, frontend builds the tree).
    pub fn get_all(conn: &Connection) -> Result<Vec<PlaylistRow>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT p.*, COALESCE((SELECT COUNT(*) FROM playlist_items pi WHERE pi.playlist_id = p.id), 0) as track_count
             FROM playlists p
             ORDER BY p.sort_order",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(PlaylistRow {
                id: row.get("id")?,
                name: row.get("name")?,
                parent_id: row.get("parent_id")?,
                is_folder: row.get::<_, i32>("is_folder")? != 0,
                is_smart: row.get::<_, i32>("is_smart")? != 0,
                smart_rules: row.get("smart_rules")?,
                sort_order: row.get("sort_order")?,
                track_count: row.get("track_count")?,
                date_created: row.get("date_created")?,
                date_modified: row.get("date_modified")?,
            })
        })?;
        let mut playlists = Vec::new();
        for p in rows {
            playlists.push(p?);
        }
        Ok(playlists)
    }

    /// Gets a playlist by id.
    pub fn get_by_id(conn: &Connection, id: &str) -> Result<PlaylistRow, AppError> {
        let mut stmt = conn.prepare(
            "SELECT p.*, COALESCE((SELECT COUNT(*) FROM playlist_items pi WHERE pi.playlist_id = p.id), 0) as track_count
             FROM playlists p WHERE p.id = ?1",
        )?;
        let playlist = stmt
            .query_row(params![id], |row| {
                Ok(PlaylistRow {
                    id: row.get("id")?,
                    name: row.get("name")?,
                    parent_id: row.get("parent_id")?,
                    is_folder: row.get::<_, i32>("is_folder")? != 0,
                    is_smart: row.get::<_, i32>("is_smart")? != 0,
                    smart_rules: row.get("smart_rules")?,
                    sort_order: row.get("sort_order")?,
                    track_count: row.get("track_count")?,
                    date_created: row.get("date_created")?,
                    date_modified: row.get("date_modified")?,
                })
            })
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    AppError::NotFound(format!("Playlist {} not found", id))
                }
                other => AppError::Database(other.to_string()),
            })?;
        Ok(playlist)
    }

    /// Adds a track to a playlist.
    pub fn add_track(
        conn: &Connection,
        item_id: &str,
        playlist_id: &str,
        track_id: &str,
    ) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        let max_order: i32 = conn
            .query_row(
                "SELECT COALESCE(MAX(sort_order), 0) FROM playlist_items WHERE playlist_id = ?1",
                params![playlist_id],
                |row| row.get(0),
            )
            .unwrap_or(0);

        conn.execute(
            "INSERT OR IGNORE INTO playlist_items (id, playlist_id, track_id, sort_order, date_added)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![item_id, playlist_id, track_id, max_order + 1, now],
        )?;

        let modified = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE playlists SET date_modified = ?1 WHERE id = ?2",
            params![modified, playlist_id],
        )?;
        Ok(())
    }

    /// Removes a track from a playlist.
    pub fn remove_track(
        conn: &Connection,
        playlist_id: &str,
        track_id: &str,
    ) -> Result<(), AppError> {
        conn.execute(
            "DELETE FROM playlist_items WHERE playlist_id = ?1 AND track_id = ?2",
            params![playlist_id, track_id],
        )?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE playlists SET date_modified = ?1 WHERE id = ?2",
            params![now, playlist_id],
        )?;
        Ok(())
    }

    /// Gets all track IDs in a playlist, ordered.
    pub fn get_track_ids(conn: &Connection, playlist_id: &str) -> Result<Vec<String>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT track_id FROM playlist_items WHERE playlist_id = ?1 ORDER BY sort_order",
        )?;
        let rows = stmt.query_map(params![playlist_id], |row| row.get::<_, String>(0))?;
        let mut ids = Vec::new();
        for id in rows {
            ids.push(id?);
        }
        Ok(ids)
    }

    /// Reorder tracks in a playlist.
    pub fn reorder_tracks(
        conn: &Connection,
        playlist_id: &str,
        track_ids: &[String],
    ) -> Result<(), AppError> {
        for (i, track_id) in track_ids.iter().enumerate() {
            conn.execute(
                "UPDATE playlist_items SET sort_order = ?1 WHERE playlist_id = ?2 AND track_id = ?3",
                params![i as i32 + 1, playlist_id, track_id],
            )?;
        }
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE playlists SET date_modified = ?1 WHERE id = ?2",
            params![now, playlist_id],
        )?;
        Ok(())
    }

    /// Update smart rules for a smart playlist.
    pub fn update_smart_rules(
        conn: &Connection,
        playlist_id: &str,
        rules_json: &str,
    ) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE playlists SET smart_rules = ?1, date_modified = ?2 WHERE id = ?3",
            params![rules_json, now, playlist_id],
        )?;
        Ok(())
    }
}
