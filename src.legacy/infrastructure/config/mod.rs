/// Data configuration structures and validation
pub mod data;
/// Configuration loading and file management
pub mod loader;
/// Metrics configuration and monitoring settings
pub mod metrics;
/// Provider configuration management and validation
pub mod providers;
/// Server-specific configuration settings and validation
pub mod server;
/// Core configuration types and data structures
pub mod types;
/// Configuration file watching and hot-reload functionality
pub mod watcher;

// Re-export types
pub use data::DataConfig;
pub use loader::ConfigLoader;
pub use metrics::MetricsConfig;
pub use providers::{EmbeddingProviderConfig, VectorStoreProviderConfig};
pub use server::ServerConfig;
pub use types::{Config, GlobalConfig, GlobalProviderConfig, ProviderConfig};
pub use watcher::ConfigWatcher;
