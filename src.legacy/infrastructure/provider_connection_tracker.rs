//! Provider Connection Tracker for managing active operations during restart
//!
//! This module tracks active connections/operations per provider to enable
//! graceful connection draining during provider restarts.

use dashmap::DashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, warn};

/// Connection guard that decrements connection count when dropped
pub struct ConnectionGuard {
    provider_id: String,
    tracker: ProviderConnectionTracker,
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        self.tracker.decrement(&self.provider_id);
    }
}

/// Tracks active connections for providers
#[derive(Clone)]
pub struct ProviderConnectionTracker {
    /// Active connections per provider
    active_connections: Arc<DashMap<String, Arc<AtomicU32>>>,
}

impl ProviderConnectionTracker {
    /// Create a new connection tracker
    pub fn new() -> Self {
        Self {
            active_connections: Arc::new(DashMap::new()),
        }
    }

    /// Start tracking a connection for a provider
    pub fn track_connection(&self, provider_id: &str) -> ConnectionGuard {
        let key = provider_id.to_string();

        // Get or create atomic counter for this provider
        let counter = self
            .active_connections
            .entry(key.clone())
            .or_insert_with(|| Arc::new(AtomicU32::new(0)))
            .clone();

        // Increment connection count
        counter.fetch_add(1, Ordering::SeqCst);
        debug!("[TRACKER] Connection started for {}", provider_id);

        ConnectionGuard {
            provider_id: key,
            tracker: self.clone(),
        }
    }

    /// Decrement active connection count (called by ConnectionGuard drop)
    fn decrement(&self, provider_id: &str) {
        if let Some(entry) = self.active_connections.get(provider_id) {
            let count = entry.fetch_sub(1, Ordering::SeqCst);
            debug!(
                "[TRACKER] Connection ended for {} (remaining: {})",
                provider_id,
                count - 1
            );
        }
    }

    /// Get current active connection count
    pub fn active_count(&self, provider_id: &str) -> u32 {
        self.active_connections
            .get(provider_id)
            .map(|counter| counter.load(Ordering::SeqCst))
            .unwrap_or(0)
    }

    /// Wait for all connections to drain with optional timeout
    pub async fn wait_for_drain(&self, provider_id: &str, timeout: Duration) -> bool {
        let start = std::time::Instant::now();
        let check_interval = Duration::from_millis(100);

        loop {
            let remaining = self.active_count(provider_id);
            if remaining == 0 {
                debug!("[TRACKER] All connections drained for {}", provider_id);
                return true;
            }

            if start.elapsed() > timeout {
                warn!(
                    "[TRACKER] Timeout waiting for {} connections to drain for {}",
                    remaining, provider_id
                );
                return false;
            }

            sleep(check_interval).await;
        }
    }

    /// Force close all connections for a provider (used as fallback)
    pub fn close_all(&self, provider_id: &str) {
        if let Some(entry) = self.active_connections.get_mut(provider_id) {
            entry.store(0, Ordering::SeqCst);
        }
        debug!(
            "[TRACKER] Forced close of all connections for {}",
            provider_id
        );
    }
}

impl Default for ProviderConnectionTracker {
    fn default() -> Self {
        Self::new()
    }
}
