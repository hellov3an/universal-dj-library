use rusqlite::Connection;

use crate::domain::error::AppError;

/// List of migrations in order. Each entry is (version, name, sql).
const MIGRATIONS: &[(u32, &str, &str)] = &[(
    1,
    "initial",
    include_str!("../../../migrations/001_initial.sql"),
)];

/// Run pending migrations on the database.
/// Uses PRAGMA user_version to track migration state.
pub fn run_migrations(conn: &Connection) -> Result<(), AppError> {
    let current_version: u32 = conn
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .unwrap_or(0);

    for (version, name, sql) in MIGRATIONS {
        if *version > current_version {
            log::info!("Running migration {}: {}", version, name);

            conn.execute_batch(sql).map_err(|e| {
                AppError::Database(format!("Migration {} ({}) failed: {}", version, name, e))
            })?;

            conn.pragma_update(None, "user_version", version)
                .map_err(|e| {
                    AppError::Database(format!(
                        "Failed to update user_version to {}: {}",
                        version, e
                    ))
                })?;

            log::info!("Migration {} ({}) completed", version, name);
        }
    }

    let final_version: u32 = conn
        .pragma_query_value(None, "user_version", |row| row.get(0))
        .unwrap_or(0);
    log::info!("Database at version {}", final_version);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::database::connection;

    #[test]
    fn test_run_migrations() {
        let temp_dir = std::env::temp_dir().join("udl_test_migrations");
        let db_path = temp_dir.join("test_migrations.db");
        let _ = std::fs::remove_dir_all(&temp_dir);

        let conn = connection::initialize(&db_path).unwrap();
        run_migrations(&conn).expect("Migrations should succeed");

        // Verify tables were created
        let track_count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='tracks'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(track_count, 1, "tracks table should exist");

        // Running migrations again should be a no-op
        run_migrations(&conn).expect("Idempotent migrations should succeed");

        // Clean up
        drop(conn);
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
