use crate::domain::error::AppError;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Traktor NML (XML) database models.
#[derive(Debug, Deserialize)]
#[serde(rename = "NML")]
pub struct TraktorDatabase {
    #[serde(rename = "@VERSION")]
    pub version: Option<String>,
    #[serde(rename = "COLLECTION")]
    pub collection: Option<TraktorCollection>,
    #[serde(rename = "PLAYLISTS")]
    pub playlists: Option<TraktorPlaylists>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorCollection {
    #[serde(rename = "@ENTRIES")]
    pub entries: Option<String>,
    #[serde(rename = "ENTRY", default)]
    pub tracks: Vec<TraktorEntry>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorEntry {
    #[serde(rename = "@TITLE")]
    pub title: Option<String>,
    #[serde(rename = "@ARTIST")]
    pub artist: Option<String>,
    #[serde(rename = "LOCATION")]
    pub location: Option<TraktorLocation>,
    #[serde(rename = "ALBUM")]
    pub album: Option<TraktorAlbum>,
    #[serde(rename = "TEMPO")]
    pub tempo: Option<TraktorTempo>,
    #[serde(rename = "MUSICAL_KEY")]
    pub musical_key: Option<TraktorMusicalKey>,
    #[serde(rename = "CUE_V2", default)]
    pub cue_points: Vec<TraktorCuePoint>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorLocation {
    #[serde(rename = "@DIR")]
    pub dir: Option<String>,
    #[serde(rename = "@FILE")]
    pub file: Option<String>,
    #[serde(rename = "@VOLUME")]
    pub volume: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorAlbum {
    #[serde(rename = "@TITLE")]
    pub title: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorTempo {
    #[serde(rename = "@BPM")]
    pub bpm: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorMusicalKey {
    #[serde(rename = "@VALUE")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorCuePoint {
    #[serde(rename = "@NAME")]
    pub name: Option<String>,
    #[serde(rename = "@TYPE")]
    pub cue_type: Option<String>,
    #[serde(rename = "@START")]
    pub start: Option<String>,
    #[serde(rename = "@LEN")]
    pub length: Option<String>,
    #[serde(rename = "@HOTCUE")]
    pub hotcue: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorPlaylists {
    #[serde(rename = "NODE")]
    pub root: Option<TraktorPlaylistNode>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorPlaylistNode {
    #[serde(rename = "@NAME")]
    pub name: Option<String>,
    #[serde(rename = "@TYPE")]
    pub node_type: Option<String>,
    #[serde(rename = "SUBNODES")]
    pub subnodes: Option<TraktorSubnodes>,
    #[serde(rename = "PLAYLIST")]
    pub playlist: Option<TraktorPlaylistEntries>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorSubnodes {
    #[serde(rename = "NODE", default)]
    pub nodes: Vec<TraktorPlaylistNode>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorPlaylistEntries {
    #[serde(rename = "ENTRY", default)]
    pub entries: Vec<TraktorPlaylistEntry>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorPlaylistEntry {
    #[serde(rename = "PRIMARYKEY")]
    pub primary_key: Option<TraktorPrimaryKey>,
}

#[derive(Debug, Deserialize)]
pub struct TraktorPrimaryKey {
    #[serde(rename = "@KEY")]
    pub key: Option<String>,
}

pub struct TraktorAdapter;

impl TraktorAdapter {
    /// Parses a Traktor NML file into memory.
    pub fn parse_nml(path: &Path) -> Result<TraktorDatabase, AppError> {
        let content = fs::read_to_string(path)
            .map_err(|e| AppError::Filesystem(e.to_string()))?;

        let db: TraktorDatabase = quick_xml::de::from_str(&content)
            .map_err(|e| AppError::Validation(format!("Failed to parse Traktor NML: {}", e)))?;

        Ok(db)
    }

    /// Extracts the file path from a Traktor location.
    pub fn location_to_path(location: &TraktorLocation) -> Option<String> {
        let volume = location.volume.as_deref().unwrap_or("");
        let dir = location.dir.as_deref().unwrap_or("");
        let file = location.file.as_deref().unwrap_or("");

        if file.is_empty() {
            return None;
        }

        // Traktor uses /:/ as path separators on Windows
        let dir_clean = dir.replace("/:/", "/").replace("//", "/");
        Some(format!("{}{}{}", volume, dir_clean, file))
    }
}
