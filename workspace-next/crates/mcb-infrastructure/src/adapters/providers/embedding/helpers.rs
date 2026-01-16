//! Common helpers for embedding providers
//!
//! Shared functionality and patterns used across multiple embedding
//! provider implementations to reduce code duplication.

use std::time::Duration;

/// Common constructor patterns used by embedding providers
///
/// Provides re-usable patterns for provider initialization.
pub mod constructor {
    use std::time::Duration;

    /// Template for validating and normalizing API keys
    pub fn validate_api_key(api_key: &str) -> String {
        api_key.trim().to_string()
    }

    /// Template for validating and normalizing URLs
    pub fn validate_url(url: Option<String>) -> Option<String> {
        url.map(|u| u.trim().to_string())
    }

    /// Template for default timeout when not specified
    pub fn default_timeout() -> Duration {
        Duration::from_secs(30)
    }

    /// Get effective URL with fallback to default
    ///
    /// Standardized approach for handling optional base URLs across all providers.
    pub fn get_effective_url(provided_url: Option<&str>, default_url: &str) -> String {
        provided_url
            .map(|url| url.trim().to_string())
            .unwrap_or_else(|| default_url.to_string())
    }
}

/// Default timeout for embedding API requests
pub const DEFAULT_EMBEDDING_TIMEOUT: Duration = Duration::from_secs(30);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_api_key() {
        assert_eq!(constructor::validate_api_key("  key  "), "key");
        assert_eq!(constructor::validate_api_key("key"), "key");
    }

    #[test]
    fn test_validate_url() {
        assert_eq!(
            constructor::validate_url(Some("  https://api.example.com  ".to_string())),
            Some("https://api.example.com".to_string())
        );
        assert_eq!(constructor::validate_url(None), None);
    }

    #[test]
    fn test_get_effective_url() {
        assert_eq!(
            constructor::get_effective_url(Some("https://custom.com"), "https://default.com"),
            "https://custom.com"
        );
        assert_eq!(
            constructor::get_effective_url(None, "https://default.com"),
            "https://default.com"
        );
    }
}
