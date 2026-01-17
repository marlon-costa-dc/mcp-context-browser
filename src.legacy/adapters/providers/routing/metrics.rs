//! Metrics Collection Module
//!
//! This module provides metrics collection using established prometheus and metrics crates,
//! following SOLID principles with proper separation of concerns.

use crate::domain::error::Result;
use metrics::{counter, gauge, histogram};
use shaku::Interface;
use std::collections::HashMap;
use tracing::{debug, info};

/// Trait for provider metrics collection
pub trait ProviderMetricsCollectorTrait: Interface + Send + Sync {
    /// Record a provider selection event with the chosen strategy
    ///
    /// # Arguments
    /// * `provider_id` - The ID of the selected provider
    /// * `strategy` - The selection strategy used (e.g., "priority", "round-robin")
    fn record_provider_selection(&self, provider_id: &str, strategy: &str);

    /// Record the response time for a provider operation
    ///
    /// # Arguments
    /// * `provider_id` - The ID of the provider
    /// * `operation` - The operation type (e.g., "embedding", "search")
    /// * `duration_seconds` - Time taken for the operation in seconds
    fn record_response_time(&self, provider_id: &str, operation: &str, duration_seconds: f64);

    /// Record a request with its outcome status
    ///
    /// # Arguments
    /// * `provider_id` - The ID of the provider
    /// * `operation` - The operation type
    /// * `status` - The outcome status (e.g., "success", "error", "timeout")
    fn record_request(&self, provider_id: &str, operation: &str, status: &str);

    /// Record an error occurrence
    ///
    /// # Arguments
    /// * `provider_id` - The ID of the provider where the error occurred
    /// * `error_type` - The type of error (e.g., "timeout", "rate_limit", "internal")
    fn record_error(&self, provider_id: &str, error_type: &str);

    /// Record cost incurred for provider usage
    ///
    /// # Arguments
    /// * `provider_id` - The ID of the provider
    /// * `amount` - The cost amount
    /// * `currency` - The currency code (e.g., "USD", "EUR")
    fn record_cost(&self, provider_id: &str, amount: f64, currency: &str);

    /// Update the count of active connections for a provider
    ///
    /// # Arguments
    /// * `provider_id` - The ID of the provider
    /// * `count` - Current number of active connections
    fn update_active_connections(&self, provider_id: &str, count: i64);

    /// Record a circuit breaker state change
    ///
    /// # Arguments
    /// * `provider_id` - The ID of the provider
    /// * `state` - New circuit breaker state ("closed", "open", "half-open")
    fn record_circuit_breaker_state(&self, provider_id: &str, state: &str);

    /// Record provider health status and score
    ///
    /// # Arguments
    /// * `provider_id` - The ID of the provider
    /// * `status` - Health status ("healthy", "unhealthy", "unknown")
    /// * `score` - Health score between 0.0 and 1.0
    fn record_provider_health(&self, provider_id: &str, status: &str, score: f64);

    /// Get a summary of all collected metrics
    ///
    /// # Returns
    /// A summary containing aggregated metrics data
    fn get_summary(&self) -> MetricsSummary;
}

/// Metrics collector for provider operations
pub struct ProviderMetricsCollector {}

impl ProviderMetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Result<Self> {
        info!("Initializing provider metrics collector");
        Ok(Self {})
    }
}

impl ProviderMetricsCollectorTrait for ProviderMetricsCollector {
    /// Record provider selection
    fn record_provider_selection(&self, provider_id: &str, strategy: &str) {
        counter!("mcp_provider_selections_total", "provider" => provider_id.to_string(), "strategy" => strategy.to_string()).increment(1);
        debug!(
            "Recorded provider selection: {} with strategy {}",
            provider_id, strategy
        );
    }

    /// Record response time for an operation
    fn record_response_time(&self, provider_id: &str, operation: &str, duration_seconds: f64) {
        histogram!("mcp_provider_response_time_seconds", "provider" => provider_id.to_string(), "operation" => operation.to_string()).record(duration_seconds);
        gauge!("mcp_provider_last_response_time", "provider" => provider_id.to_string(), "operation" => operation.to_string()).set(duration_seconds);
        debug!(
            "Recorded response time: {}s for {}:{}",
            duration_seconds, provider_id, operation
        );
    }

    /// Record request outcome
    fn record_request(&self, provider_id: &str, operation: &str, status: &str) {
        counter!("mcp_provider_requests_total", "provider" => provider_id.to_string(), "operation" => operation.to_string(), "status" => status.to_string()).increment(1);
        debug!(
            "Recorded request: {}:{} status={}",
            provider_id, operation, status
        );
    }

    /// Record error
    fn record_error(&self, provider_id: &str, error_type: &str) {
        counter!("mcp_provider_errors_total", "provider" => provider_id.to_string(), "error_type" => error_type.to_string()).increment(1);
        debug!("Recorded error: {} type={}", provider_id, error_type);
    }

    /// Record cost
    fn record_cost(&self, provider_id: &str, amount: f64, currency: &str) {
        counter!("mcp_provider_cost_total", "provider" => provider_id.to_string(), "currency" => currency.to_string()).increment(amount as u64);
        gauge!("mcp_provider_current_cost", "provider" => provider_id.to_string(), "currency" => currency.to_string()).set(amount);
        debug!("Recorded cost: {} {} for {}", amount, currency, provider_id);
    }

    /// Update active connections
    fn update_active_connections(&self, provider_id: &str, count: i64) {
        gauge!("mcp_provider_active_connections", "provider" => provider_id.to_string())
            .set(count as f64);
        debug!("Updated active connections: {} for {}", count, provider_id);
    }

    /// Record circuit breaker state change
    fn record_circuit_breaker_state(&self, provider_id: &str, state: &str) {
        counter!("mcp_circuit_breaker_state_changes_total", "provider" => provider_id.to_string(), "state" => state.to_string()).increment(1);
        gauge!("mcp_circuit_breaker_current_state", "provider" => provider_id.to_string()).set(
            match state {
                "closed" => 0.0,
                "open" => 1.0,
                "half-open" => 0.5,
                _ => -1.0,
            },
        );
        debug!(
            "Recorded circuit breaker state change: {} -> {}",
            provider_id, state
        );
    }

    /// Record provider health status
    fn record_provider_health(&self, provider_id: &str, status: &str, score: f64) {
        gauge!("mcp_provider_health_score", "provider" => provider_id.to_string()).set(score);
        counter!("mcp_provider_health_checks_total", "provider" => provider_id.to_string(), "status" => status.to_string()).increment(1);
        debug!(
            "Recorded provider health: {} status={} score={}",
            provider_id, status, score
        );
    }

    /// Get summary of collected metrics
    fn get_summary(&self) -> MetricsSummary {
        // In a real implementation, this would query the metrics registry
        // For now, return empty summary
        MetricsSummary {
            total_requests: 0,
            error_rate: 0.0,
            average_latency: 0.0,
            total_cost: 0.0,
            provider_distribution: HashMap::new(),
        }
    }
}

/// Summary of collected metrics
#[derive(Debug, Clone, serde::Serialize)]
pub struct MetricsSummary {
    /// Total number of requests processed across all providers
    pub total_requests: u64,
    /// Error rate as a fraction (0.0 to 1.0) of total requests
    pub error_rate: f64,
    /// Average response latency in seconds
    pub average_latency: f64,
    /// Total cost incurred across all providers
    pub total_cost: f64,
    /// Distribution of requests per provider (provider_id -> request_count)
    pub provider_distribution: HashMap<String, u64>,
}
