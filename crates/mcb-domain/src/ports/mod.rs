//! Domain Port Interfaces
//!
//! Defines all boundary contracts between domain and external layers.
//! Ports are organized by their purpose and enable dependency injection
//! with clear separation of concerns.
//!
//! ## Architecture
//!
//! Ports define the contracts that external layers must implement.
//! This follows the Dependency Inversion Principle:
//! - High-level modules (domain) define interfaces
//! - Low-level modules (providers, infrastructure) implement them
//!
//! ## Organization
//!
//! - **providers/** - External service provider ports (embeddings, vector stores, search)

/// External service provider ports
pub mod providers;

// Re-export commonly used port traits for convenience
pub use providers::{
    CacheEntryConfig, CacheProvider, CacheProviderFactoryInterface, CacheStats, CryptoProvider,
    EmbeddingProvider, EncryptedData, HybridSearchProvider, HybridSearchResult,
    LanguageChunkingProvider, ProviderConfigManagerInterface, VectorStoreAdmin,
    VectorStoreProvider,
};
