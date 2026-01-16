//! Moka Cache Provider Tests

use mcb_infrastructure::cache::config::CacheEntryConfig;
use mcb_infrastructure::cache::providers::MokaCacheProvider;
use mcb_infrastructure::cache::CacheProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestValue {
    data: String,
    number: i32,
}

#[tokio::test]
async fn test_moka_provider_basic_operations() {
    let provider = MokaCacheProvider::new();

    let value = TestValue {
        data: "test data".to_string(),
        number: 42,
    };

    // Test set and get
    provider
        .set("test_key", &value, CacheEntryConfig::default())
        .await
        .unwrap();

    let retrieved: Option<TestValue> = provider.get("test_key").await.unwrap();
    assert_eq!(retrieved, Some(value));

    // Test exists
    assert!(provider.exists("test_key").await.unwrap());

    // Test delete
    assert!(provider.delete("test_key").await.unwrap());
    assert!(!provider.exists("test_key").await.unwrap());

    // Test get after delete
    let retrieved: Option<TestValue> = provider.get("test_key").await.unwrap();
    assert!(retrieved.is_none());
}

#[tokio::test]
async fn test_moka_provider_nonexistent_key() {
    let provider = MokaCacheProvider::new();

    let retrieved: Option<TestValue> = provider.get("nonexistent").await.unwrap();
    assert!(retrieved.is_none());

    assert!(!provider.exists("nonexistent").await.unwrap());
    assert!(!provider.delete("nonexistent").await.unwrap());
}

#[tokio::test]
async fn test_moka_provider_clear() {
    let provider = MokaCacheProvider::new();

    // Add some entries
    provider
        .set("key1", "value1", CacheEntryConfig::default())
        .await
        .unwrap();
    provider
        .set("key2", "value2", CacheEntryConfig::default())
        .await
        .unwrap();

    assert_eq!(provider.size().await.unwrap(), 2);

    // Clear cache
    provider.clear().await.unwrap();

    assert_eq!(provider.size().await.unwrap(), 0);
    assert!(!provider.exists("key1").await.unwrap());
    assert!(!provider.exists("key2").await.unwrap());
}

#[tokio::test]
async fn test_moka_provider_stats() {
    let provider = MokaCacheProvider::new();

    provider
        .set("key1", "value1", CacheEntryConfig::default())
        .await
        .unwrap();
    provider
        .set("key2", "value2", CacheEntryConfig::default())
        .await
        .unwrap();

    let stats = provider.stats().await.unwrap();
    assert_eq!(stats.entries, 2);
}
