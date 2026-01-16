//! File Sync Coordinator
//!
//! Coordinates file synchronization with debouncing and change tracking.

use async_trait::async_trait;
use dashmap::DashMap;
use mcb_domain::error::Result;
use mcb_domain::ports::infrastructure::sync::{SyncCoordinator, SyncOptions, SyncResult};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// File-based sync coordinator with in-memory debounce tracking
///
/// Coordinates file synchronization operations with:
/// - Debounce tracking per codebase
/// - Change detection via filesystem scans
/// - Thread-safe state management
pub struct FileSyncCoordinator {
    /// Last sync timestamps per codebase path
    last_sync: Arc<DashMap<String, Instant>>,

    /// Files tracked per codebase
    tracked_files: Arc<DashMap<String, Vec<String>>>,
}

impl FileSyncCoordinator {
    /// Create a new file sync coordinator
    pub fn new() -> Self {
        Self {
            last_sync: Arc::new(DashMap::new()),
            tracked_files: Arc::new(DashMap::new()),
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

    /// Scan directory for code files
    async fn scan_files(root_path: &Path) -> Vec<String> {
        let mut files = Vec::new();
        let mut dirs_to_visit = vec![root_path.to_path_buf()];

        while let Some(dir) = dirs_to_visit.pop() {
            let mut entries = match tokio::fs::read_dir(&dir).await {
                Ok(entries) => entries,
                Err(_) => continue,
            };

            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                if path.is_dir() {
                    // Skip excluded directories
                    if !matches!(
                        file_name,
                        ".git" | "node_modules" | "target" | "__pycache__" | ".venv" | "venv"
                    ) && !file_name.starts_with('.')
                    {
                        dirs_to_visit.push(path);
                    }
                } else if Self::is_code_file(&path) {
                    if let Ok(rel_path) = path.strip_prefix(root_path) {
                        files.push(rel_path.to_string_lossy().to_string());
                    }
                }
            }
        }

        files
    }

    /// Check if a file is a code file based on extension
    fn is_code_file(path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| {
                matches!(
                    ext.to_lowercase().as_str(),
                    "rs" | "py"
                        | "js"
                        | "ts"
                        | "jsx"
                        | "tsx"
                        | "go"
                        | "java"
                        | "c"
                        | "cpp"
                        | "cc"
                        | "cxx"
                        | "cs"
                        | "rb"
                        | "php"
                        | "swift"
                        | "kt"
                )
            })
            .unwrap_or(false)
    }
}

impl Default for FileSyncCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl SyncCoordinator for FileSyncCoordinator {
    async fn should_debounce(&self, codebase_path: &Path) -> Result<bool> {
        let key = Self::path_key(codebase_path);

        if let Some(last) = self.last_sync.get(&key) {
            let elapsed = last.elapsed();
            // Default debounce is 60 seconds
            Ok(elapsed < Duration::from_secs(60))
        } else {
            Ok(false)
        }
    }

    async fn sync(&self, codebase_path: &Path, options: SyncOptions) -> Result<SyncResult> {
        let key = Self::path_key(codebase_path);

        // Check debounce unless forced
        if !options.force {
            if let Some(last) = self.last_sync.get(&key) {
                if last.elapsed() < options.debounce_duration {
                    return Ok(SyncResult::skipped());
                }
            }
        }

        // Get changed files
        let changed_files = self.get_changed_files(codebase_path).await?;

        // Update last sync time
        self.last_sync.insert(key.clone(), Instant::now());

        // Update tracked files
        let all_files = Self::scan_files(codebase_path).await;
        self.tracked_files.insert(key, all_files);

        Ok(SyncResult::completed(changed_files))
    }

    async fn get_changed_files(&self, codebase_path: &Path) -> Result<Vec<String>> {
        let key = Self::path_key(codebase_path);
        let current_files: std::collections::HashSet<String> =
            Self::scan_files(codebase_path).await.into_iter().collect();

        // Get previously tracked files
        let previous_files: std::collections::HashSet<String> = self
            .tracked_files
            .get(&key)
            .map(|v| v.iter().cloned().collect())
            .unwrap_or_default();

        // Find new or modified files (simple approach - just check for new files)
        // For a more accurate change detection, we'd need file hashing
        let changed: Vec<String> = current_files.difference(&previous_files).cloned().collect();

        Ok(changed)
    }

    async fn mark_synced(&self, codebase_path: &Path) -> Result<()> {
        let key = Self::path_key(codebase_path);
        self.last_sync.insert(key, Instant::now());
        Ok(())
    }

    fn tracked_file_count(&self) -> usize {
        self.tracked_files
            .iter()
            .map(|entry| entry.value().len())
            .sum()
    }
}
