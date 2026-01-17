//! Tests for debounce service
//!
//! Migrated from src/infrastructure/sync/debounce.rs

use mcp_context_browser::infrastructure::sync::debounce::{DebounceConfig, DebounceService};
use std::path::PathBuf;

#[test]
fn test_default_config() {
    let config = DebounceConfig::default();
    assert_eq!(config.debounce_ms, 60_000);
}

#[test]
fn test_should_debounce_new_path() {
    let service = DebounceService::default();
    let path = PathBuf::from("/test/path");
    assert!(!service.should_debounce(&path));
}

#[test]
fn test_record_and_debounce() {
    let config = DebounceConfig {
        debounce_ms: 1000, // 1 second for testing
    };
    let service = DebounceService::new(config);
    let path = PathBuf::from("/test/path");

    // First sync - should not be debounced
    assert!(!service.should_debounce(&path));

    // Record sync
    service.record_sync(&path);

    // Immediately after - should be debounced
    assert!(service.should_debounce(&path));
}

#[test]
fn test_clear() {
    let service = DebounceService::default();
    let path = PathBuf::from("/test/path");

    service.record_sync(&path);
    assert_eq!(service.tracked_count(), 1);

    service.clear(&path);
    assert_eq!(service.tracked_count(), 0);
}
