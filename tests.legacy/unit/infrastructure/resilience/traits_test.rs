//! Tests for resilience pattern traits
//!
//! Migrated from src/infrastructure/resilience/traits.rs

use mcp_context_browser::infrastructure::resilience::{CircuitBreakerState, RateLimitResult};

#[test]
fn test_rate_limit_result_allowed() {
    let result = RateLimitResult::allowed(99, 100);
    assert!(result.allowed);
    assert_eq!(result.remaining, 99);
    assert_eq!(result.current_count, 1);
    assert_eq!(result.limit, 100);
}

#[test]
fn test_rate_limit_result_denied() {
    let result = RateLimitResult::denied(30, 100);
    assert!(!result.allowed);
    assert_eq!(result.remaining, 0);
    assert_eq!(result.reset_in_seconds, 30);
}

#[test]
fn test_rate_limit_result_unlimited() {
    let result = RateLimitResult::unlimited();
    assert!(result.allowed);
    assert_eq!(result.remaining, u32::MAX);
}

#[test]
fn test_circuit_breaker_state_display() {
    assert_eq!(CircuitBreakerState::Closed.to_string(), "closed");
    assert_eq!(CircuitBreakerState::Open.to_string(), "open");
    assert_eq!(CircuitBreakerState::HalfOpen.to_string(), "half-open");
}
