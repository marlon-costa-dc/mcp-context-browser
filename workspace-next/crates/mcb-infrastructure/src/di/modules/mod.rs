//! DI Module Organization - Hierarchical by Domain (Shaku Strict Pattern)

#![allow(missing_docs)]
//!
//! This module implements a strict Shaku-based hierarchical module system
//! following Clean Architecture and Domain-Driven Design principles.
//!
//! ## Shaku Module Hierarchy Pattern
//!
//! ```text
//! ApplicationModule (Root - depends on all)
//! ├── InfrastructureModule (core services - no dependencies)
//! ├── ServerModule (MCP server components - no dependencies)
//! ├── AdaptersModule (external integrations - no dependencies)
//! └── AdminModule (admin services - depends on all above)
//! ```
//!
//! ## Module Construction Pattern
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use mcb_infrastructure::di::modules::*;
//!
//! // 1. Build leaf modules (no dependencies)
//! let infrastructure = Arc::new(InfrastructureModuleImpl::builder().build());
//! let server = Arc::new(ServerModuleImpl::builder().build());
//! let adapters = Arc::new(AdaptersModuleImpl::builder().build());
//!
//! // 2. Build application module (depends on adapters)
//! let application = Arc::new(ApplicationModuleImpl::builder(adapters.clone()).build());
//!
//! // 3. Build admin module (depends on all)
//! let admin = Arc::new(AdminModuleImpl::builder(
//!     infrastructure.clone(),
//!     server.clone(),
//!     adapters.clone(),
//!     application.clone()
//! ).build());
//!
//! // 4. Build root module (depends on all)
//! let root = McpModule::builder(infrastructure, server, adapters, application, admin).build();
//! ```
//!
//! ## Shaku Best Practices
//!
//! 1. **Trait-based Interfaces**: All module interactions via traits
//! 2. **Submodule Composition**: `use dyn ModuleTrait` for dependencies
//! 3. **Component Registration**: Concrete types only in `module!` macros
//! 4. **Provider Defaults**: Null providers as fallbacks for testing
//! 5. **Runtime Overrides**: Component overrides for production configuration

/// Domain module traits (interfaces)
pub mod traits;

/// Infrastructure module implementation (core infrastructure)
mod infrastructure;
/// Server module implementation (MCP server components)
mod server;
/// Adapters module implementation (external integrations)
mod adapters;
/// Application module implementation (business logic)
mod application;
/// Admin module implementation (admin services)
mod admin;

pub use adapters::AdaptersModuleImpl;
pub use admin::AdminModuleImpl;
pub use application::ApplicationModuleImpl;
pub use infrastructure::InfrastructureModuleImpl;
pub use server::ServerModuleImpl;
pub use traits::{
    AdaptersModule, AdminModule, ApplicationModule, InfrastructureModule, ServerModule,
};

// Re-export Shaku for convenience
pub use shaku::module;

// ============================================================================
// Root Module Definition (Shaku Strict Pattern)
// ============================================================================

use shaku::{HasComponent, HasProvider, Interface};
use std::sync::Arc;

// Import all required traits and interfaces
use crate::adapters::http_client::HttpClientProvider;
use crate::adapters::repository::{ChunkRepository, SearchRepository};
use crate::application::admin::AdminService;
use crate::cache::provider::CacheProvider;
use crate::crypto::CryptoService;
use crate::health::HealthRegistry;
use crate::infrastructure::auth::AuthServiceInterface;
use crate::infrastructure::events::EventBusProvider;
use crate::infrastructure::metrics::system::SystemMetricsCollectorInterface;
use crate::infrastructure::snapshot::SnapshotProvider;
use crate::infrastructure::sync::SyncProvider;

use mcb_domain::domain_services::search::{
    ContextServiceInterface, IndexingServiceInterface, SearchServiceInterface,
};
use mcb_domain::ports::admin::{IndexingOperationsInterface, PerformanceMetricsInterface};
use mcb_domain::ports::providers::{EmbeddingProvider, VectorStoreProvider};
use mcb_domain::ports::{ChunkingOrchestratorInterface, CodeChunker};

// ============================================================================
// Root Module Definition
// ============================================================================

/// Root dependency injection module following Shaku hierarchical pattern.
///
/// This module composes all domain modules into a single container.
/// It uses `use dyn ModuleTrait` to import services from submodules,
/// following Shaku's strict submodule composition pattern.
///
/// ## Construction
///
/// ```rust,ignore
/// let infrastructure = Arc::new(InfrastructureModuleImpl::builder().build());
/// let server = Arc::new(ServerModuleImpl::builder().build());
/// let adapters = Arc::new(AdaptersModuleImpl::builder().build());
/// let application = Arc::new(ApplicationModuleImpl::builder(adapters.clone()).build());
/// let admin = Arc::new(AdminModuleImpl::builder(infrastructure.clone(), server.clone(), adapters.clone(), application.clone()).build());
///
/// let root = McpModule::builder(infrastructure, server, adapters, application, admin).build();
/// ```
module! {
    pub McpModule {
        components = [],
        providers = [],

        // Infrastructure services (core, no dependencies)
        use dyn InfrastructureModule {
            components = [
                dyn CacheProvider,
                dyn CryptoService,
                dyn HealthRegistry,
                dyn AuthServiceInterface,
                dyn EventBusProvider,
                dyn SystemMetricsCollectorInterface,
                dyn SnapshotProvider,
                dyn SyncProvider
            ],
            providers = []
        },

        // Server components (MCP server, no dependencies)
        use dyn ServerModule {
            components = [dyn PerformanceMetricsInterface, dyn IndexingOperationsInterface],
            providers = []
        },

        // External adapters (providers, repositories, no dependencies)
        use dyn AdaptersModule {
            components = [
                dyn HttpClientProvider,
                dyn EmbeddingProvider,
                dyn VectorStoreProvider,
                dyn ChunkRepository,
                dyn SearchRepository
            ],
            providers = []
        },

        // Business logic services (depends on adapters)
        use dyn ApplicationModule {
            components = [
                dyn ContextServiceInterface,
                dyn SearchServiceInterface,
                dyn IndexingServiceInterface,
                dyn ChunkingOrchestratorInterface,
                dyn CodeChunker
            ],
            providers = []
        },

        // Admin services (depends on all modules above)
        use dyn AdminModule {
            components = [dyn AdminService],
            providers = []
        }
    }
}
