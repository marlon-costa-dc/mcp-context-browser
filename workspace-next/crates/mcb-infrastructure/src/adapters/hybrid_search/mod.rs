//! Hybrid search combining BM25 text ranking with semantic embeddings
//!
//! This module implements a hybrid search approach that combines:
//! - **BM25**: Term frequency-based text ranking algorithm for keyword matching
//! - **Semantic**: Vector similarity for conceptual understanding
//!
//! ## Why Hybrid Search?
//!
//! | Query Type | BM25 Alone | Semantic Alone | Hybrid |
//! |-----------|-----------|-------------|--------|
//! | "API authentication" | Exact matches | Concepts | Both |
//! | "login system" | Keywords | Understanding | Combines strength |
//! | "encrypt password" | Words | Intent | Optimal |
//! | Typos: "authentiction" | Misses | Understands | Better coverage |
//!
//! ## Architecture
//!
//! ```text
//! Query Input
//!     |
//! Parallel Processing:
//!     |-- BM25 Scorer (keyword matching)
//!     |   `-- Term frequency score (0-1)
//!     |
//!     `-- Embedding (semantic)
//!         `-- Vector similarity score (0-1)
//!
//! Score Fusion:
//!     Hybrid Score = alpha * BM25 + (1-alpha) * Semantic
//!     (alpha typically 0.3-0.7)
//!
//! Rank Results (highest score first)
//!     |
//! Return Top-K Results
//! ```
//!
//! ## Score Weighting
//!
//! The BM25 weight determines the balance:
//! - **BM25 weight = 0.0**: Pure semantic search (embeddings only)
//! - **BM25 weight = 0.5**: Equal weight to both methods
//! - **BM25 weight = 1.0**: Pure keyword search (BM25 only)
//! - **Default (0.4)**: Semantic-first with keyword boost

mod actor;
mod adapter;
mod bm25;
pub mod config;
mod engine;

// Re-export public types
pub use actor::{spawn_hybrid_search_actor, HybridSearchActor, HybridSearchMessage};
pub use adapter::HybridSearchAdapter;
pub use bm25::{BM25Params, BM25Scorer};
pub use config::HybridSearchConfig;
pub use engine::HybridSearchEngine;
// HybridSearchResult is defined in domain layer (Clean Architecture)
pub use mcb_domain::ports::providers::hybrid_search::HybridSearchResult;
