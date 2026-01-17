//! Unit tests for embedding provider helpers
//!
//! Tests for common helpers for embedding providers including constructor patterns.

use mcp_context_browser::adapters::providers::embedding::helpers::constructor;
use std::time::Duration;

#[test]
fn test_validate_api_key() {
    assert_eq!(constructor::validate_api_key("  key  "), "key");
    assert_eq!(constructor::validate_api_key("key"), "key");
}

#[test]
fn test_validate_url() {
    assert_eq!(
        constructor::validate_url(Some("  http://example.com  ".to_string())),
        Some("http://example.com".to_string())
    );
    assert_eq!(constructor::validate_url(None), None);
}

#[test]
fn test_default_timeout() {
    assert_eq!(constructor::default_timeout(), Duration::from_secs(30));
}
