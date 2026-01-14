//! Unit tests for configuration key constants
//!
//! Tests for type-safe configuration key management.

use mcp_context_browser::server::admin::config_keys::{
    cache, embedding, indexing, metrics, security, vector_store,
};

#[test]
fn test_config_keys_exist() {
    assert_eq!(indexing::CHUNK_SIZE, "indexing.chunk_size");
    assert_eq!(security::ENABLE_AUTH, "security.enable_auth");
    assert_eq!(metrics::ENABLED, "metrics.enabled");
    assert_eq!(cache::ENABLED, "cache.enabled");
    assert_eq!(embedding::MODEL, "embedding.model");
    assert_eq!(vector_store::TYPE_NAME, "vector_store.type");
}

#[test]
fn test_all_keys_are_non_empty() {
    assert!(!indexing::CHUNK_SIZE.is_empty());
    assert!(!security::ENABLE_AUTH.is_empty());
    assert!(!metrics::ENABLED.is_empty());
    assert!(!cache::ENABLED.is_empty());
    assert!(!embedding::MODEL.is_empty());
    assert!(!vector_store::HOST.is_empty());
}

#[test]
fn test_config_keys_use_correct_format() {
    // All keys should follow the pattern "domain.key"
    assert!(indexing::CHUNK_SIZE.contains('.'));
    assert!(security::ENABLE_AUTH.contains('.'));
    assert!(metrics::COLLECTION_INTERVAL.contains('.'));
}
