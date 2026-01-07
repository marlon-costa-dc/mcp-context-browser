//! MCP Context Browser - A semantic code search server

pub mod chunking;
pub mod config;
pub mod core;
pub mod daemon;
pub mod factory;
pub mod metrics;
pub mod providers;
pub mod registry;
pub mod server;
pub mod services;
pub mod snapshot;
pub mod sync;

// Re-export rate limiting system
pub use core::rate_limit::{RateLimiter, RateLimitConfig, RateLimitKey, RateLimitResult};

// Re-export resource limits system
pub use core::limits::{ResourceLimits, ResourceLimitsConfig, ResourceStats, ResourceViolation};

// Re-export advanced caching system
pub use core::cache::{CacheManager, CacheConfig, CacheStats, CacheResult};

// Re-export multi-provider strategy system
pub use providers::routing::{
    ProviderRouter, ProviderSelectionStrategy, ProviderContext,
    circuit_breaker::CircuitBreaker,
    metrics::ProviderMetricsCollector
};
