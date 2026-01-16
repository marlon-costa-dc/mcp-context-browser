//! Tests for InMemoryVectorStoreProvider

use mcb_domain::ports::providers::VectorStoreProvider;
use mcb_domain::value_objects::Embedding;
use mcb_infrastructure::adapters::providers::vector_store::InMemoryVectorStoreProvider;
use std::collections::HashMap;

#[tokio::test]
async fn test_in_memory_create_and_exists() {
    let provider = InMemoryVectorStoreProvider::new();

    provider.create_collection("test", 384).await.unwrap();
    assert!(provider.collection_exists("test").await.unwrap());

    provider.delete_collection("test").await.unwrap();
    assert!(!provider.collection_exists("test").await.unwrap());
}

#[tokio::test]
async fn test_in_memory_insert_and_search() {
    let provider = InMemoryVectorStoreProvider::new();

    provider.create_collection("test", 3).await.unwrap();

    let embedding = Embedding {
        vector: vec![1.0, 0.0, 0.0],
        model: "test-model".to_string(),
        dimensions: 3,
    };

    let mut metadata = HashMap::new();
    metadata.insert("file_path".to_string(), serde_json::json!("test.rs"));
    metadata.insert("content".to_string(), serde_json::json!("test content"));

    let ids = provider
        .insert_vectors("test", &[embedding], vec![metadata])
        .await
        .unwrap();

    assert_eq!(ids.len(), 1);

    let results = provider
        .search_similar("test", &[1.0, 0.0, 0.0], 10, None)
        .await
        .unwrap();

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].file_path, "test.rs");
}

#[tokio::test]
async fn test_in_memory_stats() {
    let provider = InMemoryVectorStoreProvider::new();

    provider.create_collection("test", 3).await.unwrap();

    let stats = provider.get_stats("test").await.unwrap();
    assert_eq!(stats.get("vectors_count").unwrap(), 0);

    let embedding = Embedding {
        vector: vec![1.0, 0.0, 0.0],
        model: "test-model".to_string(),
        dimensions: 3,
    };

    provider
        .insert_vectors("test", &[embedding], vec![HashMap::new()])
        .await
        .unwrap();

    let stats = provider.get_stats("test").await.unwrap();
    assert_eq!(stats.get("vectors_count").unwrap(), 1);
}
