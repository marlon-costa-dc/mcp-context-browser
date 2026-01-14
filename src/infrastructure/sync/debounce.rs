//! Debounce Service - Manages sync timing and rate limiting
//!
//! Single Responsibility: Track sync timing and prevent excessive syncs.

use dashmap::DashMap;
use std::path::Path;
use std::time::{Duration, Instant};

/// Configuration for debouncing
#[derive(Debug, Clone)]
pub struct DebounceConfig {
    /// Minimum interval between syncs in milliseconds
    pub debounce_ms: u64,
}

impl Default for DebounceConfig {
    fn default() -> Self {
        Self {
            debounce_ms: 60 * 1000, // 60 seconds
        }
    }
}

/// Service for managing sync debouncing
pub struct DebounceService {
    config: DebounceConfig,
    last_sync_times: DashMap<String, Instant>,
}

impl Default for DebounceService {
    fn default() -> Self {
        Self::new(DebounceConfig::default())
    }
}

impl DebounceService {
    /// Create a new debounce service
    pub fn new(config: DebounceConfig) -> Self {
        Self {
            config,
            last_sync_times: DashMap::new(),
        }
    }

    /// Check if a path should be debounced (synced too recently)
    pub fn should_debounce(&self, path: &Path) -> bool {
        let path_key = path.to_string_lossy().to_string();

        if let Some(last_sync) = self.last_sync_times.get(&path_key) {
            let elapsed = last_sync.elapsed();
            let debounce_duration = Duration::from_millis(self.config.debounce_ms);

            if elapsed < debounce_duration {
                tracing::debug!(
                    "[DEBOUNCE] Blocking {} - synced {}s ago (min {}s)",
                    path.display(),
                    elapsed.as_secs(),
                    debounce_duration.as_secs()
                );
                return true;
            }
        }

        false
    }

    /// Record that a sync happened for a path
    pub fn record_sync(&self, path: &Path) {
        let path_key = path.to_string_lossy().to_string();
        self.last_sync_times.insert(path_key, Instant::now());
    }

    /// Get time since last sync for a path
    pub fn time_since_last_sync(&self, path: &Path) -> Option<Duration> {
        let path_key = path.to_string_lossy().to_string();
        self.last_sync_times.get(&path_key).map(|t| t.elapsed())
    }

    /// Clear debounce history for a path
    pub fn clear(&self, path: &Path) {
        let path_key = path.to_string_lossy().to_string();
        self.last_sync_times.remove(&path_key);
    }

    /// Clear all debounce history
    pub fn clear_all(&self) {
        self.last_sync_times.clear();
    }

    /// Get number of tracked paths
    pub fn tracked_count(&self) -> usize {
        self.last_sync_times.len()
    }

    /// Clean old timestamps older than max_age
    ///
    /// Removes entries that haven't been synced within the specified duration.
    pub fn clean_older_than(&self, max_age: Duration) {
        let now = Instant::now();
        self.last_sync_times
            .retain(|_path, timestamp| now.duration_since(*timestamp) < max_age);
    }
}
