use crate::domain::error::AppError;
use crate::state::AppState;
use tauri::State;
use uuid::Uuid;

#[tauri::command]
pub async fn analyze_tracks(
    track_ids: Vec<String>,
    state: State<'_, AppState>,
) -> Result<(), AppError> {
    let uuids: Vec<Uuid> = track_ids
        .into_iter()
        .filter_map(|s| Uuid::parse_str(&s).ok())
        .collect();

    state.analysis_manager.enqueue(uuids).await
}

#[tauri::command]
pub async fn pause_analysis(state: State<'_, AppState>) -> Result<(), AppError> {
    state.analysis_manager.pause().await
}

#[tauri::command]
pub async fn resume_analysis(state: State<'_, AppState>) -> Result<(), AppError> {
    state.analysis_manager.resume().await
}

#[tauri::command]
pub async fn cancel_analysis(state: State<'_, AppState>) -> Result<(), AppError> {
    state.analysis_manager.cancel().await
}
#[derive(serde::Serialize)]
pub struct TrackAnalysisData {
    pub waveform_data: Option<Vec<u8>>,
    pub energy: Option<i64>,
    pub beatgrid_markers: Option<String>,
}

#[tauri::command]
pub async fn get_track_analysis(
    track_id: String,
    state: State<'_, AppState>,
) -> Result<TrackAnalysisData, AppError> {
    state.with_db(|conn| {
        let mut waveform_data = None;
        let mut energy = None;
        if let Ok(mut stmt) = conn.prepare("SELECT waveform_data, energy FROM analysis WHERE track_id = ?1") {
            if let Ok(mut rows) = stmt.query([&track_id]) {
                if let Ok(Some(row)) = rows.next() {
                    waveform_data = row.get(0).ok();
                    energy = row.get(1).ok();
                }
            }
        }

        let mut beatgrid_markers = None;
        if let Ok(mut stmt) = conn.prepare("SELECT markers FROM beatgrids WHERE track_id = ?1") {
            if let Ok(mut rows) = stmt.query([&track_id]) {
                if let Ok(Some(row)) = rows.next() {
                    beatgrid_markers = row.get(0).ok();
                }
            }
        }

        Ok(TrackAnalysisData {
            waveform_data,
            energy,
            beatgrid_markers,
        })
    })
}
