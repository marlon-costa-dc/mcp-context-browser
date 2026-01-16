//! BM25 Scorer Tests

use mcb_domain::entities::CodeChunk;
use mcb_infrastructure::adapters::hybrid_search::{BM25Params, BM25Scorer};
use serde_json::json;

fn create_test_chunk(id: &str, content: &str) -> CodeChunk {
    CodeChunk {
        id: id.to_string(),
        content: content.to_string(),
        file_path: "test.rs".to_string(),
        start_line: 1,
        end_line: 10,
        language: "rust".to_string(),
        metadata: json!({}),
    }
}

#[test]
fn test_tokenize() {
    let tokens = BM25Scorer::tokenize("fn authenticate_user(username: &str)");
    assert!(tokens.contains(&"authenticate".to_string()));
    assert!(tokens.contains(&"user".to_string()));
    assert!(tokens.contains(&"username".to_string()));
    assert!(tokens.contains(&"str".to_string()));
}

#[test]
fn test_bm25_scorer_creation() {
    let chunks = vec![
        create_test_chunk("1", "fn authenticate_user(username: &str) -> bool"),
        create_test_chunk("2", "fn login_with_password(password: &str) -> Result"),
    ];
    let scorer = BM25Scorer::new(&chunks, BM25Params::default());

    assert_eq!(scorer.total_docs, 2);
    assert!(scorer.avg_doc_len > 0.0);
    assert!(!scorer.document_freq.is_empty());
}

#[test]
fn test_bm25_scoring() {
    let chunks = vec![
        create_test_chunk("1", "fn authenticate_user(username: &str) -> bool"),
        create_test_chunk("2", "fn create_database_connection() -> Connection"),
    ];
    let scorer = BM25Scorer::new(&chunks, BM25Params::default());

    let auth_score = scorer.score(&chunks[0], "authenticate user");
    let db_score = scorer.score(&chunks[0], "database connection");

    // Auth chunk should score higher for auth query
    assert!(auth_score > db_score);
}

#[test]
fn test_batch_scoring() {
    let chunks = vec![
        create_test_chunk("1", "fn authenticate_user(username: &str) -> bool"),
        create_test_chunk("2", "fn validate_user(user_id: u64) -> bool"),
    ];
    let scorer = BM25Scorer::new(&chunks, BM25Params::default());

    let refs: Vec<&CodeChunk> = chunks.iter().collect();
    let scores = scorer.score_batch(&refs, "user");

    assert_eq!(scores.len(), 2);
    assert!(scores[0] > 0.0);
    assert!(scores[1] > 0.0);
}
