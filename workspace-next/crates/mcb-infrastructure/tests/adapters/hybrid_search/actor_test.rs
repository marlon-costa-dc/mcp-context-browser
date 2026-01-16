//! Hybrid Search Actor Tests

use mcb_domain::entities::CodeChunk;
use mcb_infrastructure::adapters::hybrid_search::{spawn_hybrid_search_actor, HybridSearchMessage};
use serde_json::json;
use tokio::sync::oneshot;

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

#[tokio::test]
async fn test_actor_indexing() {
    let sender = spawn_hybrid_search_actor(0.4, 0.6);

    sender
        .send(HybridSearchMessage::Index {
            collection: "test".to_string(),
            chunks: vec![create_test_chunk("1", "fn test_function()")],
        })
        .await
        .expect("send should succeed");

    let (stats_tx, stats_rx) = oneshot::channel();
    sender
        .send(HybridSearchMessage::GetStats {
            respond_to: stats_tx,
        })
        .await
        .expect("send should succeed");

    let stats = stats_rx.await.expect("should receive stats");
    assert_eq!(
        stats.get("total_indexed_documents"),
        Some(&serde_json::json!(1))
    );
}

#[tokio::test]
async fn test_actor_clear_collection() {
    let sender = spawn_hybrid_search_actor(0.4, 0.6);

    // Index some documents
    sender
        .send(HybridSearchMessage::Index {
            collection: "test".to_string(),
            chunks: vec![create_test_chunk("1", "fn test_function()")],
        })
        .await
        .expect("send should succeed");

    // Clear the collection
    sender
        .send(HybridSearchMessage::Clear {
            collection: "test".to_string(),
        })
        .await
        .expect("send should succeed");

    // Check stats
    let (stats_tx, stats_rx) = oneshot::channel();
    sender
        .send(HybridSearchMessage::GetStats {
            respond_to: stats_tx,
        })
        .await
        .expect("send should succeed");

    let stats = stats_rx.await.expect("should receive stats");
    assert_eq!(
        stats.get("total_indexed_documents"),
        Some(&serde_json::json!(0))
    );
}
