//! Code Intelligence Business Service
//!
//! The Context Service transforms raw code into semantic understanding through
//! AI embeddings and intelligent storage. This business service powers the core
//! intelligence behind semantic code search, enabling development teams to find
//! code by meaning rather than keywords.

use crate::domain::error::Result;
use crate::domain::ports::{EmbeddingProvider, HybridSearchProvider, VectorStoreProvider};
use crate::domain::types::{CodeChunk, Embedding, SearchResult};
use std::collections::HashMap;
use std::sync::Arc;

/// Enterprise Code Intelligence Coordinator
pub struct ContextService {
    embedding_provider: Arc<dyn EmbeddingProvider>,
    vector_store_provider: Arc<dyn VectorStoreProvider>,
    hybrid_search_provider: Arc<dyn HybridSearchProvider>,
}

impl ContextService {
    /// Create a new context service with specified providers
    pub fn new(
        embedding_provider: Arc<dyn EmbeddingProvider>,
        vector_store_provider: Arc<dyn VectorStoreProvider>,
        hybrid_search_provider: Arc<dyn HybridSearchProvider>,
    ) -> Self {
        Self {
            embedding_provider,
            vector_store_provider,
            hybrid_search_provider,
        }
    }

    /// Generate embeddings for text
    pub async fn embed_text(&self, text: &str) -> Result<Embedding> {
        self.embedding_provider.embed(text).await
    }

    /// Generate embeddings for multiple texts
    pub async fn embed_texts(&self, texts: &[String]) -> Result<Vec<Embedding>> {
        self.embedding_provider.embed_batch(texts).await
    }

    /// Store code chunks in vector database
    pub async fn store_chunks(&self, collection: &str, chunks: &[CodeChunk]) -> Result<()> {
        let texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
        let embeddings = self.embed_texts(&texts).await?;

        // Prepare metadata for each chunk
        let metadata: Vec<HashMap<String, serde_json::Value>> = chunks
            .iter()
            .map(|chunk| {
                let mut meta = HashMap::new();
                meta.insert("content".to_string(), serde_json::json!(chunk.content));
                meta.insert("file_path".to_string(), serde_json::json!(chunk.file_path));
                meta.insert(
                    "start_line".to_string(),
                    serde_json::json!(chunk.start_line),
                );
                meta.insert("end_line".to_string(), serde_json::json!(chunk.end_line));
                meta.insert(
                    "language".to_string(),
                    serde_json::json!(format!("{:?}", chunk.language)),
                );
                meta
            })
            .collect();

        // Ensure collection exists
        if !self
            .vector_store_provider
            .collection_exists(collection)
            .await?
        {
            self.vector_store_provider
                .create_collection(collection, self.embedding_dimensions())
                .await?;
        }

        self.vector_store_provider
            .insert_vectors(collection, &embeddings, metadata)
            .await?;

        // Index documents for hybrid search (BM25) via Provider
        self.hybrid_search_provider
            .index_chunks(collection, chunks)
            .await?;

        Ok(())
    }

