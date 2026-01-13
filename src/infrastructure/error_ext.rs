//! Error context extension for consistent error wrapping
//!
//! This module provides a trait for adding context to errors across the infrastructure layer.
//! Instead of repetitive `.map_err()` patterns, use context methods for cleaner error handling.
//!
//! # Examples
//!
//! ```ignore
//! // Before: verbose error mapping
//! result.map_err(|e| Error::internal(format!("Failed to read config: {}", e)))?
//!
//! // After: with context
//! result.internal_context("Failed to read config")?
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io_context() {
        let result: std::result::Result<(), std::io::Error> =
            Err(std::io::Error::other("read failed"));
        let err = result.io_context("Failed to read config").unwrap_err();

        match err {
            Error::Io { source } => {
                let msg = source.to_string();
                assert!(msg.contains("Failed to read config"));
                assert!(msg.contains("read failed"));
            }
            _ => panic!("Expected Io error"),
        }
    }

    #[test]
    fn test_internal_context() {
        let result: std::result::Result<(), String> = Err("connection lost".to_string());
        let err = result.internal_context("Database connection").unwrap_err();

        match err {
            Error::Internal { message } => {
                assert!(message.contains("Database connection"));
                assert!(message.contains("connection lost"));
            }
            _ => panic!("Expected Internal error"),
        }
    }

    #[test]
    fn test_embedding_context() {
        let result: std::result::Result<(), String> = Err("API timeout".to_string());
        let err = result.embedding_context("OpenAI request").unwrap_err();

        match err {
            Error::Embedding { message } => {
                assert!(message.contains("OpenAI request"));
                assert!(message.contains("API timeout"));
            }
            _ => panic!("Expected Embedding error"),
        }
    }

    #[test]
    fn test_cache_context() {
        let result: std::result::Result<(), String> = Err("key not found".to_string());
        let err = result.cache_context("Cache lookup").unwrap_err();

        match err {
            Error::Cache { message } => {
                assert!(message.contains("Cache lookup"));
                assert!(message.contains("key not found"));
            }
            _ => panic!("Expected Cache error"),
        }
    }

    #[test]
    fn test_generic_context() {
        let result: std::result::Result<(), String> = Err("something went wrong".to_string());
        let err = result.context("Operation failed").unwrap_err();

        let err_msg = format!("{}", err);
        assert!(err_msg.contains("Operation failed"));
        assert!(err_msg.contains("something went wrong"));
    }
}
