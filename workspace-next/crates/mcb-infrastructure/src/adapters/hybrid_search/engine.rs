//! Hybrid search engine combining BM25 and semantic search
//!
//! This module provides the core engine that combines BM25 text-based ranking
//! with semantic similarity scores for improved search relevance.

use mcb_domain::entities::CodeChunk;
use mcb_domain::error::Result;
use mcb_domain::ports::providers::hybrid_search::HybridSearchResult;
use mcb_domain::value_objects::SearchResult;
use std::collections::HashMap;

use super::bm25::{BM25Params, BM25Scorer};

/// Hybrid search engine combining BM25 and semantic search
#[derive(Debug)]
pub struct HybridSearchEngine {
    /// BM25 scorer
    pub bm25_scorer: Option<BM25Scorer>,
    /// Collection of indexed documents for BM25 scoring
    pub documents: Vec<CodeChunk>,
    /// Weight for BM25 score in hybrid combination (0-1)
    pub bm25_weight: f32,
    /// Weight for semantic score in hybrid combination (0-1)
    pub semantic_weight: f32,
    /// Cached document index mapping (file_path:start_line -> document index)
    /// This avoids rebuilding a HashMap on every search operation.
    document_index: HashMap<String, usize>,
}

impl HybridSearchEngine {
    /// Create a new hybrid search engine
    pub fn new(bm25_weight: f32, semantic_weight: f32) -> Self {
        Self {
            bm25_scorer: None,
            documents: Vec::new(),
            bm25_weight,
            semantic_weight,
            document_index: HashMap::new(),
        }
    }

    /// Add documents for BM25 scoring
    ///
    /// Documents are deduplicated by their file_path:start_line key.
    /// The document index is maintained incrementally for O(1) lookups during search.
    pub fn add_documents(&mut self, new_documents: Vec<CodeChunk>) {
        for doc in new_documents {
            let key = format!("{}:{}", doc.file_path, doc.start_line);
            // Use the cached index for O(1) deduplication check
            if !self.document_index.contains_key(&key) {
                let idx = self.documents.len();
                self.document_index.insert(key, idx);
                self.documents.push(doc);
            }
        }
        self.bm25_scorer = Some(BM25Scorer::new(&self.documents, BM25Params::default()));
    }

    /// Clear all documents and reset BM25 index
    pub fn clear(&mut self) {
        self.documents.clear();
        self.document_index.clear();
        self.bm25_scorer = None;
    }

    /// Perform hybrid search combining BM25 and semantic similarity
    ///
    /// This implementation is optimized to:
    /// - Use the cached document_index for O(1) lookups (no HashMap rebuild per search)
    /// - Pre-tokenize the query once and reuse for all BM25 scoring
    pub fn hybrid_search(
        &self,
        query: &str,
        semantic_results: Vec<SearchResult>,
        limit: usize,
    ) -> Result<Vec<HybridSearchResult>> {
        if self.bm25_scorer.is_none() {
            // Fallback to semantic-only search if BM25 is not indexed
            return Ok(semantic_results
                .into_iter()
                .take(limit)
                .map(|result| HybridSearchResult {
                    bm25_score: 0.0,
                    semantic_score: result.score as f32,
                    hybrid_score: result.score as f32,
                    result,
                })
                .collect());
        }

        let bm25_scorer = self
            .bm25_scorer
            .as_ref()
            .ok_or_else(|| mcb_domain::error::Error::internal("BM25 scorer not initialized"))?;

        // Pre-tokenize query once for all BM25 scoring operations
        let query_terms = BM25Scorer::tokenize(query);

        // Calculate hybrid scores for semantic results
        // Uses cached document_index for O(1) lookup instead of rebuilding HashMap
        let mut hybrid_results: Vec<HybridSearchResult> = semantic_results
            .into_iter()
            .map(|semantic_result| {
                let doc_key = format!(
                    "{}:{}",
                    semantic_result.file_path, semantic_result.start_line
                );
                let semantic_score = semantic_result.score as f32;

                // Use cached index for O(1) lookup, then reference document by index
                if let Some(&doc_idx) = self.document_index.get(&doc_key) {
                    let document = &self.documents[doc_idx];
                    // Use pre-tokenized query for efficient batch scoring
                    let bm25_score = bm25_scorer.score_with_tokens(document, &query_terms);

                    // Normalize BM25 score to 0-1 range (simple min-max normalization)
                    let normalized_bm25 = if bm25_score > 0.0 {
                        1.0 / (1.0 + (-bm25_score).exp()) // Sigmoid normalization
                    } else {
                        0.0
                    };

                    // Combine scores using weighted average
                    let hybrid_score =
                        self.bm25_weight * normalized_bm25 + self.semantic_weight * semantic_score;

                    HybridSearchResult {
                        result: semantic_result,
                        bm25_score,
                        semantic_score,
                        hybrid_score,
                    }
                } else {
                    // If document not found for BM25, use semantic score only
                    let hybrid_score = self.semantic_weight * semantic_score;
                    HybridSearchResult {
                        result: semantic_result,
                        bm25_score: 0.0,
                        semantic_score,
                        hybrid_score,
                    }
                }
            })
            .collect();

        // Sort by hybrid score (descending)
        hybrid_results.sort_by(|a, b| {
            b.hybrid_score
                .partial_cmp(&a.hybrid_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Return top results
        Ok(hybrid_results.into_iter().take(limit).collect())
    }

    /// Check if BM25 index is available
    pub fn has_bm25_index(&self) -> bool {
        self.bm25_scorer.is_some()
    }

    /// Get BM25 statistics
    pub fn get_bm25_stats(&self) -> Option<HashMap<String, serde_json::Value>> {
        self.bm25_scorer.as_ref().map(|scorer| {
            let mut stats = HashMap::new();
            stats.insert(
                "total_documents".to_string(),
                serde_json::json!(scorer.total_docs),
            );
            stats.insert(
                "unique_terms".to_string(),
                serde_json::json!(scorer.document_freq.len()),
            );
            stats.insert(
                "average_doc_length".to_string(),
                serde_json::json!(scorer.avg_doc_len),
            );
            stats.insert("bm25_k1".to_string(), serde_json::json!(scorer.params.k1));
            stats.insert("bm25_b".to_string(), serde_json::json!(scorer.params.b));
            stats
        })
    }
}

impl Default for HybridSearchEngine {
    fn default() -> Self {
        Self {
            bm25_scorer: None,
            documents: Vec::new(),
            bm25_weight: 0.4,     // 40% BM25
            semantic_weight: 0.6, // 60% semantic
            document_index: HashMap::new(),
        }
    }
}
