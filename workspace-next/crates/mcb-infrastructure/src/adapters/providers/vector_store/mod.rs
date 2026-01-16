//! Vector store provider implementations
//!
//! Persistent storage backends for semantic embeddings.
//! Different backends provide tradeoffs between performance, scalability, and features.
//!
//! ## Available Backends
//!
//! | Backend | Persistence | Scale | Latency | Features | Use Case |
//! |---------|-------------|-------|---------|----------|----------|
//! | [`InMemoryVectorStoreProvider`] | Memory-only | Single-node | Low | Fast | Testing |
//! | [`EncryptedVectorStoreProvider`] | File/DB | Variable | Variable | Encryption | Secure storage |
//! | [`NullVectorStoreProvider`] | None | N/A | N/A | Stub | Unit tests |
//!
//! ## Architecture Patterns
//!
//! ### Single-Node (Development)
//! ```text
//! Application → InMemoryVectorStore
//!            ↓ (Restart loses data)
//! ```
//!
//! ### Secure (Encrypted at Rest)
//! ```text
//! Application → EncryptedVectorStore
//!            ↓ (AES-256-GCM encryption)
//!            → InMemoryVectorStore or other backend
//! ```

pub mod encrypted;
pub mod in_memory;
pub mod null;

// Re-export for convenience
pub use encrypted::EncryptedVectorStoreProvider;
pub use in_memory::InMemoryVectorStoreProvider;
pub use null::NullVectorStoreProvider;
