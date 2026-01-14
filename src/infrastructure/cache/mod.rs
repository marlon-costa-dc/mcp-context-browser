//! Advanced distributed caching system with multiple backends
//!
//! Provides pluggable cache providers supporting:
//! - Moka: Local in-memory cache (single-node, default)
//! - Redis: Distributed cache (cluster deployments)
//!
//! ## Architecture
//!
//! Application code should depend on the [`CacheProvider`] trait, not concrete implementations:
//!
//! ```rust
//! use std::sync::Arc;
//! use mcp_context_browser::infrastructure::cache::{SharedCacheProvider, NullCacheProvider};
//!
//! fn accepts_any_cache(cache: SharedCacheProvider) {
//!     // Works with Moka, Redis, or Null cache
//!     println!("Backend: {}", cache.backend_type());
//! }
//!
//! // Create a null cache for testing
//! let cache: SharedCacheProvider = Arc::new(NullCacheProvider);
//! accepts_any_cache(cache);
//! ```

mod config;
mod factory;
mod provider;
mod providers;
mod queue;

// Re-export configuration types
pub use config::{
    CacheBackendConfig, CacheConfig, CacheEntry, CacheNamespaceConfig, CacheNamespacesConfig,
    CacheResult,
};

// Re-export provider trait and implementations
pub use provider::{
    CacheProvider, CacheStats, HealthStatus, NullCacheProvider, SharedCacheProvider,
};
pub use providers::moka::MokaCacheProvider;
pub use providers::redis::RedisCacheProvider;

// Re-export queue extension trait
pub use queue::CacheProviderQueue;

// Re-export factory function (only place where concrete implementations are created)
pub use factory::create_cache_provider;

use crate::domain::error::Error;

/// Convert Redis errors to domain errors in the infrastructure layer
impl From<::redis::RedisError> for Error {
    fn from(err: ::redis::RedisError) -> Self {
        Self::Cache {
            message: err.to_string(),
        }
    }
}
