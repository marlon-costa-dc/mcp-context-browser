//! Application DI Module Implementation
//!
//! Contains business logic services (ContextService, SearchService, IndexingService).
//!
//! ## Service Hierarchy
//!
//! Services depend on repositories from AdaptersModule:
//! - ContextService injects ChunkRepository, SearchRepository, EmbeddingProvider
//! - SearchService injects ContextServiceInterface
//! - IndexingService injects ContextServiceInterface, ChunkingOrchestratorInterface
//!
//! ## Module Dependencies
//!
//! ApplicationModule uses AdaptersModule as a submodule to provide:
//! - ChunkRepository (for ContextService)
//! - SearchRepository (for ContextService)
//! - EmbeddingProvider (for ContextService)

use shaku::module;

use super::traits::{AdaptersModule, ApplicationModule};
use crate::application::context::ContextService;
use crate::application::indexing::{ChunkingOrchestrator, IndexingService};
use crate::application::search::SearchService;
use crate::domain::ports::{ChunkRepository, EmbeddingProvider, SearchRepository};

/// Implementation of the ApplicationModule trait providing core business services.
///
/// This module contains the main business logic services:
/// - ContextService for semantic context management
/// - SearchService for code search operations
/// - IndexingService for code indexing and processing
/// - ChunkingOrchestrator for coordinating code chunking operations
///
/// Depends on AdaptersModule for external service integrations.
module! {
    pub ApplicationModuleImpl: ApplicationModule {
        components = [ContextService, SearchService, IndexingService, ChunkingOrchestrator],
        providers = [],

        use dyn AdaptersModule {
            components = [dyn ChunkRepository, dyn SearchRepository, dyn EmbeddingProvider],
            providers = []
        }
    }
}
