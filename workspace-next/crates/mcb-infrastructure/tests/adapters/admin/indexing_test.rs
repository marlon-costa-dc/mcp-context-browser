//! DefaultIndexingOperations Tests
//!
//! Comprehensive tests for the indexing operations tracker.

use mcb_domain::ports::admin::IndexingOperationsInterface;
use mcb_infrastructure::adapters::admin::DefaultIndexingOperations;
use std::sync::Arc;
use std::thread;

#[test]
fn test_start_indexing_operation() {
    let ops = DefaultIndexingOperations::new();

    let id = ops.start_operation("my-collection", 100);

    assert!(!id.is_empty());
    assert!(ops.has_active_operations());
    assert_eq!(ops.active_count(), 1);
}

#[test]
fn test_update_operation_progress() {
    let ops = DefaultIndexingOperations::new();

    let id = ops.start_operation("my-collection", 100);

    ops.update_progress(&id, Some("file1.rs".to_string()), 10);

    let operations = ops.get_operations();
    let op = operations.get(&id).expect("Operation should exist");

    assert_eq!(op.current_file, Some("file1.rs".to_string()));
    assert_eq!(op.processed_files, 10);
    assert_eq!(op.total_files, 100);
}

#[test]
fn test_complete_operation() {
    let ops = DefaultIndexingOperations::new();

    let id = ops.start_operation("my-collection", 50);
    assert!(ops.has_active_operations());

    ops.complete_operation(&id);

    assert!(!ops.has_active_operations());
    assert_eq!(ops.active_count(), 0);
}

#[test]
fn test_has_active_operations() {
    let ops = DefaultIndexingOperations::new();

    assert!(!ops.has_active_operations());

    let id1 = ops.start_operation("coll1", 10);
    assert!(ops.has_active_operations());

    let id2 = ops.start_operation("coll2", 20);
    assert!(ops.has_active_operations());

    ops.complete_operation(&id1);
    assert!(ops.has_active_operations()); // id2 still active

    ops.complete_operation(&id2);
    assert!(!ops.has_active_operations());
}

#[test]
fn test_active_count() {
    let ops = DefaultIndexingOperations::new();

    assert_eq!(ops.active_count(), 0);

    let id1 = ops.start_operation("coll1", 10);
    assert_eq!(ops.active_count(), 1);

    let id2 = ops.start_operation("coll2", 20);
    assert_eq!(ops.active_count(), 2);

    let id3 = ops.start_operation("coll3", 30);
    assert_eq!(ops.active_count(), 3);

    ops.complete_operation(&id2);
    assert_eq!(ops.active_count(), 2);

    ops.complete_operation(&id1);
    ops.complete_operation(&id3);
    assert_eq!(ops.active_count(), 0);
}

#[test]
fn test_get_active_operations() {
    let ops = DefaultIndexingOperations::new();

    let id1 = ops.start_operation("collection-a", 100);
    let id2 = ops.start_operation("collection-b", 200);

    ops.update_progress(&id1, Some("main.rs".to_string()), 50);

    let operations = ops.get_operations();

    assert_eq!(operations.len(), 2);
    assert!(operations.contains_key(&id1));
    assert!(operations.contains_key(&id2));

    let op1 = operations.get(&id1).unwrap();
    assert_eq!(op1.collection, "collection-a");
    assert_eq!(op1.total_files, 100);
    assert_eq!(op1.processed_files, 50);

    let op2 = operations.get(&id2).unwrap();
    assert_eq!(op2.collection, "collection-b");
    assert_eq!(op2.total_files, 200);
    assert_eq!(op2.processed_files, 0);
}

#[test]
fn test_concurrent_operations() {
    let ops = Arc::new(DefaultIndexingOperations::new());
    let mut handles = vec![];

    // Spawn 5 threads, each starting and completing an operation
    for i in 0..5 {
        let o = Arc::clone(&ops);
        let handle = thread::spawn(move || {
            let collection = format!("coll-{}", i);
            let id = o.start_operation(&collection, 10);

            // Simulate some progress
            for j in 0..10 {
                o.update_progress(&id, Some(format!("file{}.rs", j)), j);
            }

            o.complete_operation(&id);
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    // All operations should be complete
    assert!(!ops.has_active_operations());
    assert_eq!(ops.active_count(), 0);
}

#[test]
fn test_operation_timestamp() {
    let ops = DefaultIndexingOperations::new();

    let id = ops.start_operation("test-coll", 50);

    let operations = ops.get_operations();
    let op = operations.get(&id).unwrap();

    // Timestamp should be reasonable (within last minute)
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    assert!(op.start_timestamp > 0);
    assert!(op.start_timestamp <= now);
    assert!(now - op.start_timestamp < 60); // Within last minute
}

#[test]
fn test_default_trait() {
    let ops = DefaultIndexingOperations::default();

    assert!(!ops.has_active_operations());
    assert_eq!(ops.active_count(), 0);
}

#[test]
fn test_new_shared() {
    let ops = DefaultIndexingOperations::new_shared();

    let id = ops.start_operation("shared-test", 25);
    assert!(ops.has_active_operations());

    ops.complete_operation(&id);
    assert!(!ops.has_active_operations());
}

#[test]
fn test_update_nonexistent_operation() {
    let ops = DefaultIndexingOperations::new();

    // This should not panic - just silently do nothing
    ops.update_progress("nonexistent-id", Some("file.rs".to_string()), 10);

    // Also completing nonexistent operation should not panic
    ops.complete_operation("nonexistent-id");

    // No operations should exist
    assert!(!ops.has_active_operations());
}
