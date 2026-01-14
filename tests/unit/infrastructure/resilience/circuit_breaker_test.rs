//! Circuit breaker unit tests

use mcp_context_browser::infrastructure::resilience::{
    CircuitBreakerBackend, CircuitBreakerConfig, CircuitBreakerState, NullCircuitBreaker,
    TowerCircuitBreaker,
};
use std::time::Duration;

#[tokio::test]
async fn test_circuit_breaker_starts_closed() {
    let cb = TowerCircuitBreaker::new(CircuitBreakerConfig::new("test"));
    assert_eq!(cb.state(), CircuitBreakerState::Closed);
    assert!(cb.is_call_permitted());
}

#[tokio::test]
async fn test_circuit_breaker_opens_on_failures() {
    let config = CircuitBreakerConfig {
        failure_threshold: 50, // 50%
        window_size: 4,
        recovery_timeout: Duration::from_secs(30),
        half_open_max_requests: 1,
        name: "test".to_string(),
    };
    let cb = TowerCircuitBreaker::new(config);

    // Record 2 successes
    cb.record_success().await;
    cb.record_success().await;

    // Record 2 failures (50% failure rate)
    cb.record_failure().await;
    cb.record_failure().await;

    // Should trip to open
    assert_eq!(cb.state(), CircuitBreakerState::Open);
    assert!(!cb.is_call_permitted());
}

#[tokio::test]
async fn test_circuit_breaker_stays_closed_under_threshold() {
    let config = CircuitBreakerConfig {
        failure_threshold: 50,
        window_size: 4,
        recovery_timeout: Duration::from_secs(30),
        half_open_max_requests: 1,
        name: "test".to_string(),
    };
    let cb = TowerCircuitBreaker::new(config);

    // 3 successes, 1 failure = 25% failure rate
    cb.record_success().await;
    cb.record_success().await;
    cb.record_success().await;
    cb.record_failure().await;

    // Should stay closed (25% < 50%)
    assert_eq!(cb.state(), CircuitBreakerState::Closed);
    assert!(cb.is_call_permitted());
}

#[tokio::test]
async fn test_circuit_breaker_half_open_recovery() {
    let config = CircuitBreakerConfig {
        failure_threshold: 50,
        window_size: 2,
        recovery_timeout: Duration::from_millis(10), // Very short for test
        half_open_max_requests: 2,
        name: "test".to_string(),
    };
    let cb = TowerCircuitBreaker::new(config);

    // Trip the circuit
    cb.record_failure().await;
    cb.record_failure().await;
    assert_eq!(cb.state(), CircuitBreakerState::Open);

    // Wait for recovery timeout
    tokio::time::sleep(Duration::from_millis(20)).await;

    // Should transition to half-open
    assert!(cb.is_call_permitted());
    assert_eq!(cb.state(), CircuitBreakerState::HalfOpen);

    // Success in half-open should close
    cb.record_success().await;
    cb.record_success().await;
    assert_eq!(cb.state(), CircuitBreakerState::Closed);
}

#[tokio::test]
async fn test_circuit_breaker_half_open_failure() {
    let config = CircuitBreakerConfig {
        failure_threshold: 50,
        window_size: 2,
        recovery_timeout: Duration::from_millis(10),
        half_open_max_requests: 2,
        name: "test".to_string(),
    };
    let cb = TowerCircuitBreaker::new(config);

    // Trip the circuit
    cb.record_failure().await;
    cb.record_failure().await;

    // Wait and try half-open
    tokio::time::sleep(Duration::from_millis(20)).await;
    assert!(cb.is_call_permitted());
    assert_eq!(cb.state(), CircuitBreakerState::HalfOpen);

    // Failure in half-open should re-open
    cb.record_failure().await;
    assert_eq!(cb.state(), CircuitBreakerState::Open);
}

#[tokio::test]
async fn test_null_circuit_breaker() {
    let cb = NullCircuitBreaker::new("test");
    assert!(cb.is_call_permitted());
    assert_eq!(cb.state(), CircuitBreakerState::Closed);
    assert_eq!(cb.backend_type(), "null");

    // Recording has no effect
    cb.record_failure().await;
    cb.record_failure().await;
    assert!(cb.is_call_permitted());
}
