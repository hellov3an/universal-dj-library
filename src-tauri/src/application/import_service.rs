use crate::domain::error::AppError;
use crate::infrastructure::adapters::rekordbox::RekordboxAdapter;
use crate::infrastructure::adapters::serato::SeratoAdapter;
use crate::infrastructure::database::repositories::track::TrackRepository;
use crate::domain::track::Track;
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct ImportService;

impl ImportService {
    /// Imports tracks from a Rekordbox XML file
    pub fn import_from_rekordbox(
        conn_mutex: Arc<Mutex<Connection>>,
        xml_path: &Path,
    ) -> Result<usize, AppError> {
        let db = RekordboxAdapter::parse_xml(xml_path)?;
        
        let conn = conn_mutex.lock().map_err(|_| {
            AppError::Database("Failed to lock database".into())
        })?;
        
        let mut count = 0;
        
        if let Some(collection) = db.collection {
            for rb_track in collection.tracks {
                // Convert Rekordbox URI to standard Path
                // E.g. "file://localhost/C:/Music/song.mp3" -> "C:\Music\song.mp3"
                let file_path = rb_track.location.replace("file://localhost/", "").replace("%20", " ");
                let bpm = rb_track.bpm.and_then(|b| b.parse::<f64>().ok());
                
                use crate::domain::track::{FileIdentity, TrackMetadata, AudioFormat};
                let id = uuid::Uuid::new_v4();
                
                let track = Track {
                    id,
                    file_identity: FileIdentity {
                        hash: None, // We don't have the hash yet
                        fingerprint: None,
                        duration_ms: 0,
                        file_size: 0,
                        metadata_signature: None,
                    },
                    file_path: file_path.clone(),
                    relative_path: None,
                    filename: std::path::Path::new(&file_path)
                        .file_name()
                        .map(|s| s.to_string_lossy().to_string())
                        .unwrap_or_default(),
                    metadata: TrackMetadata {
                        title: Some(rb_track.name.clone()),
                        artist: rb_track.artist.clone(),
                        album: rb_track.album.clone(),
                        ..Default::default()
                    },
                    cue_points: vec![],
                    loops: vec![],
                    beatgrid: None,
                    tags: vec![],
                    color: None,
                    rating: 0,
                    bpm,
                    musical_key: rb_track.tonality.clone(),
                    energy: None,
                    date_added: chrono::Utc::now(),
                    date_modified: chrono::Utc::now(),
                    date_analyzed: None,
                };

                if let Ok(_) = TrackRepository::upsert(&conn, &track) {
                    count += 1;
                }
            }
        }
        
        Ok(count)
    }

    /// Imports tracks from a Serato folder
    pub fn import_from_serato(
        _conn_mutex: Arc<Mutex<Connection>>,
        serato_folder: &Path,
    ) -> Result<usize, AppError> {
        let db_path = serato_folder.join("database V2");
        if !db_path.exists() {
            return Err(AppError::Validation("database V2 not found".into()));
        }

        let _db = SeratoAdapter::parse_database(&db_path)?;
        
        // MVP: Just returning success since we mocked the parser
        Ok(0)
    }
}
