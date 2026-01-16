//! Unit tests for CodeChunker domain service interface

use mcb_domain::domain_services::chunking::{ChunkingOptions, ChunkingResult, CodeChunker};
use mcb_domain::{CodeChunk, Language};
use std::path::Path;

// Mock implementation for testing
struct MockCodeChunker;

#[async_trait::async_trait]
impl CodeChunker for MockCodeChunker {
    async fn chunk_file(
        &self,
        _file_path: &Path,
        _options: ChunkingOptions,
    ) -> mcb_domain::Result<ChunkingResult> {
        Ok(ChunkingResult::from_ast(
            "test.rs".to_string(),
            "rust".to_string(),
            vec![CodeChunk {
                id: "chunk-1".to_string(),
                content: "fn main() {}".to_string(),
                file_path: "test.rs".to_string(),
                start_line: 1,
                end_line: 1,
                language: "rust".to_string(),
                metadata: serde_json::json!({"type": "function"}),
            }],
        ))
    }

    async fn chunk_content(
        &self,
        _content: &str,
        file_name: &str,
        language: Language,
        _options: ChunkingOptions,
    ) -> mcb_domain::Result<ChunkingResult> {
        Ok(ChunkingResult::from_ast(
            file_name.to_string(),
            language,
            vec![CodeChunk {
                id: "chunk-1".to_string(),
                content: "fn main() {}".to_string(),
                file_path: file_name.to_string(),
                start_line: 1,
                end_line: 1,
                language: "rust".to_string(),
                metadata: serde_json::json!({"type": "function"}),
            }],
        ))
    }

    async fn chunk_batch(
        &self,
        file_paths: &[&Path],
        options: ChunkingOptions,
    ) -> mcb_domain::Result<Vec<ChunkingResult>> {
        let mut results = Vec::new();
        for path in file_paths {
            results.push(self.chunk_file(path, options.clone()).await?);
        }
        Ok(results)
    }

    fn supported_languages(&self) -> Vec<Language> {
        vec![
            "rust".to_string(),
            "python".to_string(),
            "javascript".to_string(),
        ]
    }
}

#[test]
fn test_code_chunker_interface() {
    let chunker = MockCodeChunker;

    // Test supported languages
    let languages = chunker.supported_languages();
    assert_eq!(languages.len(), 3);
    assert!(languages.contains(&"rust".to_string()));
    assert!(languages.contains(&"python".to_string()));
    assert!(languages.contains(&"javascript".to_string()));
}

#[test]
fn test_is_language_supported() {
    let chunker = MockCodeChunker;

    assert!(chunker.is_language_supported(&"rust".to_string()));
    assert!(chunker.is_language_supported(&"python".to_string()));
    assert!(chunker.is_language_supported(&"javascript".to_string()));

    assert!(!chunker.is_language_supported(&"go".to_string()));
    assert!(!chunker.is_language_supported(&"java".to_string()));
    assert!(!chunker.is_language_supported(&"unknown".to_string()));
}

#[tokio::test]
async fn test_chunk_content() {
    let chunker = MockCodeChunker;

    let content = "fn main() {\n    println!(\"Hello, world!\");\n}";
    let file_name = "src/main.rs";
    let language = "rust".to_string();

    let result = chunker
        .chunk_content(content, file_name, language, ChunkingOptions::default())
        .await;
    assert!(result.is_ok());

    let chunking_result = result.unwrap();
    assert_eq!(chunking_result.chunks.len(), 1);

    let chunk = &chunking_result.chunks[0];
    assert_eq!(chunk.id, "chunk-1");
    assert_eq!(chunk.content, "fn main() {}");
    assert_eq!(chunk.language, "rust");
}

#[test]
fn test_code_chunker_trait_object() {
    // Test that we can use CodeChunker as a trait object
    let chunker: Box<dyn CodeChunker> = Box::new(MockCodeChunker);

    assert!(chunker.is_language_supported(&"rust".to_string()));
    assert!(!chunker.is_language_supported(&"haskell".to_string()));
}

#[test]
fn test_chunking_options_default() {
    let options = ChunkingOptions::default();

    assert_eq!(options.max_chunk_size, 512);
    assert!(options.include_context);
    assert_eq!(options.max_chunks_per_file, 50);
}
