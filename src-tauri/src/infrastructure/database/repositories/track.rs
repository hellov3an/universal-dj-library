use crate::domain::error::AppError;
use crate::domain::track::{AudioFormat, Track, TrackMetadata, FileIdentity};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};

pub struct TrackRepository;

/// Request payload for updating track metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackMetadataUpdate {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u16>,
    pub label: Option<String>,
    pub composer: Option<String>,
    pub comment: Option<String>,
    pub grouping: Option<String>,
    pub track_number: Option<u16>,
    pub disc_number: Option<u16>,
    pub rating: Option<u8>,
    pub color: Option<String>,
}

/// A duplicate group found in the library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    pub key: String,
    pub match_type: String, // "hash" or "metadata"
    pub track_ids: Vec<String>,
}

/// Helper: map a rusqlite Row to a Track.
fn row_to_track(row: &Row<'_>) -> rusqlite::Result<Track> {
    let id_str: String = row.get("id")?;
    let id = uuid::Uuid::parse_str(&id_str).unwrap_or_default();

    let format_str: String = row.get("format")?;
    let format = AudioFormat::from_extension(&format_str);

    let bpm: Option<f64> = row.get("bpm")?;
    let musical_key: Option<String> = row.get("musical_key")?;
    let energy: Option<u8> = row.get("energy")?;
    let date_analyzed_str: Option<String> = row.get("date_analyzed")?;

    let metadata = TrackMetadata {
        title: row.get("title")?,
        artist: row.get("artist")?,
        album: row.get("album")?,
        album_artist: row.get("album_artist")?,
        genre: row.get("genre")?,
        year: row.get("year")?,
        label: row.get("label")?,
        composer: row.get("composer")?,
        comment: row.get("comment")?,
        grouping: row.get("grouping")?,
        track_number: row.get("track_number")?,
        disc_number: row.get("disc_number")?,
        duration_ms: row.get("duration_ms")?,
        bitrate: row.get("bitrate")?,
        sample_rate: row.get("sample_rate")?,
        format,
        file_size: row.get("file_size")?,
    };

    let file_identity = FileIdentity {
        hash: row.get("file_hash")?,
        fingerprint: row.get("audio_fingerprint")?,
        duration_ms: row.get("duration_ms")?,
        file_size: row.get("file_size")?,
        metadata_signature: Some(String::new()),
    };

    let date_added_str: String = row.get("date_added")?;
    let date_modified_str: String = row.get("date_modified")?;

    Ok(Track {
        id,
        file_identity,
        file_path: row.get("file_path")?,
        relative_path: row.get("relative_path")?,
        filename: row.get("filename")?,
        metadata,
        cue_points: vec![],
        loops: vec![],
        beatgrid: None,
        tags: vec![],
        color: row.get("color")?,
        rating: row.get("rating")?,
        bpm,
        musical_key,
        energy,
        date_added: chrono::DateTime::parse_from_rfc3339(&date_added_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
        date_modified: chrono::DateTime::parse_from_rfc3339(&date_modified_str)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
        date_analyzed: date_analyzed_str.and_then(|s| {
            chrono::DateTime::parse_from_rfc3339(&s)
                .map(|d| d.with_timezone(&chrono::Utc))
                .ok()
        }),
    })
}

/// Collect rows into a Vec<Track>.
fn collect_tracks(rows: rusqlite::MappedRows<'_, impl FnMut(&Row<'_>) -> rusqlite::Result<Track>>) -> Result<Vec<Track>, AppError> {
    let mut tracks = Vec::new();
    for t in rows {
        tracks.push(t?);
    }
    Ok(tracks)
}

