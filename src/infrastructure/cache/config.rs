//! Cache configuration types
//!
//! Defines configuration structures for the caching system including
//! namespace-specific settings, TTLs, and cache entry metadata.

use crate::domain::error::Error;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Cache configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CacheConfig {
    /// Redis connection URL
    /// If provided and not empty, Redis (Remote) mode is used.
    /// If empty, Moka (Local) mode is used.
    pub redis_url: String,
    /// Default TTL for cache entries (seconds)
    #[validate(range(min = 1))]
    pub default_ttl_seconds: u64,
    /// Maximum cache size (number of entries) - Applies to Local Moka cache
    #[validate(range(min = 1))]
    pub max_size: usize,
    /// Whether caching is enabled
    pub enabled: bool,
    /// Cache namespaces configuration
    #[validate(nested)]
    pub namespaces: CacheNamespacesConfig,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            redis_url: String::new(),  // Default to Local (Moka) mode
            default_ttl_seconds: 3600, // 1 hour
            max_size: 10000,
            enabled: true,
            namespaces: CacheNamespacesConfig::default(),
        }
    }
}

/// Configuration for different cache namespaces
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CacheNamespacesConfig {
    /// Embedding cache settings
    #[validate(nested)]
    pub embeddings: CacheNamespaceConfig,
    /// Search results cache settings
    #[validate(nested)]
    pub search_results: CacheNamespaceConfig,
    /// Metadata cache settings
    #[validate(nested)]
    pub metadata: CacheNamespaceConfig,
    /// Provider responses cache settings
    #[validate(nested)]
    pub provider_responses: CacheNamespaceConfig,
    /// Sync batches cache settings
    #[validate(nested)]
    pub sync_batches: CacheNamespaceConfig,
}

impl Default for CacheNamespacesConfig {
    fn default() -> Self {
        Self {
            embeddings: CacheNamespaceConfig {
                ttl_seconds: 7200, // 2 hours
                max_entries: 5000,
                compression: true,
            },
            search_results: CacheNamespaceConfig {
                ttl_seconds: 1800, // 30 minutes
                max_entries: 2000,
                compression: false,
            },
            metadata: CacheNamespaceConfig {
                ttl_seconds: 3600, // 1 hour
                max_entries: 1000,
                compression: false,
            },
            provider_responses: CacheNamespaceConfig {
                ttl_seconds: 300, // 5 minutes
                max_entries: 3000,
                compression: true,
            },
            sync_batches: CacheNamespaceConfig {
                ttl_seconds: 86400, // 24 hours
                max_entries: 1000,
                compression: false,
            },
        }
    }
}

/// Configuration for a specific cache namespace
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CacheNamespaceConfig {
    /// TTL for entries in this namespace (seconds)
    #[validate(range(min = 1))]
    pub ttl_seconds: u64,
    /// Maximum number of entries for this namespace
    #[validate(range(min = 1))]
    pub max_entries: usize,
    /// Whether to compress entries
    pub compression: bool,
}

/// Cache entry with metadata
/// Used primarily for Redis serialization to preserve metadata across instances
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    /// The cached data
    pub data: T,
    /// Timestamp when entry was created
    pub created_at: u64,
    /// Timestamp when entry was last accessed
    pub accessed_at: u64,
    /// Number of times entry was accessed
    pub access_count: u64,
    /// Size of the entry in bytes
    pub size_bytes: usize,
}

/// Cache statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CacheStats {
    /// Total number of entries
    pub total_entries: usize,
    /// Total cache size in bytes
    pub total_size_bytes: usize,
    /// Cache hit count
    pub hits: u64,
    /// Cache miss count
    pub misses: u64,
    /// Cache hit ratio (0.0 to 1.0)
    pub hit_ratio: f64,
    /// Number of evictions
    pub evictions: u64,
    /// Average access time in microseconds
    pub avg_access_time_us: f64,
}

/// Cache operation result
#[derive(Debug)]
pub enum CacheResult<T> {
    /// Cache hit with data
    Hit(T),
    /// Cache miss
    Miss,
    /// Cache error
    Error(Error),
}

impl<T> CacheResult<T> {
    /// Check if this is a cache hit
    pub fn is_hit(&self) -> bool {
        matches!(self, CacheResult::Hit(_))
    }

    /// Check if this is a cache miss
    pub fn is_miss(&self) -> bool {
        matches!(self, CacheResult::Miss)
    }

    /// Get the data if it's a hit
    pub fn data(self) -> Option<T> {
        match self {
            CacheResult::Hit(data) => Some(data),
            _ => None,
        }
    }
}
