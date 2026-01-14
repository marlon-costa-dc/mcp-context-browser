//! Vector store provider implementations
//!
//! Persistent storage backends for semantic embeddings.
//! Different backends provide tradeoffs between performance, scalability, and features.
//!
//! ## Available Backends
//!
//! | Backend | Persistence | Scale | Latency | Features | Use Case |
//! |---------|-------------|-------|---------|----------|----------|
//! | [`FilesystemVectorStore`] | File-based | Single-node | Medium | Snapshots | Development |
//! | [`InMemoryVectorStoreProvider`] | Memory-only | Single-node | Low | Fast | Testing |
//! | [`MilvusVectorStoreProvider`] | Database | Distributed | Medium | Advanced | Production |
//! | [`EdgeVecVectorStoreProvider`] | In-memory | Single-node | Very low | Optimized | Edge computing |
//! | [`EncryptedVectorStoreProvider`] | File/DB | Variable | Variable | Encryption | Secure storage |
//! | [`NullVectorStoreProvider`] | None | N/A | N/A | Stub | Unit tests |
//!
//! ## Architecture Patterns
//!
//! ### Single-Node (Development)
//! ```text
//! Application → InMemoryVectorStore
//!            ↓ (Restart loses data)
//!            or
//!            → FilesystemVectorStore
//!            ↓ (Persisted to disk)
//! ```
//!
//! ### Production (Distributed)
//! ```text
//! Application → MilvusVectorStore
//!            ↓
//!            → Milvus Cluster
//!            ↓
//!            → Multiple Replicas + Sharding
//! ```
//!
//! ### Secure (Encrypted at Rest)
//! ```text
//! Application → EncryptedVectorStore
//!            ↓ (AES-256-GCM encryption)
//!            → FilesystemVectorStore or Milvus
//! ```
//!
//! ## Selection Guide
//!
//! ### Development
//! - **Fast iteration**: [`InMemoryVectorStoreProvider`]
//! - **Persistent**: [`FilesystemVectorStore`]
//! - **Stubbing**: [`NullVectorStoreProvider`]
//!
//! ### Production
//! - **Scalable**: [`MilvusVectorStoreProvider`] with replication
//! - **Secure**: [`EncryptedVectorStoreProvider`] wrapper
//! - **Edge**: [`EdgeVecVectorStoreProvider`]
//!
//! ## Configuration
//!
//! ```toml
//! [vector_store]
//! provider = "filesystem"  # or "milvus", "in-memory", "edgevec"
//! address = "localhost:19530"  # For Milvus
//! collection = "codebase"
//! dimensions = 384
//! ```
//!
//! ## Example: Select Provider by Environment
//!
//! ```rust,no_run
//! use mcp_context_browser::adapters::providers::vector_store::{
//!     FilesystemVectorStore, FilesystemVectorStoreConfig, InMemoryVectorStoreProvider,
//!     NullVectorStoreProvider,
//! };
//! use mcp_context_browser::domain::ports::VectorStoreProvider;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Select provider based on environment
//! let provider: Box<dyn VectorStoreProvider> = match std::env::var("ENV").as_deref() {
//!     Ok("test") => {
//!         // In-memory for fast testing (data lost on restart)
//!         Box::new(InMemoryVectorStoreProvider::new())
//!     }
//!     Ok("dev") => {
//!         // File-based for local development
//!         let config = FilesystemVectorStoreConfig::default();
//!         Box::new(FilesystemVectorStore::new(config).await?)
//!     }
//!     _ => {
//!         // Production: use null provider as example
//!         Box::new(NullVectorStoreProvider::new())
//!     }
//! };
//!
//! provider.create_collection("codebase", 384).await?;
//! # Ok(())
//! # }
//! ```

pub mod edgevec;
pub mod encrypted;
pub mod filesystem;
pub mod in_memory;
#[cfg(feature = "milvus")]
pub mod milvus;

// Null provider for testing and DI default
pub mod null;

// Re-export for convenience
pub use edgevec::EdgeVecVectorStoreProvider;
pub use encrypted::EncryptedVectorStoreProvider;
pub use filesystem::{FilesystemVectorStore, FilesystemVectorStoreConfig};
pub use in_memory::InMemoryVectorStoreProvider;
#[cfg(feature = "milvus")]
pub use milvus::MilvusVectorStoreProvider;
pub use null::NullVectorStoreProvider;
