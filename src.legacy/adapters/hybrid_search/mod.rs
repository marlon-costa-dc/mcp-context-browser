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
//! | "API authentication" | ✓ Exact matches | ✓ Concepts | ✓ Both |
//! | "login system" | ✓ Keywords | ✓ Understanding | ✓ Combines strength |
//! | "encrypt password" | ✓ Words | ✓ Intent | ✓ Optimal |
//! | Typos: "authentiction" | ✗ Misses | ✓ Understands | ✓ Better coverage |
//!
//! ## Architecture
//!
//! ```text
//! Query Input
//!     ↓
//! Parallel Processing:
//!     ├─→ BM25 Scorer (keyword matching)
//!     │   └─→ Term frequency score (0-1)
//!     │
//!     └─→ Embedding (semantic)
//!         └─→ Vector similarity score (0-1)
//!
//! Score Fusion:
//!     Hybrid Score = α × BM25 + (1-α) × Semantic
//!     (α typically 0.3-0.7)
//!
//! Rank Results (highest score first)
//!     ↓
//! Return Top-K Results
//! ```
//!
//! ## Example: Hybrid Search
//!
//! ```rust,no_run
//! use mcp_context_browser::adapters::hybrid_search::HybridSearchEngine;
//! use mcp_context_browser::domain::types::{CodeChunk, Language, SearchResult};
//!
//! # fn example() -> anyhow::Result<()> {
//! // Create a hybrid search engine with 40% BM25, 60% semantic weighting
//! let mut engine = HybridSearchEngine::new(0.4, 0.6);
//!
//! // Add code chunks for BM25 indexing
//! let chunks = vec![
//!     CodeChunk {
//!         id: "chunk-1".to_string(),
//!         content: "fn authenticate_user(username: &str) -> bool { username.len() > 0 }".to_string(),
//!         file_path: "auth.rs".to_string(),
//!         start_line: 10,
//!         end_line: 12,
//!         language: Language::Rust,
//!         metadata: serde_json::json!({}),
//!     }
//! ];
//! engine.add_documents(chunks);
//!
//! // Combine with semantic search results (from a vector database)
//! let semantic_results = vec![
//!     SearchResult {
//!         id: "chunk-1".to_string(),
//!         content: "fn authenticate_user(username: &str) -> bool { username.len() > 0 }".to_string(),
//!         file_path: "auth.rs".to_string(),
//!         start_line: 10,
//!         score: 0.85, // Semantic similarity score
//!         metadata: serde_json::json!({}),
//!     }
//! ];
//!
//! // Perform hybrid search that combines BM25 and semantic scores
//! let results = engine.hybrid_search("user authentication", semantic_results, 10)?;
//!
//! for result in results {
//!     println!(
//!         "Hybrid Score: {:.3}, BM25: {:.3}, Semantic: {:.3}, File: {}",
//!         result.hybrid_score, result.bm25_score, result.semantic_score, result.result.file_path
//!     );
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Score Weighting
//!
//! The BM25 weight determines the balance:
//! - **BM25 weight = 0.0**: Pure semantic search (embeddings only)
//! - **BM25 weight = 0.5**: Equal weight to both methods
//! - **BM25 weight = 1.0**: Pure keyword search (BM25 only)
//! - **Default (0.3)**: Semantic-first with keyword boost

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
