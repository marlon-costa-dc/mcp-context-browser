//! Backup operations helper module
//!
//! Provides functions for backup creation, listing, and restoration.

use crate::admin::service::types::{
    AdminError, BackupConfig, BackupInfo, BackupResult, RestoreResult,
};
use crate::infrastructure::events::SharedEventBus;

/// Create a new backup
pub fn create_backup(
    event_bus: &SharedEventBus,
    backup_config: BackupConfig,
) -> Result<BackupResult, AdminError> {
    let backup_id = format!("backup_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"));
    let path = format!("./backups/{}.tar.gz", backup_config.name);

    // Publish backup event - actual backup created asynchronously by BackupManager
    // Use list_backups() to check completion status and get actual file size
    event_bus
        .publish(crate::infrastructure::events::SystemEvent::BackupCreate { path: path.clone() })
        .map_err(|e| {
            AdminError::McpServerError(format!("Failed to publish BackupCreate event: {}", e))
        })?;

    tracing::info!(
        "[ADMIN] Backup creation initiated: {} -> {}",
        backup_config.name,
        path
    );

    // Return immediately - size_bytes is 0 until backup completes
    // Client should poll list_backups() for completion status
    Ok(BackupResult {
        backup_id,
        name: backup_config.name,
        size_bytes: 0, // Async - check list_backups() for actual size
        created_at: chrono::Utc::now(),
        path,
    })
}

/// List all available backups
pub fn list_backups() -> Result<Vec<BackupInfo>, AdminError> {
    let backups_dir = std::path::PathBuf::from("./backups");
    if !backups_dir.exists() {
        return Ok(Vec::new());
    }

    let mut backups = Vec::new();
    let entries = std::fs::read_dir(&backups_dir)
        .map_err(|e| AdminError::ConfigError(format!("Failed to read backups directory: {}", e)))?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "gz") {
            if let Some(filename) = path.file_stem().and_then(|s| s.to_str()) {
                if let Ok(metadata) = entry.metadata() {
                    let created_at = metadata
                        .created()
                        .or_else(|_| metadata.modified())
                        .map(chrono::DateTime::<chrono::Utc>::from)
                        .unwrap_or_else(|_| chrono::Utc::now());

                    backups.push(BackupInfo {
                        id: filename.to_string(),
                        name: filename.replace("_", " ").replace(".tar", ""),
                        created_at,
                        size_bytes: metadata.len(),
                        status: "completed".to_string(),
                    });
                }
            }
        }
    }

    backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(backups)
}

/// Restore from a backup (not implemented - manual restore required)
pub fn restore_backup(
    _event_bus: &SharedEventBus,
    backup_id: &str,
) -> Result<RestoreResult, AdminError> {
    let backups_dir = std::path::PathBuf::from("./backups");
    let backup_path = backups_dir.join(format!("{}.tar.gz", backup_id));

    if !backup_path.exists() {
        return Err(AdminError::ConfigError(format!(
            "Backup not found: {}",
            backup_id
        )));
    }

    // Backup restore requires manual intervention for safety
    // Automated restore could cause data corruption if done incorrectly
    tracing::info!(
        "[ADMIN] restore_backup('{}') called - returning not implemented. \
         Manual restore: tar -xzf {} -C ./data",
        backup_id,
        backup_path.display()
    );

    Err(AdminError::NotImplemented(format!(
        "Automated backup restore not implemented for safety. \
         To restore backup '{}', manually run: tar -xzf {} -C ./data",
        backup_id,
        backup_path.display()
    )))
}
