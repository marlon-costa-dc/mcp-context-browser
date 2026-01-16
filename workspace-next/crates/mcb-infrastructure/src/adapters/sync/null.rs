//! Null Sync Implementations
//!
//! Testing stub implementations for sync coordinator and provider.

use async_trait::async_trait;
use mcb_domain::error::Result;
use mcb_domain::ports::infrastructure::snapshot::SyncProvider;
use mcb_domain::ports::infrastructure::sync::{SyncCoordinator, SyncOptions, SyncResult};
use mcb_domain::value_objects::config::SyncBatch;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// Null sync coordinator for testing
///
/// Never debounces and always reports successful syncs with no changes.
#[derive(Default)]
pub struct NullSyncCoordinator;

impl NullSyncCoordinator {
    /// Create a new null sync coordinator
    pub fn new() -> Self {
        Self
    }

    /// Create as Arc for sharing
    pub fn new_shared() -> Arc<Self> {
        Arc::new(Self::new())
    }
}

#[async_trait]
impl SyncCoordinator for NullSyncCoordinator {
    async fn should_debounce(&self, _codebase_path: &Path) -> Result<bool> {
        Ok(false)
    }

    async fn sync(&self, _codebase_path: &Path, _options: SyncOptions) -> Result<SyncResult> {
        Ok(SyncResult::completed(Vec::new()))
    }

    async fn get_changed_files(&self, _codebase_path: &Path) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    async fn mark_synced(&self, _codebase_path: &Path) -> Result<()> {
        Ok(())
    }

    fn tracked_file_count(&self) -> usize {
        0
    }
}

/// Null sync provider for testing
///
/// Always allows sync slots and returns empty file lists.
#[derive(Default)]
pub struct NullSyncProvider;

impl NullSyncProvider {
    /// Create a new null sync provider
    pub fn new() -> Self {
        Self
    }

    /// Create as Arc for sharing
    pub fn new_shared() -> Arc<Self> {
        Arc::new(Self::new())
    }
}

#[async_trait]
impl SyncProvider for NullSyncProvider {
    async fn should_debounce(&self, _codebase_path: &Path) -> Result<bool> {
        Ok(false)
    }

    async fn update_last_sync(&self, _codebase_path: &Path) {
        // No-op
    }

    async fn acquire_sync_slot(&self, codebase_path: &Path) -> Result<Option<SyncBatch>> {
        Ok(Some(SyncBatch {
            id: "null-batch".to_string(),
            collection: codebase_path.to_string_lossy().to_string(),
            files: Vec::new(),
            priority: 0,
            created_at: 0,
        }))
    }

    async fn release_sync_slot(&self, _codebase_path: &Path, _batch: SyncBatch) -> Result<()> {
        Ok(())
    }

    async fn get_changed_files(&self, _codebase_path: &Path) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    fn sync_interval(&self) -> Duration {
        Duration::from_secs(300)
    }

    fn debounce_interval(&self) -> Duration {
        Duration::from_secs(60)
    }
}
