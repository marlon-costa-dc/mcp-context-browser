//! Adapter Implementations
//!
//! Concrete implementations of domain ports for external service integration.
//! Adapters bridge the domain layer with infrastructure concerns like databases,
//! external APIs, and storage systems.
//!
//! ## Submodules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`admin`] | Admin metrics and indexing operations tracking |
//! | [`chunking`] | AST-based intelligent code chunking using tree-sitter |
//! | [`events`] | Event publishing (tokio broadcast, null) |
//! | [`http_client`] | HTTP client infrastructure for API-based providers |
//! | [`hybrid_search`] | BM25 + semantic hybrid search |
//! | [`providers`] | Provider implementations (embedding, vector store) |
//! | [`repository`] | Repository implementations for data persistence |
//! | [`snapshot`] | Codebase snapshot and change tracking |
//! | [`sync`] | File synchronization coordination |
//!
//! ## Adding New Adapters
//!
//! To add a new adapter:
//! 1. Create a new module in the appropriate subdirectory
//! 2. Implement the domain port trait from `mcb_domain::ports`
//! 3. Add Shaku component annotations for DI
//! 4. Re-export from this module

pub mod admin;
pub mod chunking;
pub mod events;
pub mod http_client;
pub mod hybrid_search;
pub mod providers;
pub mod repository;
pub mod snapshot;
pub mod sync;

// Re-export HTTP client infrastructure
pub use http_client::{HttpClientConfig, HttpClientPool, HttpClientProvider, SharedHttpClient};

// Re-export embedding providers
pub use providers::{
    FastEmbedProvider, GeminiEmbeddingProvider, NullEmbeddingProvider, OllamaEmbeddingProvider,
    OpenAIEmbeddingProvider, VoyageAIEmbeddingProvider,
};

// Re-export repository implementations
pub use repository::{VectorStoreChunkRepository, VectorStoreSearchRepository};

// Re-export chunking infrastructure
pub use chunking::{
    is_language_supported, language_from_extension, supported_languages, IntelligentChunker,
    LanguageConfig, LanguageProcessor, NodeExtractionRule, NodeExtractionRuleBuilder,
};

// Re-export hybrid search infrastructure
pub use hybrid_search::{
    spawn_hybrid_search_actor, BM25Params, BM25Scorer, HybridSearchActor, HybridSearchAdapter,
    HybridSearchConfig, HybridSearchEngine, HybridSearchMessage, HybridSearchResult,
};

// Re-export event infrastructure
pub use events::{NullEventPublisher, TokioEventPublisher};

// Re-export snapshot infrastructure
pub use snapshot::{FilesystemSnapshotProvider, NullSnapshotProvider};

// Re-export sync infrastructure
pub use sync::{DefaultSyncProvider, FileSyncCoordinator, NullSyncCoordinator, NullSyncProvider};

// Re-export admin infrastructure
pub use admin::{AtomicPerformanceMetrics, DefaultIndexingOperations};
