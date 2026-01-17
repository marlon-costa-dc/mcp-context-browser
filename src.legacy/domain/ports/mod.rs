//! Domain Port Interfaces
//!
//! Defines all boundary contracts between domain and external layers.
//! Ports are trait-based abstractions that enable dependency injection
//! and allow implementations to be swapped at runtime.
//!
//! ## Organization
//!
//! - **services.rs** - Application service interfaces (ContextService, SearchService, IndexingService, ChunkingOrchestrator)
//! - **chunking.rs** - Code chunking operations (CodeChunker)
//! - **embedding.rs** - Text embedding providers
//! - **vector_store.rs** - Vector storage backends
//! - **repository.rs** - Data repositories (ChunkRepository, SearchRepository)
//! - **hybrid_search.rs** - Hybrid search operations
//! - **admin.rs** - Admin service operations
//! - **events.rs** - Event bus abstractions
//! - **infrastructure.rs** - Infrastructure utilities (SyncProvider, SnapshotProvider)
//! - **sync.rs** - File synchronization contracts

/// Administrative interfaces for system management and monitoring
pub mod admin;
/// Code chunking strategies for optimal semantic analysis
pub mod chunking;
/// AI embedding provider interfaces for semantic text understanding
pub mod embedding;
/// Domain events for system-wide communication and decoupling
pub mod events;
/// Hybrid search combining BM25 and semantic vector approaches
pub mod hybrid_search;
/// Infrastructure port interfaces for cross-cutting concerns
pub mod infrastructure;
/// Data persistence and retrieval interfaces
pub mod repository;
/// Core business logic service interfaces
pub mod services;
/// Synchronization interfaces for distributed operations
pub mod sync;
/// Vector storage backend interfaces for enterprise-scale semantic search
pub mod vector_store;

// Re-export commonly used port traits
pub use admin::{
    IndexingOperation, IndexingOperationsInterface, PerformanceMetricsData,
    PerformanceMetricsInterface,
};
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
