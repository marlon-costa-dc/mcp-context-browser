//! Recovery manager unit tests

use mcp_context_browser::infrastructure::daemon::types::{
    RecoveryConfig, RecoveryState, RecoveryStatus,
};
use mcp_context_browser::infrastructure::events::EventBus;
use mcp_context_browser::infrastructure::recovery::{RecoveryManager, RecoveryManagerInterface};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_recovery_state_transitions() {
    let mut state = RecoveryState::new("test".to_string(), 3);
    assert_eq!(state.status, RecoveryStatus::Healthy);

    // Record failure transitions to Recovering
    state.record_failure(Some("error".to_string()));
    assert_eq!(state.status, RecoveryStatus::Recovering);
    assert_eq!(state.consecutive_failures, 1);

    // Record more failures
    state.record_failure(None);
    state.record_failure(None);
    assert_eq!(state.consecutive_failures, 3);

    // Record recovery attempts until exhausted
    state.record_recovery_attempt();
    assert_eq!(state.current_retry, 1);
    state.record_recovery_attempt();
    assert_eq!(state.current_retry, 2);
    state.record_recovery_attempt();
    assert_eq!(state.current_retry, 3);
    assert_eq!(state.status, RecoveryStatus::Exhausted);

    // Reset clears everything
    state.reset();
    assert_eq!(state.status, RecoveryStatus::Healthy);
    assert_eq!(state.consecutive_failures, 0);
    assert_eq!(state.current_retry, 0);
}

#[test]
fn test_recovery_policy_backoff() {
    let policy = mcp_context_browser::infrastructure::daemon::types::RecoveryPolicy {
        base_delay_ms: 1000,
        max_delay_ms: 30000,
        backoff_multiplier: 2.0,
        ..Default::default()
    };

    assert_eq!(policy.calculate_backoff(0), 1000);
    assert_eq!(policy.calculate_backoff(1), 2000);
    assert_eq!(policy.calculate_backoff(2), 4000);
    assert_eq!(policy.calculate_backoff(3), 8000);
    assert_eq!(policy.calculate_backoff(4), 16000);
    assert_eq!(policy.calculate_backoff(5), 30000); // Capped at max
    assert_eq!(policy.calculate_backoff(10), 30000); // Still capped
}

#[tokio::test]
async fn test_recovery_manager_lifecycle() {
    let config = RecoveryConfig::default();
    let event_bus = Arc::new(EventBus::default());
    let manager = RecoveryManager::new(config, event_bus);

    // With CancellationToken, is_running() means "not yet cancelled"
    // A fresh manager is not cancelled, so is_running() returns true
    assert!(manager.is_running());

    manager.start().await.unwrap();
    assert!(manager.is_running());

    manager.stop().await.unwrap();
    // Give a moment for the task to stop
    tokio::time::sleep(Duration::from_millis(50)).await;
    // After stop(), the cancellation token is cancelled
    assert!(!manager.is_running());
}

#[test]
fn test_recovery_manager_subsystem_registration() {
    let config = RecoveryConfig::default();
    let event_bus = Arc::new(EventBus::default());
    let manager = RecoveryManager::new(config, event_bus);

    // Register subsystem
    manager.register_subsystem("embedding:ollama");
    assert!(manager.get_recovery_state("embedding:ollama").is_some());

    // Unregister
    manager.unregister_subsystem("embedding:ollama");
    assert!(manager.get_recovery_state("embedding:ollama").is_none());
}

#[test]
fn test_mark_healthy_unhealthy() {
    let config = RecoveryConfig::default();
    let event_bus = Arc::new(EventBus::default());
    let manager = RecoveryManager::new(config, event_bus);

    manager.register_subsystem("test:provider");

    // Mark unhealthy
    manager.mark_unhealthy("test:provider", Some("Connection failed".to_string()));
    let state = manager.get_recovery_state("test:provider").unwrap();
    assert_eq!(state.status, RecoveryStatus::Recovering);
    assert_eq!(state.consecutive_failures, 1);

    // Mark healthy
    manager.mark_healthy("test:provider");
    let state = manager.get_recovery_state("test:provider").unwrap();
    assert_eq!(state.status, RecoveryStatus::Healthy);
    assert_eq!(state.consecutive_failures, 0);
}
