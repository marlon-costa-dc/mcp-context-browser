//! Cost Tracking Module
//!
//! This module provides cost tracking capabilities using established patterns
//! and libraries, following SOLID principles with proper separation of concerns.

use crate::domain::error::{Error, Result};
use dashmap::DashMap;
use shaku::Interface;
use std::sync::Arc;
use tracing::{debug, info, warn};

/// Cost information for a provider operation
#[derive(Debug, Clone)]
pub struct ProviderCost {
    /// Unique identifier for the provider
    pub provider_id: String,
    /// Type of operation (e.g., "embedding", "search", "storage")
    pub operation_type: String,
    /// Cost per unit of consumption
    pub cost_per_unit: f64,
    /// Type of unit being charged (e.g., "token", "request", "GB")
    pub unit_type: String,
    /// Optional free tier limit before charges apply
    pub free_tier_limit: Option<u64>,
    /// Currency code for the cost (e.g., "USD", "EUR")
    pub currency: String,
}

impl ProviderCost {
    /// Calculate cost for given units
    pub fn calculate_cost(&self, units: u64) -> f64 {
        if let Some(free_limit) = self.free_tier_limit {
            if units <= free_limit {
                0.0
            } else {
                (units - free_limit) as f64 * self.cost_per_unit
            }
        } else {
            units as f64 * self.cost_per_unit
        }
    }

    /// Calculate efficiency score (0.0 = expensive, 1.0 = cheap)
    pub fn efficiency_score(&self) -> f64 {
        let max_reasonable_cost = match self.unit_type.as_str() {
            "token" => 0.0001, // $0.0001 per token (reasonable max)
            "request" => 1.0,  // $1 per request (reasonable max)
            "GB" => 1.0,       // $1 per GB (reasonable max)
            "second" => 0.1,   // $0.10 per second (reasonable max)
            _ => 1.0,          // Default
        };

        (max_reasonable_cost - self.cost_per_unit.min(max_reasonable_cost)) / max_reasonable_cost
    }
}

/// Usage metrics for tracking consumption
#[derive(Debug, Clone, Default)]
pub struct UsageMetrics {
    /// Total units consumed across all periods
    pub total_units: u64,
    /// Total cost incurred across all periods
    pub total_cost: f64,
    /// Units consumed in the current billing period
    pub current_period_units: u64,
    /// Cost incurred in the current billing period
    pub current_period_cost: f64,
    /// Average cost per unit across all usage
    pub avg_cost_per_unit: f64,
    /// Number of operations performed
    pub operation_count: u64,
    /// Timestamp of the last usage
    pub last_usage: Option<chrono::DateTime<chrono::Utc>>,
}

/// Cost tracking configuration
#[derive(Debug, Clone)]
pub struct CostTrackerConfig {
    /// Whether to enable budget limit checking
    pub enable_budget_limits: bool,
    /// Default currency for cost calculations
    pub default_currency: String,
}

impl CostTrackerConfig {
    /// Create a new cost tracker configuration with explicit values
    pub fn new(enable_budget_limits: bool, default_currency: impl Into<String>) -> Self {
        Self {
            enable_budget_limits,
            default_currency: default_currency.into(),
        }
    }

    /// Create a standard production configuration
    pub fn production() -> Self {
        Self {
            enable_budget_limits: true,
            default_currency: "USD".to_string(),
        }
    }
}

/// Trait for cost tracking
pub trait CostTrackerTrait: Interface + Send + Sync {
    /// Record usage for a provider and return the calculated cost
    fn record_usage(&self, provider_id: &str, units: u64) -> Result<f64>;
    /// Get usage metrics for a specific provider
    fn get_usage_metrics(&self, provider_id: &str) -> Option<UsageMetrics>;
    /// Set budget limit for a provider
    fn set_budget(&self, provider_id: &str, budget: f64);
    /// Check if provider is within budget limits
    fn check_budget(&self, provider_id: &str) -> bool;
    /// Get efficiency score for a provider (0.0 = expensive, 1.0 = cheap)
    fn get_efficiency_score(&self, provider_id: &str) -> Option<f64>;
    /// Register cost information for a provider
    fn register_provider_cost(&self, cost: ProviderCost);
    /// Get total cost across all providers
    fn get_total_cost(&self) -> f64;
    /// Get current period cost across all providers
    fn get_current_period_cost(&self) -> f64;
}

