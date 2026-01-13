//! Vector store provider implementations

pub mod edgevec;
pub mod encrypted;
pub mod filesystem;
pub mod in_memory;
#[cfg(feature = "milvus")]
pub mod milvus;

// Note: null module is public for test access but NOT re-exported at parent level
// (Phase 5 DI audit - production code should not use null providers)
// Tests import via: mcp_context_browser::adapters::providers::vector_store::null::NullVectorStoreProvider
pub mod null;

// Re-export for convenience (production providers only)
pub use edgevec::EdgeVecVectorStoreProvider;
pub use encrypted::EncryptedVectorStoreProvider;
pub use filesystem::FilesystemVectorStore;
pub use in_memory::InMemoryVectorStoreProvider;
#[cfg(feature = "milvus")]
pub use milvus::MilvusVectorStoreProvider;

// NullVectorStoreProvider NOT re-exported - tests import from null submodule directly
