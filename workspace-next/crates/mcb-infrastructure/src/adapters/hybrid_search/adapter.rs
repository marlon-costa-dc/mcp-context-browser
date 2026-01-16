//! Hybrid search adapter implementation

use super::actor::HybridSearchMessage;
use async_trait::async_trait;
use mcb_domain::entities::CodeChunk;
use mcb_domain::error::Result;
use mcb_domain::ports::providers::HybridSearchProvider;
use mcb_domain::value_objects::SearchResult;
use std::collections::HashMap;
use tokio::sync::{mpsc, oneshot};

/// Hybrid search adapter that implements the port
pub struct HybridSearchAdapter {
    sender: mpsc::Sender<HybridSearchMessage>,
}

impl HybridSearchAdapter {
    /// Create a new hybrid search adapter
    pub fn new(sender: mpsc::Sender<HybridSearchMessage>) -> Self {
        Self { sender }
    }
}

#[async_trait]
impl HybridSearchProvider for HybridSearchAdapter {
    async fn index_chunks(&self, collection: &str, chunks: &[CodeChunk]) -> Result<()> {
        self.sender
            .send(HybridSearchMessage::Index {
                collection: collection.to_string(),
                chunks: chunks.to_vec(),
            })
            .await
            .map_err(|e| {
                mcb_domain::error::Error::internal(format!(
                    "Failed to send to hybrid search actor: {}",
                    e
                ))
            })
    }

    async fn search(
        &self,
        _collection: &str,
        query: &str,
        semantic_results: Vec<SearchResult>,
        limit: usize,
    ) -> Result<Vec<SearchResult>> {
        let (respond_to, receiver) = oneshot::channel();
        self.sender
            .send(HybridSearchMessage::Search {
                query: query.to_string(),
                semantic_results,
                limit,
                respond_to,
            })
            .await
            .map_err(|e| {
                mcb_domain::error::Error::internal(format!(
                    "Failed to send search to hybrid search actor: {}",
                    e
                ))
            })?;

        let hybrid_results = receiver.await.map_err(|e| {
            mcb_domain::error::Error::internal(format!(
                "Failed to receive hybrid search results: {}",
                e
            ))
        })??;

        Ok(hybrid_results
            .into_iter()
            .map(|hybrid_result| {
                let mut result = hybrid_result.result;
                // Update score with hybrid score (as f64)
                result.score = hybrid_result.hybrid_score as f64;
                result
            })
            .collect())
    }

    async fn clear_collection(&self, collection: &str) -> Result<()> {
        self.sender
            .send(HybridSearchMessage::Clear {
                collection: collection.to_string(),
            })
            .await
            .map_err(|e| {
                mcb_domain::error::Error::internal(format!(
                    "Failed to send clear to hybrid search actor: {}",
                    e
                ))
            })
    }

    async fn get_stats(&self) -> HashMap<String, serde_json::Value> {
        let (respond_to, receiver) = oneshot::channel();
        if self
            .sender
            .send(HybridSearchMessage::GetStats { respond_to })
            .await
            .is_err()
        {
            return HashMap::new();
        }

        receiver.await.unwrap_or_default()
    }
}
