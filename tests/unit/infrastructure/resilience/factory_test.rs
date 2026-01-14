//! Factory functions unit tests

use mcp_context_browser::infrastructure::resilience::{
    create_circuit_breaker, create_rate_limiter, determine_rate_limiter_backend,
    CircuitBreakerConfig, RateLimiterBackendType, RateLimiterConfig,
};

#[test]
fn test_create_memory_rate_limiter() {
    let limiter = create_rate_limiter(
        RateLimiterBackendType::Memory,
        RateLimiterConfig::default(),
        None,
    );
    assert_eq!(limiter.backend_type(), "memory");
    assert!(limiter.is_enabled());
}

#[test]
fn test_create_disabled_rate_limiter() {
    let limiter = create_rate_limiter(
        RateLimiterBackendType::Disabled,
        RateLimiterConfig::default(),
        None,
    );
    assert_eq!(limiter.backend_type(), "null");
    assert!(!limiter.is_enabled());
}

#[test]
fn test_create_redis_rate_limiter_fallback() {
    // Without cache, should fall back to memory
    let limiter = create_rate_limiter(
        RateLimiterBackendType::Redis,
        RateLimiterConfig::default(),
        None,
    );
    assert_eq!(limiter.backend_type(), "memory");
}

#[test]
fn test_create_circuit_breaker() {
    let cb = create_circuit_breaker(CircuitBreakerConfig::new("test"), true);
    assert_eq!(cb.backend_type(), "tower");
    assert_eq!(cb.name(), "test");
}

#[test]
fn test_create_disabled_circuit_breaker() {
    let cb = create_circuit_breaker(CircuitBreakerConfig::new("test"), false);
    assert_eq!(cb.backend_type(), "null");
}

#[test]
fn test_determine_backend_clustering_disabled() {
    let backend = determine_rate_limiter_backend(None, false);
    assert_eq!(backend, RateLimiterBackendType::Memory);
}
