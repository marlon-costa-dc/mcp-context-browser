//! DI Container Bootstrap - Clean Architecture Composition Root
//!
//! Provides the composition root for the dependency injection system following
//! Clean Architecture principles.
//!
//! ## Architecture
//!
//! External providers (embedding, vector_store, cache, language) are resolved
//! dynamically via the registry system. Internal infrastructure services use
//! simple constructor-based injection.
//!
//! ## Usage
//!
//! ```rust,ignore
//! // Create AppContext with resolved providers
//! let context = init_app(AppConfig::default()).await?;
//!
//! // Use providers
//! let embedding = context.providers.embedding.embed("hello").await?;
//! ```

use crate::config::AppConfig;
use crate::di::resolver::{resolve_providers, ResolvedProviders};
use crate::infrastructure::{
    admin::{NullIndexingOperations, NullPerformanceMetrics},
    auth::NullAuthService,
    events::TokioBroadcastEventBus,
    lifecycle::DefaultShutdownCoordinator,
    metrics::NullSystemMetricsCollector,
    snapshot::NullSnapshotProvider,
    sync::NullSyncProvider,
};
use mcb_application::ports::admin::{
    IndexingOperationsInterface, PerformanceMetricsInterface, ShutdownCoordinator,
};
use mcb_application::ports::infrastructure::{
    AuthServiceInterface, EventBusProvider, SnapshotProvider, SyncProvider,
    SystemMetricsCollectorInterface,
};
use mcb_domain::error::Result;
use std::sync::Arc;
use tracing::info;

/// Infrastructure services container
///
/// Contains all internal infrastructure services that don't come from
/// the external provider registry.
pub struct InfrastructureServices {
    /// Authentication service
    pub auth: Arc<dyn AuthServiceInterface>,
    /// Event bus for domain events
    pub event_bus: Arc<dyn EventBusProvider>,
    /// System metrics collector
    pub metrics: Arc<dyn SystemMetricsCollectorInterface>,
    /// Sync provider for file coordination
    pub sync: Arc<dyn SyncProvider>,
    /// Snapshot provider for change detection
    pub snapshot: Arc<dyn SnapshotProvider>,
    /// Shutdown coordinator
    pub shutdown: Arc<dyn ShutdownCoordinator>,
}

impl Default for InfrastructureServices {
    fn default() -> Self {
        Self {
            auth: Arc::new(NullAuthService::new()),
            event_bus: Arc::new(TokioBroadcastEventBus::new()),
            metrics: Arc::new(NullSystemMetricsCollector::new()),
            sync: Arc::new(NullSyncProvider::new()),
            snapshot: Arc::new(NullSnapshotProvider::new()),
            shutdown: Arc::new(DefaultShutdownCoordinator::new()),
        }
    }
}

impl std::fmt::Debug for InfrastructureServices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("InfrastructureServices")
            .field("auth", &"NullAuthService")
            .field("event_bus", &"TokioBroadcastEventBus")
            .field("metrics", &"NullSystemMetricsCollector")
            .field("sync", &"NullSyncProvider")
            .field("snapshot", &"NullSnapshotProvider")
            .finish()
    }
}

/// Admin services container
///
/// Contains admin-related services for monitoring and operations.
pub struct AdminServices {
    /// Performance metrics tracking
    pub performance: Arc<dyn PerformanceMetricsInterface>,
    /// Indexing operations tracking
    pub indexing: Arc<dyn IndexingOperationsInterface>,
}

impl Default for AdminServices {
    fn default() -> Self {
        Self {
            performance: Arc::new(NullPerformanceMetrics),
            indexing: Arc::new(NullIndexingOperations),
        }
    }
}

impl std::fmt::Debug for AdminServices {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdminServices")
            .field("performance", &"NullPerformanceMetrics")
            .field("indexing", &"NullIndexingOperations")
            .finish()
    }
}

/// Application context with resolved providers and infrastructure services
///
/// This is the composition root that combines:
/// - External providers resolved from registry (embedding, vector_store, cache, language)
/// - Internal infrastructure services
pub struct AppContext {
    /// Application configuration
    pub config: AppConfig,
    /// Resolved external providers
    pub providers: ResolvedProviders,
    /// Infrastructure services (auth, events, metrics, sync, snapshot)
    pub infrastructure: InfrastructureServices,
    /// Admin services (performance, indexing)
    pub admin: AdminServices,
}

impl std::fmt::Debug for AppContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AppContext")
            .field("providers", &self.providers)
            .field("infrastructure", &self.infrastructure)
            .field("admin", &self.admin)
            .finish_non_exhaustive()
    }
}

/// Initialize application context with resolved providers
///
/// Resolves external providers from registry and builds internal services.
///
/// ## Example
///
/// ```ignore
/// let context = init_app(config).await?;
/// let embedding = context.providers.embedding;
/// ```
pub async fn init_app(config: AppConfig) -> Result<AppContext> {
    info!("Initializing application context");

    // Resolve external providers from registry
    let providers = resolve_providers(&config)?;
    info!("Resolved providers: {:?}", providers);

    // Build internal infrastructure services
    let infrastructure = InfrastructureServices::default();
    let admin = AdminServices::default();

    info!("Application context initialized");

    Ok(AppContext {
        config,
        providers,
        infrastructure,
        admin,
    })
}

/// Initialize application for testing
///
/// Uses null/test providers by default.
pub async fn init_test_app() -> Result<AppContext> {
    let config = AppConfig::default();
    init_app(config).await
}

/// Type alias for dispatch.rs compatibility
pub type DiContainer = AppContext;

/// Convenience function to create context for testing
pub async fn create_test_container() -> Result<AppContext> {
    init_test_app().await
}