impl TrackRepository {
    /// Inserts or updates a track in the database.
    pub fn upsert(conn: &Connection, track: &Track) -> Result<(), AppError> {
        conn.execute(
            "INSERT INTO tracks (
                id, file_path, relative_path, filename,
                title, artist, album, album_artist, genre,
                year, label, composer, comment, grouping,
                track_number, disc_number, duration_ms, bitrate, sample_rate,
                format, file_size, file_hash, audio_fingerprint,
                bpm, musical_key, energy, rating, color, artwork_hash,
                date_added, date_modified, date_analyzed
            ) VALUES (
                ?1, ?2, ?3, ?4,
                ?5, ?6, ?7, ?8, ?9,
                ?10, ?11, ?12, ?13, ?14,
                ?15, ?16, ?17, ?18, ?19,
                ?20, ?21, ?22, ?23,
                ?24, ?25, ?26, ?27, ?28, ?29,
                ?30, ?31, ?32
            )
            ON CONFLICT(id) DO UPDATE SET
                title = excluded.title,
                artist = excluded.artist,
                album = excluded.album,
                album_artist = excluded.album_artist,
                genre = excluded.genre,
                duration_ms = excluded.duration_ms,
                file_size = excluded.file_size,
                date_modified = excluded.date_modified",
            params![
                track.id.to_string(),
                track.file_path,
                track.relative_path,
                track.filename,
                track.metadata.title,
                track.metadata.artist,
                track.metadata.album,
                track.metadata.album_artist,
                track.metadata.genre,
                track.metadata.year,
                track.metadata.label,
                track.metadata.composer,
                track.metadata.comment,
                track.metadata.grouping,
                track.metadata.track_number,
                track.metadata.disc_number,
                track.metadata.duration_ms,
                track.metadata.bitrate,
                track.metadata.sample_rate,
                format!("{:?}", track.metadata.format).to_lowercase(),
                track.metadata.file_size,
                track.file_identity.hash.clone(),
                track.file_identity.fingerprint.clone(),
                track.bpm,
                track.musical_key.clone(),
                track.energy,
                track.rating,
                track.color.clone(),
                None::<String>, // Artwork hash placeholder
                track.date_added.to_rfc3339(),
                track.date_modified.to_rfc3339(),
                track.date_analyzed.map(|d| d.to_rfc3339()),
            ],
        )?;

        // Insert or update file identity
        conn.execute(
            "INSERT INTO file_identities (id, track_id, hash, fingerprint, duration_ms, file_size, metadata_sig)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(track_id) DO UPDATE SET
                hash = excluded.hash,
                fingerprint = excluded.fingerprint,
                duration_ms = excluded.duration_ms,
                file_size = excluded.file_size,
                metadata_sig = excluded.metadata_sig",
             params![
                uuid::Uuid::new_v4().to_string(),
                track.id.to_string(),
                track.file_identity.hash.clone(),
                track.file_identity.fingerprint.clone(),
                track.file_identity.duration_ms,
                track.file_identity.file_size,
                track.file_identity.metadata_signature.clone(),
             ]
        )?;

