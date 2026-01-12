//! Maintenance operations helper module
//!
//! Provides functions for cache management, provider restart, index rebuilding, and data cleanup.

use crate::admin::service::types::{AdminError, CacheType, CleanupConfig, MaintenanceResult};
use crate::infrastructure::events::SharedEventBus;
use crate::infrastructure::logging::SharedLogBuffer;

/// Clear cache by type
pub fn clear_cache(
    event_bus: &SharedEventBus,
    cache_type: CacheType,
) -> Result<MaintenanceResult, AdminError> {
    let start_time = std::time::Instant::now();
    let namespace = match cache_type {
        CacheType::All => None,
        CacheType::QueryResults => Some("search_results".to_string()),
        CacheType::Embeddings => Some("embeddings".to_string()),
        CacheType::Indexes => Some("indexes".to_string()),
    };

    event_bus
        .publish(crate::infrastructure::events::SystemEvent::CacheClear {
            namespace: namespace.clone(),
        })
        .map_err(|e| {
            AdminError::McpServerError(format!("Failed to publish CacheClear event: {}", e))
        })?;

    Ok(MaintenanceResult {
        success: true,
        operation: format!("clear_cache_{:?}", cache_type),
        message: format!("Successfully requested cache clear for {:?}", cache_type),
        affected_items: 0,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
    })
}

/// Request provider restart (not implemented - providers auto-reconnect on failure)
pub fn restart_provider(provider_id: &str) -> Result<MaintenanceResult, AdminError> {
    // Provider hot-restart is not implemented. Providers maintain their own connections
    // and automatically reconnect on failure. This is intentional design.
    tracing::info!(
        "[ADMIN] restart_provider('{}') called - returning not implemented. \
         Providers auto-reconnect on connection failure.",
        provider_id
    );

    Err(AdminError::NotImplemented(format!(
        "Provider hot-restart not implemented for '{}'. Providers automatically \
         reconnect on connection failure - manual restart is not supported.",
        provider_id
    )))
}

/// Request index rebuild
pub fn rebuild_index(
    event_bus: &SharedEventBus,
    index_id: &str,
) -> Result<MaintenanceResult, AdminError> {
    let start_time = std::time::Instant::now();
    event_bus
        .publish(crate::infrastructure::events::SystemEvent::IndexRebuild {
            collection: Some(index_id.to_string()),
        })
        .map_err(|e| {
            AdminError::McpServerError(format!("Failed to publish IndexRebuild event: {}", e))
        })?;

    Ok(MaintenanceResult {
        success: true,
        operation: "rebuild_index".to_string(),
        message: format!("Successfully requested rebuild for index {}", index_id),
        affected_items: 0,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
    })
}

/// Clean up old data based on configuration
pub async fn cleanup_data(
    log_buffer: &SharedLogBuffer,
    cleanup_config: CleanupConfig,
) -> Result<MaintenanceResult, AdminError> {
    let start_time = std::time::Instant::now();
    let mut affected_items = 0;

    for cleanup_type in &cleanup_config.cleanup_types {
        match cleanup_type.as_str() {
            "logs" => {
                let count = log_buffer.get_all().await.len();
                log_buffer.clear();
                affected_items += count as u64;
                tracing::info!("[ADMIN] Cleared {} log entries from buffer", count);
            }
            "exports" => {
                let export_dir = std::path::PathBuf::from("./exports");
                if export_dir.exists() {
                    if let Ok(entries) = std::fs::read_dir(export_dir) {
                        for entry in entries.flatten() {
                            if let Ok(metadata) = entry.metadata() {
                                let created =
                                    metadata.created().unwrap_or(std::time::SystemTime::now());
                                let age = std::time::SystemTime::now()
                                    .duration_since(created)
                                    .unwrap_or_default();
                                if age.as_secs() > (cleanup_config.older_than_days * 86400) as u64
                                    && std::fs::remove_file(entry.path()).is_ok()
                                {
                                    affected_items += 1;
                                }
                            }
                        }
                    }
                }
            }
            unknown => {
                tracing::warn!(
                    "[ADMIN] Unknown cleanup type '{}' ignored. Valid types: logs, exports",
                    unknown
                );
            }
        }
    }

    Ok(MaintenanceResult {
        success: true,
        operation: "cleanup_data".to_string(),
        message: format!("Cleanup completed. Affected {} items.", affected_items),
        affected_items,
        execution_time_ms: start_time.elapsed().as_millis() as u64,
    })
}
