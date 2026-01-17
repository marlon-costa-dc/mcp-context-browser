//! Resilience pattern traits
//!
//! Abstract interfaces for rate limiting and circuit breaking.
//! Implementations can use tower-resilience (single-node) or Redis (cluster).

use crate::domain::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Rate limit check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitResult {
    /// Whether the request is allowed
    pub allowed: bool,
    /// Remaining requests in current window
    pub remaining: u32,
    /// Seconds until window resets
    pub reset_in_seconds: u64,
    /// Current request count in window
    pub current_count: u32,
    /// Total limit (max requests per window)
    pub limit: u32,
}

impl RateLimitResult {
    /// Create a result indicating the request is allowed
    pub fn allowed(remaining: u32, limit: u32) -> Self {
        Self {
            allowed: true,
            remaining,
            reset_in_seconds: 60,
            current_count: limit - remaining,
            limit,
        }
    }

    /// Create a result indicating the request is denied
    pub fn denied(reset_in_seconds: u64, limit: u32) -> Self {
        Self {
            allowed: false,
            remaining: 0,
            reset_in_seconds,
            current_count: limit,
            limit,
        }
    }

    /// Create a result for when rate limiting is disabled
    pub fn unlimited() -> Self {
        Self {
            allowed: true,
            remaining: u32::MAX,
            reset_in_seconds: 0,
            current_count: 0,
            limit: u32::MAX,
        }
    }
}

/// Abstract rate limiter backend
///
/// Implementations:
/// - `TowerRateLimiter`: tower-resilience based (single-node, in-memory)
/// - `RedisRateLimiter`: Redis-backed (cluster, distributed state)
#[async_trait]
pub trait RateLimiterBackend: Send + Sync {
    /// Check if a request is allowed for the given key
    ///
    /// Keys typically include:
    /// - `ip:{address}` for IP-based limiting
    /// - `user:{id}` for user-based limiting
    /// - `api:{key}` for API key limiting
    async fn check(&self, key: &str) -> Result<RateLimitResult>;

    /// Reset the rate limit for a key (admin function)
    async fn reset(&self, key: &str) -> Result<()>;

    /// Check if rate limiting is enabled
    fn is_enabled(&self) -> bool;

    /// Get the backend type for logging
    fn backend_type(&self) -> &'static str;
}

/// Shared rate limiter type
pub type SharedRateLimiter = Arc<dyn RateLimiterBackend>;

/// Circuit breaker backend trait
///
/// Implementations:
/// - `TowerCircuitBreaker`: tower-resilience based (single-node)
/// - Future: Redis-backed for cluster coordination
#[async_trait]
pub trait CircuitBreakerBackend: Send + Sync {
    /// Check if the circuit is allowing calls
    fn is_call_permitted(&self) -> bool;

    /// Record a successful call
    async fn record_success(&self);

    /// Record a failed call
    async fn record_failure(&self);

    /// Get current circuit state
    fn state(&self) -> CircuitBreakerState;

    /// Get the circuit breaker name/id
    fn name(&self) -> &str;

    /// Get backend type for logging
    fn backend_type(&self) -> &'static str;
}

/// Circuit breaker state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CircuitBreakerState {
    /// Circuit is closed, calls are allowed
    Closed,
    /// Circuit is open, calls are blocked
    Open,
    /// Circuit is testing if service recovered
    HalfOpen,
}

impl std::fmt::Display for CircuitBreakerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed => write!(f, "closed"),
            Self::Open => write!(f, "open"),
            Self::HalfOpen => write!(f, "half-open"),
        }
    }
}

/// Shared circuit breaker type
pub type SharedCircuitBreaker = Arc<dyn CircuitBreakerBackend>;
