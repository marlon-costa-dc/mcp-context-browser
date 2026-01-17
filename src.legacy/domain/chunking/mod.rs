//! Intelligent code chunking using tree-sitter for structural parsing
//!
//! Provides language-aware chunking that respects code structure rather than
//! naive line-based or character-based splitting.
//!
//! ## Overview
//!
//! The chunking module breaks source code into semantically meaningful segments
//! for embedding and indexing. Each chunk represents a logical unit (function,
//! class, method) extracted via Abstract Syntax Tree (AST) parsing.
//!
//! ## Supported Languages
//!
//! | Language | Processor | Extraction Rules |
//! |----------|-----------|------------------|
//! | Rust | `RustProcessor` | Functions, traits, impls, modules |
//! | Python | `PythonProcessor` | Functions, classes, methods |
//! | JavaScript | `JavaScriptProcessor` | Functions, classes, methods |
//! | TypeScript | `JavaScriptProcessor` | Types, interfaces, classes |
//! | Java | `JavaProcessor` | Classes, methods, interfaces |
//! | Go | `GoProcessor` | Functions, interfaces, structs |
//! | C/C++ | `CProcessor`, `CppProcessor` | Functions, structs, classes |
//! | C# | `CSharpProcessor` | Classes, methods, namespaces |
//! | Ruby | `RubyProcessor` | Methods, classes, modules |
//! | PHP | `PhpProcessor` | Functions, classes, methods |
//! | Swift | `SwiftProcessor` | Functions, classes, structs |
//! | Kotlin | `KotlinProcessor` | Functions, classes, data classes |
//!
//! ## Architecture
//!
//! ```text
//! Code Source
//!     ↓
//! Language Detection (from file extension)
//!     ↓
//! Tree-Sitter AST Parsing
//!     ↓
//! IntelligentChunker
//!     ├── Try: LanguageProcessor (language-specific extraction)
//!     └── Fallback: GenericFallbackChunker (regex-based, brace-matching)
//!     ↓
//! CodeChunk[] (with start_line, end_line, metadata)
//!     ↓
//! Embedding → Vector Storage → Semantic Search
//! ```
//!
//! ## Example: Chunk a Rust File
//!
//! ```rust
//! use mcp_context_browser::domain::chunking::IntelligentChunker;
//! use mcp_context_browser::domain::types::Language;
//!
//! let chunker = IntelligentChunker::new();
//!
//! let rust_code = r#"pub fn authenticate(user: &str) -> bool {
//!     user.len() > 0
//! }
//!
//! pub fn authorize(user: &str, role: &str) -> bool {
//!     role == "admin"
//! }"#;
//!
//! let chunks = chunker.chunk_code(rust_code, "auth.rs", Language::Rust);
//! println!("Generated {} chunks", chunks.len());
//! for chunk in chunks {
//!     println!("Lines {}-{}: {}", chunk.start_line, chunk.end_line, &chunk.content[..20.min(chunk.content.len())]);
//! }
//! ```
//!
//! ## Chunking Strategy
//!
//! 1. **Primary**: Language-specific AST extraction
//!    - Parse code into syntax tree using tree-sitter
//!    - Extract meaningful nodes (functions, classes, methods)
//!    - Preserve line numbers and context
//!
//! 2. **Fallback**: Generic regex-based chunking
//!    - Used if language not supported or parsing fails
//!    - Detects block structures using brace matching
//!    - Extracts functions/classes by pattern matching
//!
//! 3. **Filtering**: Remove trivial chunks
//!    - Minimum character length (typically 20 chars)
//!    - Minimum line count (typically 2 lines)
//!    - Maximum chunks per file to prevent explosion

// Public API re-exports
pub use config::{LanguageConfig, NodeExtractionRule, NodeExtractionRuleBuilder};
pub use engine::IntelligentChunker;
pub use processor::LanguageProcessor;

// Module declarations
pub mod config;
pub mod constants;
pub mod engine;
pub mod fallback;
pub mod languages;
pub mod processor;
pub mod traverser;

// Re-export language processors
pub use languages::*;

// Language configurations registry
use crate::domain::types::Language;
use std::collections::HashMap;
use std::sync::LazyLock;

pub(crate) static LANGUAGE_CONFIGS: LazyLock<
    HashMap<Language, Box<dyn LanguageProcessor + Send + Sync>>,
> = LazyLock::new(|| {
    let mut configs = HashMap::new();

    // Register all supported languages
    configs.insert(
        Language::Rust,
        Box::new(RustProcessor::new()) as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::Python,
        Box::new(PythonProcessor::new()) as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::JavaScript,
        Box::new(JavaScriptProcessor::new(Language::JavaScript))
            as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::TypeScript,
        Box::new(JavaScriptProcessor::new(Language::TypeScript))
            as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::Java,
        Box::new(JavaProcessor::new()) as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::Go,
        Box::new(GoProcessor::new()) as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::C,
        Box::new(CProcessor::new()) as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::Cpp,
        Box::new(CppProcessor::new()) as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::CSharp,
        Box::new(CSharpProcessor::new()) as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::Ruby,
        Box::new(RubyProcessor::new()) as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::Php,
        Box::new(PhpProcessor::new()) as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::Swift,
        Box::new(SwiftProcessor::new()) as Box<dyn LanguageProcessor + Send + Sync>,
    );
    configs.insert(
        Language::Kotlin,
        Box::new(KotlinProcessor::new()) as Box<dyn LanguageProcessor + Send + Sync>,
    );

    configs
});
