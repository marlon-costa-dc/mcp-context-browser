//! Embedding provider implementations

pub mod fastembed;
pub mod gemini;
pub mod helpers;
pub mod ollama;
pub mod openai;
pub mod voyageai;

// Note: null module is public for test access but NOT re-exported at parent level
// (Phase 5 DI audit - production code should not use null providers)
// Tests import via: mcp_context_browser::adapters::providers::embedding::null::NullEmbeddingProvider
pub mod null;

// Re-export for convenience (production providers only)
pub use fastembed::FastEmbedProvider;
pub use gemini::GeminiEmbeddingProvider;
pub use helpers::{constructor, EmbeddingProviderHelper};
pub use ollama::OllamaEmbeddingProvider;
pub use openai::OpenAIEmbeddingProvider;
pub use voyageai::VoyageAIEmbeddingProvider;

// NullEmbeddingProvider NOT re-exported - tests import from null submodule directly
