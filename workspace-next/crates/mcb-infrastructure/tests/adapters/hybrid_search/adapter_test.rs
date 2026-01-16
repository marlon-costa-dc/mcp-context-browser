//! Hybrid Search Adapter Tests

use mcb_domain::entities::CodeChunk;
use mcb_domain::ports::providers::HybridSearchProvider;
use mcb_domain::value_objects::SearchResult;
use mcb_infrastructure::adapters::hybrid_search::{spawn_hybrid_search_actor, HybridSearchAdapter};
use serde_json::json;

fn create_test_chunk(id: &str, content: &str) -> CodeChunk {
    CodeChunk {
        id: id.to_string(),
        content: content.to_string(),
        file_path: "test.rs".to_string(),
        start_line: 1,
        end_line: 10,
        language: "rust".to_string(),
        metadata: json!({}),
    }
}

#[tokio::test]
async fn test_adapter_index_chunks() {
    let sender = spawn_hybrid_search_actor(0.4, 0.6);
    let adapter = HybridSearchAdapter::new(sender);

    let chunks = vec![create_test_chunk("1", "fn test_function()")];
    adapter
        .index_chunks("test", &chunks)
        .await
        .expect("indexing should succeed");
}

#[tokio::test]
async fn test_adapter_clear_collection() {
    let sender = spawn_hybrid_search_actor(0.4, 0.6);
    let adapter = HybridSearchAdapter::new(sender);

    adapter
        .clear_collection("test")
        .await
        .expect("clear should succeed");
}

#[tokio::test]
async fn test_adapter_get_stats() {
    let sender = spawn_hybrid_search_actor(0.4, 0.6);
    let adapter = HybridSearchAdapter::new(sender);

    let stats = adapter.get_stats().await;
    assert!(stats.contains_key("hybrid_search_enabled"));
}

#[tokio::test]
async fn test_adapter_search() {
    let sender = spawn_hybrid_search_actor(0.4, 0.6);
    let adapter = HybridSearchAdapter::new(sender);

    // Index first
    let chunks = vec![create_test_chunk(
        "1",
        "fn authenticate_user(username: &str)",
    )];
    adapter
        .index_chunks("test", &chunks)
        .await
        .expect("indexing should succeed");

    // Search with semantic results
    let semantic_results = vec![SearchResult {
        id: "test.rs:1".to_string(),
        content: "fn authenticate_user(username: &str)".to_string(),
        file_path: "test.rs".to_string(),
        start_line: 1,
        score: 0.8,
        language: "rust".to_string(),
    }];

    let results = adapter
        .search("test", "authenticate user", semantic_results, 10)
        .await
        .expect("search should succeed");

    assert_eq!(results.len(), 1);
    // Result should have hybrid score as the score
    assert!(results[0].score > 0.0);
}
