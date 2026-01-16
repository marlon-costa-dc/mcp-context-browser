//! Repository for managing search operations
//!
//! This module provides search functionality over indexed code chunks.
//! Supports both semantic vector search and hybrid search combining
//! semantic similarity with keyword-based (BM25) relevance.

use mcb_domain::entities::CodeChunk;
use mcb_domain::error::Result;
use mcb_domain::ports::VectorStoreProvider;
use mcb_domain::repositories::search_repository::SearchStats;
use mcb_domain::repositories::SearchRepository;
use mcb_domain::value_objects::SearchResult;

use crate::utils::TimedOperation;

use async_trait::async_trait;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

// TODO: Import HybridSearchEngine when hybrid_search module is migrated (Phase 1.5)
// use crate::adapters::hybrid_search::HybridSearchEngine;

/// Vector store backed search repository with hybrid search support
#[derive(shaku::Component)]
#[shaku(interface = SearchRepository)]
pub struct VectorStoreSearchRepository {
    /// Provider for vector storage operations
    #[shaku(inject)]
    vector_store_provider: Arc<dyn VectorStoreProvider>,
    // TODO: Add hybrid engine when migrated (Phase 1.5)
    // #[shaku(default = Arc::new(RwLock::new(HybridSearchEngine::new(0.3, 0.7))))]
    // hybrid_engine: Arc<RwLock<HybridSearchEngine>>,
    /// Search statistics tracker
    #[shaku(default)]
    stats: SearchStatsTracker,
}

/// Tracks search statistics using atomic counters
pub struct SearchStatsTracker {
    total_queries: AtomicU64,
    total_response_time_ms: AtomicU64,
    cache_hits: AtomicU64,
    indexed_documents: AtomicU64,
}

impl Default for SearchStatsTracker {
    fn default() -> Self {
        Self {
            total_queries: AtomicU64::new(0),
            total_response_time_ms: AtomicU64::new(0),
            cache_hits: AtomicU64::new(0),
            indexed_documents: AtomicU64::new(0),
        }
    }
}

impl VectorStoreSearchRepository {
    /// Create a new vector store search repository
    ///
    /// # Arguments
    /// * `vector_store_provider` - Provider for vector storage operations
    pub fn new(vector_store_provider: Arc<dyn VectorStoreProvider>) -> Self {
        Self {
            vector_store_provider,
            // TODO: Initialize hybrid engine when migrated
            stats: SearchStatsTracker::default(),
        }
    }

    fn collection_name(&self, collection: &str) -> String {
        format!("mcp_chunks_{}", collection)
    }
}

#[async_trait]
impl SearchRepository for VectorStoreSearchRepository {
    async fn semantic_search(
        &self,
        collection: &str,
        query_vector: &[f32],
        limit: usize,
        filter: Option<&str>,
    ) -> Result<Vec<SearchResult>> {
        let collection_name = self.collection_name(collection);

        if !self
            .vector_store_provider
            .collection_exists(&collection_name)
            .await?
        {
            return Ok(vec![]);
        }

        self.vector_store_provider
            .search_similar(&collection_name, query_vector, limit, filter)
            .await
    }

    async fn index_for_hybrid_search(&self, chunks: &[CodeChunk]) -> Result<()> {
        // TODO: Implement when HybridSearchEngine is migrated (Phase 1.5)
        // For now, just track the document count
        self.stats
            .indexed_documents
            .fetch_add(chunks.len() as u64, Ordering::Relaxed);
        Ok(())
    }

    async fn hybrid_search(
        &self,
        collection: &str,
        _query: &str,
        query_vector: &[f32],
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        let timer = TimedOperation::start();
        self.stats.total_queries.fetch_add(1, Ordering::Relaxed);

        // TODO: Implement proper hybrid search when HybridSearchEngine is migrated
        // For now, fall back to pure semantic search
        let results = self
            .semantic_search(collection, query_vector, limit, None)
            .await?;

        self.stats
            .total_response_time_ms
            .fetch_add(timer.elapsed_ms(), Ordering::Relaxed);

        Ok(results)
    }

    async fn clear_index(&self, collection: &str) -> Result<()> {
        let collection_name = self.collection_name(collection);
        if self
            .vector_store_provider
            .collection_exists(&collection_name)
            .await?
        {
            self.vector_store_provider
                .delete_collection(&collection_name)
                .await?;
        }

        // TODO: Clear hybrid engine when migrated
        self.stats.indexed_documents.store(0, Ordering::Relaxed);
        Ok(())
    }

    async fn stats(&self) -> Result<SearchStats> {
        let total_queries = self.stats.total_queries.load(Ordering::Relaxed);
        let total_time = self.stats.total_response_time_ms.load(Ordering::Relaxed);
        let cache_hits = self.stats.cache_hits.load(Ordering::Relaxed);
        let indexed_docs = self.stats.indexed_documents.load(Ordering::Relaxed);

        let avg_response_time = if total_queries > 0 {
            total_time as f64 / total_queries as f64
        } else {
            0.0
        };

        let cache_hit_rate = if total_queries > 0 {
            cache_hits as f64 / total_queries as f64
        } else {
            0.0
        };

        Ok(SearchStats {
            total_queries,
            avg_response_time_ms: avg_response_time,
            cache_hit_rate,
            indexed_documents: indexed_docs,
        })
    }
}
