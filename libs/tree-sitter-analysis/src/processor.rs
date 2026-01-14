//! Language processor implementations for AST analysis
//!
//! This module provides implementations of LanguageProcessor for supported languages.
//! In v0.2.0, chunking logic is still in src/domain/chunking/ and will be
//! migrated here in subsequent releases.

pub use crate::{ChunkConfig, CodeChunk, LanguageProcessor};

use anyhow::Result;

/// Multi-language code processor
///
/// Supports 12+ languages through Tree-sitter parsers.
/// Chunking implementation uses AST-aware semantic boundaries.
#[derive(Debug, Clone)]
pub struct MultiLanguageProcessor {
    language: String,
}

impl MultiLanguageProcessor {
    /// Create a new processor for the given language
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: language.into(),
        }
    }

    /// Get supported languages
    pub fn supported_languages() -> &'static [&'static str] {
        &[
            "rust",
            "python",
            "javascript",
            "typescript",
            "go",
            "java",
            "c",
            "cpp",
            "csharp",
            "ruby",
            "php",
            "swift",
            "kotlin",
        ]
    }
}

impl LanguageProcessor for MultiLanguageProcessor {
    fn chunk_code(&self, source: &str, config: &ChunkConfig) -> Result<Vec<CodeChunk>> {
        // TODO: Migrate chunking logic from src/domain/chunking/engine.rs
        // For v0.2.0, this remains in the main crate
        Err(anyhow::anyhow!(
            "Chunking implementation not yet migrated to workspace library. \
             Please use the main crate's chunking implementation."
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_languages() {
        let langs = MultiLanguageProcessor::supported_languages();
        assert!(langs.contains(&"rust"));
        assert!(langs.contains(&"python"));
        assert!(langs.contains(&"javascript"));
    }

    #[test]
    fn test_new_processor() {
        let _processor = MultiLanguageProcessor::new("rust");
        // More detailed tests when chunking is migrated
    }
}
