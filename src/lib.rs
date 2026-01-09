//! MCP Context Browser - A semantic code search server

pub mod adapters;
pub mod admin;
pub mod application;
pub mod chunking;
pub mod daemon;
pub mod domain;
pub mod infrastructure;
pub mod server;
pub mod snapshot;
pub mod sync;

// Re-export stable interfaces from domain
pub use domain::error::{Error, Result};
pub use domain::types::*;

// Re-export core systems from infrastructure
pub use infrastructure::cache::{CacheConfig, CacheManager, CacheResult, CacheStats};
pub use infrastructure::limits::{
    ResourceLimits, ResourceLimitsConfig, ResourceStats, ResourceViolation,
};
pub use infrastructure::rate_limit::{RateLimitConfig, RateLimitKey, RateLimitResult, RateLimiter};

// Re-export hybrid search from adapters
pub use adapters::hybrid_search::{BM25Params, BM25Scorer, HybridSearchConfig, HybridSearchEngine};

// Re-export routing from adapters::providers
pub use adapters::providers::routing::{
    ProviderContext, ProviderRouter, ProviderSelectionStrategy, circuit_breaker::CircuitBreaker,
    metrics::ProviderMetricsCollector,
};
