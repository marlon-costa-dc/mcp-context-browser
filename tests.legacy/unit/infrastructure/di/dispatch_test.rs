//! Tests for type-safe provider dispatch
//!
//! Migrated from src/infrastructure/di/dispatch.rs

use mcp_context_browser::domain::types::{EmbeddingProviderKind, VectorStoreProviderKind};

#[test]
fn test_embedding_provider_kind_from_string() {
    assert_eq!(
        EmbeddingProviderKind::from_string("openai"),
        Some(EmbeddingProviderKind::OpenAI)
    );
    assert_eq!(
        EmbeddingProviderKind::from_string("OPENAI"),
        Some(EmbeddingProviderKind::OpenAI)
    );
    assert_eq!(
        EmbeddingProviderKind::from_string("fastembed"),
        Some(EmbeddingProviderKind::FastEmbed)
    );
    assert_eq!(EmbeddingProviderKind::from_string("invalid"), None);
}

#[test]
fn test_vector_store_provider_kind_from_string() {
    assert_eq!(
        VectorStoreProviderKind::from_string("in-memory"),
        Some(VectorStoreProviderKind::InMemory)
    );
    assert_eq!(
        VectorStoreProviderKind::from_string("inmemory"),
        Some(VectorStoreProviderKind::InMemory)
    );
    assert_eq!(
        VectorStoreProviderKind::from_string("filesystem"),
        Some(VectorStoreProviderKind::Filesystem)
    );
    assert_eq!(VectorStoreProviderKind::from_string("invalid"), None);
}

#[test]
fn test_supported_providers() {
    let embedding_providers = EmbeddingProviderKind::supported_providers();
    assert!(embedding_providers.contains(&"openai"));
    assert!(embedding_providers.contains(&"fastembed"));

    let vector_store_providers = VectorStoreProviderKind::supported_providers();
    assert!(vector_store_providers.contains(&"filesystem"));
    assert!(vector_store_providers.contains(&"in-memory"));
}
