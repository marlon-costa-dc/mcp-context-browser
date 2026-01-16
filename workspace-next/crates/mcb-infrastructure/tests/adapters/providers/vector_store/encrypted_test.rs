//! Tests for EncryptedVectorStoreProvider

use mcb_domain::ports::providers::VectorStoreProvider;
use mcb_infrastructure::adapters::providers::vector_store::{
    EncryptedVectorStoreProvider, NullVectorStoreProvider,
};
use mcb_infrastructure::crypto::CryptoService;

#[tokio::test]
async fn test_encrypted_provider_creation() {
    let inner = NullVectorStoreProvider::new();
    let master_key = CryptoService::generate_master_key();

    let _provider = EncryptedVectorStoreProvider::new(inner, master_key).unwrap();
}

#[tokio::test]
async fn test_encrypted_provider_stats() {
    let inner = NullVectorStoreProvider::new();
    let master_key = CryptoService::generate_master_key();
    let provider = EncryptedVectorStoreProvider::new(inner, master_key).unwrap();

    let stats = provider.get_stats("test").await.unwrap();

    assert_eq!(stats.get("encryption_enabled").unwrap(), true);
    assert_eq!(stats.get("encryption_algorithm").unwrap(), "AES-256-GCM");
}

#[tokio::test]
async fn test_encrypted_provider_name() {
    let inner = NullVectorStoreProvider::new();
    let master_key = CryptoService::generate_master_key();
    let provider = EncryptedVectorStoreProvider::new(inner, master_key).unwrap();

    assert_eq!(provider.provider_name(), "encrypted");
}
