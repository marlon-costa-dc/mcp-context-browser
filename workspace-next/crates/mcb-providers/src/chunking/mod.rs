//! Intelligent code chunking adapter using tree-sitter for structural parsing
//!
//! Provides language-aware chunking that respects code structure rather than
//! naive line-based or character-based splitting.
//!
//! ## Overview
//!
//! The chunking adapter breaks source code into semantically meaningful segments
//! for embedding and indexing. Each chunk represents a logical unit (function,
//! class, method) extracted via Abstract Syntax Tree (AST) parsing.
//!
//! ## Supported Languages
//!
//! - Rust, Python, JavaScript, TypeScript
//! - Go, Java, C, C++, C#
//! - Ruby, PHP, Swift, Kotlin
//!
//! ## Architecture
//!
//! This adapter implements the `CodeChunker` port trait from mcb-domain,
//! providing the actual AST-based chunking implementation using tree-sitter.
//!
//! Language-specific processors are provided by the `crate::language` module.

// Chunking-specific constants
pub mod constants;

// Language detection utilities
pub mod language_helpers;

// Core chunking engine
pub mod engine;

// Public re-exports
pub use constants::*;
pub use engine::IntelligentChunker;
pub use language_helpers::{
    get_chunk_size, is_language_supported, language_from_extension, supported_languages,
};

// Re-export language processors from the language module
pub use crate::language::{
    BaseProcessor, CProcessor, CSharpProcessor, CppProcessor, GoProcessor, JavaProcessor,
    JavaScriptProcessor, KotlinProcessor, LanguageConfig, LanguageProcessor, NodeExtractionRule,
    PhpProcessor, PythonProcessor, RubyProcessor, RustProcessor, SwiftProcessor,
};
