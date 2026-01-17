//! Gemini (Google AI) embedding provider implementation

use crate::adapters::providers::embedding::helpers::constructor;
use crate::domain::error::{Error, Result};
use crate::domain::ports::EmbeddingProvider;
use crate::domain::types::Embedding;
use crate::infrastructure::utils::HttpResponseUtils;
use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;

/// Gemini embedding provider
pub struct GeminiEmbeddingProvider {
    api_key: String,
    base_url: Option<String>,
    model: String,
    timeout: Duration,
    http_client: Arc<dyn crate::adapters::http_client::HttpClientProvider>,
}

impl GeminiEmbeddingProvider {
    /// Create a new Gemini embedding provider with injected HTTP client
    ///
    /// # Arguments
    /// * `api_key` - Google AI API key
    /// * `base_url` - Optional custom base URL (defaults to Google AI API)
    /// * `model` - Model name (e.g., "text-embedding-004")
    /// * `timeout` - Request timeout duration
    /// * `http_client` - Injected HTTP client (required for DI compliance)
    pub fn new(
        api_key: String,
        base_url: Option<String>,
        model: String,
        timeout: Duration,
        http_client: Arc<dyn crate::adapters::http_client::HttpClientProvider>,
    ) -> Self {
        let api_key = constructor::validate_api_key(&api_key);
        let base_url = constructor::validate_url(base_url);
        Self {
            api_key,
            base_url,
            model,
            timeout,
            http_client,
        }
    }

    /// Get the effective base URL
    fn effective_base_url(&self) -> String {
        constructor::get_effective_url(
            self.base_url.as_deref(),
            "https://generativelanguage.googleapis.com",
        )
    }

    /// Get the model name for API calls (remove prefix if present)
    pub fn api_model_name(&self) -> &str {
        self.model.strip_prefix("models/").unwrap_or(&self.model)
    }
}

#[async_trait]
impl EmbeddingProvider for GeminiEmbeddingProvider {
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>> {
        if texts.is_empty() {
            return Ok(Vec::new());
        }

        let mut results = Vec::new();

        // Gemini API currently doesn't support batch embedding in a single request
        // So we need to make individual requests for each text
        for text in texts {
            // Prepare request payload
            let payload = serde_json::json!({
                "content": {
                    "parts": [
                        {
                            "text": text
                        }
                    ]
                }
            });

            // Use pooled HTTP client
            let client = if self.timeout != self.http_client.config().timeout {
                // Create custom client if timeout differs from pool default
                self.http_client.client_with_timeout(self.timeout)?
            } else {
                self.http_client.client().clone()
            };

            let base_url = self.effective_base_url();
            let url = format!(
                "{}/v1beta/models/{}:embedContent?key={}",
                base_url,
                self.api_model_name(),
                self.api_key
            );

            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .json(&payload)
                .send()
                .await
                .map_err(|e| {
                    if e.is_timeout() {
                        Error::embedding(format!("Request timed out after {:?}", self.timeout))
                    } else {
                        Error::embedding(format!("HTTP request failed: {}", e))
                    }
                })?;

            let response_data: serde_json::Value =
                HttpResponseUtils::check_and_parse(response, "Gemini").await?;

            // Parse embedding from response
            let embedding_vec = response_data["embedding"]["values"]
                .as_array()
                .ok_or_else(|| {
                    Error::embedding(
                        "Invalid response format: missing embedding values".to_string(),
                    )
                })?
                .iter()
                .map(|v| v.as_f64().unwrap_or(0.0) as f32)
                .collect::<Vec<f32>>();

            let dimensions = embedding_vec.len();
            results.push(Embedding {
                vector: embedding_vec,
                model: self.model.clone(),
                dimensions,
            });
        }

        Ok(results)
    }

    fn dimensions(&self) -> usize {
        match self.api_model_name() {
            "gemini-embedding-001" => 768,
            "text-embedding-004" => 768,
            _ => 768, // Default for Gemini embedding models
        }
    }

    fn provider_name(&self) -> &str {
        "gemini"
    }
}

impl GeminiEmbeddingProvider {
    /// Get the model name for this provider
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Get the maximum tokens supported by this provider
    pub fn max_tokens(&self) -> usize {
        match self.api_model_name() {
            "gemini-embedding-001" => 2048,
            "text-embedding-004" => 2048,
            _ => 2048, // Default max tokens for Gemini
        }
    }

    /// Get the API key for this provider
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Get the base URL for this provider
    pub fn base_url(&self) -> String {
        self.effective_base_url()
    }
}
