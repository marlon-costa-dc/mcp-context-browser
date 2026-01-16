//! HTTP Client Provider Trait
//!
//! Defines the internal interface for HTTP client operations.
//! This is an infrastructure-level abstraction, not a domain port.

use reqwest::Client;
use std::time::Duration;

use super::HttpClientConfig;

/// Trait for HTTP client pool operations
///
/// This trait enables dependency injection for HTTP-based adapters,
/// allowing providers to be tested with mock HTTP clients.
///
/// ## Implementation Note
///
/// This trait uses `Send + Sync` bounds for thread-safe sharing
/// across async contexts. It does NOT extend `shaku::Interface`
/// because it's not a domain port - it's infrastructure plumbing.
pub trait HttpClientProvider: Send + Sync {
    /// Get a reference to the underlying reqwest Client
    fn client(&self) -> &Client;

    /// Get the configuration
    fn config(&self) -> &HttpClientConfig;

    /// Create a new client with custom timeout for specific operations
    ///
    /// Some API calls (like large batch embeddings) may need longer
    /// timeouts than the default pool configuration.
    fn client_with_timeout(
        &self,
        timeout: Duration,
    ) -> Result<Client, Box<dyn std::error::Error + Send + Sync>>;

    /// Check if the client pool is enabled
    ///
    /// Returns `false` for null/test implementations.
    fn is_enabled(&self) -> bool;
}
