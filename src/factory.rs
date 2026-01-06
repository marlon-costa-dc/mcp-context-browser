//! Provider factory for creating configured services

use crate::config::Config;
use crate::core::error::{Error, Result};
use crate::providers::{
    EmbeddingProvider, VectorStoreProvider,
    embedding::{MockEmbeddingProvider, OpenAIEmbeddingProvider},
    vector_store::{InMemoryVectorStoreProvider, MilvusVectorStoreProvider},
};
use crate::services::ContextService;
use std::sync::Arc;

/// Provider factory for creating configured services
pub struct ProviderFactory;

impl ProviderFactory {
    /// Create embedding provider based on configuration
    pub fn create_embedding_provider(config: &crate::core::types::EmbeddingConfig) -> Result<Arc<dyn EmbeddingProvider>> {
        match config.provider.as_str() {
            "openai" => {
                let api_key = config.api_key.as_ref()
                    .ok_or_else(|| Error::config("OpenAI API key required"))?;

                Ok(Arc::new(OpenAIEmbeddingProvider::new(
                    api_key.clone(),
                    config.base_url.clone(),
                    config.model.clone(),
                )))
            }
            "mock" => Ok(Arc::new(MockEmbeddingProvider::new())),
            _ => Err(Error::config(format!("Unsupported embedding provider: {}", config.provider))),
        }
    }

    /// Create vector store provider based on configuration
    pub fn create_vector_store_provider(config: &crate::core::types::VectorStoreConfig) -> Result<Arc<dyn VectorStoreProvider>> {
        match config.provider.as_str() {
            "in-memory" => Ok(Arc::new(InMemoryVectorStoreProvider::new())),
            "milvus" => {
                // TODO: Implement Milvus provider
                Err(Error::config("Milvus provider not yet implemented"))
            }
            _ => Err(Error::config(format!("Unsupported vector store provider: {}", config.provider))),
        }
    }

    /// Create context service with providers from configuration
    pub fn create_context_service(config: &Config) -> Result<ContextService> {
        let embedding_provider = Self::create_embedding_provider(&config.providers.embedding)?;
        let vector_store_provider = Self::create_vector_store_provider(&config.providers.vector_store)?;

        Ok(ContextService::new(embedding_provider, vector_store_provider))
    }

    /// Create context service with default providers
    pub fn create_default_context_service() -> Result<ContextService> {
        Ok(ContextService::default())
    }
}