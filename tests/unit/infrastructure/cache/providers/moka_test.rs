//! Tests for Moka-based cache provider
//!
//! Migrated from src/infrastructure/cache/providers/moka.rs

use mcp_context_browser::infrastructure::cache::{
    CacheConfig, CacheProvider, HealthStatus, MokaCacheProvider,
};
use std::time::Duration;

fn test_config() -> CacheConfig {
    CacheConfig::default()
}

#[tokio::test]
async fn test_moka_provider_set_and_get() {
    let provider = MokaCacheProvider::new(test_config()).unwrap();
    let namespace = "test";
    let key = "test_key";
    let value = vec![1, 2, 3, 4, 5];

    provider
        .set(namespace, key, value.clone(), Duration::from_secs(60))
        .await
        .unwrap();

    let retrieved = provider.get(namespace, key).await.unwrap();
    assert_eq!(retrieved, Some(value));
}

#[tokio::test]
async fn test_moka_provider_delete() {
    let provider = MokaCacheProvider::new(test_config()).unwrap();
    let namespace = "test";
    let key = "test_key";
    let value = vec![1, 2, 3];

    provider
        .set(namespace, key, value, Duration::from_secs(60))
        .await
        .unwrap();

    provider.delete(namespace, key).await.unwrap();

    let retrieved = provider.get(namespace, key).await.unwrap();
    assert_eq!(retrieved, None);
}

#[tokio::test]
async fn test_moka_provider_clear_namespace() {
    let provider = MokaCacheProvider::new(test_config()).unwrap();
    let namespace = "embeddings";

    provider
        .set(namespace, "key1", vec![1, 2], Duration::from_secs(60))
        .await
        .unwrap();
    provider
        .set(namespace, "key2", vec![3, 4], Duration::from_secs(60))
        .await
        .unwrap();

    provider.clear(Some(namespace)).await.unwrap();

    assert_eq!(provider.get(namespace, "key1").await.unwrap(), None);
    assert_eq!(provider.get(namespace, "key2").await.unwrap(), None);
}

#[tokio::test]
async fn test_moka_provider_clear_all() {
    let provider = MokaCacheProvider::new(test_config()).unwrap();

    provider
        .set("embeddings", "key1", vec![1], Duration::from_secs(60))
        .await
        .unwrap();
    provider
        .set("search_results", "key2", vec![2], Duration::from_secs(60))
        .await
        .unwrap();

    provider.clear(None).await.unwrap();

    assert_eq!(provider.get("embeddings", "key1").await.unwrap(), None);
    assert_eq!(provider.get("search_results", "key2").await.unwrap(), None);
}

#[tokio::test]
async fn test_moka_provider_health_check() {
    let provider = MokaCacheProvider::new(test_config()).unwrap();
    let health = provider.health_check().await.unwrap();
    assert_eq!(health, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_moka_provider_stats() {
    let provider = MokaCacheProvider::new(test_config()).unwrap();

    provider
        .set("test", "key1", vec![1, 2, 3], Duration::from_secs(60))
        .await
        .unwrap();
    provider.get("test", "key1").await.unwrap(); // Hit
    provider.get("test", "nonexistent").await.unwrap(); // Miss

    let stats = provider.get_stats("test").await.unwrap();
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 1);
    assert!(stats.hit_ratio > 0.0);
}

#[tokio::test]
async fn test_moka_provider_backend_type() {
    let provider = MokaCacheProvider::new(test_config()).unwrap();
    assert_eq!(provider.backend_type(), "moka".to_string());
}
