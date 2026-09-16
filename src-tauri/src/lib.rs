#![allow(dead_code)]

mod application;
mod commands;
mod domain;
mod infrastructure;
mod state;

use infrastructure::database::{connection, migrations};
use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            // Resolve the database path in the app data directory
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to resolve app data directory");

            let db_path = app_data_dir.join("library.db");
            log::info!("Database path: {:?}", db_path);

            // Initialize database connection
            let conn = connection::initialize(&db_path).expect("Failed to initialize database");

            // Run migrations
            migrations::run_migrations(&conn).expect("Failed to run database migrations");

            // Store in managed state
            app.manage(AppState::new(conn, app.handle().clone()));

            // Start hotplug detector
            crate::infrastructure::devices::manager::DeviceManager::start_hotplug_detector(app.handle().clone());

            log::info!("Universal DJ Library initialized");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Library
            commands::library::get_library_stats,
            commands::library::get_app_version,
            commands::library::scan_directory,
            commands::library::get_tracks,
            commands::library::get_track_by_id,
            commands::library::update_track,
            commands::library::update_tracks_bulk,
            commands::library::delete_tracks,
            // Playlists
            commands::playlist::get_playlists,
            commands::playlist::create_playlist,
            commands::playlist::create_smart_playlist,
            commands::playlist::rename_playlist,
            commands::playlist::delete_playlist,
            commands::playlist::add_track_to_playlist,
            commands::playlist::remove_track_from_playlist,
            commands::playlist::get_playlist_track_ids,
            commands::playlist::reorder_playlist,
            // Tags
            commands::tags::get_tags,
            commands::tags::create_tag,
            commands::tags::rename_tag,
            commands::tags::update_tag_color,
            commands::tags::delete_tag,
            commands::tags::assign_tag,
            commands::tags::remove_tag_from_track,
            commands::tags::get_tags_for_track,
            commands::tags::batch_assign_tag,
            // Analysis
            commands::analysis::analyze_tracks,
            commands::analysis::pause_analysis,
            commands::analysis::resume_analysis,
            commands::analysis::cancel_analysis,
            commands::analysis::get_track_analysis,
            // Devices & Transfer
            commands::devices::get_devices,
            commands::transfer::create_transfer_plan,
            commands::transfer::execute_transfer,
            // Adapters
            commands::adapters::import_rekordbox,
            commands::adapters::import_serato,
            commands::adapters::import_traktor,
            commands::adapters::import_engine_dj,
            // Health
            commands::health::get_health_report,
            commands::health::get_missing_file_tracks,
            commands::health::relink_track,
            // Settings & History
            commands::history::get_history,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