        Ok(())
    }

    /// Fetches a single track by ID.
    pub fn get_by_id(conn: &Connection, id: &uuid::Uuid) -> Result<Track, AppError> {
        let mut stmt = conn.prepare("SELECT * FROM tracks WHERE id = ?1")?;
        let track = stmt
            .query_row(params![id.to_string()], row_to_track)
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => {
                    AppError::NotFound(format!("Track {} not found", id))
                }
                other => AppError::Database(other.to_string()),
            })?;
        Ok(track)
    }

    /// Fetches all tracks
    pub fn get_all(conn: &Connection) -> Result<Vec<Track>, AppError> {
        let mut stmt =
            conn.prepare("SELECT * FROM tracks ORDER BY artist, album, track_number, title")?;
        let rows = stmt.query_map([], row_to_track)?;
        collect_tracks(rows)
    }

    /// Searches tracks using FTS5 virtual table
    pub fn search(conn: &Connection, query: &str) -> Result<Vec<Track>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT t.*
             FROM tracks_fts f
             JOIN tracks t ON f.rowid = t.rowid
             WHERE tracks_fts MATCH ?1
             ORDER BY rank",
        )?;
        let query_str = format!("\"{}*\"", query.replace("\"", ""));
        let rows = stmt.query_map(params![query_str], row_to_track)?;
        collect_tracks(rows)
    }

    /// Field-specific search (e.g. artist:Daft, genre:House, bpm:120-130)
    pub fn search_field(
        conn: &Connection,
        field: &str,
        value: &str,
    ) -> Result<Vec<Track>, AppError> {
        let (sql, param) = match field {
            "artist" => (
                "SELECT * FROM tracks WHERE artist LIKE ?1 ORDER BY artist, title",
                format!("%{}%", value),
            ),
            "genre" => (
                "SELECT * FROM tracks WHERE genre LIKE ?1 ORDER BY genre, artist, title",
                format!("%{}%", value),
            ),
            "album" => (
                "SELECT * FROM tracks WHERE album LIKE ?1 ORDER BY album, track_number",
                format!("%{}%", value),
            ),
            "label" => (
                "SELECT * FROM tracks WHERE label LIKE ?1 ORDER BY label, artist",
                format!("%{}%", value),
            ),
            "key" => (
                "SELECT * FROM tracks WHERE musical_key LIKE ?1 ORDER BY musical_key, artist",
                format!("%{}%", value),
            ),
            "bpm" => {
                // Handle bpm ranges like "120-130"
                if let Some((lo, hi)) = value.split_once('-') {
                    let lo_val: f64 = lo.trim().parse().unwrap_or(0.0);
                    let hi_val: f64 = hi.trim().parse().unwrap_or(999.0);
                    let mut stmt = conn.prepare(
                        "SELECT * FROM tracks WHERE bpm BETWEEN ?1 AND ?2 ORDER BY bpm",
                    )?;
                    let rows = stmt.query_map(params![lo_val, hi_val], row_to_track)?;
                    return collect_tracks(rows);
                } else {
                    let bpm_val: f64 = value.trim().parse().unwrap_or(0.0);
                    let mut stmt = conn.prepare(
                        "SELECT * FROM tracks WHERE bpm BETWEEN ?1 AND ?2 ORDER BY bpm",
                    )?;
                    let rows =
                        stmt.query_map(params![bpm_val - 2.0, bpm_val + 2.0], row_to_track)?;
                    return collect_tracks(rows);
                }
            }
            "rating" => (
                "SELECT * FROM tracks WHERE rating >= ?1 ORDER BY rating DESC, artist",
                value.to_string(),
            ),
            "year" => (
                "SELECT * FROM tracks WHERE year = ?1 ORDER BY artist, album, track_number",
                value.to_string(),
            ),
            _ => {
                // Fall back to FTS search
                return Self::search(conn, value);
            }
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map(params![param], row_to_track)?;
        collect_tracks(rows)
    }

    /// Updates metadata for a single track.
    pub fn update_metadata(
        conn: &Connection,
        track_id: &uuid::Uuid,
        update: &TrackMetadataUpdate,
    ) -> Result<(), AppError> {
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE tracks SET
                title = COALESCE(?1, title),
                artist = COALESCE(?2, artist),
                album = COALESCE(?3, album),
                album_artist = COALESCE(?4, album_artist),
                genre = COALESCE(?5, genre),
                year = COALESCE(?6, year),
                label = COALESCE(?7, label),
                composer = COALESCE(?8, composer),
                comment = COALESCE(?9, comment),
                grouping = COALESCE(?10, grouping),
                track_number = COALESCE(?11, track_number),
                disc_number = COALESCE(?12, disc_number),
                rating = COALESCE(?13, rating),
                color = COALESCE(?14, color),
                date_modified = ?15
            WHERE id = ?16",
            params![
                update.title,
                update.artist,
                update.album,
                update.album_artist,
                update.genre,
                update.year,
                update.label,
                update.composer,
                update.comment,
                update.grouping,
                update.track_number,
                update.disc_number,
                update.rating,
                update.color,
                now,
                track_id.to_string(),
            ],
        )?;
        Ok(())
    }

    /// Deletes tracks by IDs.
    pub fn delete_tracks(conn: &Connection, ids: &[uuid::Uuid]) -> Result<u64, AppError> {
        if ids.is_empty() {
            return Ok(0);
        }
        let placeholders: Vec<String> = ids.iter().enumerate().map(|(i, _)| format!("?{}", i + 1)).collect();
        let sql = format!(
            "DELETE FROM tracks WHERE id IN ({})",
            placeholders.join(", ")
        );
        let mut stmt = conn.prepare(&sql)?;
        let params: Vec<Box<dyn rusqlite::types::ToSql>> = ids
            .iter()
            .map(|id| Box::new(id.to_string()) as Box<dyn rusqlite::types::ToSql>)
            .collect();
        let count = stmt.execute(rusqlite::params_from_iter(params.iter().map(|b| b.as_ref())))?;
        Ok(count as u64)
    }

    /// Finds duplicate tracks by file hash.
    pub fn get_duplicates_by_hash(conn: &Connection) -> Result<Vec<DuplicateGroup>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT file_hash, GROUP_CONCAT(id) as ids
             FROM tracks
             WHERE file_hash IS NOT NULL AND file_hash != ''
             GROUP BY file_hash
             HAVING COUNT(*) > 1",
        )?;
        let rows = stmt.query_map([], |row| {
            let hash: String = row.get(0)?;
            let ids_str: String = row.get(1)?;
            Ok(DuplicateGroup {
                key: hash,
                match_type: "hash".to_string(),
                track_ids: ids_str.split(',').map(|s| s.to_string()).collect(),
            })
        })?;
        let mut groups = Vec::new();
        for g in rows {
            groups.push(g?);
        }
        Ok(groups)
    }

    /// Finds duplicate tracks by metadata (same title + artist).
    pub fn get_duplicates_by_metadata(
        conn: &Connection,
    ) -> Result<Vec<DuplicateGroup>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT title || ' - ' || artist as meta_key, GROUP_CONCAT(id) as ids
             FROM tracks
             WHERE title IS NOT NULL AND artist IS NOT NULL
             GROUP BY LOWER(title), LOWER(artist)
             HAVING COUNT(*) > 1",
        )?;
        let rows = stmt.query_map([], |row| {
            let key: String = row.get(0)?;
            let ids_str: String = row.get(1)?;
            Ok(DuplicateGroup {
                key,
                match_type: "metadata".to_string(),
                track_ids: ids_str.split(',').map(|s| s.to_string()).collect(),
            })
        })?;
        let mut groups = Vec::new();
        for g in rows {
            groups.push(g?);
        }
        Ok(groups)
    }

    /// Finds tracks whose files no longer exist on disk.
    pub fn get_missing_files(conn: &Connection) -> Result<Vec<Track>, AppError> {
        let all = Self::get_all(conn)?;
        let missing: Vec<Track> = all
            .into_iter()
            .filter(|t| !std::path::Path::new(&t.file_path).exists())
            .collect();
        Ok(missing)
    }

    /// Relinks a track to a new file path.
    pub fn relink_track(
        conn: &Connection,
        track_id: &uuid::Uuid,
        new_path: &str,
    ) -> Result<(), AppError> {
        let filename = std::path::Path::new(new_path)
            .file_name()
            .map(|f| f.to_string_lossy().to_string())
            .unwrap_or_default();
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE tracks SET file_path = ?1, filename = ?2, date_modified = ?3 WHERE id = ?4",
            params![new_path, filename, now, track_id.to_string()],
        )?;
        Ok(())
    }

    /// Gets count of tracks that haven't been analyzed yet.
    pub fn get_unanalyzed_count(conn: &Connection) -> Result<u64, AppError> {
        let count: u64 = conn
            .query_row(
                "SELECT COUNT(*) FROM tracks WHERE date_analyzed IS NULL",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);
        Ok(count)
    }

    /// Gets tracks missing metadata (no title or no artist).
    pub fn get_tracks_without_metadata(conn: &Connection) -> Result<Vec<Track>, AppError> {
        let mut stmt = conn.prepare(
            "SELECT * FROM tracks WHERE title IS NULL OR artist IS NULL ORDER BY filename",
        )?;
        let rows = stmt.query_map([], row_to_track)?;
        collect_tracks(rows)
    }
}
