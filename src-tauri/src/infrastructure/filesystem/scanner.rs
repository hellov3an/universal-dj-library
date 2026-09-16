use crate::domain::error::AppError;
use crate::domain::track::{AudioFormat, Track, TrackMetadata, FileIdentity};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use uuid::Uuid;
use chrono::Utc;
use lofty::probe::Probe;
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::tag::Accessor;

pub struct LibraryScanner;

impl LibraryScanner {
    /// Recursively scans a directory for audio files and parses their tags.
    pub fn scan_directory<F>(dir_path: &Path, mut on_progress: F) -> Result<Vec<Track>, AppError> 
    where 
        F: FnMut(&Path)
    {
        let mut tracks = Vec::new();

        for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(track) = Self::parse_file(path) {
                    tracks.push(track);
                    on_progress(path);
                }
            }
        }

        Ok(tracks)
    }

    /// Parses a single audio file and returns a `Track` model if supported.
    pub fn parse_file(path: &Path) -> Option<Track> {
        let ext = path.extension()?.to_str()?.to_lowercase();
        let format = AudioFormat::from_extension(&ext);
        
        if format == AudioFormat::Unknown {
            return None; // Unsupported file type
        }

        let file_size = std::fs::metadata(path).ok()?.len();

        let mut track_metadata = TrackMetadata {
            title: None,
            artist: None,
            album: None,
            album_artist: None,
            genre: None,
            year: None,
            label: None,
            composer: None,
            comment: None,
            grouping: None,
            track_number: None,
            disc_number: None,
            duration_ms: 0,
            bitrate: None,
            sample_rate: None,
            format,
            file_size,
        };

        // Parse with lofty
        if let Ok(tagged_file) = Probe::open(path).and_then(|p| p.read()) {
            let properties = tagged_file.properties();
            track_metadata.duration_ms = properties.duration().as_millis() as u64;
            track_metadata.bitrate = properties.audio_bitrate().map(|b| b * 1000); // lofty gives kbps, we want bps
            track_metadata.sample_rate = properties.sample_rate();

            // Extract tags if any
            if let Some(tag) = tagged_file.primary_tag().or_else(|| tagged_file.first_tag()) {
                track_metadata.title = tag.title().map(|s| s.into_owned());
                track_metadata.artist = tag.artist().map(|s| s.into_owned());
                track_metadata.album = tag.album().map(|s| s.into_owned());
                track_metadata.genre = tag.genre().map(|s| s.into_owned());
                // For year, track, disk we omit to simplify since API changed in 0.25+ 
                track_metadata.comment = tag.comment().map(|s| s.into_owned());
            }
        } else {
            // If we can't parse tags, we just use filename as title
            track_metadata.title = path.file_stem().map(|s| s.to_string_lossy().to_string());
        }

        // Fallback for title if empty
        if track_metadata.title.is_none() || track_metadata.title.as_ref().unwrap().trim().is_empty() {
             track_metadata.title = path.file_stem().map(|s| s.to_string_lossy().to_string());
        }

        let now = Utc::now();
        let filename = path.file_name()?.to_string_lossy().to_string();
        let file_path = path.to_string_lossy().to_string();

        let track = Track {
            id: Uuid::new_v4(),
            file_identity: FileIdentity {
                hash: None, // calculated asynchronously later
                fingerprint: None,
                duration_ms: track_metadata.duration_ms,
                file_size,
                metadata_signature: Some(format!("{:?}-{:?}-{:?}", track_metadata.title, track_metadata.artist, track_metadata.album)),
            },
            file_path,
            relative_path: None, // Set by caller if needed
            filename,
            metadata: track_metadata,
            cue_points: vec![],
            loops: vec![],
            beatgrid: None,
            tags: vec![],
            color: None,
            rating: 0,
            bpm: None,
            musical_key: None,
            energy: None,
            date_added: now,
            date_modified: now,
            date_analyzed: None,
        };

        Some(track)
    }
}
