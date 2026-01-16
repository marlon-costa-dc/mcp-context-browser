//! Factory tests
//!
//! Tests for embedding and vector store provider factories.

use mcb_domain::value_objects::{EmbeddingConfig, VectorStoreConfig};
use mcb_infrastructure::adapters::providers::factory::{
    EmbeddingProviderFactory, VectorStoreProviderFactory,
};

#[test]
fn test_create_null_embedding_provider() {
    let config = EmbeddingConfig {
        provider: "null".to_string(),
        model: "test".to_string(),
        api_key: None,
        base_url: None,
        dimensions: None,
        max_tokens: None,
    };

    let provider = EmbeddingProviderFactory::create(&config, None);
    assert!(provider.is_ok());
    assert_eq!(provider.unwrap().provider_name(), "null");
}

#[test]
fn test_create_null_vector_store_provider() {
    let config = VectorStoreConfig {
        provider: "null".to_string(),
        address: None,
        token: None,
        collection: None,
        dimensions: None,
        timeout_secs: None,
    };

    let provider = VectorStoreProviderFactory::create(&config, None);
    assert!(provider.is_ok());
}

#[test]
fn test_create_in_memory_vector_store_provider() {
    let config = VectorStoreConfig {
        provider: "in_memory".to_string(),
        address: None,
        token: None,
        collection: None,
        dimensions: None,
        timeout_secs: None,
    };

    let provider = VectorStoreProviderFactory::create(&config, None);
    assert!(provider.is_ok());
}

#[test]
fn test_unknown_embedding_provider() {
    let config = EmbeddingConfig {
        provider: "unknown_provider".to_string(),
        model: "test".to_string(),
        api_key: None,
        base_url: None,
        dimensions: None,
        max_tokens: None,
    };

    let provider = EmbeddingProviderFactory::create(&config, None);
    assert!(provider.is_err());
}

#[test]
fn test_unknown_vector_store_provider() {
    let config = VectorStoreConfig {
        provider: "unknown_provider".to_string(),
        address: None,
        token: None,
        collection: None,
        dimensions: None,
        timeout_secs: None,
    };

    let provider = VectorStoreProviderFactory::create(&config, None);
    assert!(provider.is_err());
}
