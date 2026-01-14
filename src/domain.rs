//! # Domain Layer
//!
//! Core business logic and domain types for semantic code analysis.
//!
//! This layer contains:
//!
//! - [`chunking`] - AST-based code chunking for 12+ programming languages
//! - [`constants`] - Domain-level constants and configuration values
//! - [`error`] - Domain error types with rich context
//! - [`ports`] - Port traits (interfaces) for dependency injection
//! - [`types`] - Core domain types like [`CodeChunk`], [`Embedding`], [`SearchResult`]
//! - [`validation`] - Input validation utilities
//!
//! ## Architecture
//!
//! The domain layer follows Clean Architecture principles:
//!
//! - **No external dependencies** - Only standard library and core traits
//! - **Port-based abstraction** - All external interactions via trait ports
//! - **Value objects** - Immutable domain types with validation
//!
//! ## Example
//!
//! ```rust
//! use mcp_context_browser::domain::types::{CodeChunk, Language};
//!
//! // Domain types are the core of the application
//! let chunk = CodeChunk {
//!     id: "chunk-1".to_string(),
//!     content: "fn main() {}".to_string(),
//!     file_path: "example.rs".to_string(),
//!     start_line: 1,
//!     end_line: 1,
//!     language: Language::Rust,
//!     metadata: serde_json::json!({}),
//! };
//! ```
//!
//! [`CodeChunk`]: types::CodeChunk
//! [`Embedding`]: types::Embedding
//! [`SearchResult`]: types::SearchResult

pub mod chunking;
pub mod constants;
pub mod error;
pub mod ports;
pub mod types;
pub mod validation;

pub use error::{Error, Result};
pub use types::*;
