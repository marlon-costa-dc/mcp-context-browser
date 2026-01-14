//! Tests for data directory configuration
//!
//! Migrated from src/infrastructure/config/data.rs

use std::path::PathBuf;

use mcp_context_browser::infrastructure::config::data::DataConfig;

#[test]
fn test_expand_path() {
    let config = DataConfig::default();
    let expanded = config.base_path();
    assert!(expanded
        .to_string_lossy()
        .contains(".local/share/mcp-context-browser"));
}

#[test]
fn test_expand_path_home_shortcut() {
    // Test that ~ expands to home directory
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));

    let config = DataConfig {
        base_dir: "~".to_string(),
        ..Default::default()
    };

    assert_eq!(config.base_path(), home);
}

#[test]
fn test_expand_path_absolute() {
    let config = DataConfig {
        base_dir: "/tmp/test".to_string(),
        ..Default::default()
    };
    assert_eq!(config.base_path(), PathBuf::from("/tmp/test"));
}

#[test]
fn test_snapshots_path_default() {
    let config = DataConfig::default();
    let path = config.snapshots_path();
    assert!(path.to_string_lossy().contains("snapshots"));
}

#[test]
fn test_config_history_path_default() {
    let config = DataConfig::default();
    let path = config.config_history_path();
    assert!(path.to_string_lossy().contains("config-history"));
}

#[test]
fn test_encryption_keys_path_default() {
    let config = DataConfig::default();
    let path = config.encryption_keys_path();
    assert!(path.to_string_lossy().contains("encryption"));
}

#[test]
fn test_circuit_breakers_path_default() {
    let config = DataConfig::default();
    let path = config.circuit_breakers_path();
    assert!(path.to_string_lossy().contains("circuit-breakers"));
}

#[test]
fn test_custom_snapshots_dir() {
    let config = DataConfig {
        base_dir: "/data".to_string(),
        snapshots_dir: Some("/custom/snapshots".to_string()),
        ..Default::default()
    };
    assert_eq!(config.snapshots_path(), PathBuf::from("/custom/snapshots"));
}