/// Cost tracker for providers with thread-safe operations
pub struct CostTracker {
    /// Cost information for each provider, keyed by provider ID
    costs: Arc<DashMap<String, ProviderCost>>,
    /// Usage metrics for each provider, keyed by provider ID
    usage_metrics: Arc<DashMap<String, UsageMetrics>>,
    /// Budget limits for each provider, keyed by provider ID
    budgets: Arc<DashMap<String, f64>>,
    /// Configuration settings for cost tracking
    config: CostTrackerConfig,
}

impl CostTracker {
    /// Create a new cost tracker with custom configuration (canonical constructor)
    pub fn new(config: CostTrackerConfig) -> Self {
        Self {
            costs: Arc::new(DashMap::new()),
            usage_metrics: Arc::new(DashMap::new()),
            budgets: Arc::new(DashMap::new()),
            config,
        }
    }

    /// Get total cost across all providers
    pub fn get_total_cost(&self) -> f64 {
        self.usage_metrics.iter().map(|m| m.total_cost).sum()
    }

    /// Get current period cost across all providers
    pub fn get_current_period_cost(&self) -> f64 {
        self.usage_metrics
            .iter()
            .map(|m| m.current_period_cost)
            .sum()
    }
}

impl CostTrackerTrait for CostTracker {
    /// Register cost information for a provider
    fn register_provider_cost(&self, cost: ProviderCost) {
        info!(
            "Registered cost for provider {}: {} per {}",
            cost.provider_id, cost.cost_per_unit, cost.unit_type
        );
        self.costs.insert(cost.provider_id.clone(), cost);
    }

    /// Set monthly budget for a provider
    fn set_budget(&self, provider_id: &str, budget: f64) {
        info!("Set budget for provider {}: {}", provider_id, budget);
        self.budgets.insert(provider_id.to_string(), budget);
    }

    /// Record usage and calculate cost
    fn record_usage(&self, provider_id: &str, units: u64) -> Result<f64> {
        let cost_info = self
            .costs
            .get(provider_id)
            .ok_or_else(|| Error::not_found(format!("Cost info not found for {}", provider_id)))?;

        let cost = cost_info.calculate_cost(units);

        let mut metrics = self
            .usage_metrics
            .entry(provider_id.to_string())
            .or_default();
        metrics.total_units += units;
        metrics.total_cost += cost;
        metrics.current_period_units += units;
        metrics.current_period_cost += cost;
        metrics.operation_count += 1;
        metrics.last_usage = Some(chrono::Utc::now());

        if metrics.total_units > 0 {
            metrics.avg_cost_per_unit = metrics.total_cost / metrics.total_units as f64;
        }

        debug!(
            "Recorded {} units for provider {}, cost: {}",
            units, provider_id, cost
        );

        // Check budget
        if self.config.enable_budget_limits {
            if let Some(budget) = self.budgets.get(provider_id) {
                if metrics.current_period_cost > *budget {
                    warn!(
                        "Budget exceeded for provider {}: current={}, limit={}",
                        provider_id, metrics.current_period_cost, *budget
                    );
                }
            }
        }

        Ok(cost)
    }

    /// Check if provider is within budget
    fn check_budget(&self, provider_id: &str) -> bool {
        if !self.config.enable_budget_limits {
            return true;
        }

        let budget = match self.budgets.get(provider_id) {
            Some(b) => *b,
            None => return true, // No budget limit
        };

        let metrics = match self.usage_metrics.get(provider_id) {
            Some(m) => m.clone(),
            None => return true, // No usage yet
        };

        metrics.current_period_cost <= budget
    }

    /// Get usage metrics for a provider
    fn get_usage_metrics(&self, provider_id: &str) -> Option<UsageMetrics> {
        self.usage_metrics.get(provider_id).map(|m| m.clone())
    }

    /// Get efficiency score for a provider (0.0 = expensive, 1.0 = cheap)
    fn get_efficiency_score(&self, provider_id: &str) -> Option<f64> {
        self.costs.get(provider_id).map(|c| c.efficiency_score())
    }

    fn get_total_cost(&self) -> f64 {
        self.usage_metrics.iter().map(|m| m.total_cost).sum()
    }

    fn get_current_period_cost(&self) -> f64 {
        self.usage_metrics
            .iter()
            .map(|m| m.current_period_cost)
            .sum()
    }
}
