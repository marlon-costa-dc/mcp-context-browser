use crate::core::rate_limit::RateLimitConfig;
use serde::{Deserialize, Serialize};
use validator::Validate;

/// Metrics API configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
#[serde(default)]
pub struct MetricsConfig {
    /// Port for metrics HTTP API
    #[serde(default = "default_metrics_port")]
    pub port: u16,
    /// Enable metrics collection
    #[serde(default = "default_metrics_enabled")]
    pub enabled: bool,
    /// Rate limiting configuration
    #[serde(default)]
    pub rate_limiting: RateLimitConfig,
}

fn default_metrics_port() -> u16 {
    3001
}

fn default_metrics_enabled() -> bool {
    true
}
