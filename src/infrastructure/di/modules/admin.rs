//! Admin Service DI Module Implementation
//!
//! Contains admin service with dependencies on infrastructure and server modules.

use shaku::module;

use super::traits::{AdaptersModule, AdminModule, InfrastructureModule, ServerModule};
use crate::adapters::http_client::HttpClientProvider;
use crate::domain::ports::admin::{IndexingOperationsInterface, PerformanceMetricsInterface};
use crate::infrastructure::di::factory::ServiceProviderInterface;
use crate::infrastructure::events::EventBusProvider;
use crate::infrastructure::metrics::system::SystemMetricsCollectorInterface;
use crate::server::admin::service::AdminServiceImpl;

module! {
    pub AdminModuleImpl: AdminModule {
        components = [AdminServiceImpl],
        providers = [],

        use dyn InfrastructureModule {
            components = [dyn SystemMetricsCollectorInterface, dyn ServiceProviderInterface, dyn EventBusProvider],
            providers = []
        },

        use dyn ServerModule {
            components = [dyn PerformanceMetricsInterface, dyn IndexingOperationsInterface],
            providers = []
        },

        use dyn AdaptersModule {
            components = [dyn HttpClientProvider],
            providers = []
        }
    }
}
