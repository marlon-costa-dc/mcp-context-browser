//! Null Cache Provider Tests

use mcb_infrastructure::cache::config::CacheEntryConfig;
use mcb_infrastructure::cache::providers::NullCacheProvider;
use mcb_infrastructure::cache::CacheProvider;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestValue {
    data: String,
    number: i32,
}

#[tokio::test]
async fn test_null_provider_operations() {
    let provider = NullCacheProvider::new();

    // Test get (should always return None)
    let result: Option<TestValue> = provider.get("test_key").await.unwrap();
    assert!(result.is_none());

    // Test set (should succeed)
    let value = TestValue {
        data: "test".to_string(),
        number: 42,
    };
    assert!(provider
        .set("test_key", value, CacheEntryConfig::default())
        .await
        .is_ok());

    // Test exists (should always return false)
    assert!(!provider.exists("test_key").await.unwrap());

    // Test delete (should return false)
    assert!(!provider.delete("test_key").await.unwrap());

    // Test clear (should succeed)
    assert!(provider.clear().await.is_ok());

    // Test stats (should be empty)
    let stats = provider.stats().await.unwrap();
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.entries, 0);

    // Test size (should be 0)
    assert_eq!(provider.size().await.unwrap(), 0);
}

#[test]
fn test_null_provider_default() {
    let provider = NullCacheProvider::default();
    // Just verify it creates successfully
    assert!(true); // If we get here, Default works
}
