//! Repository Adapter Implementations
//!
//! Concrete implementations of repository ports from `mcb_domain::repositories`.
//! These adapters bridge domain repository interfaces with vector storage backends.
//!
//! ## Implementations
//!
//! | Repository | Description |
//! |------------|-------------|
//! | [`VectorStoreChunkRepository`] | Chunk persistence using vector store |
//! | [`VectorStoreSearchRepository`] | Search operations with hybrid search |

mod chunk_repository;
mod search_repository;

// Re-export implementations
pub use chunk_repository::VectorStoreChunkRepository;
pub use search_repository::{SearchStatsTracker, VectorStoreSearchRepository};

// Re-export domain traits for convenience
pub use mcb_domain::repositories::{ChunkRepository, SearchRepository};
