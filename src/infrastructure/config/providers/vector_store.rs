//! # Vector Store Configuration
//!
//! Configuration types for vector storage backends (Milvus, EdgeVec, Filesystem).
//! Defines connection pools, indexing parameters, and persistence settings.

use serde::{Deserialize, Serialize};
use validator::Validate;

/// Vector store provider configuration types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "provider")]
pub enum VectorStoreProviderConfig {
    /// EdgeVec in-memory vector store configuration
    #[serde(rename = "edgevec")]
    EdgeVec {
        /// Maximum number of vectors to store
        #[serde(default)]
        max_vectors: Option<usize>,
        /// Default collection name
        #[serde(default)]
        collection: Option<String>,
        /// HNSW M parameter (neighbors per layer)
        #[serde(default)]
        hnsw_m: Option<usize>,
        /// HNSW ef_construction parameter (search candidates during build)
        #[serde(default)]
        hnsw_ef_construction: Option<usize>,
        /// Distance metric (cosine, euclidean, dot)
        #[serde(default)]
        distance_metric: Option<String>,
        /// Whether to use quantization for memory efficiency
        #[serde(default)]
        use_quantization: Option<bool>,
    },
    /// Milvus vector database configuration
    #[serde(rename = "milvus")]
    Milvus {
        /// Milvus server address (host:port)
        address: String,
        /// Authentication token (optional)
        #[serde(default)]
        token: Option<String>,
        /// Default collection name
        #[serde(default)]
        collection: Option<String>,
        /// Expected vector dimensions
        #[serde(default)]
        dimensions: Option<usize>,
    },
    /// Pinecone vector database configuration
    #[serde(rename = "pinecone")]
    Pinecone {
        /// Pinecone API key for authentication
        api_key: String,
        /// Pinecone environment name
        environment: String,
        /// Pinecone index name
        index_name: String,
        /// Expected vector dimensions
        #[serde(default)]
        dimensions: Option<usize>,
    },
    /// Qdrant vector database configuration
    #[serde(rename = "qdrant")]
    Qdrant {
        /// Qdrant server URL
        url: String,
        /// API key for authentication (optional)
        #[serde(default)]
        api_key: Option<String>,
        /// Default collection name
        #[serde(default)]
        collection: Option<String>,
        /// Expected vector dimensions
        #[serde(default)]
        dimensions: Option<usize>,
    },
    /// In-memory vector store for development and testing
    #[serde(rename = "in-memory")]
    InMemory {
        /// Expected vector dimensions
        #[serde(default)]
        dimensions: Option<usize>,
    },
    /// Filesystem-based vector store configuration
    #[serde(rename = "filesystem")]
    Filesystem {
        /// Base directory path for storing vectors
        #[serde(default)]
        base_path: Option<String>,
        /// Maximum vectors per shard file
        #[serde(default)]
        max_vectors_per_shard: Option<usize>,
        /// Expected vector dimensions
        #[serde(default)]
        dimensions: Option<usize>,
        /// Whether to enable data compression
        #[serde(default)]
        compression_enabled: Option<bool>,
        /// Size of the index cache in bytes
        #[serde(default)]
        index_cache_size: Option<usize>,
        /// Whether to use memory mapping for performance
        #[serde(default)]
        memory_mapping_enabled: Option<bool>,
    },
}

impl Validate for VectorStoreProviderConfig {
    fn validate(&self) -> std::result::Result<(), validator::ValidationErrors> {
        let mut errors = validator::ValidationErrors::new();
        match self {
            VectorStoreProviderConfig::Milvus { address, .. } => {
                if address.is_empty() {
                    errors.add("address", validator::ValidationError::new("length"));
                }
            }
            VectorStoreProviderConfig::Pinecone {
                api_key,
                environment,
                index_name,
                ..
            } => {
                if api_key.is_empty() {
                    errors.add("api_key", validator::ValidationError::new("length"));
                }
                if environment.is_empty() {
                    errors.add("environment", validator::ValidationError::new("length"));
                }
                if index_name.is_empty() {
                    errors.add("index_name", validator::ValidationError::new("length"));
                }
            }
            VectorStoreProviderConfig::Qdrant { url, .. } => {
                if url.is_empty() {
                    errors.add("url", validator::ValidationError::new("length"));
                }
            }
            _ => {}
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
