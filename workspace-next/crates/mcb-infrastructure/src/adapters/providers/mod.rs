//! Provider Implementations
//!
//! Concrete implementations of domain provider ports for external service integration.
//!
//! ## Submodules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`embedding`] | Embedding provider implementations (OpenAI, Ollama, etc.) |
//! | [`vector_store`] | Vector store provider implementations (InMemory, Encrypted, etc.) |
//!
//! ## Provider Pattern
//!
//! All providers implement traits from `mcb_domain::ports::providers`:
//! - `EmbeddingProvider` - Text-to-vector embeddings
//! - `VectorStoreProvider` - Vector storage and search
//!
//! Providers are registered with Shaku DI for injection.

pub mod embedding;
pub mod factory;
pub mod vector_store;

// Re-export factories
pub use factory::{EmbeddingProviderFactory, VectorStoreProviderFactory};

// Re-export embedding providers
pub use embedding::{
    FastEmbedProvider, GeminiEmbeddingProvider, NullEmbeddingProvider, OllamaEmbeddingProvider,
    OpenAIEmbeddingProvider, VoyageAIEmbeddingProvider,
};

// Re-export vector store providers
pub use vector_store::{
    EncryptedVectorStoreProvider, InMemoryVectorStoreProvider, NullVectorStoreProvider,
};
