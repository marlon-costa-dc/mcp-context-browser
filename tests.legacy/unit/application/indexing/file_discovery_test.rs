//! Unit tests for file discovery service
//!
//! Tests for file scanning and filtering for indexing.

use mcp_context_browser::application::indexing::{DiscoveryOptions, FileDiscoveryService};
use mcp_context_browser::domain::types::Language;
use std::path::Path;

#[test]
fn test_default_options() {
    let options = DiscoveryOptions::default();
    assert!(!options.exclude_patterns.is_empty());
    assert!(options.max_file_size.is_some());
}

#[test]
fn test_is_supported() {
    let service = FileDiscoveryService::default();
    assert!(service.is_supported("rs"));
    assert!(service.is_supported("py"));
    assert!(service.is_supported("js"));
    assert!(!service.is_supported("xyz"));
}

#[test]
fn test_detect_language() {
    let service = FileDiscoveryService::default();
    assert_eq!(
        service.detect_language(Path::new("test.rs")),
        Language::Rust
    );
    assert_eq!(
        service.detect_language(Path::new("test.py")),
        Language::Python
    );
}
