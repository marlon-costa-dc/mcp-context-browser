//! Tests for NullVectorStoreProvider

use mcb_domain::ports::providers::{VectorStoreAdmin, VectorStoreProvider};
use mcb_infrastructure::adapters::providers::vector_store::NullVectorStoreProvider;

#[tokio::test]
async fn test_null_provider_create_collection() {
    let provider = NullVectorStoreProvider::new();

    // Should create collection successfully
    provider.create_collection("test", 384).await.unwrap();

    // Should report collection exists
    assert!(provider.collection_exists("test").await.unwrap());

    // Should fail on duplicate
    assert!(provider.create_collection("test", 384).await.is_err());
}

#[tokio::test]
async fn test_null_provider_delete_collection() {
    let provider = NullVectorStoreProvider::new();

    provider.create_collection("test", 384).await.unwrap();
    provider.delete_collection("test").await.unwrap();

    assert!(!provider.collection_exists("test").await.unwrap());
}

#[tokio::test]
async fn test_null_provider_returns_empty_results() {
    let provider = NullVectorStoreProvider::new();

    let results = provider
        .search_similar("test", &[0.1, 0.2, 0.3], 10, None)
        .await
        .unwrap();

    assert!(results.is_empty());
}

#[tokio::test]
async fn test_null_provider_stats() {
    let provider = NullVectorStoreProvider::new();

    let stats = provider.get_stats("test").await.unwrap();

    assert_eq!(stats.get("provider").unwrap(), "null");
    assert_eq!(stats.get("vectors_count").unwrap(), 0);
}
