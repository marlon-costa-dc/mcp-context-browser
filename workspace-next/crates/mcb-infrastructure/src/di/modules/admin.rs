//! Admin Module Implementation - Administrative Services
//!
//! This module provides administrative services that depend on all other modules.
//! It follows the Shaku strict pattern as the composition root for admin functionality.
//!
//! ## Services Provided
//!
//! - Admin service providing configuration, monitoring, and control interfaces

use shaku::module;

// Import concrete implementation from mcb-providers
use mcb_providers::admin::AdminServiceImpl;

// Import all module traits
use super::traits::{AdaptersModule, AdminModule, ApplicationModule, InfrastructureModule, ServerModule};

/// Admin module implementation following Shaku strict pattern.
///
/// This module provides administrative services that depend on all other modules.
/// Uses `use dyn ModuleTrait` to import services from all domain modules.
///
/// ## Dependencies
///
/// Depends on all other modules for comprehensive admin functionality:
/// - `InfrastructureModule`: Core services (cache, crypto, health)
/// - `ServerModule`: Server components (metrics, indexing operations)
/// - `AdaptersModule`: External integrations (providers, repositories)
/// - `ApplicationModule`: Business logic services
///
/// ## Construction
///
/// ```rust,ignore
/// let infrastructure = Arc::new(InfrastructureModuleImpl::builder().build());
/// let server = Arc::new(ServerModuleImpl::builder().build());
/// let adapters = Arc::new(AdaptersModuleImpl::builder().build());
/// let application = Arc::new(ApplicationModuleImpl::builder(adapters.clone()).build());
///
/// let admin = AdminModuleImpl::builder(
///     infrastructure,
///     server,
///     adapters,
///     application
/// ).build();
/// ```
module! {
    pub AdminModuleImpl: AdminModule {
        components = [
            // Administrative services
            AdminServiceImpl
        ],
        providers = [],

        // Dependencies from all other modules
        use dyn InfrastructureModule {
            components = [
                dyn crate::cache::provider::CacheProvider,
                dyn crate::crypto::CryptoService,
                dyn crate::health::HealthRegistry,
                dyn crate::infrastructure::auth::AuthServiceInterface,
                dyn crate::infrastructure::events::EventBusProvider,
                dyn crate::infrastructure::metrics::system::SystemMetricsCollectorInterface
            ],
            providers = []
        },

        use dyn ServerModule {
            components = [
                dyn mcb_domain::ports::admin::PerformanceMetricsInterface,
                dyn mcb_domain::ports::admin::IndexingOperationsInterface
            ],
            providers = []
        },

        use dyn AdaptersModule {
            components = [
                dyn crate::adapters::http_client::HttpClientProvider,
                dyn mcb_domain::ports::providers::EmbeddingProvider,
                dyn mcb_domain::ports::providers::VectorStoreProvider
            ],
            providers = []
        },

        use dyn ApplicationModule {
            components = [
                dyn mcb_domain::domain_services::search::ContextServiceInterface,
                dyn mcb_domain::domain_services::search::SearchServiceInterface,
                dyn mcb_domain::domain_services::search::IndexingServiceInterface
            ],
            providers = []
        }
    }
}