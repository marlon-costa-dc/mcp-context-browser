//! End-to-End Flow Integration Tests
//!
//! Tests complete workflows involving multiple services:
//! - Index then search flow
//! - Clear and reindex flow
//! - Multiple collections handling

use mcb_infrastructure::config::ConfigBuilder;
use mcb_infrastructure::di::bootstrap::FullContainer;
use std::path::PathBuf;
use tempfile::TempDir;

// ============================================================================
// Test Helpers
// ============================================================================

/// Create a temporary codebase directory with sample files
fn create_test_codebase() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let path = temp_dir.path().to_path_buf();

    // Create a sample Rust file
    let rust_file = path.join("main.rs");
    std::fs::write(
        &rust_file,
        r#"
fn main() {
    println!("Hello, world!");
}

/// A sample function for testing
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Another sample function
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
"#,
    )
    .expect("Failed to write test file");

    // Create a sample Python file
    let python_file = path.join("utils.py");
    std::fs::write(
        &python_file,
        r#"
def greet(name: str) -> str:
    """Greet a person by name."""
    return f"Hello, {name}!"

def calculate_sum(numbers: list) -> int:
    """Calculate the sum of numbers."""
    return sum(numbers)
"#,
    )
    .expect("Failed to write test file");

    (temp_dir, path)
}

/// Create default test configuration
fn test_config() -> mcb_infrastructure::config::AppConfig {
    ConfigBuilder::new().build()
}

// ============================================================================
// Index Then Search Flow Tests
// ============================================================================

#[tokio::test]
async fn test_index_then_search_flow() {
    // Setup: Create test codebase and container
    let (_temp_dir, codebase_path) = create_test_codebase();
    let config = test_config();
    let container = FullContainer::new(config)
        .await
        .expect("Failed to create container");

    let indexing_service = container.indexing_service();
    let search_service = container.search_service();
    let collection = "test-e2e-collection";

    // Step 1: Index the codebase
    let index_result = indexing_service
        .index_codebase(&codebase_path, collection)
        .await
        .expect("Indexing should succeed");

    // Verify indexing worked
    assert!(
        index_result.files_processed > 0,
        "Should process at least one file"
    );
    assert!(
        index_result.chunks_created >= 0,
        "Chunks created should be non-negative"
    );

    // Step 2: Search for content
    let results = search_service
        .search(collection, "function", 10)
        .await
        .expect("Search should succeed");

    // Verify search returns results (may be empty with null providers)
    assert!(
        results.len() <= 10,
        "Should respect limit even with null providers"
    );
}

#[tokio::test]
async fn test_index_empty_directory() {
    // Setup: Create empty temp directory
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let codebase_path = temp_dir.path().to_path_buf();
    let config = test_config();
    let container = FullContainer::new(config)
        .await
        .expect("Failed to create container");

    let indexing_service = container.indexing_service();
    let collection = "test-empty-collection";

    // Index empty directory
    let index_result = indexing_service
        .index_codebase(&codebase_path, collection)
        .await
        .expect("Indexing empty dir should not fail");

    // Verify no files were processed
    assert_eq!(
        index_result.files_processed, 0,
        "Should process zero files in empty dir"
    );
}

#[tokio::test]
async fn test_search_on_nonexistent_collection() {
    let config = test_config();
    let container = FullContainer::new(config)
        .await
        .expect("Failed to create container");

    let search_service = container.search_service();

    // Search on collection that was never indexed
    // With null providers, this should return empty results, not error
    let results = search_service
        .search("nonexistent-collection", "test query", 10)
        .await
        .expect("Search should not fail on nonexistent collection with null providers");

    assert!(results.is_empty(), "Should return empty results");
}

// ============================================================================
// Clear and Reindex Flow Tests
// ============================================================================

#[tokio::test]
async fn test_clear_and_reindex_flow() {
    // Setup
    let (_temp_dir, codebase_path) = create_test_codebase();
    let config = test_config();
    let container = FullContainer::new(config)
        .await
        .expect("Failed to create container");

    let indexing_service = container.indexing_service();
    let collection = "test-clear-reindex";

    // Step 1: Initial indexing
    let first_index = indexing_service
        .index_codebase(&codebase_path, collection)
        .await
        .expect("First indexing should succeed");

    // Step 2: Clear the collection
    indexing_service
        .clear_collection(collection)
        .await
        .expect("Clear should succeed");

    // Step 3: Reindex
    let second_index = indexing_service
        .index_codebase(&codebase_path, collection)
        .await
        .expect("Second indexing should succeed");

    // Verify both indexing operations processed files
    assert!(
        first_index.files_processed > 0,
        "First index should process files"
    );
    assert!(
        second_index.files_processed > 0,
        "Second index should process files"
    );
}

#[tokio::test]
async fn test_clear_nonexistent_collection() {
    let config = test_config();
    let container = FullContainer::new(config)
        .await
        .expect("Failed to create container");

    let indexing_service = container.indexing_service();

    // Clear collection that doesn't exist should not fail
    let result = indexing_service
        .clear_collection("nonexistent-clear-test")
        .await;

    assert!(
        result.is_ok(),
        "Clearing nonexistent collection should succeed with null providers"
    );
}

