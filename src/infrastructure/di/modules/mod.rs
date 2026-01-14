//! DI Module Organization - Hierarchical by Domain
//!
//! This module organizes DI components by domain following Clean Architecture:
//!
//! - `AdaptersModule` - External adapters (HTTP clients, providers)
//! - `InfrastructureModule` - Core infrastructure (metrics, service provider)
//! - `ServerModule` - Server components (performance, indexing)
//! - `AdminModule` - Admin service (depends on infrastructure and server)
//!
//! The root `McpModule` composes all domain modules for application use.
//!
//! ## Building the Module
//!
//! ```rust,ignore
//! use std::sync::Arc;
//! use mcp_context_browser::infrastructure::di::modules::*;
//!
//! // Build submodules first (those without dependencies)
//! let adapters = Arc::new(AdaptersModuleImpl::builder().build());
//! let infrastructure = Arc::new(InfrastructureModuleImpl::builder().build());
//! let server = Arc::new(ServerModuleImpl::builder().build());
//!
//! // Build admin module with its dependencies
//! let admin = Arc::new(
//!     AdminModuleImpl::builder(infrastructure.clone(), server.clone(), adapters.clone())
//!         .build()
//! );
//!
//! // Build root module with all submodules
//! let module = McpModule::builder(adapters, infrastructure, server, admin).build();
//! ```

mod adapters;
mod admin;
mod application;
mod infrastructure;
mod server;
pub mod traits;

pub use adapters::AdaptersModuleImpl;
pub use admin::AdminModuleImpl;
pub use application::ApplicationModuleImpl;
pub use infrastructure::InfrastructureModuleImpl;
pub use server::ServerModuleImpl;
pub use traits::{
    AdaptersModule, AdminModule, ApplicationModule, InfrastructureModule, ServerModule,
};

// Future module trait exports (v0.3.0+)
#[cfg(feature = "analysis")]
pub use traits::AnalysisModule;
#[cfg(feature = "git")]
pub use traits::GitModule;
#[cfg(feature = "quality")]
pub use traits::QualityModule;

use shaku::module;

use crate::adapters::http_client::HttpClientProvider;
use crate::domain::ports::{
    ChunkRepository, ContextServiceInterface, EmbeddingProvider, IndexingOperationsInterface,
    IndexingServiceInterface, PerformanceMetricsInterface, SearchRepository,
    SearchServiceInterface, VectorStoreProvider,
};
use crate::infrastructure::auth::AuthServiceInterface;
use crate::infrastructure::di::factory::ServiceProviderInterface;
use crate::infrastructure::events::EventBusProvider;
use crate::infrastructure::metrics::system::SystemMetricsCollectorInterface;
use crate::server::admin::service::AdminService;

// Root module that composes all domain modules.
//
// Use this module for production initialization. Submodules can be used
// independently for testing or partial initialization.
///
/// This module provides the complete dependency injection container
/// for the MCP Context Browser application, composing adapters,
/// infrastructure, server, and admin modules.
///
/// # Usage
/// ```rust
/// let adapters = Arc::new(AdaptersModuleImpl::builder().build());
/// let infrastructure = Arc::new(InfrastructureModuleImpl::builder().build());
/// let server = Arc::new(ServerModuleImpl::builder().build());
/// let admin = Arc::new(AdminModuleImpl::builder().build());
/// let module = McpModule::builder(adapters, infrastructure, server, admin).build();
/// ```
module! {
    pub McpModule {
        components = [],
        providers = [],

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

        use dyn InfrastructureModule {
            components = [dyn SystemMetricsCollectorInterface, dyn ServiceProviderInterface, dyn EventBusProvider, dyn AuthServiceInterface],
            providers = []
        },

        use dyn ServerModule {
            components = [dyn PerformanceMetricsInterface, dyn IndexingOperationsInterface],
            providers = []
        },

        use dyn AdminModule {
            components = [dyn AdminService],
            providers = []
        },

        use dyn ApplicationModule {
            components = [dyn ContextServiceInterface, dyn SearchServiceInterface, dyn IndexingServiceInterface],
            providers = []
        }
    }
}
