//! # Configuration Types
//!
//! Shared configuration type definitions and utilities.
//! Provides validation, serialization, and default value handling.

// Architecture Note: Configuration aggregation requires importing from adapters layer.
// This is an acceptable deviation for the root config type that aggregates all settings.
// The alternative (duplicating config types) would violate DRY principle.
use crate::adapters::database::DatabaseConfig;
use crate::adapters::hybrid_search::HybridSearchConfig;
use crate::infrastructure::auth::AuthConfig;
use crate::infrastructure::cache::CacheConfig;
use crate::infrastructure::daemon::DaemonConfig;
use crate::infrastructure::limits::ResourceLimitsConfig;
use crate::infrastructure::sync::SyncConfig;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::data::DataConfig;
use super::metrics::MetricsConfig;
use super::providers::{EmbeddingProviderConfig, VectorStoreProviderConfig};
use super::server::ServerConfig;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GlobalConfig {
    /// Server configuration
    #[serde(default)]
    #[validate(nested)]
    pub server: ServerConfig,
    /// Provider configurations
    #[validate(nested)]
    pub providers: GlobalProviderConfig,
    /// Metrics configuration
    #[serde(default)]
    #[validate(nested)]
    pub metrics: MetricsConfig,
    /// Sync configuration
    #[serde(default)]
    #[validate(nested)]
    pub sync: SyncConfig,
    /// Daemon configuration
    #[serde(default)]
    #[validate(nested)]
    pub daemon: DaemonConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GlobalProviderConfig {
    #[validate(nested)]
    /// Embedding
    pub embedding: EmbeddingProviderConfig,
    #[validate(nested)]
    /// Vector Store
    pub vector_store: VectorStoreProviderConfig,
}

/// Legacy provider config (maintained for backward compatibility)
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct ProviderConfig {
    #[validate(nested)]
    /// Embedding
    pub embedding: crate::domain::types::EmbeddingConfig,
    #[validate(nested)]
    /// Vector Store
    pub vector_store: crate::domain::types::VectorStoreConfig,
}

/// Main application configuration
///
/// Central configuration structure containing all settings for the MCP Context Browser.
/// Supports hierarchical configuration with validation and environment variable overrides.
///
/// ## Future Domains (v0.3.0+)
///
/// Analysis, Quality, and Git configurations are prepared for incremental feature integration.
/// These are feature-gated and default to disabled in v0.2.0.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Config {
    /// Application name
    pub name: String,
    /// Application version
    pub version: String,
    /// Server configuration (host, port, etc.)
    #[validate(nested)]
    pub server: ServerConfig,
    /// AI and vector store provider configurations
    #[validate(nested)]
    pub providers: ProviderConfig,
    /// Metrics and monitoring configuration
    #[validate(nested)]
    pub metrics: MetricsConfig,
    /// Admin interface configuration (optional - only if credentials provided)
    pub admin: Option<crate::server::admin::AdminConfig>,
    /// Authentication and authorization settings
    #[validate(nested)]
    pub auth: AuthConfig,

    /// Database configuration
    #[validate(nested)]
    pub database: DatabaseConfig,
    /// Sync coordination configuration
    #[validate(nested)]
    pub sync: SyncConfig,
    /// Background daemon configuration
    #[validate(nested)]
    pub daemon: DaemonConfig,

    /// Resource limits configuration
    #[validate(nested)]
    pub resource_limits: ResourceLimitsConfig,

    /// Advanced caching configuration
    pub cache: CacheConfig,
    #[validate(nested)]
    /// Hybrid Search
    pub hybrid_search: HybridSearchConfig,

    /// Data directory configuration (XDG standard locations)
    #[validate(nested)]
    pub data: DataConfig,

    // === Future domains (v0.3.0+) - prepared for incremental feature integration ===
    /// Analysis domain configuration (complexity, technical debt, SATD detection) - v0.3.0+
    #[cfg(feature = "analysis")]
    #[serde(default)]
    #[validate(nested)]
    pub analysis: AnalysisConfig,

    /// Quality domain configuration (quality gates, assessment) - v0.5.0+
    #[cfg(feature = "quality")]
    #[serde(default)]
    #[validate(nested)]
    pub quality: QualityConfig,

    /// Git domain configuration (git operations, repository analysis) - v0.5.0+
    #[cfg(feature = "git")]
    #[serde(default)]
    #[validate(nested)]
    pub git: GitConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            name: "MCP Context Browser".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            server: ServerConfig::default(),
            providers: ProviderConfig::default(),
            metrics: MetricsConfig::default(),
            admin: None,
            auth: AuthConfig::default(),
            database: DatabaseConfig::default(),
            sync: SyncConfig::default(),
            daemon: DaemonConfig::default(),
            resource_limits: ResourceLimitsConfig::default(),
            cache: CacheConfig::default(),
            hybrid_search: HybridSearchConfig::default(),
            data: DataConfig::default(),
            #[cfg(feature = "analysis")]
            analysis: AnalysisConfig::default(),
            #[cfg(feature = "quality")]
            quality: QualityConfig::default(),
            #[cfg(feature = "git")]
            git: GitConfig::default(),
        }
    }
}

