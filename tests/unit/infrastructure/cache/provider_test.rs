//! Tests for the NullCacheProvider
//!
//! Migrated from src/infrastructure/cache/provider.rs

use mcp_context_browser::infrastructure::cache::{CacheProvider, HealthStatus, NullCacheProvider};
use std::time::Duration;

#[tokio::test]
async fn test_null_cache_provider() {
    let cache = NullCacheProvider;

    // All operations should succeed with no-op behavior
    assert!(cache
        .set("test", "key", vec![1, 2, 3], Duration::from_secs(60))
        .await
        .is_ok());
    assert_eq!(cache.get("test", "key").await.unwrap(), None);
    assert!(cache.delete("test", "key").await.is_ok());
    assert!(cache.clear(None).await.is_ok());
    assert_eq!(cache.health_check().await.unwrap(), HealthStatus::Healthy);
    assert_eq!(cache.backend_type(), "null");
}

#[tokio::test]
async fn test_null_cache_exists() {
    let cache = NullCacheProvider;
    assert!(!cache.exists("test", "key").await.unwrap());
}

#[tokio::test]
async fn test_null_cache_stats() {
    let cache = NullCacheProvider;
    let stats = cache.get_stats("test").await.unwrap();
    assert_eq!(stats.total_entries, 0);
    assert_eq!(stats.hits, 0);
}

#[tokio::test]
async fn test_null_cache_multiple_operations() {
    let cache = NullCacheProvider;

    let items = vec![
        ("key1".to_string(), vec![1, 2, 3]),
        ("key2".to_string(), vec![4, 5, 6]),
    ];

    assert!(cache
        .set_multiple("test", &items, Duration::from_secs(60))
        .await
        .is_ok());

    let result = cache.get_multiple("test", &["key1", "key2"]).await.unwrap();
    assert!(result.is_empty());

    assert!(cache
        .delete_multiple("test", &["key1", "key2"])
        .await
        .is_ok());
}
