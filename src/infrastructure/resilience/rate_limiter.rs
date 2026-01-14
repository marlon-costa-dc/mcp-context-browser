//! Rate limiter implementations
//!
//! - `TowerRateLimiter`: In-memory sliding window (single-node)
//! - `RedisRateLimiter`: Redis-backed (cluster mode via CacheProvider)

use super::traits::{RateLimitResult, RateLimiterBackend};
use crate::domain::error::{Error, Result};
use crate::infrastructure::cache::SharedCacheProvider;
use crate::infrastructure::constants::{
    RATE_LIMIT_BURST_ALLOWANCE, RATE_LIMIT_DEFAULT_MAX_REQUESTS, RATE_LIMIT_WINDOW_SECONDS,
};
use async_trait::async_trait;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Rate limiter configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimiterConfig {
    /// Maximum requests per window
    pub max_requests_per_window: u32,
    /// Window duration in seconds
    pub window_seconds: u64,
    /// Burst allowance (additional requests beyond max)
    pub burst_allowance: u32,
    /// Whether rate limiting is enabled
    pub enabled: bool,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            max_requests_per_window: RATE_LIMIT_DEFAULT_MAX_REQUESTS,
            window_seconds: RATE_LIMIT_WINDOW_SECONDS,
            burst_allowance: RATE_LIMIT_BURST_ALLOWANCE,
            enabled: true,
        }
    }
}

/// Sliding window entry for in-memory rate limiting
#[derive(Debug)]
struct WindowEntry {
    /// Request timestamps in current window
    timestamps: Vec<Instant>,
    /// Window start time
    window_start: Instant,
}

impl WindowEntry {
    fn new() -> Self {
        Self {
            timestamps: Vec::new(),
            window_start: Instant::now(),
        }
    }
}

/// In-memory rate limiter using sliding window algorithm
///
/// Suitable for single-node deployments. Uses DashMap for lock-free
/// concurrent access.
pub struct TowerRateLimiter {
    /// Sliding windows per key
    windows: Arc<DashMap<String, WindowEntry>>,
    /// Configuration
    config: RateLimiterConfig,
    /// Enabled flag (atomic for lock-free reads)
    enabled: AtomicBool,
}

impl TowerRateLimiter {
    /// Create a new in-memory rate limiter
    pub fn new(config: RateLimiterConfig) -> Self {
        let enabled = config.enabled;
        Self {
            windows: Arc::new(DashMap::new()),
            config,
            enabled: AtomicBool::new(enabled),
        }
    }

    /// Get the effective limit (max + burst)
    fn effective_limit(&self) -> u32 {
        self.config.max_requests_per_window + self.config.burst_allowance
    }

    /// Clean old entries from a window
    fn clean_window(entry: &mut WindowEntry, window_duration: Duration) {
        let now = Instant::now();
        let cutoff = now - window_duration;

        // Remove timestamps older than the window
        entry.timestamps.retain(|&ts| ts > cutoff);

        // Update window start if needed
        if entry.window_start < cutoff {
            entry.window_start = now;
        }
    }
}

#[async_trait]
impl RateLimiterBackend for TowerRateLimiter {
    async fn check(&self, key: &str) -> Result<RateLimitResult> {
        if !self.enabled.load(Ordering::Relaxed) {
            return Ok(RateLimitResult::unlimited());
        }

        let window_duration = Duration::from_secs(self.config.window_seconds);
        let limit = self.effective_limit();
        let now = Instant::now();

        let mut entry = self
            .windows
            .entry(key.to_string())
            .or_insert_with(WindowEntry::new);

        // Clean old entries
        Self::clean_window(&mut entry, window_duration);

        let current_count = entry.timestamps.len() as u32;

        if current_count < limit {
            // Allow request, record timestamp
            entry.timestamps.push(now);
            let remaining = limit - current_count - 1;

            Ok(RateLimitResult {
                allowed: true,
                remaining,
                reset_in_seconds: self.config.window_seconds,
                current_count: current_count + 1,
                limit,
            })
        } else {
            // Deny request
            let reset_in = if let Some(&oldest) = entry.timestamps.first() {
                let elapsed = now.duration_since(oldest);
                window_duration.saturating_sub(elapsed).as_secs()
            } else {
                self.config.window_seconds
            };

            Ok(RateLimitResult {
                allowed: false,
                remaining: 0,
                reset_in_seconds: reset_in,
                current_count,
                limit,
            })
        }
    }

