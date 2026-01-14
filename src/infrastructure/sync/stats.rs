//! Sync Statistics Service - Tracks sync operation metrics
//!
//! Single Responsibility: Collect and report sync statistics.

use std::sync::atomic::{AtomicU64, Ordering};

/// Sync statistics snapshot
#[derive(Debug, Clone, Default)]
pub struct SyncStats {
    /// Total sync attempts
    pub total_attempts: u64,
    /// Successful syncs
    pub successful: u64,
    /// Skipped syncs (debounced or deferred)
    pub skipped: u64,
    /// Failed syncs
    pub failed: u64,
    /// Skip rate percentage
    pub skipped_rate: f64,
}

/// Atomic statistics collector for thread-safe updates
pub struct SyncStatsCollector {
    total_attempts: AtomicU64,
    successful: AtomicU64,
    skipped: AtomicU64,
    failed: AtomicU64,
}

impl Default for SyncStatsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl SyncStatsCollector {
    /// Create a new stats collector
    pub fn new() -> Self {
        Self {
            total_attempts: AtomicU64::new(0),
            successful: AtomicU64::new(0),
            skipped: AtomicU64::new(0),
            failed: AtomicU64::new(0),
        }
    }

    /// Record a sync attempt
    pub fn record_attempt(&self) {
        self.total_attempts.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a successful sync
    pub fn record_success(&self) {
        self.successful.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a skipped sync
    pub fn record_skip(&self) {
        self.skipped.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a failed sync
    pub fn record_failure(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current statistics snapshot
    pub fn snapshot(&self) -> SyncStats {
        let total = self.total_attempts.load(Ordering::Relaxed);
        let skipped = self.skipped.load(Ordering::Relaxed);
        let skipped_rate = if total > 0 {
            (skipped as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        SyncStats {
            total_attempts: total,
            successful: self.successful.load(Ordering::Relaxed),
            skipped,
            failed: self.failed.load(Ordering::Relaxed),
            skipped_rate,
        }
    }

    /// Reset all statistics
    pub fn reset(&self) {
        self.total_attempts.store(0, Ordering::Relaxed);
        self.successful.store(0, Ordering::Relaxed);
        self.skipped.store(0, Ordering::Relaxed);
        self.failed.store(0, Ordering::Relaxed);
    }
}
