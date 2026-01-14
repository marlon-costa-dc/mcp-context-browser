//! Infrastructure DI Module Implementation
//!
//! Contains system metrics, service providers, event bus, auth, and core infrastructure.

use shaku::module;

use super::traits::InfrastructureModule;
use crate::infrastructure::auth::AuthService;
use crate::infrastructure::di::factory::ServiceProvider;
use crate::infrastructure::events::EventBus;
use crate::infrastructure::metrics::system::SystemMetricsCollector;

/// Implementation of the InfrastructureModule trait providing cross-cutting concerns.
///
/// This module contains infrastructure services that support the entire application:
/// - SystemMetricsCollector for system resource monitoring
/// - ServiceProvider factory for creating service instances
/// - EventBus for system-wide event communication
/// - AuthService for authentication and authorization
module! {
    pub InfrastructureModuleImpl: InfrastructureModule {
        components = [SystemMetricsCollector, ServiceProvider, EventBus, AuthService],
        providers = []
    }
}
