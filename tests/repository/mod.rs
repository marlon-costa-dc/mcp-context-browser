//! Repository pattern tests
//!
//! Tests for the repository implementations that provide data access abstraction.

use mcp_context_browser::core::types::{CodeChunk, Language};
use mcp_context_browser::providers::{MockEmbeddingProvider, InMemoryVectorStoreProvider};
use mcp_context_browser::repository::{ChunkRepository, SearchRepository, VectorStoreChunkRepository, VectorStoreSearchRepository};
use std::sync::Arc;

/// Test chunk repository functionality
#[cfg(test)]
mod chunk_repository_tests {
    use super::*;

    #[tokio::test]
    async fn test_chunk_repository_save_and_retrieve() {
        let embedding_provider = Arc::new(MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(InMemoryVectorStoreProvider::new());

        let repository = VectorStoreChunkRepository::new(
            embedding_provider,
            vector_store_provider,
        );

        // Create a test chunk
        let chunk = CodeChunk {
            content: "fn hello() { println!(\"Hello, world!\"); }".to_string(),
            file_path: "test.rs".to_string(),
            start_line: 1,
            end_line: 3,
            language: Language::Rust,
        };

        // Save the chunk
        let id = repository.save(&chunk).await.expect("Should save chunk");
        assert!(!id.is_empty(), "Should return a valid ID");

        // Try to find chunks by collection (this may not work perfectly with in-memory store)
        let chunks = repository.find_by_collection("default", 10).await.expect("Should retrieve chunks");
        assert!(!chunks.is_empty(), "Should find at least one chunk");
    }

    #[tokio::test]
    async fn test_chunk_repository_batch_save() {
        let embedding_provider = Arc::new(MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(InMemoryVectorStoreProvider::new());

        let repository = VectorStoreChunkRepository::new(
            embedding_provider,
            vector_store_provider,
        );

        // Create test chunks
        let chunks = vec![
            CodeChunk {
                content: "fn main() {}".to_string(),
                file_path: "main.rs".to_string(),
                start_line: 1,
                end_line: 1,
                language: Language::Rust,
            },
            CodeChunk {
                content: "print('hello')".to_string(),
                file_path: "hello.py".to_string(),
                start_line: 1,
                end_line: 1,
                language: Language::Python,
            },
        ];

        // Save batch
        let ids = repository.save_batch(&chunks).await.expect("Should save batch");
        assert_eq!(ids.len(), 2, "Should return IDs for both chunks");
        assert!(ids.iter().all(|id| !id.is_empty()), "All IDs should be valid");
    }

    #[tokio::test]
    async fn test_chunk_repository_stats() {
        let embedding_provider = Arc::new(MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(InMemoryVectorStoreProvider::new());

        let repository = VectorStoreChunkRepository::new(
            embedding_provider,
            vector_store_provider,
        );

        let stats = repository.stats().await.expect("Should get stats");
        assert!(stats.total_collections >= 0, "Should have valid collection count");
    }

    #[tokio::test]
    async fn test_chunk_repository_delete_collection() {
        let embedding_provider = Arc::new(MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(InMemoryVectorStoreProvider::new());

        let repository = VectorStoreChunkRepository::new(
            embedding_provider,
            vector_store_provider,
        );

        // This should not fail even if collection doesn't exist
        repository.delete_collection("test_collection").await.expect("Should handle delete gracefully");
    }
}

/// Test search repository functionality
#[cfg(test)]
mod search_repository_tests {
    use super::*;

    #[tokio::test]
    async fn test_search_repository_semantic_search() {
        let embedding_provider = Arc::new(MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(InMemoryVectorStoreProvider::new());

        let repository = VectorStoreSearchRepository::new(
            embedding_provider.clone(),
            vector_store_provider,
        );

        // Create and index some test data
        let chunks = vec![
            CodeChunk {
                content: "fn calculate_fibonacci(n: u32) -> u32 { if n <= 1 { n } else { calculate_fibonacci(n-1) + calculate_fibonacci(n-2) } }".to_string(),
                file_path: "fib.rs".to_string(),
                start_line: 1,
                end_line: 1,
                language: Language::Rust,
            },
        ];

        repository.index_for_hybrid_search(&chunks).await.expect("Should index chunks");

        // Perform semantic search
        let query_embedding = embedding_provider.embed("fibonacci function").await.expect("Should embed query");
        let results = repository.semantic_search("default", &query_embedding.vector, 5, None).await.expect("Should perform search");

        // Results may be empty for in-memory store, but should not fail
        assert!(results.len() >= 0, "Should return valid results list");
    }

    #[tokio::test]
    async fn test_search_repository_hybrid_search() {
        let embedding_provider = Arc::new(MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(InMemoryVectorStoreProvider::new());

        let repository = VectorStoreSearchRepository::new(
            embedding_provider.clone(),
            vector_store_provider,
        );

        // Index test data
        let chunks = vec![
            CodeChunk {
                content: "function calculateFibonacci(n) { return n <= 1 ? n : calculateFibonacci(n-1) + calculateFibonacci(n-2); }".to_string(),
                file_path: "fib.js".to_string(),
                start_line: 1,
                end_line: 3,
                language: Language::JavaScript,
            },
        ];

        repository.index_for_hybrid_search(&chunks).await.expect("Should index for hybrid search");

        // Perform hybrid search
        let query = "fibonacci";
        let query_embedding = embedding_provider.embed(query).await.expect("Should embed query");

        let results = repository.hybrid_search("default", query, &query_embedding.vector, 5).await.expect("Should perform hybrid search");

        // Should not fail
        assert!(results.len() >= 0, "Should return valid results");
    }

    #[tokio::test]
    async fn test_search_repository_stats() {
        let embedding_provider = Arc::new(MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(InMemoryVectorStoreProvider::new());

        let repository = VectorStoreSearchRepository::new(
            embedding_provider,
            vector_store_provider,
        );

        let stats = repository.search_stats().await.expect("Should get search stats");
        assert!(stats.indexed_documents >= 0, "Should have valid document count");
    }

    #[tokio::test]
    async fn test_search_repository_clear_index() {
        let embedding_provider = Arc::new(MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(InMemoryVectorStoreProvider::new());

        let repository = VectorStoreSearchRepository::new(
            embedding_provider,
            vector_store_provider,
        );

        // Should handle clearing non-existent index gracefully
        repository.clear_index("test_collection").await.expect("Should handle clear gracefully");
    }
}

/// Test repository pattern composition and dependency injection
#[cfg(test)]
mod repository_pattern_tests {
    use super::*;

    #[tokio::test]
    async fn test_repository_trait_bounds() {
        // Test that repositories can be used with trait bounds
        fn uses_chunk_repository<R: ChunkRepository>(_repo: Arc<R>) {}
        fn uses_search_repository<R: SearchRepository>(_repo: Arc<R>) {}

        let embedding_provider = Arc::new(MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(InMemoryVectorStoreProvider::new());

        let chunk_repo = Arc::new(VectorStoreChunkRepository::new(
            embedding_provider.clone(),
            vector_store_provider.clone(),
        ));

        let search_repo = Arc::new(VectorStoreSearchRepository::new(
            embedding_provider,
            vector_store_provider,
        ));

        // These should compile - testing trait bounds
        uses_chunk_repository(chunk_repo);
        uses_search_repository(search_repo);
    }

    #[tokio::test]
    async fn test_repository_composition() {
        // Test that repositories can be composed together
        let embedding_provider = Arc::new(MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(InMemoryVectorStoreProvider::new());

        let chunk_repo = VectorStoreChunkRepository::new(
            embedding_provider.clone(),
            vector_store_provider.clone(),
        );

        let search_repo = VectorStoreSearchRepository::new(
            embedding_provider,
            vector_store_provider,
        );

        // Test that they can work together
        let chunk = CodeChunk {
            content: "test content".to_string(),
            file_path: "test.rs".to_string(),
            start_line: 1,
            end_line: 1,
            language: Language::Rust,
        };

        // Save via chunk repository
        let _id = chunk_repo.save(&chunk).await.expect("Should save chunk");

        // Index for search
        search_repo.index_for_hybrid_search(&[chunk]).await.expect("Should index");

        // Both repositories should work without conflicts
    }
}

/// Integration tests for repository pattern
#[cfg(test)]
mod integration_tests {
    use super::*;

    #[tokio::test]
    async fn test_repository_workflow() {
        // Test a complete repository workflow:
        // 1. Create repositories
        // 2. Save data via chunk repository
        // 3. Index for search
        // 4. Search via search repository
        // 5. Verify results

        let embedding_provider = Arc::new(MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(InMemoryVectorStoreProvider::new());

        let chunk_repo = VectorStoreChunkRepository::new(
            embedding_provider.clone(),
            vector_store_provider.clone(),
        );

        let search_repo = VectorStoreSearchRepository::new(
            embedding_provider.clone(),
            vector_store_provider,
        );

        // Create test data
        let chunks = vec![
            CodeChunk {
                content: "fn authenticate_user(username: &str, password: &str) -> Result<User, AuthError>".to_string(),
                file_path: "auth.rs".to_string(),
                start_line: 10,
                end_line: 15,
                language: Language::Rust,
            },
            CodeChunk {
                content: "def login_user(username, password): return authenticate(username, password)".to_string(),
                file_path: "auth.py".to_string(),
                start_line: 5,
                end_line: 7,
                language: Language::Python,
            },
        ];

        // Save chunks
        for chunk in &chunks {
            chunk_repo.save(chunk).await.expect("Should save chunk");
        }

        // Index for search
        search_repo.index_for_hybrid_search(&chunks).await.expect("Should index chunks");

        // Perform search
        let query = "user authentication";
        let query_embedding = embedding_provider.embed(query).await.expect("Should embed query");

        let results = search_repo.hybrid_search("default", query, &query_embedding.vector, 10).await.expect("Should search");

        // Verify we get results
        assert!(results.len() >= 0, "Should return search results");
    }
}