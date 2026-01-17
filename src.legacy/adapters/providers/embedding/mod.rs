//! Embedding provider implementations
//!
//! Converts text into dense vector embeddings for semantic search.
//! Each provider offers different tradeoffs between quality, cost, and privacy.
//!
//! ## Available Providers
//!
//! | Provider | Model | Dimensions | Deployment | Use Case |
//! |----------|-------|-----------|------------|----------|
//! | [`OpenAIEmbeddingProvider`] | text-embedding-3 | 384/1536 | Cloud | High quality, enterprise |
//! | [`OllamaEmbeddingProvider`] | Local models | Various | On-premise | Privacy, offline |
//! | [`VoyageAIEmbeddingProvider`] | voyage-2 | 1536 | Cloud | Code-optimized |
//! | [`GeminiEmbeddingProvider`] | embedding-001 | 768 | Cloud | Google ecosystem |
//! | [`FastEmbedProvider`] | BAAI/bge | 384 | Local | Fast, default, free |
//! | [`NullEmbeddingProvider`] | N/A | 0 | N/A | Testing, stubs |
//!
//! ## Provider Selection Guide
//!
//! ### Production Use
//! - **High Quality**: OpenAI text-embedding-3-small (cost: $0.02/1M tokens)
//! - **Code-Optimized**: VoyageAI voyage-code-2 (specialized for programming)
//! - **Privacy**: Ollama locally hosted (requires GPU for speed)
//!
//! ### Development/Testing
//! - **Default**: FastEmbed (no API keys needed, works offline)
//! - **Stubbing**: NullEmbeddingProvider (unit tests)
//!
//! ## Configuration
//!
//! ```toml
//! [embedding]
//! provider = "fastembed"  # or "openai", "ollama", "voyageai", "gemini"
//! model = "BAAI/bge-small-en-v1.5"
//! api_key = "${OPENAI_API_KEY}"  # for cloud providers
//! dimensions = 384
//! ```
//!
//! ## Example: Using Different Providers
//!
//! ```rust,no_run
//! use mcp_context_browser::adapters::providers::embedding::*;
//! use mcp_context_browser::domain::ports::EmbeddingProvider;
//!
//! # async fn example() -> anyhow::Result<()> {
//! // Local, no dependencies (loads ONNX model on first use)
//! let local = FastEmbedProvider::new()?;
//! let embeddings = local.embed_batch(&["authentication logic".to_string()]).await?;
//! assert_eq!(embeddings[0].dimensions, 384);
//!
//! // For testing/mocking
//! let stub = NullEmbeddingProvider::new();
//! assert_eq!(stub.dimensions(), 0);
//! assert_eq!(stub.provider_name(), "null");
//! # Ok(())
//! # }
//! ```
//!
//! ## Implementing Custom Providers
//!
//! Implement the [`crate::domain::ports::EmbeddingProvider`] trait:
//!
//! ```rust,no_run
//! use mcp_context_browser::domain::error::Result;
//! use mcp_context_browser::domain::ports::EmbeddingProvider;
//! use mcp_context_browser::domain::types::Embedding;
//! use async_trait::async_trait;
//!
//! pub struct CustomProvider;
//!
//! #[async_trait]
//! impl EmbeddingProvider for CustomProvider {
//!     async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>> {
//!         // Implement your embedding logic
//!         todo!()
//!     }
//!
//!     fn dimensions(&self) -> usize { 384 }
//!     fn provider_name(&self) -> &str { "custom" }
//! }
//! ```

pub mod fastembed;
pub mod gemini;
pub mod helpers;
pub mod ollama;
pub mod openai;
pub mod voyageai;

// Null provider for testing and DI default
pub mod null;

// Re-export for convenience
pub use fastembed::FastEmbedProvider;
pub use gemini::GeminiEmbeddingProvider;
pub use helpers::{constructor, EmbeddingProviderHelper};
pub use null::NullEmbeddingProvider;
pub use ollama::OllamaEmbeddingProvider;
pub use openai::OpenAIEmbeddingProvider;
pub use voyageai::VoyageAIEmbeddingProvider;
