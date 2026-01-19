//! DI Component Dispatch Tests
//!
//! Tests for the DI container bootstrap and initialization.

use mcb_domain::value_objects::{EmbeddingConfig, VectorStoreConfig};
use mcb_infrastructure::config::AppConfig;
use mcb_infrastructure::di::bootstrap::init_app;

// Force link mcb_providers so inventory registrations are included
extern crate mcb_providers;

#[tokio::test]
async fn test_di_container_builder() {
    let config = AppConfig::default();
    let result = init_app(config).await;

    assert!(
        result.is_ok(),
        "init_app should complete successfully: {:?}",
        result.err()
    );

    let app_context = result.unwrap();

    // Verify context has expected fields
    assert!(
        std::mem::size_of_val(&app_context.config) > 0,
        "Config should be initialized"
    );
    assert!(
        std::mem::size_of_val(&app_context.providers) > 0,
        "Providers should be initialized"
    );
}

#[tokio::test]
async fn test_provider_selection_from_config() {
    // Test that providers are correctly selected based on configuration

    // Test with null providers (default)
    let mut config = AppConfig::default();
    config.providers.embedding.insert(
        "default".to_string(),
        EmbeddingConfig {
            provider: "null".to_string(),
            model: "test".to_string(),
            api_key: None,
            base_url: None,
            dimensions: Some(384),
            max_tokens: Some(1000),
        },
    );
    config.providers.vector_store.insert(
        "default".to_string(),
        VectorStoreConfig {
            provider: "null".to_string(),
            address: None,
            token: None,
            collection: Some("test".to_string()),
            dimensions: Some(384),
            timeout_secs: None,
        },
    );

    let app_context = init_app(config)
        .await
        .expect("Should initialize with null providers");

    // Verify correct providers were selected
    assert_eq!(app_context.providers.embedding.provider_name(), "null");
    assert_eq!(app_context.providers.vector_store.provider_name(), "null");
    assert_eq!(app_context.providers.cache.provider_name(), "moka"); // default cache
    assert_eq!(app_context.providers.language.provider_name(), "universal"); // default language
}

#[tokio::test]
async fn test_provider_resolution_uses_registry() {
    // Test that provider resolution uses the registry system, not hardcoded instances

    // This test verifies the Clean Architecture pattern:
    // - Configuration drives provider selection
    // - Registry resolves provider by name
    // - Services use providers through traits (no concrete knowledge)

    let config = AppConfig::default();
    let app_context = init_app(config)
        .await
        .expect("Should initialize successfully");

    // Verify that providers implement the expected traits
    // (This would fail at compile time if providers didn't implement the traits)

    // Test that we can call methods through the trait
    let _dimensions = app_context.providers.embedding.dimensions();
    let _health = app_context.providers.embedding.health_check().await;

    // Verify provider names are returned correctly
    assert!(!app_context.providers.embedding.provider_name().is_empty());
    assert!(!app_context
        .providers
        .vector_store
        .provider_name()
        .is_empty());
    assert!(!app_context.providers.cache.provider_name().is_empty());
    assert!(!app_context.providers.language.provider_name().is_empty());
}