    async fn reset(&self, key: &str) -> Result<()> {
        self.windows.remove(key);
        Ok(())
    }

    fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    fn backend_type(&self) -> &'static str {
        "memory"
    }
}

/// Redis-backed rate limiter for cluster deployments
///
/// Uses CacheProvider for distributed state, enabling rate limit
/// coordination across multiple nodes.
pub struct RedisRateLimiter {
    /// Cache provider (Redis in cluster mode)
    cache: SharedCacheProvider,
    /// Configuration
    config: RateLimiterConfig,
    /// Cache namespace for rate limit keys
    namespace: String,
}

impl RedisRateLimiter {
    /// Create a new Redis-backed rate limiter
    pub fn new(cache: SharedCacheProvider, config: RateLimiterConfig) -> Self {
        Self {
            cache,
            config,
            namespace: "rate_limit".to_string(),
        }
    }

    /// Get the effective limit (max + burst)
    fn effective_limit(&self) -> u32 {
        self.config.max_requests_per_window + self.config.burst_allowance
    }
}

/// Stored rate limit state in cache
#[derive(Debug, Serialize, Deserialize)]
struct CachedRateLimitState {
    /// Request count in current window
    count: u32,
    /// Window start timestamp (unix seconds)
    window_start: u64,
}

#[async_trait]
impl RateLimiterBackend for RedisRateLimiter {
    async fn check(&self, key: &str) -> Result<RateLimitResult> {
        if !self.config.enabled {
            return Ok(RateLimitResult::unlimited());
        }

        let limit = self.effective_limit();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let window_start = now - (now % self.config.window_seconds);

        // Get current state from cache
        let state: Option<CachedRateLimitState> = match self.cache.get(&self.namespace, key).await {
            Ok(Some(data)) => serde_json::from_slice(&data).ok(),
            Ok(None) => None,
            Err(e) => {
                tracing::warn!("Cache read error for rate limit key {}: {}", key, e);
                None
            }
        };

        let (current_count, is_new_window) = match state {
            Some(s) if s.window_start == window_start => (s.count, false),
            _ => (0, true),
        };

        if current_count < limit {
            // Allow request, increment count
            let new_state = CachedRateLimitState {
                count: current_count + 1,
                window_start,
            };

            let data =
                serde_json::to_vec(&new_state).map_err(|e| Error::internal(e.to_string()))?;

            // TTL = 2x window to handle edge cases
            let ttl = Duration::from_secs(self.config.window_seconds * 2);

            if let Err(e) = self.cache.set(&self.namespace, key, data, ttl).await {
                tracing::warn!("Cache write error for rate limit key {}: {}", key, e);
            }

            let remaining = limit - current_count - 1;
            let reset_in = if is_new_window {
                self.config.window_seconds
            } else {
                self.config.window_seconds - (now - window_start)
            };

            Ok(RateLimitResult {
                allowed: true,
                remaining,
                reset_in_seconds: reset_in,
                current_count: current_count + 1,
                limit,
            })
        } else {
            // Deny request
            let reset_in = self.config.window_seconds - (now - window_start);

            Ok(RateLimitResult {
                allowed: false,
                remaining: 0,
                reset_in_seconds: reset_in,
                current_count,
                limit,
            })
        }
    }

    async fn reset(&self, key: &str) -> Result<()> {
        self.cache.delete(&self.namespace, key).await
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }

    fn backend_type(&self) -> &'static str {
        "redis"
    }
}

/// Null rate limiter - always allows (for testing/disabled scenarios)
pub struct NullRateLimiter;

#[async_trait]
impl RateLimiterBackend for NullRateLimiter {
    async fn check(&self, _key: &str) -> Result<RateLimitResult> {
        Ok(RateLimitResult::unlimited())
    }

    async fn reset(&self, _key: &str) -> Result<()> {
        Ok(())
    }

    fn is_enabled(&self) -> bool {
        false
    }

    fn backend_type(&self) -> &'static str {
        "null"
    }
}
