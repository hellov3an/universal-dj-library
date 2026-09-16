use crate::domain::error::AppError;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuePointRow {
    pub id: String,
    pub track_id: String,
    pub position_ms: f64,
    pub cue_type: String,
    pub name: Option<String>,
    pub color: Option<String>,
    pub hot_cue_index: Option<u8>,
    pub loop_end_ms: Option<f64>,
    pub loop_length_ms: Option<f64>,
}

pub struct CuePointRepository;

impl CuePointRepository {
    /// Creates a cue point.
    pub fn create(conn: &Connection, cue: &CuePointRow) -> Result<(), AppError> {
        conn.execute(
            "INSERT INTO cue_points (id, track_id, position_ms, cue_type, name, color, hot_cue_index, loop_end_ms, loop_length_ms)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                cue.id,
                cue.track_id,
                cue.position_ms,
                cue.cue_type,
                cue.name,
                cue.color,
                cue.hot_cue_index,
                cue.loop_end_ms,
                cue.loop_length_ms,
            ],
        )?;
        Ok(())
    }

    /// Updates a cue point.
    pub fn update(conn: &Connection, cue: &CuePointRow) -> Result<(), AppError> {
        conn.execute(
            "UPDATE cue_points SET position_ms = ?1, cue_type = ?2, name = ?3, color = ?4,
             hot_cue_index = ?5, loop_end_ms = ?6, loop_length_ms = ?7
             WHERE id = ?8",
            params![
                cue.position_ms,
                cue.cue_type,
                cue.name,
                cue.color,
                cue.hot_cue_index,
                cue.loop_end_ms,
                cue.loop_length_ms,
                cue.id,
            ],
        )?;
        Ok(())
    }

    /// Deletes a cue point.
    pub fn delete(conn: &Connection, id: &str) -> Result<(), AppError> {
        conn.execute("DELETE FROM cue_points WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Gets all cue points for a track.
    pub fn get_for_track(conn: &Connection, track_id: &str) -> Result<Vec<CuePointRow>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT * FROM cue_points WHERE track_id = ?1 ORDER BY position_ms",
        )?;
        let rows = stmt.query_map(params![track_id], |row| {
            Ok(CuePointRow {
                id: row.get("id")?,
                track_id: row.get("track_id")?,
                position_ms: row.get("position_ms")?,
                cue_type: row.get("cue_type")?,
                name: row.get("name")?,
                color: row.get("color")?,
                hot_cue_index: row.get("hot_cue_index")?,
                loop_end_ms: row.get("loop_end_ms")?,
                loop_length_ms: row.get("loop_length_ms")?,
            })
        })?;
        let mut cues = Vec::new();
        for c in rows {
            cues.push(c?);
        }
        Ok(cues)
    }

    /// Deletes all cue points for a track.
    pub fn delete_all_for_track(conn: &Connection, track_id: &str) -> Result<(), AppError> {
        conn.execute(
            "DELETE FROM cue_points WHERE track_id = ?1",
            params![track_id],
        )?;
        Ok(())
    }
}
