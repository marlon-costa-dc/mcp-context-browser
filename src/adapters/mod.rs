//! # Adapters Layer
//!
//! External service implementations and integrations.
//!
//! This layer implements the port traits defined in [`domain::ports`] and provides
//! concrete adapters for external services.
//!
//! ## Providers
//!
//! ### Embedding Providers
//!
//! Convert text to vector embeddings:
//!
//! - [`providers::embedding::OpenAIProvider`] - OpenAI text-embedding-3 models
//! - [`providers::embedding::OllamaProvider`] - Local Ollama models
//! - [`providers::embedding::VoyageAIProvider`] - VoyageAI embeddings
//! - [`providers::embedding::FastEmbedProvider`] - Local FastEmbed models
//! - [`providers::embedding::GeminiProvider`] - Google Gemini embeddings
//!
//! ### Vector Store Providers
//!
//! Store and search vector embeddings:
//!
//! - [`providers::vector_store::MilvusProvider`] - Milvus vector database
//! - [`providers::vector_store::EdgeVecProvider`] - In-memory EdgeVec store
//! - [`providers::vector_store::FilesystemProvider`] - Local filesystem storage
//!
//! ### Intelligent Routing
//!
//! - [`providers::routing::IntelligentRouter`] - Load-balanced provider selection
//! - [`providers::routing::CircuitBreaker`] - Fault tolerance with circuit breakers
//!
//! ## Hybrid Search
//!
//! - [`HybridSearchAdapter`] - Combines BM25 lexical + vector semantic search
//!
//! ## Example
//!
//! Providers are typically created through the dependency injection container,
//! but can be configured via the configuration system:
//!
//! ```toml
//! # config.toml
//! [providers.embedding]
//! provider = "openai"
//! model = "text-embedding-3-small"
//! api_key = "${OPENAI_API_KEY}"
//! ```
//!
//! [`domain::ports`]: crate::domain::ports
//! [`providers::embedding::OpenAIProvider`]: providers::embedding::OpenAIProvider
//! [`providers::vector_store::MilvusProvider`]: providers::vector_store::MilvusProvider
//! [`HybridSearchAdapter`]: hybrid_search::HybridSearchAdapter

pub mod database;
pub mod http_client;
pub mod hybrid_search;
pub mod providers;
pub mod repository;

pub use hybrid_search::{HybridSearchActor, HybridSearchAdapter};
