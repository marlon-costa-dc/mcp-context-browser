//! Rate limiter unit tests

use mcp_context_browser::infrastructure::resilience::{
    NullRateLimiter, RateLimiterBackend, RateLimiterConfig, TowerRateLimiter,
};

#[tokio::test]
async fn test_tower_rate_limiter_allows_under_limit() {
    let config = RateLimiterConfig {
        max_requests_per_window: 5,
        window_seconds: 60,
        burst_allowance: 0,
        enabled: true,
    };
    let limiter = TowerRateLimiter::new(config);

    for i in 0..5 {
        let result = limiter.check("test_key").await.unwrap();
        assert!(result.allowed, "Request {} should be allowed", i);
        assert_eq!(result.remaining, 4 - i);
    }
}

#[tokio::test]
async fn test_tower_rate_limiter_denies_over_limit() {
    let config = RateLimiterConfig {
        max_requests_per_window: 2,
        window_seconds: 60,
        burst_allowance: 0,
        enabled: true,
    };
    let limiter = TowerRateLimiter::new(config);

    // Use up the limit
    limiter.check("test_key").await.unwrap();
    limiter.check("test_key").await.unwrap();

    // Third request should be denied
    let result = limiter.check("test_key").await.unwrap();
    assert!(!result.allowed);
    assert_eq!(result.remaining, 0);
}

#[tokio::test]
async fn test_tower_rate_limiter_burst_allowance() {
    let config = RateLimiterConfig {
        max_requests_per_window: 2,
        window_seconds: 60,
        burst_allowance: 1,
        enabled: true,
    };
    let limiter = TowerRateLimiter::new(config);

    // Should allow 3 requests (2 + 1 burst)
    for _ in 0..3 {
        let result = limiter.check("test_key").await.unwrap();
        assert!(result.allowed);
    }

    // Fourth should be denied
    let result = limiter.check("test_key").await.unwrap();
    assert!(!result.allowed);
}

#[tokio::test]
async fn test_tower_rate_limiter_disabled() {
    let config = RateLimiterConfig {
        max_requests_per_window: 1,
        window_seconds: 60,
        burst_allowance: 0,
        enabled: false,
    };
    let limiter = TowerRateLimiter::new(config);

    // Should always allow when disabled
    for _ in 0..100 {
        let result = limiter.check("test_key").await.unwrap();
        assert!(result.allowed);
        assert_eq!(result.remaining, u32::MAX);
    }
}

#[tokio::test]
async fn test_tower_rate_limiter_reset() {
    let config = RateLimiterConfig {
        max_requests_per_window: 1,
        window_seconds: 60,
        burst_allowance: 0,
        enabled: true,
    };
    let limiter = TowerRateLimiter::new(config);

    // Use up limit
    limiter.check("test_key").await.unwrap();
    let result = limiter.check("test_key").await.unwrap();
    assert!(!result.allowed);

    // Reset
    limiter.reset("test_key").await.unwrap();

    // Should allow again
    let result = limiter.check("test_key").await.unwrap();
    assert!(result.allowed);
}

#[tokio::test]
async fn test_tower_rate_limiter_different_keys() {
    let config = RateLimiterConfig {
        max_requests_per_window: 1,
        window_seconds: 60,
        burst_allowance: 0,
        enabled: true,
    };
    let limiter = TowerRateLimiter::new(config);

    // Different keys have independent limits
    let result1 = limiter.check("key1").await.unwrap();
    let result2 = limiter.check("key2").await.unwrap();

    assert!(result1.allowed);
    assert!(result2.allowed);
}

#[tokio::test]
async fn test_null_rate_limiter() {
    let limiter = NullRateLimiter;

    let result = limiter.check("any_key").await.unwrap();
    assert!(result.allowed);
    assert!(!limiter.is_enabled());
    assert_eq!(limiter.backend_type(), "null");
}
