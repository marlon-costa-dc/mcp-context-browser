//! DI Container Bootstrap
//!
//! This module provides the bootstrap function for creating the complete
//! Shaku module hierarchy. Use this instead of manual component construction.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use mcp_context_browser::infrastructure::di::bootstrap::DiContainer;
//!
//! let container = DiContainer::build()?;
//!
//! // Resolve components from the container
//! let http_client: Arc<dyn HttpClientProvider> = container.resolve();
//! let admin_service: Arc<dyn AdminService> = container.resolve();
//! ```

use std::sync::Arc;

use crate::domain::error::Result;
use shaku::Interface;

use super::modules::{
    AdaptersModule, AdaptersModuleImpl, InfrastructureModule,
    InfrastructureModuleImpl, ServerModule, ServerModuleImpl,
};

/// DI Container holding the module hierarchy
///
/// This container builds and holds Shaku modules, providing a single
/// point for resolving registered components.
///
/// Note: AdminModule is currently excluded because AdminServiceImpl has
/// complex dependencies that require runtime configuration. AdminService
/// is manually constructed in builder.rs.
pub struct DiContainer {
    /// Adapters module (HTTP clients, providers)
    pub adapters_module: Arc<dyn AdaptersModule>,
    /// Infrastructure module (metrics, service provider)
    pub infrastructure_module: Arc<dyn InfrastructureModule>,
    /// Server module (performance, indexing operations)
    pub server_module: Arc<dyn ServerModule>,
}

impl DiContainer {
    /// Build the DI container with available modules
    ///
    /// Currently builds:
    /// - AdaptersModule (HTTP client)
    /// - InfrastructureModule (SystemMetricsCollector, ServiceProvider, EventBus, AuthService)
    /// - ServerModule (PerformanceMetrics, IndexingOperations)
    ///
    /// AdminModule is excluded due to complex runtime dependencies.
    pub fn build() -> Result<Self> {
        // Build leaf modules (those without dependencies)
        let adapters_module: Arc<dyn AdaptersModule> =
            Arc::new(AdaptersModuleImpl::builder().build());
        let infrastructure_module: Arc<dyn InfrastructureModule> =
            Arc::new(InfrastructureModuleImpl::builder().build());
        let server_module: Arc<dyn ServerModule> =
            Arc::new(ServerModuleImpl::builder().build());

        Ok(Self {
            adapters_module,
            infrastructure_module,
            server_module,
        })
    }

    /// Convenience method to resolve any component
    ///
    /// Uses trait bounds to automatically select the right module.
    pub fn resolve<T: Interface + ?Sized + 'static>(&self) -> Arc<T>
    where
        Self: ComponentResolver<T>,
    {
        <Self as ComponentResolver<T>>::resolve(self)
    }
}

/// Trait for resolving components from the right module
pub trait ComponentResolver<T: Interface + ?Sized> {
    fn resolve(&self) -> Arc<T>;
}

// Implement resolvers for each component type
impl ComponentResolver<dyn crate::adapters::http_client::HttpClientProvider> for DiContainer {
    fn resolve(&self) -> Arc<dyn crate::adapters::http_client::HttpClientProvider> {
        self.adapters_module.resolve()
    }
}

impl ComponentResolver<dyn crate::infrastructure::metrics::system::SystemMetricsCollectorInterface>
    for DiContainer
{
    fn resolve(&self) -> Arc<dyn crate::infrastructure::metrics::system::SystemMetricsCollectorInterface> {
        self.infrastructure_module.resolve()
    }
}

impl ComponentResolver<dyn crate::infrastructure::di::factory::ServiceProviderInterface>
    for DiContainer
{
    fn resolve(&self) -> Arc<dyn crate::infrastructure::di::factory::ServiceProviderInterface> {
        self.infrastructure_module.resolve()
    }
}

impl ComponentResolver<dyn crate::server::metrics::PerformanceMetricsInterface> for DiContainer {
    fn resolve(&self) -> Arc<dyn crate::server::metrics::PerformanceMetricsInterface> {
        self.server_module.resolve()
    }
}

impl ComponentResolver<dyn crate::server::operations::IndexingOperationsInterface> for DiContainer {
    fn resolve(&self) -> Arc<dyn crate::server::operations::IndexingOperationsInterface> {
        self.server_module.resolve()
    }
}

impl ComponentResolver<dyn crate::infrastructure::events::EventBusProvider> for DiContainer {
    fn resolve(&self) -> Arc<dyn crate::infrastructure::events::EventBusProvider> {
        self.infrastructure_module.resolve()
    }
}

impl ComponentResolver<dyn crate::infrastructure::auth::AuthServiceInterface> for DiContainer {
    fn resolve(&self) -> Arc<dyn crate::infrastructure::auth::AuthServiceInterface> {
        self.infrastructure_module.resolve()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::http_client::HttpClientProvider;
    use crate::infrastructure::di::factory::ServiceProviderInterface;
    use crate::infrastructure::events::EventBusProvider;
    use crate::infrastructure::metrics::system::SystemMetricsCollectorInterface;
    use crate::server::metrics::PerformanceMetricsInterface;
    use crate::server::operations::IndexingOperationsInterface;

    #[tokio::test]
    async fn test_di_container_resolves_http_client() {
        let container = DiContainer::build().expect("DiContainer should build");
        let http_client: Arc<dyn HttpClientProvider> = container.resolve();
        assert!(Arc::strong_count(&http_client) >= 1);
    }

    #[tokio::test]
    async fn test_di_container_resolves_performance_metrics() {
        let container = DiContainer::build().expect("DiContainer should build");
        let metrics: Arc<dyn PerformanceMetricsInterface> = container.resolve();
        assert!(Arc::strong_count(&metrics) >= 1);
    }

    #[tokio::test]
    async fn test_di_container_resolves_indexing_operations() {
        let container = DiContainer::build().expect("DiContainer should build");
        let ops: Arc<dyn IndexingOperationsInterface> = container.resolve();
        assert!(Arc::strong_count(&ops) >= 1);
    }

    #[tokio::test]
    async fn test_di_container_resolves_service_provider() {
        let container = DiContainer::build().expect("DiContainer should build");
        let provider: Arc<dyn ServiceProviderInterface> = container.resolve();
        assert!(Arc::strong_count(&provider) >= 1);
    }

    #[tokio::test]
    async fn test_di_container_resolves_system_collector() {
        let container = DiContainer::build().expect("DiContainer should build");
        let collector: Arc<dyn SystemMetricsCollectorInterface> = container.resolve();
        assert!(Arc::strong_count(&collector) >= 1);
    }

    #[tokio::test]
    async fn test_di_container_resolves_event_bus() {
        let container = DiContainer::build().expect("DiContainer should build");
        let event_bus: Arc<dyn EventBusProvider> = container.resolve();
        assert!(Arc::strong_count(&event_bus) >= 1);
    }

    #[tokio::test]
    async fn test_di_container_resolves_auth_service() {
        use crate::infrastructure::auth::AuthServiceInterface;
        let container = DiContainer::build().expect("DiContainer should build");
        let auth_service: Arc<dyn AuthServiceInterface> = container.resolve();
        assert!(Arc::strong_count(&auth_service) >= 1);
    }

    // NOTE: AdminService resolution requires complex runtime dependencies
    // which is not yet complete. This will be enabled in Phase 3.
    // #[tokio::test]
    // async fn test_di_container_resolves_admin_service() {
    //     let container = DiContainer::build().expect("DiContainer should build");
    //     let admin_service: Arc<dyn AdminService> = container.resolve();
    //     assert!(Arc::strong_count(&admin_service) >= 1);
    // }
}
