//! Infrastructure Module Implementation - Core Services
//!
//! This module provides core infrastructure services that other modules depend on.
//! It follows the Shaku strict pattern with no external dependencies.
//!
//! ## Services Provided
//!
//! - Cache provider for data caching
//! - Crypto service for encryption/decryption
//! - Health registry for system health monitoring
//! - Auth service for authentication
//! - Event bus for internal messaging
//! - System metrics collector for performance monitoring
//! - Snapshot provider for state persistence
//! - Sync provider for data synchronization

use shaku::module;

// Import concrete implementations
use crate::cache::provider::CacheProviderImpl;
use crate::crypto::CryptoService;
use crate::health::HealthRegistry;
use crate::infrastructure::auth::AuthServiceImpl;
use crate::infrastructure::events::EventBusImpl;
use crate::infrastructure::metrics::system::SystemMetricsCollectorImpl;
use crate::infrastructure::snapshot::SnapshotProviderImpl;
use crate::infrastructure::sync::SyncProviderImpl;

// Import traits
use super::traits::InfrastructureModule;

/// Infrastructure module implementation following Shaku strict pattern.
///
/// This module provides core infrastructure services with no external dependencies.
/// All services are concrete implementations that can be resolved at runtime.
///
/// ## Component Registration
///
/// Uses `#[derive(Component)]` and `#[shaku(interface = ...)]` for all services.
/// Services with dependencies use `#[shaku(inject)]` for dependency injection.
///
/// ## Construction
///
/// ```rust,ignore
/// let infrastructure = InfrastructureModuleImpl::builder().build();
/// ```
module! {
    pub InfrastructureModuleImpl: InfrastructureModule {
        components = [
            // Core infrastructure services
            CacheProviderImpl,
            CryptoService,
            HealthRegistry,
            AuthServiceImpl,
            EventBusImpl,
            SystemMetricsCollectorImpl,
            SnapshotProviderImpl,
            SyncProviderImpl
        ],
        providers = []
    }
}