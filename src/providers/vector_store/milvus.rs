//! Milvus vector store provider implementation

use crate::core::error::{Error, Result};
use crate::core::types::{Embedding, SearchResult};
use crate::providers::VectorStoreProvider;
use async_trait::async_trait;
use milvus::client::Client;
use milvus::data::FieldColumn;
use milvus::schema::{CollectionSchemaBuilder, FieldSchema};
// TODO: Import search functionality when milvus crate API is available
// use milvus::search::{SearchIter, SearchResult as MilvusSearchResult};
use std::collections::HashMap;
use std::sync::Arc;

/// Milvus vector store provider
pub struct MilvusVectorStoreProvider {
    client: Arc<Client>,
}

impl MilvusVectorStoreProvider {
    /// Create a new Milvus vector store provider
    pub async fn new(address: String, token: Option<String>) -> Result<Self> {
        let mut client = Client::new(&address)
            .await
            .map_err(|e| Error::vector_db(format!("Failed to connect to Milvus: {}", e)))?;

        if let Some(token) = token {
            // Set authentication if token provided
            client.set_token(token);
        }

        Ok(Self {
            client: Arc::new(client),
        })
    }
}

#[async_trait]
impl VectorStoreProvider for MilvusVectorStoreProvider {
    async fn create_collection(&self, _name: &str, _dimensions: usize) -> Result<()> {
        // TODO: Implement Milvus collection creation when crate API is available
        Err(Error::vector_db("Milvus collection creation not yet implemented".to_string()))
    }

    async fn delete_collection(&self, name: &str) -> Result<()> {
        self.client
            .drop_collection(name)
            .await
            .map_err(|e| Error::vector_db(format!("Failed to delete collection: {}", e)))?;
        Ok(())
    }

    async fn collection_exists(&self, name: &str) -> Result<bool> {
        match self.client.describe_collection(name).await {
            Ok(_) => Ok(true),
            Err(milvus::error::Error::CollectionNotExists(_)) => Ok(false),
            Err(e) => Err(Error::vector_db(format!(
                "Failed to check collection: {}",
                e
            ))),
        }
    }

    async fn insert_vectors(
        &self,
        collection: &str,
        vectors: &[Embedding],
        metadata: Vec<HashMap<String, serde_json::Value>>,
    ) -> Result<Vec<String>> {
        if vectors.len() != metadata.len() {
            return Err(Error::vector_db("Vectors and metadata count mismatch"));
        }

        let mut ids = Vec::new();
        let mut vector_data = Vec::new();
        let mut content_data = Vec::new();
        let mut file_path_data = Vec::new();
        let mut start_line_data = Vec::new();
        let mut end_line_data = Vec::new();

        for (i, (vector, meta)) in vectors.iter().zip(metadata).enumerate() {
            let id = format!("{}_{}", collection, i);
            ids.push(id.clone());

            vector_data.push(vector.vector.clone());
            content_data.push(
                meta.get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            );
            file_path_data.push(
                meta.get("file_path")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            );
            start_line_data
                .push(meta.get("start_line").and_then(|v| v.as_u64()).unwrap_or(0) as i64);
            end_line_data.push(meta.get("end_line").and_then(|v| v.as_u64()).unwrap_or(0) as i64);
        }

        // Insert data
        let mut columns = Vec::new();
        columns.push(FieldColumn::new("id", ids));
        columns.push(FieldColumn::new("vector", vector_data));
        columns.push(FieldColumn::new("content", content_data));
        columns.push(FieldColumn::new("file_path", file_path_data));
        columns.push(FieldColumn::new("start_line", start_line_data));
        columns.push(FieldColumn::new("end_line", end_line_data));

        self.client
            .insert(collection, "", columns)
            .await
            .map_err(|e| Error::vector_db(format!("Failed to insert vectors: {}", e)))?;

        // Flush to make data searchable
        self.client
            .flush(collection)
            .await
            .map_err(|e| Error::vector_db(format!("Failed to flush: {}", e)))?;

        Ok((0..vectors.len())
            .map(|i| format!("{}_{}", collection, i))
            .collect())
    }

    async fn search_similar(
        &self,
        _collection: &str,
        _query_vector: &[f32],
        _limit: usize,
        _filter: Option<&str>,
    ) -> Result<Vec<crate::core::types::SearchResult>> {
        // TODO: Implement Milvus search when crate API is available
        Err(Error::vector_db("Milvus search not yet implemented".to_string()))
    }

    async fn delete_vectors(&self, collection: &str, ids: &[String]) -> Result<()> {
        // Delete by expression (ID in list)
        let expr = format!(
            "id in {}",
            serde_json::to_string(ids)
                .map_err(|e| Error::vector_db(format!("Failed to serialize IDs: {}", e)))?
        );

        self.client
            .delete(collection, &expr)
            .await
            .map_err(|e| Error::vector_db(format!("Failed to delete vectors: {}", e)))?;

        Ok(())
    }

    async fn get_stats(&self, collection: &str) -> Result<HashMap<String, serde_json::Value>> {
        let stats = self
            .client
            .get_collection_statistics(collection)
            .await
            .map_err(|e| Error::vector_db(format!("Failed to get stats: {}", e)))?;

        let mut result = HashMap::new();
        result.insert("count".to_string(), serde_json::json!(stats.row_count));
        result.insert(
            "segments".to_string(),
            serde_json::json!(stats.segment_count),
        );

        Ok(result)
    }

    async fn flush(&self, collection: &str) -> Result<()> {
        self.client
            .flush(collection)
            .await
            .map_err(|e| Error::vector_db(format!("Failed to flush: {}", e)))?;
        Ok(())
    }

    fn provider_name(&self) -> &str {
        "milvus"
    }
}
