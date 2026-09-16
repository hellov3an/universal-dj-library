use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a connected storage device (USB, SSD, internal drive).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    pub id: String,
    pub volume_name: String,
    pub mount_path: PathBuf,
    pub filesystem: Option<String>,
    pub capacity_bytes: u64,
    pub free_space_bytes: u64,
    pub is_removable: bool,
    pub detected_libraries: Vec<DetectedDjLibrary>,
}

/// A DJ software library detected on a storage device.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedDjLibrary {
    pub adapter_id: String,
    pub adapter_name: String,
    pub library_path: PathBuf,
    pub track_count: Option<usize>,
}

/// Sync status between the universal library and a device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceSyncStatus {
    UpToDate,
    ChangesAvailable {
        added: u32,
        modified: u32,
        removed: u32,
    },
    NeverSynced,
    Disconnected,
}
