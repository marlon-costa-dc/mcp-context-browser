//! Factory functions for resilience pattern backends
//!
//! Provides DI-based backend selection:
//! - Single-node: In-memory backends (fast, no external deps)
//! - Cluster mode: CacheProvider-backed (distributed state via Redis)

use super::circuit_breaker::{CircuitBreakerConfig, NullCircuitBreaker, TowerCircuitBreaker};
use super::rate_limiter::{NullRateLimiter, RateLimiterConfig, RedisRateLimiter, TowerRateLimiter};
use super::traits::{SharedCircuitBreaker, SharedRateLimiter};
use crate::infrastructure::cache::SharedCacheProvider;
use std::sync::Arc;

/// Rate limiter backend type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RateLimiterBackendType {
    /// In-memory (single-node)
    Memory,
    /// Redis-backed via CacheProvider (cluster)
    Redis,
    /// Disabled (null implementation)
    Disabled,
}

/// Create a rate limiter with appropriate backend
///
/// # Arguments
///
/// * `backend_type` - Which backend to use
/// * `config` - Rate limiter configuration
/// * `cache` - CacheProvider for Redis backend (ignored for Memory)
///
/// # Example
///
/// ```rust,no_run
/// use mcp_context_browser::infrastructure::resilience::{
///     create_rate_limiter, RateLimiterBackendType, RateLimiterConfig,
/// };
///
/// // Single-node deployment
/// let limiter = create_rate_limiter(
///     RateLimiterBackendType::Memory,
///     RateLimiterConfig::default(),
///     None,
/// );
/// ```
pub fn create_rate_limiter(
    backend_type: RateLimiterBackendType,
    config: RateLimiterConfig,
    cache: Option<SharedCacheProvider>,
) -> SharedRateLimiter {
    match backend_type {
        RateLimiterBackendType::Memory => {
            tracing::info!(
                backend = "memory",
                max_requests_per_window = config.max_requests_per_window,
                window_seconds = config.window_seconds,
                "Creating in-memory rate limiter"
            );
            Arc::new(TowerRateLimiter::new(config))
        }
        RateLimiterBackendType::Redis => {
            if let Some(cache) = cache {
                tracing::info!(
                    backend = "redis",
                    max_requests_per_window = config.max_requests_per_window,
                    window_seconds = config.window_seconds,
                    "Creating Redis-backed rate limiter"
                );
                Arc::new(RedisRateLimiter::new(cache, config))
            } else {
                tracing::warn!(
                    "Redis rate limiter requested but no CacheProvider available, falling back to memory"
                );
                Arc::new(TowerRateLimiter::new(config))
            }
        }
        RateLimiterBackendType::Disabled => {
            tracing::info!("Rate limiting disabled");
            Arc::new(NullRateLimiter)
        }
    }
}

/// Create a circuit breaker
///
/// Currently only supports in-memory backend. Future versions may
/// support Redis for cluster coordination.
///
/// # Arguments
///
/// * `config` - Circuit breaker configuration
/// * `enabled` - Whether circuit breaker is enabled
///
/// # Example
///
/// ```rust,no_run
/// use mcp_context_browser::infrastructure::resilience::{
///     create_circuit_breaker, CircuitBreakerConfig,
/// };
///
/// # async fn example() {
/// let cb = create_circuit_breaker(
///     CircuitBreakerConfig::new("openai-embedding"),
///     true,
/// );
///
/// if cb.is_call_permitted() {
///     // Make external call
///     let result: Result<(), &str> = Ok(());
///     match result {
///         Ok(_) => cb.record_success().await,
///         Err(_) => cb.record_failure().await,
///     }
/// }
/// # }
/// ```
pub fn create_circuit_breaker(config: CircuitBreakerConfig, enabled: bool) -> SharedCircuitBreaker {
    if enabled {
        tracing::info!(
            name = %config.name,
            failure_threshold = config.failure_threshold,
            recovery_timeout_secs = config.recovery_timeout.as_secs(),
            "Creating circuit breaker"
        );
        Arc::new(TowerCircuitBreaker::new(config))
    } else {
        tracing::info!(name = %config.name, "Circuit breaker disabled");
        Arc::new(NullCircuitBreaker::new(config.name))
    }
}

/// Determine rate limiter backend from cache configuration
///
/// If cache is Redis-backed and enabled, use Redis backend.
/// Otherwise, use in-memory backend.
pub fn determine_rate_limiter_backend(
    cache: Option<&SharedCacheProvider>,
    clustering_enabled: bool,
) -> RateLimiterBackendType {
    if !clustering_enabled {
        return RateLimiterBackendType::Memory;
    }

    match cache {
        Some(c) if c.backend_type() == "redis" => RateLimiterBackendType::Redis,
        _ => RateLimiterBackendType::Memory,
    }
}
