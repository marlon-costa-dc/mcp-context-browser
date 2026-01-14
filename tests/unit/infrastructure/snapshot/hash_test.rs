//! Tests for hash calculator service
//!
//! Migrated from src/infrastructure/snapshot/hash.rs

use mcp_context_browser::infrastructure::snapshot::HashCalculator;

#[test]
fn test_hash_content() {
    let calculator = HashCalculator::new();
    let hash = calculator.hash_content("hello world");

    // SHA-256 of "hello world"
    assert_eq!(
        hash,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
}

#[test]
fn test_hash_content_deterministic() {
    let calculator = HashCalculator::new();
    let hash1 = calculator.hash_content("test content");
    let hash2 = calculator.hash_content("test content");
    assert_eq!(hash1, hash2);
}

#[test]
fn test_hash_different_content() {
    let calculator = HashCalculator::new();
    let hash1 = calculator.hash_content("content a");
    let hash2 = calculator.hash_content("content b");
    assert_ne!(hash1, hash2);
}

#[test]
fn test_hash_bytes() {
    let calculator = HashCalculator::new();
    let hash = calculator.hash_bytes(b"hello world");
    assert_eq!(
        hash,
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    );
}
