//! Tests for domain ports - sync interfaces
//!
//! Migrated from src/domain/ports/sync.rs inline tests.
//! Tests the SyncCoordinator trait, SyncOptions, and SyncResult types.

use async_trait::async_trait;
use mcp_context_browser::domain::error::Result;
use mcp_context_browser::domain::ports::sync::{SyncCoordinator, SyncOptions, SyncResult};
use std::collections::HashSet;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;

/// Mock sync coordinator for testing
struct MockSyncCoordinator {
    should_debounce: AtomicBool,
    synced_paths: Mutex<HashSet<String>>,
    tracked_count: AtomicUsize,
    changed_files: Vec<String>,
}

impl MockSyncCoordinator {
    fn new() -> Self {
        Self {
            should_debounce: AtomicBool::new(false),
            synced_paths: Mutex::new(HashSet::new()),
            tracked_count: AtomicUsize::new(0),
            changed_files: vec!["changed.rs".to_string()],
        }
    }

    fn set_debounce(&self, debounce: bool) {
        self.should_debounce.store(debounce, Ordering::Relaxed);
    }
}

#[async_trait]
impl SyncCoordinator for MockSyncCoordinator {
    async fn should_debounce(&self, _codebase_path: &Path) -> Result<bool> {
        Ok(self.should_debounce.load(Ordering::Relaxed))
    }

    async fn sync(&self, codebase_path: &Path, options: SyncOptions) -> Result<SyncResult> {
        if !options.force && self.should_debounce.load(Ordering::Relaxed) {
            return Ok(SyncResult::skipped());
        }

        let path_str = codebase_path.to_string_lossy().to_string();
        self.synced_paths.lock().unwrap().insert(path_str);

        Ok(SyncResult::completed(self.changed_files.clone()))
    }

    async fn get_changed_files(&self, _codebase_path: &Path) -> Result<Vec<String>> {
        Ok(self.changed_files.clone())
    }

    async fn mark_synced(&self, codebase_path: &Path) -> Result<()> {
        let path_str = codebase_path.to_string_lossy().to_string();
        self.synced_paths.lock().unwrap().insert(path_str);
        Ok(())
    }

    fn tracked_file_count(&self) -> usize {
        self.tracked_count.load(Ordering::Relaxed)
    }
}

#[tokio::test]
async fn test_sync_coordinator_sync() {
    let coordinator = MockSyncCoordinator::new();
    let result = coordinator
        .sync(Path::new("/test"), SyncOptions::default())
        .await;

    assert!(result.is_ok());
    let sync_result = result.unwrap();
    assert!(sync_result.performed);
    assert_eq!(sync_result.files_changed, 1);
}

#[tokio::test]
async fn test_sync_coordinator_debounce() {
    let coordinator = MockSyncCoordinator::new();
    coordinator.set_debounce(true);

    let result = coordinator
        .sync(Path::new("/test"), SyncOptions::default())
        .await;

    assert!(result.is_ok());
    let sync_result = result.unwrap();
    assert!(!sync_result.performed);
}

#[tokio::test]
async fn test_sync_coordinator_force_sync() {
    let coordinator = MockSyncCoordinator::new();
    coordinator.set_debounce(true);

    let options = SyncOptions {
        force: true,
        ..Default::default()
    };

    let result = coordinator.sync(Path::new("/test"), options).await;

    assert!(result.is_ok());
    let sync_result = result.unwrap();
    assert!(sync_result.performed);
}

#[tokio::test]
async fn test_sync_coordinator_get_changed_files() {
    let coordinator = MockSyncCoordinator::new();
    let result = coordinator.get_changed_files(Path::new("/test")).await;

    assert!(result.is_ok());
    let files = result.unwrap();
    assert_eq!(files.len(), 1);
    assert!(files.contains(&"changed.rs".to_string()));
}

#[test]
fn test_sync_result_skipped() {
    let result = SyncResult::skipped();
    assert!(!result.performed);
    assert_eq!(result.files_changed, 0);
}

#[test]
fn test_sync_result_completed() {
    let result = SyncResult::completed(vec!["a.rs".to_string(), "b.rs".to_string()]);
    assert!(result.performed);
    assert_eq!(result.files_changed, 2);
}
