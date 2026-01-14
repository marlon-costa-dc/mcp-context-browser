//! Configuration key constants for type-safe configuration management
//!
//! Centralizes all configuration key strings used in the admin interface.
//! This eliminates magic strings and provides IDE autocompletion support.
//!
//! # Example
//!
//! ```rust
//! use mcp_context_browser::server::admin::config_keys::{indexing, security, cache};
//!
//! // Access typed configuration keys
//! assert_eq!(indexing::CHUNK_SIZE, "indexing.chunk_size");
//! assert_eq!(security::ENABLE_AUTH, "security.enable_auth");
//! assert_eq!(cache::ENABLED, "cache.enabled");
//! ```

/// Indexing configuration keys
pub mod indexing {
    pub const CHUNK_SIZE: &str = "indexing.chunk_size";
    pub const CHUNK_OVERLAP: &str = "indexing.chunk_overlap";
    pub const MAX_FILE_SIZE: &str = "indexing.max_file_size";
    pub const SUPPORTED_EXTENSIONS: &str = "indexing.supported_extensions";
    pub const EXCLUDE_PATTERNS: &str = "indexing.exclude_patterns";
}

/// Security configuration keys
pub mod security {
    pub const ENABLE_AUTH: &str = "security.enable_auth";
    pub const RATE_LIMITING: &str = "security.rate_limiting";
    pub const MAX_REQUESTS_PER_MINUTE: &str = "security.max_requests_per_minute";
}

/// Metrics configuration keys
pub mod metrics {
    pub const ENABLED: &str = "metrics.enabled";
    pub const COLLECTION_INTERVAL: &str = "metrics.collection_interval";
    pub const RETENTION_DAYS: &str = "metrics.retention_days";
}

/// Cache configuration keys
pub mod cache {
    pub const ENABLED: &str = "cache.enabled";
    pub const BACKEND_TYPE: &str = "cache.backend_type";
    pub const MAX_ENTRIES: &str = "cache.max_entries";
    pub const TTL_SECONDS: &str = "cache.ttl_seconds";
}

/// Embedding provider configuration keys
pub mod embedding {
    pub const MODEL: &str = "embedding.model";
    pub const API_KEY: &str = "embedding.api_key";
    pub const BASE_URL: &str = "embedding.base_url";
}

/// Vector store configuration keys
pub mod vector_store {
    pub const TYPE_NAME: &str = "vector_store.type";
    pub const HOST: &str = "vector_store.host";
    pub const PORT: &str = "vector_store.port";
    pub const COLLECTION: &str = "vector_store.collection";
}
