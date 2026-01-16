//! Tests for HTTP client configuration
//!
//! Validates HttpClientConfig defaults, construction, and customization.

use mcb_infrastructure::adapters::http_client::HttpClientConfig;
use mcb_infrastructure::constants::{
    HTTP_CLIENT_IDLE_TIMEOUT_SECS, HTTP_KEEPALIVE_SECS, HTTP_MAX_IDLE_PER_HOST,
    HTTP_REQUEST_TIMEOUT_SECS,
};
use std::time::Duration;

#[test]
fn test_http_client_config_defaults() {
    let config = HttpClientConfig::default();

    assert_eq!(config.max_idle_per_host, HTTP_MAX_IDLE_PER_HOST);
    assert_eq!(
        config.idle_timeout,
        Duration::from_secs(HTTP_CLIENT_IDLE_TIMEOUT_SECS)
    );
    assert_eq!(config.keepalive, Duration::from_secs(HTTP_KEEPALIVE_SECS));
    assert_eq!(
        config.timeout,
        Duration::from_secs(HTTP_REQUEST_TIMEOUT_SECS)
    );
    assert!(config.user_agent.contains("MCP-Context-Browser"));
}

#[test]
fn test_http_client_config_new() {
    let config = HttpClientConfig::new(
        20,
        Duration::from_secs(120),
        Duration::from_secs(30),
        Duration::from_secs(60),
        "Custom-Agent/1.0".to_string(),
    );

    assert_eq!(config.max_idle_per_host, 20);
    assert_eq!(config.idle_timeout, Duration::from_secs(120));
    assert_eq!(config.keepalive, Duration::from_secs(30));
    assert_eq!(config.timeout, Duration::from_secs(60));
    assert_eq!(config.user_agent, "Custom-Agent/1.0");
}

#[test]
fn test_http_client_config_with_timeout() {
    let custom_timeout = Duration::from_secs(45);
    let config = HttpClientConfig::with_timeout(custom_timeout);

    // Timeout should be customized
    assert_eq!(config.timeout, custom_timeout);

    // Other values should be defaults
    assert_eq!(config.max_idle_per_host, HTTP_MAX_IDLE_PER_HOST);
    assert_eq!(
        config.idle_timeout,
        Duration::from_secs(HTTP_CLIENT_IDLE_TIMEOUT_SECS)
    );
    assert_eq!(config.keepalive, Duration::from_secs(HTTP_KEEPALIVE_SECS));
}

#[test]
fn test_http_client_config_clone() {
    let original = HttpClientConfig::new(
        15,
        Duration::from_secs(100),
        Duration::from_secs(50),
        Duration::from_secs(30),
        "Test-Agent".to_string(),
    );

    let cloned = original.clone();

    assert_eq!(cloned.max_idle_per_host, original.max_idle_per_host);
    assert_eq!(cloned.idle_timeout, original.idle_timeout);
    assert_eq!(cloned.keepalive, original.keepalive);
    assert_eq!(cloned.timeout, original.timeout);
    assert_eq!(cloned.user_agent, original.user_agent);
}

#[test]
fn test_http_client_config_debug() {
    let config = HttpClientConfig::default();

    let debug_str = format!("{:?}", config);

    assert!(debug_str.contains("HttpClientConfig"));
    assert!(debug_str.contains("max_idle_per_host"));
    assert!(debug_str.contains("timeout"));
}

#[test]
fn test_http_client_config_user_agent_contains_version() {
    let config = HttpClientConfig::default();

    // User agent should contain package version
    assert!(
        config.user_agent.contains('/'),
        "User agent should have format Name/Version"
    );
}

#[test]
fn test_http_client_config_timeout_values_reasonable() {
    let config = HttpClientConfig::default();

    // All timeouts should be positive
    assert!(config.timeout.as_secs() > 0);
    assert!(config.idle_timeout.as_secs() > 0);
    assert!(config.keepalive.as_secs() > 0);

    // Idle timeout should be greater than or equal to keepalive
    assert!(
        config.idle_timeout >= config.keepalive,
        "Idle timeout should be >= keepalive"
    );
}

#[test]
fn test_http_client_config_zero_timeout() {
    // Creating config with zero timeout should work (validation is at use time)
    let config = HttpClientConfig::with_timeout(Duration::ZERO);
    assert_eq!(config.timeout, Duration::ZERO);
}

#[test]
fn test_http_client_config_max_idle_per_host_range() {
    // Test with minimum reasonable value
    let min_config = HttpClientConfig::new(
        1,
        Duration::from_secs(60),
        Duration::from_secs(30),
        Duration::from_secs(30),
        "Test".to_string(),
    );
    assert_eq!(min_config.max_idle_per_host, 1);

    // Test with high value
    let max_config = HttpClientConfig::new(
        100,
        Duration::from_secs(60),
        Duration::from_secs(30),
        Duration::from_secs(30),
        "Test".to_string(),
    );
    assert_eq!(max_config.max_idle_per_host, 100);
}
