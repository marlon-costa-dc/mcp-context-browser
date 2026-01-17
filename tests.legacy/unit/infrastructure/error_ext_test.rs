//! Error extension unit tests

use mcp_context_browser::domain::error::Error;
use mcp_context_browser::infrastructure::error_ext::ErrorContext;

#[test]
fn test_io_context() {
    let result: std::result::Result<(), std::io::Error> = Err(std::io::Error::other("read failed"));
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
