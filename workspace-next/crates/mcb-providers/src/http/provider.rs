//! HTTP Client Provider Trait
//!
//! Defines the interface for HTTP client operations required by API-based providers.
//! This trait follows Dependency Inversion Principle: providers define what they need,
//! infrastructure provides the implementation.

use reqwest::Client;
use std::time::Duration;

use super::HttpClientConfig;

/// HTTP Client Provider trait
///
/// Defines the contract for HTTP client operations. Embedding and other API-based
/// providers depend on this interface for making HTTP requests.
///
/// ## Dependency Inversion
///
/// - **Interface**: Defined here in mcb-providers (consumed by providers)
/// - **Implementation**: Provided by mcb-infrastructure (HttpClientPool)
///
/// This allows providers to be tested with mock HTTP clients and decouples
/// them from the concrete connection pooling implementation.
///
/// ## Example
///
/// ```ignore
/// use mcb_providers::http::HttpClientProvider;
/// use std::sync::Arc;
///
/// struct MyEmbeddingProvider {
///     http: Arc<dyn HttpClientProvider>,
/// }
///
/// impl MyEmbeddingProvider {
///     async fn call_api(&self) -> Result<String, Box<dyn std::error::Error>> {
///         let response = self.http.client()
///             .get("https://api.example.com/embeddings")
///             .send()
///             .await?;
///         Ok(response.text().await?)
///     }
/// }
/// ```
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
