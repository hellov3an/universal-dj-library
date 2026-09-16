use crate::domain::error::AppError;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagRow {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub track_count: u64,
}

pub struct TagRepository;

impl TagRepository {
    /// Creates a new tag.
    pub fn create(conn: &Connection, id: &str, name: &str, color: Option<&str>) -> Result<(), AppError> {
        conn.execute(
            "INSERT INTO tags (id, name, color) VALUES (?1, ?2, ?3)",
            params![id, name, color],
        )?;
        Ok(())
    }

    /// Renames a tag.
    pub fn rename(conn: &Connection, id: &str, name: &str) -> Result<(), AppError> {
        conn.execute("UPDATE tags SET name = ?1 WHERE id = ?2", params![name, id])?;
        Ok(())
    }

    /// Updates tag color.
    pub fn update_color(conn: &Connection, id: &str, color: Option<&str>) -> Result<(), AppError> {
        conn.execute("UPDATE tags SET color = ?1 WHERE id = ?2", params![color, id])?;
        Ok(())
    }

    /// Deletes a tag (cascade removes track_tags).
    pub fn delete(conn: &Connection, id: &str) -> Result<(), AppError> {
        conn.execute("DELETE FROM tags WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Gets all tags with their track counts.
    pub fn get_all(conn: &Connection) -> Result<Vec<TagRow>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.color,
                    COALESCE((SELECT COUNT(*) FROM track_tags tt WHERE tt.tag_id = t.id), 0) as track_count
             FROM tags t
             ORDER BY t.name",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(TagRow {
                id: row.get(0)?,
                name: row.get(1)?,
                color: row.get(2)?,
                track_count: row.get(3)?,
            })
        })?;
        let mut tags = Vec::new();
        for t in rows {
            tags.push(t?);
        }
        Ok(tags)
    }

    /// Assigns a tag to a track.
    pub fn assign_to_track(conn: &Connection, track_id: &str, tag_id: &str) -> Result<(), AppError> {
        conn.execute(
            "INSERT OR IGNORE INTO track_tags (track_id, tag_id) VALUES (?1, ?2)",
            params![track_id, tag_id],
        )?;
        Ok(())
    }

    /// Removes a tag from a track.
    pub fn remove_from_track(conn: &Connection, track_id: &str, tag_id: &str) -> Result<(), AppError> {
        conn.execute(
            "DELETE FROM track_tags WHERE track_id = ?1 AND tag_id = ?2",
            params![track_id, tag_id],
        )?;
        Ok(())
    }

    /// Gets all tag IDs for a track.
    pub fn get_tags_for_track(conn: &Connection, track_id: &str) -> Result<Vec<String>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT tag_id FROM track_tags WHERE track_id = ?1",
        )?;
        let rows = stmt.query_map(params![track_id], |row| row.get::<_, String>(0))?;
        let mut ids = Vec::new();
        for id in rows {
            ids.push(id?);
        }
        Ok(ids)
    }

    /// Gets all track IDs that have a given tag.
    pub fn get_tracks_for_tag(conn: &Connection, tag_id: &str) -> Result<Vec<String>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT track_id FROM track_tags WHERE tag_id = ?1",
        )?;
        let rows = stmt.query_map(params![tag_id], |row| row.get::<_, String>(0))?;
        let mut ids = Vec::new();
        for id in rows {
            ids.push(id?);
        }
        Ok(ids)
    }

    /// Batch assign a tag to multiple tracks.
    pub fn batch_assign(conn: &Connection, track_ids: &[String], tag_id: &str) -> Result<u64, AppError> {
        let mut count = 0u64;
        for track_id in track_ids {
            conn.execute(
                "INSERT OR IGNORE INTO track_tags (track_id, tag_id) VALUES (?1, ?2)",
                params![track_id, tag_id],
            )?;
            count += 1;
        }
        Ok(count)
    }
}
