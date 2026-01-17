//! HTTP Rate Limiting Middleware
//!
//! Axum middleware for rate limiting HTTP requests.
//! Uses the resilience module's pluggable rate limiter backend.

use axum::extract::ConnectInfo;
use std::net::SocketAddr;

use crate::infrastructure::resilience::{RateLimitResult, SharedRateLimiter};

/// Simple rate limiting check function
/// This can be used in route handlers directly
pub async fn check_rate_limit_for_ip(
    rate_limiter: &Option<SharedRateLimiter>,
    addr: &ConnectInfo<SocketAddr>,
) -> Result<(), (axum::http::StatusCode, String)> {
    let Some(limiter) = rate_limiter else {
        // No rate limiter configured, allow request
        return Ok(());
    };

    if !limiter.is_enabled() {
        return Ok(());
    }

    let key = format!("ip:{}", addr.0.ip());

    match limiter.check(&key).await {
        Ok(result) if result.allowed => Ok(()),
        Ok(result) => {
            let message = format!(
                "Rate limit exceeded. {} requests remaining. Resets in {} seconds.",
                result.remaining, result.reset_in_seconds
            );
            Err((axum::http::StatusCode::TOO_MANY_REQUESTS, message))
        }
        Err(e) => {
            // Log error but allow request to avoid blocking legitimate users
            tracing::error!("Rate limiting check failed: {}", e);
            Ok(())
        }
    }
}

/// Helper function to add rate limit headers to a response
pub fn add_rate_limit_headers(headers: &mut axum::http::HeaderMap, result: &RateLimitResult) {
    headers.insert(
        "X-RateLimit-Limit",
        result
            .limit
            .to_string()
            .parse()
            .unwrap_or_else(|_| axum::http::HeaderValue::from_static("0")),
    );
    headers.insert(
        "X-RateLimit-Remaining",
        result
            .remaining
            .to_string()
            .parse()
            .unwrap_or_else(|_| axum::http::HeaderValue::from_static("0")),
    );
    headers.insert(
        "X-RateLimit-Reset",
        result
            .reset_in_seconds
            .to_string()
            .parse()
            .unwrap_or_else(|_| axum::http::HeaderValue::from_static("0")),
    );
}
