//! Hybrid search combining BM25 text ranking with semantic embeddings
//!
//! This module implements a hybrid search approach that combines:
//! - BM25: Term frequency-based text ranking algorithm
//! - Semantic Embeddings: Vector similarity for semantic understanding

mod actor;
mod adapter;
mod bm25;
pub mod config;
mod engine;

// Re-export public types
pub use actor::{HybridSearchActor, HybridSearchMessage};
pub use adapter::HybridSearchAdapter;
pub use bm25::{BM25Params, BM25Scorer};
pub use config::HybridSearchConfig;
pub use engine::{HybridSearchEngine, HybridSearchResult};
