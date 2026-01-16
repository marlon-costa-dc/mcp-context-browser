//! Hybrid Search Engine Tests

use mcb_domain::entities::CodeChunk;
use mcb_domain::value_objects::SearchResult;
use mcb_infrastructure::adapters::hybrid_search::HybridSearchEngine;
use serde_json::json;

fn create_test_chunk(id: &str, content: &str, file_path: &str, start_line: u32) -> CodeChunk {
    CodeChunk {
        id: id.to_string(),
        content: content.to_string(),
        file_path: file_path.to_string(),
        start_line,
        end_line: start_line + 10,
        language: "rust".to_string(),
        metadata: json!({}),
    }
}

fn create_test_search_result(file_path: &str, start_line: u32, score: f64) -> SearchResult {
    SearchResult {
        id: format!("{}:{}", file_path, start_line),
        content: "test content".to_string(),
        file_path: file_path.to_string(),
        start_line,
        score,
        language: "rust".to_string(),
    }
}

#[test]
fn test_hybrid_engine_creation() {
    let engine = HybridSearchEngine::new(0.4, 0.6);
    assert!(!engine.has_bm25_index());
    assert!(engine.documents.is_empty());
}

#[test]
fn test_add_documents() {
    let mut engine = HybridSearchEngine::new(0.4, 0.6);
    let chunks = vec![
        create_test_chunk("1", "fn authenticate_user()", "auth.rs", 10),
        create_test_chunk("2", "fn validate_password()", "auth.rs", 20),
    ];
    engine.add_documents(chunks);

    assert!(engine.has_bm25_index());
    assert_eq!(engine.documents.len(), 2);
}

#[test]
fn test_deduplication() {
    let mut engine = HybridSearchEngine::new(0.4, 0.6);
    let chunk = create_test_chunk("1", "fn test()", "test.rs", 10);
    engine.add_documents(vec![chunk.clone()]);
    engine.add_documents(vec![chunk.clone()]);

    assert_eq!(engine.documents.len(), 1);
}

#[test]
fn test_clear() {
    let mut engine = HybridSearchEngine::new(0.4, 0.6);
    engine.add_documents(vec![create_test_chunk("1", "fn test()", "test.rs", 10)]);
    assert!(engine.has_bm25_index());

    engine.clear();
    assert!(!engine.has_bm25_index());
    assert!(engine.documents.is_empty());
}

#[test]
fn test_hybrid_search_without_index() {
    let engine = HybridSearchEngine::new(0.4, 0.6);
    let semantic_results = vec![create_test_search_result("test.rs", 10, 0.9)];

    let results = engine
        .hybrid_search("test", semantic_results, 10)
        .expect("search should succeed");

    assert_eq!(results.len(), 1);
    assert!((results[0].hybrid_score - 0.9).abs() < 0.01);
}

#[test]
fn test_hybrid_search_with_index() {
    let mut engine = HybridSearchEngine::new(0.4, 0.6);
    let chunks = vec![create_test_chunk(
        "1",
        "fn authenticate_user(username: &str)",
        "auth.rs",
        10,
    )];
    engine.add_documents(chunks);

    let semantic_results = vec![create_test_search_result("auth.rs", 10, 0.8)];
    let results = engine
        .hybrid_search("authenticate user", semantic_results, 10)
        .expect("search should succeed");

    assert_eq!(results.len(), 1);
    assert!(results[0].bm25_score > 0.0);
    assert!(results[0].hybrid_score > 0.0);
}

#[test]
fn test_get_bm25_stats() {
    let mut engine = HybridSearchEngine::new(0.4, 0.6);
    assert!(engine.get_bm25_stats().is_none());

    engine.add_documents(vec![create_test_chunk(
        "1",
        "fn test_function()",
        "test.rs",
        1,
    )]);
    let stats = engine.get_bm25_stats().expect("stats should be available");

    assert!(stats.contains_key("total_documents"));
    assert!(stats.contains_key("unique_terms"));
    assert!(stats.contains_key("average_doc_length"));
}
