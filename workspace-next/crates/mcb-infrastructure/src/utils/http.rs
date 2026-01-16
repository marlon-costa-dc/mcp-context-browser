//! HTTP Response Utilities
//!
//! Consolidates common patterns for handling HTTP responses from external APIs.
//! Reduces boilerplate in embedding and vector store providers.

use mcb_domain::error::{Error, Result};

/// HTTP response utilities for API clients
///
/// Provides helper methods for checking response status and parsing JSON,
/// reducing ~8-12 lines of boilerplate per API call.
///
/// ## Example
///
/// ```rust,no_run
/// use mcb_infrastructure::utils::HttpResponseUtils;
///
/// async fn call_api() -> mcb_domain::error::Result<serde_json::Value> {
///     let client = reqwest::Client::new();
///     let response = client.get("https://api.example.com").send().await
///         .map_err(|e| mcb_domain::error::Error::embedding(e.to_string()))?;
///     HttpResponseUtils::check_and_parse(response, "Example").await
/// }
/// ```
pub struct HttpResponseUtils;

impl HttpResponseUtils {
    /// Check HTTP response and return error if not successful
    ///
    /// Saves ~8 lines per use by consolidating status check and error extraction.
    pub async fn check_response(
        response: reqwest::Response,
        provider_name: &str,
    ) -> Result<reqwest::Response> {
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(Error::embedding(format!(
                "{} API error {}: {}",
                provider_name, status, error_text
            )));
        }
        Ok(response)
    }

    /// Parse JSON from response with provider-specific error
    ///
    /// Saves ~6 lines per use.
    pub async fn json_response<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
        provider_name: &str,
    ) -> Result<T> {
        response
            .json()
            .await
            .map_err(|e| Error::embedding(format!("{} response parse error: {}", provider_name, e)))
    }

    /// Combined: check response status and parse JSON
    ///
    /// Most common pattern - saves ~12 lines per use.
    pub async fn check_and_parse<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
        provider_name: &str,
    ) -> Result<T> {
        let response = Self::check_response(response, provider_name).await?;
        Self::json_response(response, provider_name).await
    }
}

#[cfg(test)]
mod tests {
    // HTTP response tests would require mocking - tested via integration tests
}
