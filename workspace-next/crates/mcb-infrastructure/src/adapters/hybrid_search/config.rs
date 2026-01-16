//! Hybrid search configuration
//!
//! This module provides configuration options for the hybrid search system,
//! including weights for BM25 and semantic scores.

use crate::constants::{
    HYBRID_SEARCH_BM25_B, HYBRID_SEARCH_BM25_K1, HYBRID_SEARCH_BM25_WEIGHT,
    HYBRID_SEARCH_SEMANTIC_WEIGHT,
};
use serde::{Deserialize, Serialize};

/// Hybrid search configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HybridSearchConfig {
    /// Enable hybrid search
    pub enabled: bool,
    /// Weight for BM25 score (0-1)
    pub bm25_weight: f32,
    /// Weight for semantic score (0-1)
    pub semantic_weight: f32,
    /// BM25 k1 parameter
    pub bm25_k1: f32,
    /// BM25 b parameter
    pub bm25_b: f32,
}

impl Default for HybridSearchConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bm25_weight: HYBRID_SEARCH_BM25_WEIGHT as f32,
            semantic_weight: HYBRID_SEARCH_SEMANTIC_WEIGHT as f32,
            bm25_k1: HYBRID_SEARCH_BM25_K1 as f32,
            bm25_b: HYBRID_SEARCH_BM25_B as f32,
        }
    }
}

impl HybridSearchConfig {
    /// Create config from environment variables
    pub fn from_env() -> Self {
        Self {
            enabled: std::env::var("HYBRID_SEARCH_ENABLED")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            bm25_weight: std::env::var("HYBRID_SEARCH_BM25_WEIGHT")
                .unwrap_or_else(|_| HYBRID_SEARCH_BM25_WEIGHT.to_string())
                .parse()
                .unwrap_or(HYBRID_SEARCH_BM25_WEIGHT as f32),
            semantic_weight: std::env::var("HYBRID_SEARCH_SEMANTIC_WEIGHT")
                .unwrap_or_else(|_| HYBRID_SEARCH_SEMANTIC_WEIGHT.to_string())
                .parse()
                .unwrap_or(HYBRID_SEARCH_SEMANTIC_WEIGHT as f32),
            bm25_k1: std::env::var("HYBRID_SEARCH_BM25_K1")
                .unwrap_or_else(|_| HYBRID_SEARCH_BM25_K1.to_string())
                .parse()
                .unwrap_or(HYBRID_SEARCH_BM25_K1 as f32),
            bm25_b: std::env::var("HYBRID_SEARCH_BM25_B")
                .unwrap_or_else(|_| HYBRID_SEARCH_BM25_B.to_string())
                .parse()
                .unwrap_or(HYBRID_SEARCH_BM25_B as f32),
        }
    }

    /// Create a new config with custom weights
    pub fn with_weights(bm25_weight: f32, semantic_weight: f32) -> Self {
        Self {
            bm25_weight,
            semantic_weight,
            ..Default::default()
        }
    }

    /// Create a semantic-only config (no BM25)
    pub fn semantic_only() -> Self {
        Self {
            bm25_weight: 0.0,
            semantic_weight: 1.0,
            ..Default::default()
        }
    }

    /// Create a BM25-only config (no semantic)
    pub fn bm25_only() -> Self {
        Self {
            bm25_weight: 1.0,
            semantic_weight: 0.0,
            ..Default::default()
        }
    }
}
