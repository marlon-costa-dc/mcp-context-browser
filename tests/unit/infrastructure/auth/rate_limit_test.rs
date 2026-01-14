//! Tests for auth rate limiting
//!
//! Migrated from src/infrastructure/auth/rate_limit.rs inline tests.

use mcp_context_browser::infrastructure::auth::rate_limit::{
    AuthRateLimiter, RateLimitConfig, RateLimitStatus,
};
use std::time::Duration;

#[test]
fn test_rate_limit_allows_requests() {
    let limiter = AuthRateLimiter::new(RateLimitConfig {
        max_requests: 5,
        window: Duration::from_secs(60),
        lockout_duration: Duration::from_secs(300),
        max_failed_attempts: 3,
    });

    // Should allow first 5 requests
    for _ in 0..5 {
        assert!(limiter.check_request("client1").is_ok());
    }

    // 6th request should be rate limited
    assert!(limiter.check_request("client1").is_err());
}

#[test]
fn test_failed_attempts_lockout() {
    let limiter = AuthRateLimiter::new(RateLimitConfig {
        max_requests: 10,
        window: Duration::from_secs(60),
        lockout_duration: Duration::from_secs(300),
        max_failed_attempts: 3,
    });

    // Record failed attempts
    limiter.record_failed_attempt("client1");
    limiter.record_failed_attempt("client1");
    assert!(limiter.check_request("client1").is_ok());

    // Third failure triggers lockout
    limiter.record_failed_attempt("client1");

    // Should be locked out
    let result = limiter.check_request("client1");
    assert!(result.is_err());
}

#[test]
fn test_success_resets_failed_attempts() {
    let limiter = AuthRateLimiter::new(RateLimitConfig {
        max_requests: 10,
        window: Duration::from_secs(60),
        lockout_duration: Duration::from_secs(300),
        max_failed_attempts: 3,
    });

    limiter.record_failed_attempt("client1");
    limiter.record_failed_attempt("client1");

    // Success should reset counter
    limiter.record_success("client1");

    // Should need 3 more failures to lock out
    limiter.record_failed_attempt("client1");
    limiter.record_failed_attempt("client1");
    assert!(limiter.check_request("client1").is_ok());
}

#[test]
fn test_different_clients_independent() {
    let limiter = AuthRateLimiter::new(RateLimitConfig {
        max_requests: 2,
        window: Duration::from_secs(60),
        lockout_duration: Duration::from_secs(300),
        max_failed_attempts: 3,
    });

    // Exhaust client1's limit
    limiter.check_request("client1").unwrap();
    limiter.check_request("client1").unwrap();
    assert!(limiter.check_request("client1").is_err());

    // client2 should still be allowed
    assert!(limiter.check_request("client2").is_ok());
}

#[test]
fn test_status_reporting() {
    let limiter = AuthRateLimiter::new(RateLimitConfig {
        max_requests: 5,
        window: Duration::from_secs(60),
        lockout_duration: Duration::from_secs(300),
        max_failed_attempts: 3,
    });

    limiter.check_request("client1").unwrap();
    limiter.check_request("client1").unwrap();

    match limiter.get_status("client1") {
        RateLimitStatus::Ok {
            remaining_requests,
            failed_attempts,
            ..
        } => {
            assert_eq!(remaining_requests, 3);
            assert_eq!(failed_attempts, 0);
        }
        _ => panic!("Expected Ok status"),
    }
}
