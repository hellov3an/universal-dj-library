use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Core track model — the heart of the Universal DJ Model.
/// Software-agnostic representation of a music track with all metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: Uuid,
    pub file_identity: FileIdentity,
    pub file_path: String,
    pub relative_path: Option<String>,
    pub filename: String,
    pub metadata: TrackMetadata,
    pub cue_points: Vec<CuePoint>,
    pub loops: Vec<Loop>,
    pub beatgrid: Option<Beatgrid>,
    pub tags: Vec<String>,
    pub color: Option<String>,
    pub rating: u8,
    pub bpm: Option<f64>,
    pub musical_key: Option<String>,
    pub energy: Option<u8>,
    pub date_added: DateTime<Utc>,
    pub date_modified: DateTime<Utc>,
    pub date_analyzed: Option<DateTime<Utc>>,
}


/// File identity used for matching tracks after moves/renames.
/// A track can be identified even if its path changes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileIdentity {
    pub hash: Option<String>,
    pub fingerprint: Option<String>,
    pub duration_ms: u64,
    pub file_size: u64,
    pub metadata_signature: Option<String>,
}

/// Track metadata — all standard music metadata fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrackMetadata {
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
    pub duration_ms: u64,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub format: AudioFormat,
    pub file_size: u64,
}

/// Supported audio formats.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum AudioFormat {
    Mp3,
    Flac,
    Wav,
    Aac,
    Ogg,
    Aiff,
    Alac,
    Wma,
    #[default]
    Unknown,
}

impl AudioFormat {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "mp3" => AudioFormat::Mp3,
            "flac" => AudioFormat::Flac,
            "wav" => AudioFormat::Wav,
            "aac" | "m4a" => AudioFormat::Aac,
            "ogg" => AudioFormat::Ogg,
            "aiff" | "aif" => AudioFormat::Aiff,
            "alac" => AudioFormat::Alac,
            "wma" => AudioFormat::Wma,
            _ => AudioFormat::Unknown,
        }
    }
}

/// A cue point on a track.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuePoint {
    pub id: Uuid,
    pub position_ms: f64,
    pub cue_type: CueType,
    pub name: Option<String>,
    pub color: Option<String>,
    pub hot_cue_index: Option<u8>,
}

/// Types of cue points in the universal model.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CueType {
    HotCue,
    MemoryCue,
}

/// A loop region on a track.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Loop {
    pub id: Uuid,
    pub start_ms: f64,
    pub end_ms: f64,
    pub length_beats: Option<f64>,
    pub name: Option<String>,
    pub color: Option<String>,
}

/// Beatgrid information for a track.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beatgrid {
    pub bpm: f64,
    pub offset_ms: f64,
    pub markers: Vec<BeatMarker>,
}

/// Individual beat marker for variable BPM tracks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeatMarker {
    pub position_ms: f64,
    pub beat_number: u32,
    pub bpm: f64,
}

/// Musical key representation supporting multiple notation systems.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicalKey {
    pub camelot: String,
    pub open_key: String,
    pub standard: String,
}

impl MusicalKey {
    /// Create a MusicalKey from Camelot notation (e.g., "8A").
    pub fn from_camelot(camelot: &str) -> Option<Self> {
        // Basic validation
        if camelot.len() < 2 || camelot.len() > 3 {
            return None;
        }
        Some(MusicalKey {
            camelot: camelot.to_uppercase(),
            open_key: String::new(), // TODO: convert
            standard: String::new(), // TODO: convert
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_format_from_extension() {
        assert_eq!(AudioFormat::from_extension("mp3"), AudioFormat::Mp3);
        assert_eq!(AudioFormat::from_extension("MP3"), AudioFormat::Mp3);
        assert_eq!(AudioFormat::from_extension("flac"), AudioFormat::Flac);
        assert_eq!(AudioFormat::from_extension("wav"), AudioFormat::Wav);
        assert_eq!(AudioFormat::from_extension("m4a"), AudioFormat::Aac);
        assert_eq!(AudioFormat::from_extension("aiff"), AudioFormat::Aiff);
        assert_eq!(AudioFormat::from_extension("xyz"), AudioFormat::Unknown);
    }

    #[test]
    fn test_musical_key_from_camelot() {
        let key = MusicalKey::from_camelot("8A").unwrap();
        assert_eq!(key.camelot, "8A");

        assert!(MusicalKey::from_camelot("X").is_none());
    }
}
