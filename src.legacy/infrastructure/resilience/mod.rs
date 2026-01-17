//! Resilience patterns - circuit breaker and rate limiting
//!
//! Provides unified interfaces for resilience patterns with pluggable backends:
//! - **Single-node**: tower-resilience (in-memory, fast)
//! - **Cluster mode**: Redis-backed (distributed state via CacheProvider)
//!
//! ## Architecture
//!
//! Application code depends on traits (`RateLimiterBackend`, `CircuitBreakerBackend`),
//! not concrete implementations. Backend selection happens at startup based on config.
//!
//! ```rust,no_run
//! use mcp_context_browser::infrastructure::resilience::{
//!     create_rate_limiter, RateLimiterBackendType, RateLimiterConfig,
//!     RateLimiterBackend,
//! };
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Factory creates appropriate backend based on config
//! let rate_limiter = create_rate_limiter(
//!     RateLimiterBackendType::Memory,
//!     RateLimiterConfig::default(),
//!     None,
//! );
//!
//! // Use via trait - works with any backend
//! let result = rate_limiter.check("user:123").await?;
//! if result.allowed {
//!     // proceed with request
//! }
//! # Ok(())
//! # }
//! ```

mod circuit_breaker;
mod factory;
mod rate_limiter;
mod traits;

pub use circuit_breaker::{CircuitBreakerConfig, NullCircuitBreaker, TowerCircuitBreaker};
pub use factory::{
    create_circuit_breaker, create_rate_limiter, determine_rate_limiter_backend,
    RateLimiterBackendType,
};
pub use rate_limiter::{NullRateLimiter, RateLimiterConfig, RedisRateLimiter, TowerRateLimiter};
pub use traits::{
    CircuitBreakerBackend, CircuitBreakerState, RateLimitResult, RateLimiterBackend,
    SharedCircuitBreaker, SharedRateLimiter,
};
