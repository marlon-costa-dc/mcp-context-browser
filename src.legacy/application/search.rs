//! Search Service
//!
//! Provides semantic code search across indexed collections.
//!
//! The search service delegates to [`ContextServiceInterface`] for actual
//! vector similarity search, providing a focused API for search operations.
//!
//! # Example
//!
//! ```rust,no_run
//! use std::sync::Arc;
//! use mcp_context_browser::application::SearchService;
//! use mcp_context_browser::domain::ports::ContextServiceInterface;
//!
//! async fn example(context: Arc<dyn ContextServiceInterface>) -> anyhow::Result<()> {
//!     let search = SearchService::new(context);
//!     let results = search.search("collection", "error handling", 5).await?;
//!     println!("Found {} results", results.len());
//!     Ok(())
//! }
//! ```

use crate::domain::error::Result;
use crate::domain::ports::{ContextServiceInterface, SearchServiceInterface};
use crate::domain::types::SearchResult;
use async_trait::async_trait;
use std::sync::Arc;

/// Simple search service for MVP
#[derive(shaku::Component)]
#[shaku(interface = SearchServiceInterface)]
pub struct SearchService {
    #[shaku(inject)]
    context_service: Arc<dyn ContextServiceInterface>,
}

impl SearchService {
    /// Create a new search service
    pub fn new(context_service: Arc<dyn ContextServiceInterface>) -> Self {
        Self { context_service }
    }

    /// Search for code similar to the query
    pub async fn search(
        &self,
        collection: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        self.context_service
            .search_similar(collection, query, limit)
            .await
    }
}

#[async_trait]
impl SearchServiceInterface for SearchService {
    async fn search(
        &self,
        collection: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        self.search(collection, query, limit).await
    }
}
