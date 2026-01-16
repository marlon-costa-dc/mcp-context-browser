//! Default Sync Provider
//!
//! Sync provider implementation with slot management and file tracking.

use async_trait::async_trait;
use dashmap::DashMap;
use mcb_domain::error::Result;
use mcb_domain::ports::infrastructure::snapshot::SyncProvider;
use mcb_domain::value_objects::config::SyncBatch;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

/// Default sync provider with in-memory state
///
/// Manages sync slots and tracks file changes.
pub struct DefaultSyncProvider {
    /// Last sync timestamps per codebase
    last_sync: Arc<DashMap<String, Instant>>,

    /// Active sync slots (codebase -> batch)
    active_slots: Arc<DashMap<String, SyncBatch>>,

    /// Sync interval
    sync_interval: Duration,

    /// Debounce interval
    debounce_interval: Duration,
}

impl DefaultSyncProvider {
    /// Create a new default sync provider
    pub fn new() -> Self {
        Self {
            last_sync: Arc::new(DashMap::new()),
            active_slots: Arc::new(DashMap::new()),
            sync_interval: Duration::from_secs(300),
            debounce_interval: Duration::from_secs(60),
        }
    }

    /// Create with custom intervals
    pub fn with_intervals(sync_interval: Duration, debounce_interval: Duration) -> Self {
        Self {
            last_sync: Arc::new(DashMap::new()),
            active_slots: Arc::new(DashMap::new()),
            sync_interval,
            debounce_interval,
        }
    }

    /// Create as Arc for sharing
    pub fn new_shared() -> Arc<Self> {
        Arc::new(Self::new())
    }

    /// Normalize path to string for consistent key usage
    fn path_key(path: &Path) -> String {
        path.canonicalize()
            .unwrap_or_else(|_| path.to_path_buf())
            .to_string_lossy()
            .to_string()
    }

    /// Get current timestamp
    fn current_timestamp() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }
}

impl Default for DefaultSyncProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SyncProvider for DefaultSyncProvider {
    async fn should_debounce(&self, codebase_path: &Path) -> Result<bool> {
        let key = Self::path_key(codebase_path);

        if let Some(last) = self.last_sync.get(&key) {
            Ok(last.elapsed() < self.debounce_interval)
        } else {
            Ok(false)
        }
    }

    async fn update_last_sync(&self, codebase_path: &Path) {
        let key = Self::path_key(codebase_path);
        self.last_sync.insert(key, Instant::now());
    }

    async fn acquire_sync_slot(&self, codebase_path: &Path) -> Result<Option<SyncBatch>> {
        let key = Self::path_key(codebase_path);

        // Check if slot is already taken
        if self.active_slots.contains_key(&key) {
            return Ok(None);
        }

        // Create new batch
        let batch = SyncBatch {
            id: Uuid::new_v4().to_string(),
            collection: key.clone(),
            files: Vec::new(),
            priority: 0,
            created_at: Self::current_timestamp(),
        };

        self.active_slots.insert(key, batch.clone());
        Ok(Some(batch))
    }

    async fn release_sync_slot(&self, codebase_path: &Path, _batch: SyncBatch) -> Result<()> {
        let key = Self::path_key(codebase_path);
        self.active_slots.remove(&key);
        self.update_last_sync(codebase_path).await;
        Ok(())
    }

    async fn get_changed_files(&self, codebase_path: &Path) -> Result<Vec<String>> {
        // Simple implementation - scan for code files
        let mut files = Vec::new();
        let mut dirs_to_visit = vec![codebase_path.to_path_buf()];

        while let Some(dir) = dirs_to_visit.pop() {
            let mut entries = match tokio::fs::read_dir(&dir).await {
                Ok(entries) => entries,
                Err(_) => continue,
            };

            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if path.is_dir() {
                    if !matches!(
                        file_name,
                        ".git" | "node_modules" | "target" | "__pycache__"
                    ) && !file_name.starts_with('.')
                    {
                        dirs_to_visit.push(path);
                    }
                } else {
                    // Check for code file extensions
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        if matches!(
                            ext.to_lowercase().as_str(),
                            "rs" | "py" | "js" | "ts" | "go" | "java" | "c" | "cpp" | "rb"
                        ) {
                            if let Ok(rel_path) = path.strip_prefix(codebase_path) {
                                files.push(rel_path.to_string_lossy().to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(files)
    }

    fn sync_interval(&self) -> Duration {
        self.sync_interval
    }

    fn debounce_interval(&self) -> Duration {
        self.debounce_interval
    }
}
