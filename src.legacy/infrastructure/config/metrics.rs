use crate::infrastructure::resilience::RateLimiterConfig;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Metrics API configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct MetricsConfig {
    /// Port for metrics HTTP API
    #[validate(range(min = 1))]
    pub port: u16,
    /// Enable metrics collection
    pub enabled: bool,
    /// Rate limiting configuration
    pub rate_limiting: RateLimiterConfig,
    /// Whether to use distributed rate limiting (requires Redis cache)
    #[serde(default)]
    pub clustering_enabled: bool,
}

impl Default for MetricsConfig {
    fn default() -> Self {
        Self {
            port: 3001,
            enabled: true,
            rate_limiting: RateLimiterConfig::default(),
            clustering_enabled: false,
        }
    }
}
