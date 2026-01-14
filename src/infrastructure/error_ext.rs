//! Error context extension for consistent error wrapping
//!
//! This module provides a trait for adding context to errors across the infrastructure layer.
//! Instead of repetitive `.map_err()` patterns, use context methods for cleaner error handling.
//!
//! # Examples
//!
//! ```rust
//! use mcp_context_browser::infrastructure::error_ext::ErrorContext;
//! use mcp_context_browser::domain::error::Result;
//!
//! fn read_config() -> Result<String> {
//!     // Before: verbose error mapping
//!     // std::fs::read_to_string("config.toml")
//!     //     .map_err(|e| Error::internal(format!("Failed to read config: {}", e)))?
//!
//!     // After: with context - cleaner!
//!     std::fs::read_to_string("config.toml")
//!         .internal_context("Failed to read config")
//! }
//! ```

use crate::domain::error::{Error, Result};

/// Extension trait for adding context to any error
///
/// Provides convenient methods to wrap errors with descriptive context.
/// All methods consume the Result and return a new Result with the error wrapped.
pub trait ErrorContext<T> {
    /// Add generic context to an error
    fn context(self, msg: impl Into<String>) -> Result<T>;

    /// Add I/O context to an error
    fn io_context(self, msg: impl Into<String>) -> Result<T>;

    /// Add internal error context
    fn internal_context(self, msg: impl Into<String>) -> Result<T>;

    /// Add embedding provider context
    fn embedding_context(self, msg: impl Into<String>) -> Result<T>;

    /// Add cache context
    fn cache_context(self, msg: impl Into<String>) -> Result<T>;
}

/// Implementation for any error that implements Display
impl<T, E: std::fmt::Display> ErrorContext<T> for std::result::Result<T, E> {
    fn context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error::generic(format!("{}: {}", msg.into(), e)))
    }

    fn io_context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error::io(format!("{}: {}", msg.into(), e)))
    }

    fn internal_context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error::internal(format!("{}: {}", msg.into(), e)))
    }

    fn embedding_context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error::embedding(format!("{}: {}", msg.into(), e)))
    }

    fn cache_context(self, msg: impl Into<String>) -> Result<T> {
        self.map_err(|e| Error::cache(format!("{}: {}", msg.into(), e)))
    }
}