    /// Search for similar code chunks using hybrid search (BM25 + semantic embeddings)
    pub async fn search_similar(
        &self,
        collection: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        let query_embedding = self.embed_text(query).await?;

        // Get semantic search results
        let expanded_limit = (limit * 2).clamp(20, 100);
        let semantic_results = self
            .vector_store_provider
            .search_similar(collection, &query_embedding.vector, expanded_limit, None)
            .await?;

        let semantic_search_results: Vec<SearchResult> = semantic_results
            .into_iter()
            .map(|result| SearchResult {
                id: result.id.clone(),
                file_path: result
                    .metadata
                    .get("file_path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
                line_number: result
                    .metadata
                    .get("start_line")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32,
                content: result
                    .metadata
                    .get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                score: result.score,
                metadata: result.metadata,
            })
            .collect();

        // Request hybrid search from Provider
        self.hybrid_search_provider
            .search(collection, query, semantic_search_results, limit)
            .await
    }

    /// Clear a collection
    pub async fn clear_collection(&self, collection: &str) -> Result<()> {
        self.vector_store_provider
            .delete_collection(collection)
            .await?;

        self.hybrid_search_provider
            .clear_collection(collection)
            .await?;

        Ok(())
    }

    /// Get embedding dimensions
    pub fn embedding_dimensions(&self) -> usize {
        self.embedding_provider.dimensions()
    }

    /// Get hybrid search statistics
    pub async fn get_hybrid_search_stats(&self) -> HashMap<String, serde_json::Value> {
        self.hybrid_search_provider.get_stats().await
    }
}

/// Generic context service using Strategy pattern with trait bounds
pub struct GenericContextService<E, V, H>
where
    E: EmbeddingProvider + Send + Sync,
    V: VectorStoreProvider + Send + Sync,
    H: HybridSearchProvider + Send + Sync,
{
    embedding_provider: Arc<E>,
    vector_store_provider: Arc<V>,
    hybrid_search_provider: Arc<H>,
}

impl<E, V, H> GenericContextService<E, V, H>
where
    E: EmbeddingProvider + Send + Sync,
    V: VectorStoreProvider + Send + Sync,
    H: HybridSearchProvider + Send + Sync,
{
    /// Create a new generic context service with specified provider strategies
    pub fn new(
        embedding_provider: Arc<E>,
        vector_store_provider: Arc<V>,
        hybrid_search_provider: Arc<H>,
    ) -> Self {
        Self {
            embedding_provider,
            vector_store_provider,
            hybrid_search_provider,
        }
    }

    /// Generate embeddings for text
    pub async fn embed_text(&self, text: &str) -> Result<Embedding> {
        self.embedding_provider.embed(text).await
    }

    /// Generate embeddings for multiple texts
    pub async fn embed_texts(&self, texts: &[String]) -> Result<Vec<Embedding>> {
        self.embedding_provider.embed_batch(texts).await
    }

    /// Search for similar code chunks
    pub async fn search_similar(
        &self,
        collection: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        let query_embedding = self.embedding_provider.embed(query).await?;

        let expanded_limit = (limit * 2).clamp(20, 100);
        let semantic_results = self
            .vector_store_provider
            .search_similar(collection, &query_embedding.vector, expanded_limit, None)
            .await?;

        let semantic_search_results: Vec<SearchResult> = semantic_results
            .into_iter()
            .map(|result| SearchResult {
                id: result.id.clone(),
                file_path: result
                    .metadata
                    .get("file_path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("unknown")
                    .to_string(),
                line_number: result
                    .metadata
                    .get("start_line")
                    .and_then(|v| v.as_u64())
                    .unwrap_or(0) as u32,
                content: result
                    .metadata
                    .get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                score: result.score,
                metadata: result.metadata,
            })
            .collect();

        self.hybrid_search_provider
            .search(collection, query, semantic_search_results, limit)
            .await
    }

    /// Get embedding dimensions
    pub fn embedding_dimensions(&self) -> usize {
        self.embedding_provider.dimensions()
    }

    /// Get hybrid search statistics
    pub async fn get_hybrid_search_stats(&self) -> HashMap<String, serde_json::Value> {
        self.hybrid_search_provider.get_stats().await
    }
}

/// Repository-based context service using Repository pattern
pub struct RepositoryContextService<C, S, E>
where
    C: crate::adapters::repository::ChunkRepository + Send + Sync,
    S: crate::adapters::repository::SearchRepository + Send + Sync,
    E: EmbeddingProvider + Send + Sync,
{
    chunk_repository: Arc<C>,
    search_repository: Arc<S>,
    embedding_provider: Arc<E>,
}

impl<C, S, E> RepositoryContextService<C, S, E>
where
    C: crate::adapters::repository::ChunkRepository + Send + Sync,
    S: crate::adapters::repository::SearchRepository + Send + Sync,
    E: EmbeddingProvider + Send + Sync,
{
    /// Create a new repository-based context service
    pub fn new(
        chunk_repository: Arc<C>,
        search_repository: Arc<S>,
        embedding_provider: Arc<E>,
    ) -> Self {
        Self {
            chunk_repository,
            search_repository,
            embedding_provider,
        }
    }

    /// Generate embeddings for text
    pub async fn embed_text(&self, text: &str) -> Result<Embedding> {
        self.embedding_provider.embed(text).await
    }

    /// Store code chunks using the chunk repository
    pub async fn store_chunks(&self, _collection: &str, chunks: &[CodeChunk]) -> Result<()> {
        self.chunk_repository.save_batch(chunks).await?;
        self.search_repository
            .index_for_hybrid_search(chunks)
            .await?;
        Ok(())
    }

    /// Search for similar code chunks using repository-based search
    pub async fn search_similar(
        &self,
        collection: &str,
        query: &str,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        let query_embedding = self.embedding_provider.embed(query).await?;
        self.search_repository
            .hybrid_search(collection, query, &query_embedding.vector, limit)
            .await
    }

    /// Clear a collection using repositories
    pub async fn clear_collection(&self, collection: &str) -> Result<()> {
        self.chunk_repository.delete_collection(collection).await?;
        self.search_repository.clear_index(collection).await?;
        Ok(())
    }

    /// Get repository statistics
    pub async fn get_repository_stats(
        &self,
    ) -> Result<(
        crate::adapters::repository::RepositoryStats,
        crate::adapters::repository::SearchStats,
    )> {
        let chunk_stats = self.chunk_repository.stats().await?;
        let search_stats = self.search_repository.search_stats().await?;
        Ok((chunk_stats, search_stats))
    }
}

impl Default for ContextService {
    fn default() -> Self {
        let embedding_provider =
            Arc::new(crate::adapters::providers::embedding::MockEmbeddingProvider::new());
        let vector_store_provider =
            Arc::new(crate::adapters::providers::vector_store::InMemoryVectorStoreProvider::new());

        // Default hybrid search implementation
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        let actor = crate::adapters::hybrid_search::HybridSearchActor::new(receiver, 0.4, 0.6);
        tokio::spawn(async move {
            actor.run().await;
        });
        let hybrid_search_provider = Arc::new(
            crate::adapters::hybrid_search::HybridSearchAdapter::new(sender),
        );

        Self::new(
            embedding_provider,
            vector_store_provider,
            hybrid_search_provider,
        )
    }
}