impl Config {
    /// Get metrics port
    pub fn metrics_port(&self) -> u16 {
        self.metrics.port
    }

    /// Check if metrics are enabled
    pub fn metrics_enabled(&self) -> bool {
        self.metrics.enabled
    }

    /// Get server address string
    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.server.host, self.server.port)
    }

    /// Get metrics server address string
    pub fn metrics_addr(&self) -> String {
        format!("0.0.0.0:{}", self.metrics.port)
    }
}

// === Future Domain Configurations (v0.3.0+) ===
// These configurations are prepared for incremental feature integration.
// They are feature-gated and remain empty/placeholder in v0.2.0.

/// Analysis domain configuration (v0.3.0+)
///
/// Configuration for code analysis features including:
/// - Cyclomatic and cognitive complexity measurement
/// - Technical debt grading (TDG)
/// - Self-Admitted Technical Debt (SATD) detection
///
/// **v0.2.0**: Placeholder - no fields yet
/// **v0.3.0+**: Will contain analysis-specific settings
#[cfg(feature = "analysis")]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct AnalysisConfig {
    /// Enable analysis features (default: false in v0.2.0)
    #[serde(default)]
    pub enabled: bool,

    // Future fields (v0.3.0+):
    // pub cache_ttl: Duration,
    // pub max_file_size: u64,
    // pub complexity_thresholds: ComplexityThresholds,
    // pub tdg_weights: TdgWeights,
}

/// Quality domain configuration (v0.5.0+)
///
/// Configuration for quality assessment features including:
/// - Quality gate enforcement
/// - Metric aggregation and baselines
/// - Quality reports and visualization
///
/// **v0.2.0**: Placeholder - no fields yet
/// **v0.5.0+**: Will contain quality-specific settings
#[cfg(feature = "quality")]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct QualityConfig {
    /// Enable quality features (default: false in v0.2.0)
    #[serde(default)]
    pub enabled: bool,

    // Future fields (v0.5.0+):
    // pub quality_gates: Vec<QualityGate>,
    // pub baseline_ttl: Duration,
    // pub report_formats: Vec<ReportFormat>,
}

/// Git domain configuration (v0.5.0+)
///
/// Configuration for Git integration features including:
/// - Git repository operations
/// - Commit analysis and history search
/// - Context generation from repository metadata
///
/// **v0.2.0**: Placeholder - no fields yet
/// **v0.5.0+**: Will contain Git-specific settings
#[cfg(feature = "git")]
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct GitConfig {
    /// Enable Git features (default: false in v0.2.0)
    #[serde(default)]
    pub enabled: bool,

    // Future fields (v0.5.0+):
    // pub detect_monorepos: bool,
    // pub follow_submodules: bool,
    // pub max_history_depth: usize,
    // pub context_generation_rules: Vec<Rule>,
}
