//! Submodule Trait Interfaces
//!
//! These traits define the interfaces for domain-specific DI modules.
//! Concrete implementations can be swapped out for testing or different environments.
//!
//! Note: These traits only extend `HasComponent<T>`, not `Module`.
//! The `Module` trait is automatically implemented by the `module!` macro.

use shaku::HasComponent;

use crate::adapters::http_client::HttpClientProvider;
use crate::domain::ports::{
    ChunkRepository, ChunkingOrchestratorInterface, ContextServiceInterface, EmbeddingProvider,
    IndexingOperationsInterface, IndexingServiceInterface, PerformanceMetricsInterface,
    SearchRepository, SearchServiceInterface, VectorStoreProvider,
};
use crate::infrastructure::auth::AuthServiceInterface;
use crate::infrastructure::di::factory::ServiceProviderInterface;
use crate::infrastructure::events::EventBusProvider;
use crate::infrastructure::metrics::system::SystemMetricsCollectorInterface;
use crate::server::admin::service::AdminService;

/// Adapters module trait - external adapters like HTTP clients, providers, and repositories
pub trait AdaptersModule:
    HasComponent<dyn HttpClientProvider>
    + HasComponent<dyn EmbeddingProvider>
    + HasComponent<dyn VectorStoreProvider>
    + HasComponent<dyn ChunkRepository>
    + HasComponent<dyn SearchRepository>
{
}

/// Infrastructure module trait - core infrastructure services
pub trait InfrastructureModule:
    HasComponent<dyn SystemMetricsCollectorInterface>
    + HasComponent<dyn ServiceProviderInterface>
    + HasComponent<dyn EventBusProvider>
    + HasComponent<dyn AuthServiceInterface>
{
}

/// Server module trait - MCP server components
pub trait ServerModule:
    HasComponent<dyn PerformanceMetricsInterface> + HasComponent<dyn IndexingOperationsInterface>
{
}

/// Admin module trait - admin service with dependencies
pub trait AdminModule: HasComponent<dyn AdminService> {}

/// Application module trait - business logic services
pub trait ApplicationModule:
    HasComponent<dyn ContextServiceInterface>
    + HasComponent<dyn SearchServiceInterface>
    + HasComponent<dyn IndexingServiceInterface>
    + HasComponent<dyn ChunkingOrchestratorInterface>
{
}
