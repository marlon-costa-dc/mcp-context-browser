//! Fallback chunking using regex patterns
//!
//! This module provides regex-based chunking as a fallback when tree-sitter
//! parsing is not available or fails.

use super::config::LanguageConfig;
use crate::domain::types::{CodeChunk, Language};
use regex::Regex;
use std::collections::HashMap;

/// Generic fallback chunker using regex patterns
///
/// This chunker precompiles regex patterns at construction time to avoid
/// the overhead of repeated regex compilation during line-by-line processing.
pub struct GenericFallbackChunker<'a> {
    #[allow(dead_code)]
    config: &'a LanguageConfig,
    /// Precompiled regex patterns for block detection (avoids per-line compilation)
    compiled_patterns: Vec<Regex>,
}

impl<'a> GenericFallbackChunker<'a> {
    /// Create a new generic fallback chunker with language configuration
    ///
    /// Regex patterns are compiled once at construction time for performance.
    /// Invalid patterns are filtered out gracefully.
    ///
    /// # Arguments
    /// * `config` - The language configuration containing fallback patterns
    ///
    /// # Returns
    /// A new instance of the fallback chunker with precompiled patterns
    pub fn new(config: &'a LanguageConfig) -> Self {
        // Precompile all patterns once at construction time
        let compiled_patterns = config
            .fallback_patterns
            .iter()
            .filter_map(|pattern| Regex::new(pattern).ok())
            .collect();

        Self {
            config,
            compiled_patterns,
        }
    }

    /// Chunk content using regex patterns as a fallback when tree-sitter parsing fails
    ///
    /// # Arguments
    /// * `content` - The source code content to chunk
    /// * `file_name` - The name of the file being processed
    /// * `language` - The programming language of the content
    ///
    /// # Returns
    /// A vector of code chunks extracted using regex patterns
    pub fn chunk_with_patterns(
        &self,
        content: &str,
        file_name: &str,
        language: &Language,
    ) -> Vec<CodeChunk> {
        let mut chunks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut current_block = Vec::new();
        let mut block_start = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Check if line matches any precompiled pattern (no per-line compilation)
            let is_block_start = self
                .compiled_patterns
                .iter()
                .any(|regex| regex.is_match(trimmed));

            if is_block_start {
                if !current_block.is_empty() {
                    self.create_chunk(
                        &current_block,
                        block_start,
                        i - 1,
                        file_name,
                        language,
                        &mut chunks,
                    );
                    current_block.clear();
                }
                current_block.push(line.to_string());
                block_start = i;
            } else if !current_block.is_empty() {
                current_block.push(line.to_string());

                // Check for block end using brace counting
                if self.is_block_complete(&current_block) {
                    self.create_chunk(
                        &current_block,
                        block_start,
                        i,
                        file_name,
                        language,
                        &mut chunks,
                    );
                    current_block.clear();
                    block_start = i + 1;
                }
            }
        }

        if !current_block.is_empty() {
            self.create_chunk(
                &current_block,
                block_start,
                lines.len() - 1,
                file_name,
                language,
                &mut chunks,
            );
        }

        chunks
    }

    fn is_block_complete(&self, block: &[String]) -> bool {
        let open_count: usize = block
            .iter()
            .map(|line| line.chars().filter(|&c| c == '{').count())
            .sum();
        let close_count: usize = block
            .iter()
            .map(|line| line.chars().filter(|&c| c == '}').count())
            .sum();

        open_count > 0 && open_count == close_count && block.len() > 2
    }

    fn create_chunk(
        &self,
        lines: &[String],
        start_line: usize,
        end_line: usize,
        file_name: &str,
        language: &Language,
        chunks: &mut Vec<CodeChunk>,
    ) {
        let content = lines.join("\n").trim().to_string();
        if content.is_empty() || content.len() < 20 {
            return;
        }

        let chunk = CodeChunk {
            id: format!("{}_{}_{}", file_name, start_line, end_line),
            content,
            file_path: file_name.to_string(),
            start_line: start_line as u32,
            end_line: end_line as u32,
            language: language.clone(),
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("file".to_string(), serde_json::json!(file_name));
                meta.insert("chunk_type".to_string(), serde_json::json!("fallback"));
                serde_json::to_value(meta).unwrap_or(serde_json::json!({}))
            },
        };
        chunks.push(chunk);
    }
}
