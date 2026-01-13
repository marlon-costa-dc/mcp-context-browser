//! Provider Implementations - Adapter Layer
//!
//! This module contains concrete implementations of domain ports.
//! All implementations are isolated in their respective submodules.

pub mod embedding;
pub mod routing;
pub mod vector_store;

// Re-export implementations for convenience (Infrastructure layer usage)
// Note: NullEmbeddingProvider NOT re-exported (Phase 5 DI audit - test-only)
pub use embedding::OllamaEmbeddingProvider;
pub use embedding::OpenAIEmbeddingProvider;
pub use vector_store::InMemoryVectorStoreProvider;

pub use routing::{
    circuit_breaker::CircuitBreaker, metrics::ProviderMetricsCollector, ProviderContext,
    ProviderRouter, ProviderSelectionStrategy,
};
