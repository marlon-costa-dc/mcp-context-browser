//! Application Module Implementation - Business Logic Services
//!
//! This module provides business logic services following Clean Architecture.
//! It depends on adapters for data access but has no infrastructure dependencies.
//!
//! ## Services Provided
//!
//! - Context service for codebase context management
//! - Search service for semantic search operations
//! - Indexing service for background indexing
//! - Chunking orchestrator for code chunking coordination
//! - Code chunker for AST-based code parsing

use shaku::module;

// Import concrete implementations from domain services
use mcb_domain::domain_services::search::{
    ContextServiceImpl, IndexingServiceImpl, SearchServiceImpl,
};

// Import chunking services
use mcb_domain::chunking::{ChunkingOrchestratorImpl, LanguageChunker};

// Import traits
use super::traits::{AdaptersModule, ApplicationModule};

/// Application module implementation following Shaku strict pattern.
///
/// This module provides business logic services that depend on adapters.
/// Uses `use dyn AdaptersModule` to import required external services.
///
/// ## Dependencies
///
/// Depends on `AdaptersModule` for:
/// - Embedding and vector store providers
/// - Repository services for data persistence
///
/// ## Construction
///
/// ```rust,ignore
/// let adapters = Arc::new(AdaptersModuleImpl::builder().build());
/// let application = ApplicationModuleImpl::builder(adapters).build();
/// ```
module! {
    pub ApplicationModuleImpl: ApplicationModule {
        components = [
            // Business logic services
            ContextServiceImpl,
            SearchServiceImpl,
            IndexingServiceImpl,
            ChunkingOrchestratorImpl,
            LanguageChunker
        ],
        providers = [],

        // Dependencies from adapters module
        use dyn AdaptersModule {
            components = [
                dyn mcb_domain::ports::providers::EmbeddingProvider,
                dyn mcb_domain::ports::providers::VectorStoreProvider,
                dyn crate::adapters::repository::ChunkRepository,
                dyn crate::adapters::repository::SearchRepository
            ],
            providers = []
        }
    }
}