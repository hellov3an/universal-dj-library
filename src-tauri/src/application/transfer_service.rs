use crate::domain::error::AppError;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

/// Transfer plan item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferItem {
    pub track_id: String,
    pub source_path: String,
    pub dest_path: String,
    pub action: TransferAction,
    pub status: TransferItemStatus,
    pub file_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransferAction {
    Copy,
    Update,
    Skip,
    Remove,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransferItemStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    Verified,
}

/// Transfer plan with preview before execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferPlan {
    pub id: String,
    pub source_name: String,
    pub dest_path: String,
    pub adapter_id: String,
    pub items: Vec<TransferItem>,
    pub total_size: u64,
    pub added_count: u32,
    pub updated_count: u32,
    pub removed_count: u32,
    pub skipped_count: u32,
}

/// Transfer report after completion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferReport {
    pub plan_id: String,
    pub total_items: u32,
    pub completed: u32,
    pub failed: u32,
    pub verified: u32,
    pub duration_ms: u64,
    pub errors: Vec<String>,
}

pub struct TransferService;

impl TransferService {
    /// Creates a transfer plan by comparing source library with destination.
    pub fn create_transfer_plan(
        _conn_mutex: Arc<Mutex<Connection>>,
        _adapter_id: &str,
        dest_path: &str,
        track_ids: &[String],
    ) -> Result<TransferPlan, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let mut items = Vec::new();
        let mut total_size = 0u64;

        for track_id in track_ids {
            // In a real implementation, we'd query the DB for each track and check
            // if it already exists at the destination.
            items.push(TransferItem {
                track_id: track_id.clone(),
                source_path: String::new(), // Would be populated from DB
                dest_path: format!("{}/Contents/{}", dest_path, track_id),
                action: TransferAction::Copy,
                status: TransferItemStatus::Pending,
                file_size: 0,
            });
        }

        let added_count = items.iter().filter(|i| i.action == TransferAction::Copy).count() as u32;

        Ok(TransferPlan {
            id,
            source_name: "Universal DJ Library".to_string(),
            dest_path: dest_path.to_string(),
            adapter_id: _adapter_id.to_string(),
            items,
            total_size,
            added_count,
            updated_count: 0,
            removed_count: 0,
            skipped_count: 0,
        })
    }

    /// Executes a transfer plan, copying files with SHA-256 verification.
    pub fn execute_transfer(
        plan: &mut TransferPlan,
    ) -> Result<TransferReport, AppError> {
        let start = std::time::Instant::now();
        let mut completed = 0u32;
        let mut failed = 0u32;
        let mut verified = 0u32;
        let mut errors = Vec::new();

        for item in plan.items.iter_mut() {
            if item.action == TransferAction::Skip {
                continue;
            }

            item.status = TransferItemStatus::InProgress;

            let source = Path::new(&item.source_path);
            let dest = Path::new(&item.dest_path);

            if !source.exists() {
                let err = format!("Source file not found: {}", item.source_path);
                item.status = TransferItemStatus::Failed(err.clone());
                errors.push(err);
                failed += 1;
                continue;
            }

            // Ensure destination directory exists
            if let Some(parent) = dest.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    let err = format!("Failed to create directory: {}", e);
                    item.status = TransferItemStatus::Failed(err.clone());
                    errors.push(err);
                    failed += 1;
                    continue;
                }
            }

            // Copy file
            match std::fs::copy(source, dest) {
                Ok(_) => {
                    item.status = TransferItemStatus::Completed;
                    completed += 1;

                    // Verify with SHA-256
                    match (Self::hash_file(source), Self::hash_file(dest)) {
                        (Ok(src_hash), Ok(dst_hash)) if src_hash == dst_hash => {
                            item.status = TransferItemStatus::Verified;
                            verified += 1;
                        }
                        (Ok(_), Ok(_)) => {
                            let err = format!("Hash mismatch for {}", item.source_path);
                            item.status = TransferItemStatus::Failed(err.clone());
                            errors.push(err);
                            failed += 1;
                            completed -= 1;
                        }
                        (Err(e), _) | (_, Err(e)) => {
                            log::warn!("Could not verify hash: {}", e);
                            // File was copied but could not be verified
                        }
                    }
                }
                Err(e) => {
                    let err = format!("Copy failed: {}", e);
                    item.status = TransferItemStatus::Failed(err.clone());
                    errors.push(err);
                    failed += 1;
                }
            }
        }

        Ok(TransferReport {
            plan_id: plan.id.clone(),
            total_items: plan.items.len() as u32,
            completed,
            failed,
            verified,
            duration_ms: start.elapsed().as_millis() as u64,
            errors,
        })
    }

    /// Compute SHA-256 hash of a file.
    fn hash_file(path: &Path) -> Result<String, AppError> {
        let data = std::fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }
}
