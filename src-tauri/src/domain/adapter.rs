use serde::{Deserialize, Serialize};

use super::device::StorageDevice;
use super::error::AppError;

/// Maturity status of a DJ adapter.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdapterStatus {
    Experimental,
    Alpha,
    Beta,
    Stable,
    Unsupported,
}

/// Declares the features a DJ adapter supports.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DjCapabilities {
    pub import: bool,
    pub export: bool,
    pub usb: bool,
    pub playlists: bool,
    pub smart_playlists: bool,
    pub playlist_folders: bool,
    pub hot_cues: bool,
    pub max_hot_cues: Option<u8>,
    pub memory_cues: bool,
    pub loops: bool,
    pub max_loops: Option<u8>,
    pub beatgrid: bool,
    pub ratings: bool,
    pub colors: bool,
    pub comments: bool,
    pub artwork: bool,
    pub waveforms: bool,
    pub tags: bool,
}

impl DjCapabilities {
    /// Returns a capabilities set with everything disabled.
    pub fn none() -> Self {
        DjCapabilities {
            import: false,
            export: false,
            usb: false,
            playlists: false,
            smart_playlists: false,
            playlist_folders: false,
            hot_cues: false,
            max_hot_cues: None,
            memory_cues: false,
            loops: false,
            max_loops: None,
            beatgrid: false,
            ratings: false,
            colors: false,
            comments: false,
            artwork: false,
            waveforms: false,
            tags: false,
        }
    }
}

/// Result of detecting a DJ library on a device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionResult {
    pub detected: bool,
    pub adapter_id: String,
    pub adapter_name: String,
    pub library_path: Option<String>,
    pub track_count: Option<usize>,
}

/// Summary info about a DJ adapter for the frontend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdapterInfo {
    pub id: String,
    pub name: String,
    pub status: AdapterStatus,
    pub capabilities: DjCapabilities,
}

/// Trait defining the interface all DJ adapters must implement.
/// Each DJ software (Rekordbox, Serato, Traktor, etc.) provides
/// a concrete implementation.
pub trait DjAdapter: Send + Sync {
    /// Unique identifier for this adapter (e.g., "rekordbox").
    fn id(&self) -> &str;

    /// Human-readable display name (e.g., "Rekordbox").
    fn name(&self) -> &str;

    /// Detect if this DJ software's library exists on a device.
    fn detect(&self, device: &StorageDevice) -> Result<DetectionResult, AppError>;

    /// Declare what this adapter can import/export.
    fn capabilities(&self) -> DjCapabilities;

    /// Adapter maturity level.
    fn status(&self) -> AdapterStatus;

    /// Return summary info about this adapter.
    fn info(&self) -> AdapterInfo {
        AdapterInfo {
            id: self.id().to_string(),
            name: self.name().to_string(),
            status: self.status(),
            capabilities: self.capabilities(),
        }
    }
}
