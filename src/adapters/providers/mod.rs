pub use crate::domain::ports::{EmbeddingProvider, VectorStoreProvider};

pub mod embedding;
pub mod routing;
pub mod vector_store;

// Re-export implementations for convenience/backward compatibility
pub use embedding::NullEmbeddingProvider as MockEmbeddingProvider;
pub use embedding::OllamaEmbeddingProvider;
pub use embedding::OpenAIEmbeddingProvider;
pub use vector_store::InMemoryVectorStoreProvider;

pub use routing::{
    ProviderContext, ProviderRouter, ProviderSelectionStrategy, circuit_breaker::CircuitBreaker,
    metrics::ProviderMetricsCollector,
};
