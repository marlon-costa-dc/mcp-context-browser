//! Active health monitor unit tests

use mcp_context_browser::adapters::providers::routing::health::HealthMonitor;
use mcp_context_browser::infrastructure::di::registry::ProviderRegistry;
use mcp_context_browser::infrastructure::events::EventBus;
use mcp_context_browser::infrastructure::health::{ActiveHealthConfig, ActiveHealthMonitor};
use std::sync::Arc;

#[test]
fn test_active_health_config_defaults() {
    let config = ActiveHealthConfig::default();
    assert_eq!(config.probe_interval_secs, 10);
    assert_eq!(config.probe_timeout_secs, 5);
    assert_eq!(config.failure_threshold, 3);
}

#[test]
fn test_monitor_lifecycle() {
    let registry = Arc::new(ProviderRegistry::new());
    let health = Arc::new(HealthMonitor::with_registry(registry.clone()));

    let monitor = ActiveHealthMonitor::with_defaults(health, registry, Arc::new(EventBus::new(10)));

    // CancellationToken starts not cancelled, so is_running() returns true
    // (monitor is ready to run, not yet cancelled)
    assert!(monitor.is_running());

    // After stop(), is_running() returns false
    monitor.stop();
    assert!(!monitor.is_running());
}