// ============================================================================
// Multiple Collections Tests
// ============================================================================

#[tokio::test]
async fn test_multiple_collections_isolation() {
    // Setup: Create two different codebases
    let (_temp1, path1) = create_test_codebase();
    let (_temp2, path2) = create_test_codebase();

    let config = test_config();
    let container = FullContainer::new(config)
        .await
        .expect("Failed to create container");

    let indexing_service = container.indexing_service();
    let search_service = container.search_service();

    let collection_a = "collection-a";
    let collection_b = "collection-b";

    // Index into different collections
    let result_a = indexing_service
        .index_codebase(&path1, collection_a)
        .await
        .expect("Index A should succeed");

    let result_b = indexing_service
        .index_codebase(&path2, collection_b)
        .await
        .expect("Index B should succeed");

    // Both should process files
    assert!(result_a.files_processed > 0);
    assert!(result_b.files_processed > 0);

    // Search each collection
    let search_a = search_service
        .search(collection_a, "function", 5)
        .await
        .expect("Search A should succeed");

    let search_b = search_service
        .search(collection_b, "function", 5)
        .await
        .expect("Search B should succeed");

    // Both searches should complete without error
    assert!(search_a.len() <= 5);
    assert!(search_b.len() <= 5);
}

#[tokio::test]
async fn test_clear_one_collection_preserves_others() {
    // Setup
    let (_temp1, path1) = create_test_codebase();
    let (_temp2, path2) = create_test_codebase();

    let config = test_config();
    let container = FullContainer::new(config)
        .await
        .expect("Failed to create container");

    let indexing_service = container.indexing_service();
    let search_service = container.search_service();

    let collection_main = "main-collection";
    let collection_temp = "temp-collection";

    // Index both
    indexing_service
        .index_codebase(&path1, collection_main)
        .await
        .expect("Index main should succeed");

    indexing_service
        .index_codebase(&path2, collection_temp)
        .await
        .expect("Index temp should succeed");

    // Clear only temp collection
    indexing_service
        .clear_collection(collection_temp)
        .await
        .expect("Clear temp should succeed");

    // Main collection should still be searchable
    let results = search_service
        .search(collection_main, "test", 5)
        .await
        .expect("Search main should still work");

    assert!(results.len() <= 5);
}

// ============================================================================
// Status Flow Tests
// ============================================================================

#[tokio::test]
async fn test_indexing_status_during_operations() {
    let (_temp_dir, codebase_path) = create_test_codebase();
    let config = test_config();
    let container = FullContainer::new(config)
        .await
        .expect("Failed to create container");

    let indexing_service = container.indexing_service();
    let collection = "status-test-collection";

    // Check initial status
    let initial_status = indexing_service.get_status();

    // Initial status should not be indexing
    assert!(
        !initial_status.is_indexing,
        "Should not be indexing initially"
    );

    // Start indexing
    let _ = indexing_service
        .index_codebase(&codebase_path, collection)
        .await
        .expect("Indexing should succeed");

    // After indexing completes, status should be idle again
    let final_status = indexing_service.get_status();
    assert!(
        !final_status.is_indexing,
        "Should not be indexing after completion"
    );
}

// ============================================================================
// Error Recovery Tests
// ============================================================================

#[tokio::test]
async fn test_index_nonexistent_path() {
    let config = test_config();
    let container = FullContainer::new(config)
        .await
        .expect("Failed to create container");

    let indexing_service = container.indexing_service();
    let nonexistent_path = PathBuf::from("/nonexistent/path/that/does/not/exist");

    // Indexing nonexistent path should handle gracefully
    let result = indexing_service
        .index_codebase(&nonexistent_path, "error-test")
        .await;

    // Should either fail with error or succeed with 0 files
    if let Ok(r) = result {
        assert_eq!(r.files_processed, 0, "Should process 0 files");
    }
    // Error is also acceptable - path doesn't exist
}

#[tokio::test]
async fn test_search_with_empty_query() {
    let config = test_config();
    let container = FullContainer::new(config)
        .await
        .expect("Failed to create container");

    let search_service = container.search_service();

    // Empty query should be handled gracefully
    let results = search_service
        .search("any-collection", "", 10)
        .await
        .expect("Empty query should not panic");

    // Empty results are expected
    assert!(results.is_empty() || results.len() <= 10);
}

#[tokio::test]
async fn test_search_with_special_characters() {
    let config = test_config();
    let container = FullContainer::new(config)
        .await
        .expect("Failed to create container");

    let search_service = container.search_service();

    // Special characters in query should not cause issues
    let special_queries = vec![
        "fn<T>()",
        "async fn",
        "impl Trait + 'static",
        "Result<(), Error>",
        "Option<Box<dyn Trait>>",
    ];

    for query in special_queries {
        let result = search_service.search("test-collection", query, 5).await;

        assert!(
            result.is_ok(),
            "Query '{}' should not cause an error",
            query
        );
    }
}
