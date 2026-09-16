use crate::domain::error::AppError;
use crate::infrastructure::audio::analyzer::{AudioAnalyzer, AnalysisResult};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;
use uuid::Uuid;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct AnalysisProgress {
    pub current_track: String,
    pub total: usize,
    pub completed: usize,
    pub state: String, // "analyzing", "paused", "cancelled", "finished"
}

pub enum AnalysisCommand {
    Enqueue(Vec<Uuid>),
    Pause,
    Resume,
    Cancel,
}

pub struct AnalysisManager {
    sender: mpsc::Sender<AnalysisCommand>,
}

impl AnalysisManager {
    pub fn new(conn_mutex: Arc<Mutex<Connection>>, app: AppHandle) -> Self {
        let (tx, mut rx) = mpsc::channel::<AnalysisCommand>(100);
        
        let mut queue: Vec<Uuid> = Vec::new();
        let mut is_paused = false;
        let mut completed = 0;

        // Background worker loop
        tokio::spawn(async move {
            loop {
                // Try to receive commands
                if is_paused || queue.is_empty() {
                    // Block until we get a command
                    if let Some(cmd) = rx.recv().await {
                        match cmd {
                            AnalysisCommand::Enqueue(ids) => {
                                queue.extend(ids);
                                // Don't unpause automatically if paused
                            }
                            AnalysisCommand::Pause => is_paused = true,
                            AnalysisCommand::Resume => is_paused = false,
                            AnalysisCommand::Cancel => {
                                queue.clear();
                                completed = 0;
                                let _ = app.emit("analysis_progress", AnalysisProgress {
                                    current_track: "".into(),
                                    total: 0,
                                    completed: 0,
                                    state: "cancelled".into(),
                                });
                            }
                        }
                    } else {
                        break; // Channel closed
                    }
                } else {
                    // We have items and are not paused. 
                    // Use try_recv to process any pending commands without blocking.
                    match rx.try_recv() {
                        Ok(cmd) => match cmd {
                            AnalysisCommand::Enqueue(ids) => queue.extend(ids),
                            AnalysisCommand::Pause => { is_paused = true; continue; },
                            AnalysisCommand::Resume => is_paused = false,
                            AnalysisCommand::Cancel => {
                                queue.clear();
                                completed = 0;
                                let _ = app.emit("analysis_progress", AnalysisProgress {
                                    current_track: "".into(),
                                    total: 0,
                                    completed: 0,
                                    state: "cancelled".into(),
                                });
                                continue;
                            }
                        },
                        Err(mpsc::error::TryRecvError::Empty) => {} // No commands pending
                        Err(mpsc::error::TryRecvError::Disconnected) => break, // Channel closed
                    }
                }

                if !is_paused && !queue.is_empty() {
                    let track_id = queue.remove(0);
                    let total = completed + queue.len() + 1;
                    
                    let _ = app.emit("analysis_progress", AnalysisProgress {
                        current_track: track_id.to_string(),
                        total,
                        completed,
                        state: "analyzing".into(),
                    });

                    // Perform analysis (await blocking)
                    let _ = Self::analyze_track_inner(conn_mutex.clone(), track_id).await;
                    
                    completed += 1;

                    if queue.is_empty() {
                        let _ = app.emit("analysis_progress", AnalysisProgress {
                            current_track: "".into(),
                            total,
                            completed,
                            state: "finished".into(),
                        });
                        completed = 0; // reset
                    }
                }
            }
        });

        Self { sender: tx }
    }

    pub async fn enqueue(&self, ids: Vec<Uuid>) -> Result<(), AppError> {
        self.sender.send(AnalysisCommand::Enqueue(ids)).await
            .map_err(|_| AppError::Validation("Analysis worker died".into()))
    }

    pub async fn pause(&self) -> Result<(), AppError> {
        self.sender.send(AnalysisCommand::Pause).await
            .map_err(|_| AppError::Validation("Analysis worker died".into()))
    }

    pub async fn resume(&self) -> Result<(), AppError> {
        self.sender.send(AnalysisCommand::Resume).await
            .map_err(|_| AppError::Validation("Analysis worker died".into()))
    }

    pub async fn cancel(&self) -> Result<(), AppError> {
        self.sender.send(AnalysisCommand::Cancel).await
            .map_err(|_| AppError::Validation("Analysis worker died".into()))
    }

    async fn analyze_track_inner(
        conn_mutex: Arc<Mutex<Connection>>,
        track_id: Uuid,
    ) -> Result<AnalysisResult, AppError> {
        // 1. Fetch the track
        let track = {
            let conn = conn_mutex.lock().map_err(|_| AppError::Database(rusqlite::Error::InvalidQuery.to_string()))?;
            let mut stmt = conn.prepare("SELECT file_path FROM tracks WHERE id = ?1")?;
            let mut rows = stmt.query([track_id.to_string()])?;
            if let Some(row) = rows.next()? {
                let path: String = row.get(0)?;
                std::path::PathBuf::from(path)
            } else {
                return Err(AppError::Validation("Track not found".into()));
            }
        };

        // 2. Perform the analysis
        let result = AudioAnalyzer::analyze_file(&track).await?;

        // 3. Save the result back to the database
        let now = chrono::Utc::now().to_rfc3339();
        {
            let conn = conn_mutex.lock().map_err(|_| AppError::Database(rusqlite::Error::InvalidQuery.to_string()))?;
            conn.execute_batch("BEGIN TRANSACTION;")?;
            
            conn.execute(
                "UPDATE tracks SET 
                 bpm = ?1, 
                 musical_key = ?2, 
                 energy = ?3, 
                 date_analyzed = ?4 
                 WHERE id = ?5",
                rusqlite::params![
                    result.bpm,
                    result.key_camelot,
                    result.energy,
                    now,
                    track_id.to_string()
                ],
            )?;

            let analysis_id = Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO analysis (id, track_id, waveform_data, energy, analysis_version)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(track_id) DO UPDATE SET
                 waveform_data = excluded.waveform_data,
                 energy = excluded.energy,
                 analysis_version = excluded.analysis_version",
                rusqlite::params![
                    analysis_id,
                    track_id.to_string(),
                    result.waveform,
                    result.energy,
                    "1.0"
                ]
            )?;

            let beatgrid_id = Uuid::new_v4().to_string();
            let markers_json = serde_json::to_string(&result.beatgrid)
                .unwrap_or_else(|_| "[]".to_string());
                
            conn.execute(
                "INSERT INTO beatgrids (id, track_id, bpm, offset_ms, markers)
                 VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(track_id) DO UPDATE SET
                 bpm = excluded.bpm,
                 offset_ms = excluded.offset_ms,
                 markers = excluded.markers",
                rusqlite::params![
                    beatgrid_id,
                    track_id.to_string(),
                    result.bpm,
                    0.0,
                    markers_json
                ]
            )?;

            conn.execute_batch("COMMIT;")?;
        }

        Ok(result)
    }
}
