use crate::domain::error::Result;
use crate::domain::types::Embedding;
use async_trait::async_trait;

/// AI Semantic Understanding Interface
///
/// Defines the business contract for AI providers that transform text into
/// semantic embeddings. This abstraction enables the platform to work with
/// any AI service that can understand code semantics, from enterprise OpenAI
/// deployments to self-hosted Ollama instances.
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Embedding>;
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>>;
    fn dimensions(&self) -> usize;
    fn provider_name(&self) -> &str;

    /// Health check for the provider (default implementation provided)
    async fn health_check(&self) -> Result<()> {
        // Default implementation - try a simple embed operation
        self.embed("health check").await?;
        Ok(())
    }
}
