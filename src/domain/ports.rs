/// Code chunking functionality and interfaces
pub mod chunking;
/// Text embedding providers and interfaces
pub mod embedding;
/// Domain events and event publishing
pub mod events;
/// Hybrid search capabilities combining BM25 and semantic search
pub mod hybrid_search;
/// Infrastructure services like snapshots and synchronization
pub mod infrastructure;
/// Repository interfaces for data persistence
pub mod repository;
/// High-level service interfaces for business operations
pub mod services;
/// Synchronization coordination and management
pub mod sync;
/// Vector storage providers and interfaces
pub mod vector_store;

pub use chunking::{ChunkingOptions, ChunkingResult, CodeChunker, SharedCodeChunker};
pub use embedding::EmbeddingProvider;
pub use events::{DomainEvent, EventPublisher, SharedEventPublisher};
pub use hybrid_search::HybridSearchProvider;
pub use infrastructure::{SnapshotProvider, SyncProvider};
pub use repository::{ChunkRepository, SearchRepository};
pub use services::{
    ChunkingOrchestratorInterface, ContextServiceInterface, IndexingResult,
    IndexingServiceInterface, IndexingStatus, SearchServiceInterface,
};
pub use sync::{SharedSyncCoordinator, SyncCoordinator, SyncOptions, SyncResult};
pub use vector_store::VectorStoreProvider;
