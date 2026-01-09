//! Null embedding provider for testing and development

use crate::adapters::providers::EmbeddingProvider;
use crate::domain::error::Result;
use crate::domain::types::Embedding;
use async_trait::async_trait;

/// Null embedding provider for testing
/// Returns fixed-size vectors filled with test values
pub struct NullEmbeddingProvider;

impl NullEmbeddingProvider {
    /// Create a new null embedding provider
    pub fn new() -> Self {
        Self
    }
}

impl Default for NullEmbeddingProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EmbeddingProvider for NullEmbeddingProvider {
    async fn embed(&self, text: &str) -> Result<Embedding> {
        let embeddings = self.embed_batch(&[text.to_string()]).await?;
        embeddings.into_iter().next().ok_or_else(|| {
            crate::domain::error::Error::embedding("No embedding returned".to_string())
        })
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>> {
        let embeddings = texts
            .iter()
            .map(|_| Embedding {
                vector: vec![0.1; 1], // Small fixed dimension for testing
                model: "null".to_string(),
                dimensions: 1,
            })
            .collect();

        Ok(embeddings)
    }

    fn dimensions(&self) -> usize {
        1
    }

    fn provider_name(&self) -> &str {
        "null"
    }
}

impl NullEmbeddingProvider {
    /// Get the model name for this provider
    pub fn model(&self) -> &str {
        "null"
    }

    /// Get the maximum tokens supported by this provider
    pub fn max_tokens(&self) -> usize {
        512
    }
}
