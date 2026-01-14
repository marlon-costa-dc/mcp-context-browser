//! Adapters DI Module Implementation
//!
//! Contains HTTP client, external provider adapters, and repositories.
//!
//! ## Provider Strategy
//!
//! The module registers null providers (NullEmbeddingProvider, NullVectorStoreProvider)
//! as defaults. Production code uses `with_component_override` to inject
//! config-based providers at runtime.
//!
//! ## Repository Integration
//!
//! Repositories inject providers from this same module:
//! - VectorStoreChunkRepository injects EmbeddingProvider + VectorStoreProvider
//! - VectorStoreSearchRepository injects VectorStoreProvider

use shaku::module;

use super::traits::AdaptersModule;
use crate::adapters::http_client::HttpClientPool;
use crate::adapters::providers::embedding::NullEmbeddingProvider;
use crate::adapters::providers::vector_store::NullVectorStoreProvider;
use crate::adapters::repository::{VectorStoreChunkRepository, VectorStoreSearchRepository};

/// Implementation of the AdaptersModule trait providing external service integrations.
///
/// This module contains concrete implementations of adapters for:
/// - HTTP client pools for external API communication
/// - Embedding providers (with null fallback for testing)
/// - Vector storage providers (with null fallback for testing)
/// - Repository implementations using vector stores
module! {
    pub AdaptersModuleImpl: AdaptersModule {
        components = [
            HttpClientPool,
            NullEmbeddingProvider,
            NullVectorStoreProvider,
            VectorStoreChunkRepository,
            VectorStoreSearchRepository
        ],
        providers = []
    }
}
