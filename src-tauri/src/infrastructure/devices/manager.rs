use sysinfo::Disks;
use serde::Serialize;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum DjLibraryType {
    Rekordbox,
    EngineDj,
    Serato,
    VirtualDj,
    Mixxx,
    Djay,
    None,
}

#[derive(Debug, Serialize, Clone)]
pub struct Device {
    pub name: String,
    pub path: PathBuf,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
    pub library_type: DjLibraryType,
}

pub struct DeviceManager;

impl DeviceManager {
    /// Detects all connected storage devices, particularly removable USB drives.
    pub fn get_devices() -> Vec<Device> {
        let disks = Disks::new_with_refreshed_list();
        let mut devices = Vec::new();

        for disk in disks.list() {
            let path = disk.mount_point().to_path_buf();
            let library_type = Self::detect_library(&path);

            devices.push(Device {
                name: disk.name().to_string_lossy().to_string(),
                path,
                total_space: disk.total_space(),
                available_space: disk.available_space(),
                is_removable: disk.is_removable(),
                library_type,
            });
        }

        devices
    }

    /// Detects the type of DJ library on the drive.
    fn detect_library(path: &Path) -> DjLibraryType {
        if path.join("Engine Library").exists() {
            DjLibraryType::EngineDj
        } else if path.join("VirtualDJ").exists() {
            DjLibraryType::VirtualDj
        } else if path.join("_Serato_").exists() {
            DjLibraryType::Serato
        } else if path.join("PIONEER").exists() || path.join(".PIONEER").exists() || path.join("rekordbox").exists() {
            DjLibraryType::Rekordbox
        } else {
            DjLibraryType::None
        }
    }

    /// Starts a background task to poll for hot-plugged devices and emits an event if changes occur.
    pub fn start_hotplug_detector(app: AppHandle) {
        tokio::spawn(async move {
            let mut last_devices = Self::get_devices();
            loop {
                sleep(Duration::from_secs(3)).await;
                let current_devices = Self::get_devices();
                
                // A simple comparison based on path and total_space to detect changes
                let changed = current_devices.len() != last_devices.len() ||
                    current_devices.iter().any(|curr| 
                        !last_devices.iter().any(|last| last.path == curr.path && last.total_space == curr.total_space)
                    );
                
                if changed {
                    let _ = app.emit("devices_changed", current_devices.clone());
                    last_devices = current_devices;
                }
            }
        });
    }
}
