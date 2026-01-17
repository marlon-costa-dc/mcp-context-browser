//! Tests for domain ports - chunking interfaces
//!
//! Migrated from src/domain/ports/chunking.rs inline tests.
//! Tests the CodeChunker trait and ChunkingOptions/ChunkingResult types.

use async_trait::async_trait;
use mcp_context_browser::domain::error::Result;
use mcp_context_browser::domain::ports::chunking::{ChunkingOptions, ChunkingResult, CodeChunker};
use mcp_context_browser::domain::types::{CodeChunk, Language};
use serde_json::json;
use std::path::Path;

/// Mock code chunker for testing
struct MockCodeChunker {
    supported: Vec<Language>,
}

impl MockCodeChunker {
    fn new() -> Self {
        Self {
            supported: vec![
                Language::Rust,
                Language::Python,
                Language::JavaScript,
                Language::TypeScript,
            ],
        }
    }
}

#[async_trait]
impl CodeChunker for MockCodeChunker {
    async fn chunk_file(
        &self,
        file_path: &Path,
        options: ChunkingOptions,
    ) -> Result<ChunkingResult> {
        let file_name = file_path.to_string_lossy().to_string();
        let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
        let language = Language::from_extension(ext);

        self.chunk_content("// mock content", &file_name, language, options)
            .await
    }

    async fn chunk_content(
        &self,
        content: &str,
        file_name: &str,
        language: Language,
        _options: ChunkingOptions,
    ) -> Result<ChunkingResult> {
        let chunk = CodeChunk {
            id: format!("chunk-{}", file_name),
            content: content.to_string(),
            file_path: file_name.to_string(),
            start_line: 1,
            end_line: 10,
            language: language.clone(),
            metadata: json!({"mock": true}),
        };

        Ok(ChunkingResult::from_ast(
            file_name.to_string(),
            language,
            vec![chunk],
        ))
    }

    async fn chunk_batch(
        &self,
        file_paths: &[&Path],
        options: ChunkingOptions,
    ) -> Result<Vec<ChunkingResult>> {
        let mut results = Vec::new();
        for path in file_paths {
            results.push(self.chunk_file(path, options.clone()).await?);
        }
        Ok(results)
    }

    fn supported_languages(&self) -> Vec<Language> {
        self.supported.clone()
    }
}

#[tokio::test]
async fn test_chunk_file() {
    let chunker = MockCodeChunker::new();
    let result = chunker
        .chunk_file(Path::new("test.rs"), ChunkingOptions::default())
        .await;

    assert!(result.is_ok());
    let chunking_result = result.unwrap();
    assert_eq!(chunking_result.language, Language::Rust);
    assert!(chunking_result.used_ast);
    assert!(!chunking_result.chunks.is_empty());
}

#[tokio::test]
async fn test_chunk_content() {
    let chunker = MockCodeChunker::new();
    let result = chunker
        .chunk_content(
            "fn main() {}",
            "main.rs",
            Language::Rust,
            ChunkingOptions::default(),
        )
        .await;

    assert!(result.is_ok());
    let chunking_result = result.unwrap();
    assert_eq!(chunking_result.file_path, "main.rs");
}

#[tokio::test]
async fn test_chunk_batch() {
    let chunker = MockCodeChunker::new();
    let paths = vec![Path::new("a.rs"), Path::new("b.py")];
    let result = chunker
        .chunk_batch(&paths, ChunkingOptions::default())
        .await;

    assert!(result.is_ok());
    let results = result.unwrap();
    assert_eq!(results.len(), 2);
}

#[test]
fn test_supported_languages() {
    let chunker = MockCodeChunker::new();
    let supported = chunker.supported_languages();

    assert!(supported.contains(&Language::Rust));
    assert!(supported.contains(&Language::Python));
}

#[test]
fn test_is_language_supported() {
    let chunker = MockCodeChunker::new();

    assert!(chunker.is_language_supported(&Language::Rust));
    assert!(!chunker.is_language_supported(&Language::SQL));
}

#[test]
fn test_chunking_options_default() {
    let options = ChunkingOptions::default();

    assert_eq!(options.max_chunk_size, 512);
    assert!(options.include_context);
    assert_eq!(options.max_chunks_per_file, 50);
}

#[test]
fn test_chunking_result_from_ast() {
    let result = ChunkingResult::from_ast("test.rs".to_string(), Language::Rust, vec![]);

    assert!(result.used_ast);
}

#[test]
fn test_chunking_result_from_fallback() {
    let result = ChunkingResult::from_fallback("test.txt".to_string(), Language::Unknown, vec![]);

    assert!(!result.used_ast);
}
