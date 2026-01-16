//! Encrypted vector store wrapper
//!
//! Provides encryption at rest for any vector store provider.
//! Wraps existing providers with AES-256-GCM encryption.

use crate::crypto::CryptoService;
use async_trait::async_trait;
use mcb_domain::error::{Error, Result};
use mcb_domain::ports::providers::VectorStoreProvider;
use mcb_domain::value_objects::{Embedding, SearchResult};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// Encrypted vector store provider
///
/// Wraps any VectorStoreProvider implementation to provide encryption at rest.
/// Vectors are stored unencrypted for searchability, but metadata is encrypted
/// using AES-256-GCM.
pub struct EncryptedVectorStoreProvider<P: VectorStoreProvider> {
    /// Underlying vector store provider
    inner: P,
    /// Cryptography service
    crypto: Arc<CryptoService>,
}

impl<P: VectorStoreProvider> EncryptedVectorStoreProvider<P> {
    /// Create a new encrypted vector store provider with a master key
    pub fn new(inner: P, master_key: Vec<u8>) -> Result<Self> {
        let crypto = Arc::new(CryptoService::new(master_key)?);
        Ok(Self { inner, crypto })
    }

    /// Create with existing crypto service
    pub fn with_crypto_service(inner: P, crypto: Arc<CryptoService>) -> Self {
        Self { inner, crypto }
    }
}

#[async_trait]
impl<P: VectorStoreProvider> VectorStoreProvider for EncryptedVectorStoreProvider<P> {
    async fn create_collection(&self, name: &str, dimensions: usize) -> Result<()> {
        self.inner.create_collection(name, dimensions).await
    }

    async fn delete_collection(&self, name: &str) -> Result<()> {
        self.inner.delete_collection(name).await
    }

    async fn collection_exists(&self, name: &str) -> Result<bool> {
        self.inner.collection_exists(name).await
    }

    async fn insert_vectors(
        &self,
        collection: &str,
        vectors: &[Embedding],
        metadata: Vec<HashMap<String, Value>>,
    ) -> Result<Vec<String>> {
        if vectors.len() != metadata.len() {
            return Err(Error::invalid_argument(
                "Vectors and metadata length mismatch",
            ));
        }

        // Keep vectors unencrypted for searchability, encrypt only sensitive metadata
        let mut processed_metadata = Vec::new();

        for meta in metadata {
            // Serialize sensitive metadata for encryption
            let metadata_json =
                serde_json::to_string(&meta).map_err(|e| Error::Infrastructure {
                    message: format!("Failed to serialize metadata: {}", e),
                    source: Some(Box::new(e)),
                })?;

            let encrypted_data = self.crypto.encrypt(metadata_json.as_bytes())?;

            // Create metadata with encryption info
            let mut processed_meta = HashMap::new();
            processed_meta.insert(
                "encrypted_metadata".to_string(),
                serde_json::to_value(&encrypted_data).map_err(|e| Error::Infrastructure {
                    message: format!("Failed to serialize encrypted data: {}", e),
                    source: Some(Box::new(e)),
                })?,
            );

            // Keep non-sensitive metadata unencrypted for filtering and SearchResult construction
            if let Some(content) = meta.get("content") {
                processed_meta.insert("content".to_string(), content.clone());
            }
            if let Some(file_path) = meta.get("file_path") {
                processed_meta.insert("file_path".to_string(), file_path.clone());
            }
            if let Some(start_line) = meta.get("start_line").or_else(|| meta.get("line_number")) {
                processed_meta.insert("start_line".to_string(), start_line.clone());
            }
            if let Some(language) = meta.get("language") {
                processed_meta.insert("language".to_string(), language.clone());
            }

            processed_metadata.push(processed_meta);
        }

        self.inner
            .insert_vectors(collection, vectors, processed_metadata)
            .await
    }

    async fn search_similar(
        &self,
        collection: &str,
        query_vector: &[f32],
        limit: usize,
        filter: Option<&str>,
    ) -> Result<Vec<SearchResult>> {
        // Search using the inner provider (vectors are unencrypted)
        // Note: The inner provider returns results with partial metadata (unencrypted fields only)
        // We trust the inner provider's SearchResult structure is correct
        self.inner
            .search_similar(collection, query_vector, limit, filter)
            .await
    }

    async fn delete_vectors(&self, collection: &str, ids: &[String]) -> Result<()> {
        self.inner.delete_vectors(collection, ids).await
    }

    async fn get_vectors_by_ids(
        &self,
        collection: &str,
        ids: &[String],
    ) -> Result<Vec<SearchResult>> {
        // Delegate to inner provider - SearchResult fields are extracted from stored metadata
        self.inner.get_vectors_by_ids(collection, ids).await
    }

    async fn list_vectors(&self, collection: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // Delegate to inner provider - SearchResult fields are extracted from stored metadata
        self.inner.list_vectors(collection, limit).await
    }

    async fn get_stats(&self, collection: &str) -> Result<HashMap<String, Value>> {
        let mut stats = self.inner.get_stats(collection).await?;
        stats.insert("encryption_enabled".to_string(), serde_json::json!(true));
        stats.insert(
            "encryption_algorithm".to_string(),
            serde_json::json!("AES-256-GCM"),
        );
        Ok(stats)
    }

    async fn flush(&self, collection: &str) -> Result<()> {
        self.inner.flush(collection).await
    }

    fn provider_name(&self) -> &str {
        "encrypted"
    }
}
