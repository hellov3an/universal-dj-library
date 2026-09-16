use serde::Deserialize;
use std::fs;
use crate::domain::error::AppError;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct RekordboxTrack {
    #[serde(rename = "@TrackID")]
    pub id: String,
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Artist")]
    pub artist: Option<String>,
    #[serde(rename = "@Album")]
    pub album: Option<String>,
    #[serde(rename = "@TotalTime")]
    pub total_time: Option<String>,
    #[serde(rename = "@AverageBpm")]
    pub bpm: Option<String>,
    #[serde(rename = "@Tonality")]
    pub tonality: Option<String>,
    #[serde(rename = "@Location")]
    pub location: String, // URI like file://localhost/C:/Music/song.mp3
}

#[derive(Debug, Deserialize)]
pub struct Collection {
    #[serde(rename = "@Entries")]
    pub entries: Option<String>,
    #[serde(rename = "TRACK", default)]
    pub tracks: Vec<RekordboxTrack>,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistNode {
    #[serde(rename = "@Name")]
    pub name: String,
    #[serde(rename = "@Type")]
    pub node_type: String, // "0" for Playlist, "1" for Folder
    #[serde(rename = "NODE", default)]
    pub children: Vec<PlaylistNode>,
    #[serde(rename = "TRACK", default)]
    pub tracks: Vec<PlaylistTrack>,
}

#[derive(Debug, Deserialize)]
pub struct PlaylistTrack {
    #[serde(rename = "@Key")]
    pub key: String, // Matches TrackID in Collection
}

#[derive(Debug, Deserialize)]
pub struct Playlists {
    #[serde(rename = "NODE", default)]
    pub root: Option<PlaylistNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "DJ_PLAYLISTS")]
pub struct RekordboxDatabase {
    #[serde(rename = "@Version")]
    pub version: String,
    #[serde(rename = "COLLECTION")]
    pub collection: Option<Collection>,
    #[serde(rename = "PLAYLISTS")]
    pub playlists: Option<Playlists>,
}

pub struct RekordboxAdapter;

impl RekordboxAdapter {
    /// Parses a rekordbox.xml file into memory
    pub fn parse_xml(path: &Path) -> Result<RekordboxDatabase, AppError> {
        let content = fs::read_to_string(path)
            .map_err(|e| AppError::Filesystem(e.to_string()))?;
            
        let db: RekordboxDatabase = quick_xml::de::from_str(&content)
            .map_err(|e| AppError::Validation(format!("Failed to parse XML: {}", e)))?;
            
        Ok(db)
    }
}
