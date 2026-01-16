//! BM25 text ranking algorithm implementation
//!
//! BM25 (Best Matching 25) is a ranking function used for information retrieval.
//! It ranks documents based on the query terms appearing in each document.

use crate::constants::BM25_TOKEN_MIN_LENGTH;
use mcb_domain::entities::CodeChunk;
use std::collections::{HashMap, HashSet};

/// BM25 parameters
#[derive(Debug, Clone)]
pub struct BM25Params {
    /// k1 parameter (term frequency saturation)
    pub k1: f32,
    /// b parameter (document length normalization)
    pub b: f32,
}

impl Default for BM25Params {
    fn default() -> Self {
        use crate::constants::{HYBRID_SEARCH_BM25_B, HYBRID_SEARCH_BM25_K1};
        Self {
            k1: HYBRID_SEARCH_BM25_K1 as f32,
            b: HYBRID_SEARCH_BM25_B as f32,
        }
    }
}

/// BM25 scorer for text-based ranking
#[derive(Debug)]
pub struct BM25Scorer {
    /// Document frequencies for each term
    pub document_freq: HashMap<String, usize>,
    /// Total number of documents
    pub total_docs: usize,
    /// Average document length
    pub avg_doc_len: f32,
    /// BM25 parameters
    pub params: BM25Params,
}

impl BM25Scorer {
    /// Create a new BM25 scorer from a collection of documents
    pub fn new(documents: &[CodeChunk], params: BM25Params) -> Self {
        let total_docs = documents.len();
        let mut document_freq = HashMap::new();
        let mut total_length = 0.0;

        // Calculate document frequencies and total length
        for doc in documents {
            let tokens = Self::tokenize(&doc.content);
            let doc_length = tokens.len() as f32;
            total_length += doc_length;

            let mut unique_terms = HashSet::new();
            for token in tokens {
                unique_terms.insert(token);
            }

            for term in unique_terms {
                *document_freq.entry(term).or_insert(0) += 1;
            }
        }

        let avg_doc_len = if total_docs > 0 {
            total_length / total_docs as f32
        } else {
            0.0
        };

        Self {
            document_freq,
            total_docs,
            avg_doc_len,
            params,
        }
    }

    /// Score a document against a query using BM25
    pub fn score(&self, document: &CodeChunk, query: &str) -> f32 {
        let query_terms = Self::tokenize(query);
        self.score_with_tokens(document, &query_terms)
    }

    /// Score a document with pre-tokenized query terms (optimized for batch operations)
    ///
    /// This method avoids re-tokenizing the query for each document, improving performance
    /// when scoring multiple documents against the same query.
    pub fn score_with_tokens(&self, document: &CodeChunk, query_terms: &[String]) -> f32 {
        let doc_terms = Self::tokenize(&document.content);
        let doc_length = doc_terms.len() as f32;

        let mut score = 0.0;
        let mut doc_term_freq = HashMap::new();

        // Count term frequencies in document
        for term in &doc_terms {
            *doc_term_freq.entry(term.clone()).or_insert(0) += 1;
        }

        // Calculate BM25 score for each query term
        for query_term in query_terms {
            let tf = *doc_term_freq.get(query_term).unwrap_or(&0) as f32;
            let df = *self.document_freq.get(query_term).unwrap_or(&0) as f32;

            if df > 0.0 {
                // BM25+ IDF formula: ensures positive values even for common terms
                // ln(1 + (N - df + 0.5) / (df + 0.5)) is always > 0
                let idf = if self.total_docs > 1 {
                    (1.0 + (self.total_docs as f32 - df + 0.5) / (df + 0.5)).ln()
                } else {
                    // Simplified IDF for single document (always positive)
                    1.0
                };

                let tf_normalized = (tf * (self.params.k1 + 1.0))
                    / (tf
                        + self.params.k1
                            * (1.0 - self.params.b
                                + self.params.b * doc_length / self.avg_doc_len));

                score += idf * tf_normalized;
            }
        }

        score
    }

    /// Score multiple documents with a single tokenization pass (batch optimization)
    ///
    /// This is more efficient than calling `score()` for each document because
    /// the query is tokenized only once.
    pub fn score_batch(&self, documents: &[&CodeChunk], query: &str) -> Vec<f32> {
        let query_terms = Self::tokenize(query);
        documents
            .iter()
            .map(|doc| self.score_with_tokens(doc, &query_terms))
            .collect()
    }

    /// Tokenize text into terms (split on whitespace, underscores, and non-alphanumeric)
    pub fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty() && s.len() > BM25_TOKEN_MIN_LENGTH)
            .map(|s| s.to_string())
            .collect()
    }
}
