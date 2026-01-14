//! Tests for snapshot comparator service
//!
//! Migrated from src/infrastructure/snapshot/comparator.rs

use mcp_context_browser::infrastructure::snapshot::{
    CodebaseSnapshot, FileSnapshot, SnapshotComparator,
};
use std::collections::HashMap;

fn create_test_snapshot(files: Vec<(&str, &str)>) -> CodebaseSnapshot {
    let mut file_map = HashMap::new();
    for (path, hash) in files {
        file_map.insert(
            path.to_string(),
            FileSnapshot {
                path: path.to_string(),
                hash: hash.to_string(),
                size: 100,
                modified: 0,
                extension: "rs".to_string(),
            },
        );
    }
    CodebaseSnapshot {
        root_path: "/test".to_string(),
        created_at: 0,
        files: file_map,
        file_count: 0,
        total_size: 0,
    }
}

#[test]
fn test_no_changes() {
    let comparator = SnapshotComparator::new();
    let old = create_test_snapshot(vec![("a.rs", "hash1"), ("b.rs", "hash2")]);
    let new = create_test_snapshot(vec![("a.rs", "hash1"), ("b.rs", "hash2")]);

    let changes = comparator.compare(&old, &new);
    assert!(!comparator.has_changes(&changes));
    assert_eq!(changes.unchanged.len(), 2);
}

#[test]
fn test_added_file() {
    let comparator = SnapshotComparator::new();
    let old = create_test_snapshot(vec![("a.rs", "hash1")]);
    let new = create_test_snapshot(vec![("a.rs", "hash1"), ("b.rs", "hash2")]);

    let changes = comparator.compare(&old, &new);
    assert!(comparator.has_changes(&changes));
    assert_eq!(changes.added, vec!["b.rs"]);
}

#[test]
fn test_removed_file() {
    let comparator = SnapshotComparator::new();
    let old = create_test_snapshot(vec![("a.rs", "hash1"), ("b.rs", "hash2")]);
    let new = create_test_snapshot(vec![("a.rs", "hash1")]);

    let changes = comparator.compare(&old, &new);
    assert!(comparator.has_changes(&changes));
    assert_eq!(changes.removed, vec!["b.rs"]);
}

#[test]
fn test_modified_file() {
    let comparator = SnapshotComparator::new();
    let old = create_test_snapshot(vec![("a.rs", "hash1")]);
    let new = create_test_snapshot(vec![("a.rs", "hash_changed")]);

    let changes = comparator.compare(&old, &new);
    assert!(comparator.has_changes(&changes));
    assert_eq!(changes.modified, vec!["a.rs"]);
}

#[test]
fn test_change_count() {
    let comparator = SnapshotComparator::new();
    let old = create_test_snapshot(vec![("a.rs", "hash1"), ("b.rs", "hash2")]);
    let new = create_test_snapshot(vec![("a.rs", "hash_new"), ("c.rs", "hash3")]);

    let changes = comparator.compare(&old, &new);
    // a.rs modified, b.rs removed, c.rs added = 3 changes
    assert_eq!(comparator.change_count(&changes), 3);
}
