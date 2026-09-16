use crate::domain::error::AppError;
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// VirtualDJ uses an XML database format.
#[derive(Debug, Deserialize)]
#[serde(rename = "VirtualFolder")]
pub struct VirtualDjDatabase {
    #[serde(rename = "Song", default)]
    pub songs: Vec<VirtualDjSong>,
}

#[derive(Debug, Deserialize)]
pub struct VirtualDjSong {
    #[serde(rename = "@FilePath")]
    pub file_path: Option<String>,
    #[serde(rename = "@Title")]
    pub title: Option<String>,
    #[serde(rename = "@Artist")]
    pub artist: Option<String>,
    #[serde(rename = "@Album")]
    pub album: Option<String>,
    #[serde(rename = "@Genre")]
    pub genre: Option<String>,
    #[serde(rename = "@Bpm")]
    pub bpm: Option<String>,
    #[serde(rename = "@Key")]
    pub key: Option<String>,
}

pub struct VirtualDjAdapter;

impl VirtualDjAdapter {
    /// Parses a VirtualDJ database XML file.
    pub fn parse_database(path: &Path) -> Result<VirtualDjDatabase, AppError> {
        let content = fs::read_to_string(path)
            .map_err(|e| AppError::Filesystem(e.to_string()))?;

        let db: VirtualDjDatabase = quick_xml::de::from_str(&content)
            .map_err(|e| AppError::Validation(format!("Failed to parse VirtualDJ XML: {}", e)))?;

        Ok(db)
    }

    /// Detects if a VirtualDJ database exists at the standard location.
    pub fn detect(root: &Path) -> bool {
        let db_path = root.join("VirtualDJ").join("database.xml");
        db_path.exists()
    }
}
