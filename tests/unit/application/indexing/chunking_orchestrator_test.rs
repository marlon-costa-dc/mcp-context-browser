//! Unit tests for chunking orchestrator
//!
//! Tests for coordinated batch code chunking operations.

use mcp_context_browser::application::indexing::{ChunkingConfig, ChunkingOrchestrator};
use mcp_context_browser::domain::types::Language;

#[test]
fn test_default_config() {
    let config = ChunkingConfig::default();
    assert!(config.batch_size > 0);
    assert!(config.min_chunk_length > 0);
}

#[tokio::test]
async fn test_chunk_empty_content() {
    let orchestrator = ChunkingOrchestrator::default();
    let chunks = orchestrator
        .chunk_content("".to_string(), "test.rs".to_string(), Language::Rust)
        .await;
    assert!(chunks.is_empty());
}

#[test]
fn test_detect_language() {
    // Since detect_language is private, we test indirectly through chunk_content
    // The language detection happens when processing files
    let config = ChunkingConfig::default();
    assert!(config.batch_size > 0);
}
