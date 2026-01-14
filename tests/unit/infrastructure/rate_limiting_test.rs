//! Tests for rate limiting functionality
//!
//! Tests both the core rate limiter and HTTP middleware integration.
//! Uses the new resilience module's rate limiter implementation.

use mcp_context_browser::infrastructure::resilience::{
    create_rate_limiter, RateLimitResult, RateLimiterBackend, RateLimiterBackendType,
    RateLimiterConfig, SharedRateLimiter, TowerRateLimiter,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_creation() {
        let config = RateLimiterConfig {
            max_requests_per_window: 100,
            window_seconds: 60,
            burst_allowance: 20,
            enabled: true,
        };

        let limiter = TowerRateLimiter::new(config);
        assert!(limiter.is_enabled());
        assert_eq!(limiter.backend_type(), "memory");
    }

    #[tokio::test]
    async fn test_rate_limiter_disabled() {
        let config = RateLimiterConfig {
            enabled: false,
            ..Default::default()
        };

        let limiter = TowerRateLimiter::new(config);
        let result = limiter.check("ip:127.0.0.1").await.unwrap();

        assert!(result.allowed);
        assert_eq!(result.remaining, u32::MAX);
        assert_eq!(result.reset_in_seconds, 0);
    }

    #[tokio::test]
    async fn test_rate_limit_keys() {
        // New module uses simple string keys instead of enum
        let ip_key = "ip:192.168.1.1";
        let user_key = "user:user123";
        let api_key = "apikey:key456";
        let endpoint_key = "endpoint:/api/search";

        assert!(ip_key.starts_with("ip:"));
        assert!(user_key.starts_with("user:"));
        assert!(api_key.starts_with("apikey:"));
        assert!(endpoint_key.starts_with("endpoint:"));
    }

    #[tokio::test]
    async fn test_rate_limit_config_default() {
        let config = RateLimiterConfig::default();

        assert_eq!(config.window_seconds, 60);
        assert_eq!(config.max_requests_per_window, 100);
        assert_eq!(config.burst_allowance, 20);
        assert!(config.enabled);
    }

    #[tokio::test]
    async fn test_rate_limiter_memory_backend() {
        let config = RateLimiterConfig {
            max_requests_per_window: 10,
            window_seconds: 60,
            burst_allowance: 5,
            enabled: true,
        };

        let limiter = TowerRateLimiter::new(config);

        // First request should be allowed
        let result = limiter.check("ip:127.0.0.1").await.unwrap();
        assert!(result.allowed);
        assert_eq!(result.limit, 15); // 10 + 5 burst

        // Use up remaining requests
        for _ in 0..14 {
            limiter.check("ip:127.0.0.1").await.unwrap();
        }

        // 16th request should be denied
        let result = limiter.check("ip:127.0.0.1").await.unwrap();
        assert!(!result.allowed);
        assert_eq!(result.remaining, 0);
    }

    #[tokio::test]
    async fn test_rate_limit_result_structure() {
        let result = RateLimitResult {
            allowed: true,
            remaining: 95,
            reset_in_seconds: 45,
            current_count: 5,
            limit: 100,
        };

        assert!(result.allowed);
        assert_eq!(result.remaining, 95);
        assert_eq!(result.reset_in_seconds, 45);
        assert_eq!(result.current_count, 5);
    }

    #[tokio::test]
    async fn test_rate_limiter_reset() {
        let config = RateLimiterConfig {
            max_requests_per_window: 2,
            window_seconds: 60,
            burst_allowance: 0,
            enabled: true,
        };

        let limiter = TowerRateLimiter::new(config);

        // Use up limit
        limiter.check("ip:test").await.unwrap();
        limiter.check("ip:test").await.unwrap();

        // Third should be denied
        let result = limiter.check("ip:test").await.unwrap();
        assert!(!result.allowed);

        // Reset
        limiter.reset("ip:test").await.unwrap();

        // Should be allowed again
        let result = limiter.check("ip:test").await.unwrap();
        assert!(result.allowed);
    }

    #[tokio::test]
    async fn test_rate_limiter_different_keys() {
        let config = RateLimiterConfig {
            max_requests_per_window: 1,
            window_seconds: 60,
            burst_allowance: 0,
            enabled: true,
        };

        let limiter = TowerRateLimiter::new(config);

        // Different keys have independent limits
        let result1 = limiter.check("ip:key1").await.unwrap();
        let result2 = limiter.check("ip:key2").await.unwrap();

        assert!(result1.allowed);
        assert!(result2.allowed);
    }

    #[tokio::test]
    async fn test_factory_creates_memory_backend() {
        let config = RateLimiterConfig::default();
        let limiter = create_rate_limiter(RateLimiterBackendType::Memory, config, None);

        assert_eq!(limiter.backend_type(), "memory");
        assert!(limiter.is_enabled());
    }

    #[tokio::test]
    async fn test_factory_creates_disabled_backend() {
        let config = RateLimiterConfig::default();
        let limiter = create_rate_limiter(RateLimiterBackendType::Disabled, config, None);

        assert_eq!(limiter.backend_type(), "null");
        assert!(!limiter.is_enabled());
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_shared_rate_limiter() {
        let config = RateLimiterConfig {
            max_requests_per_window: 10,
            window_seconds: 60,
            burst_allowance: 5,
            enabled: true,
        };

        let limiter: SharedRateLimiter =
            create_rate_limiter(RateLimiterBackendType::Memory, config, None);

        // Test basic rate limiting functionality
        let result = limiter.check("ip:127.0.0.1").await.unwrap();
        assert!(result.allowed);
        assert_eq!(result.limit, 15); // 10 + 5 burst allowance
    }

    #[tokio::test]
    async fn test_rate_limit_result_helpers() {
        let allowed = RateLimitResult::allowed(99, 100);
        assert!(allowed.allowed);
        assert_eq!(allowed.remaining, 99);

        let denied = RateLimitResult::denied(30, 100);
        assert!(!denied.allowed);
        assert_eq!(denied.reset_in_seconds, 30);

        let unlimited = RateLimitResult::unlimited();
        assert!(unlimited.allowed);
        assert_eq!(unlimited.remaining, u32::MAX);
    }
}
