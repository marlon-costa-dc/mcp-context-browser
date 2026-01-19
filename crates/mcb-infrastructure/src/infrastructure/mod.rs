//! Infrastructure Services
//!
//! Infrastructure service implementations for port traits defined in mcb-application.
//!
//! ## ARCHITECTURE RULE
//!
//! **CONCRETE TYPES ARE INTERNAL ONLY.**
//!
//! All implementations are composed in the DI bootstrap module.
//! External code SHOULD use `init_app()` to get an `AppContext` with resolved services.
//! NEVER import concrete types directly from here - use the trait abstractions.

// Internal modules - implementations NOT exported
pub(crate) mod admin;
pub(crate) mod auth;
pub(crate) mod events;
pub(crate) mod lifecycle;
pub(crate) mod metrics;
pub(crate) mod snapshot;
pub(crate) mod sync;

// Public data types (NOT implementations) - these are pure DTOs
pub use lifecycle::{ServiceInfo, ServiceManager, ServiceManagerError};

// Crate-internal re-exports for tests and DI modules
pub(crate) use admin::{NullIndexingOperations, NullPerformanceMetrics};
pub(crate) use auth::NullAuthService;
pub(crate) use events::{NullEventBusProvider, TokioBroadcastEventBus};
pub(crate) use lifecycle::DefaultShutdownCoordinator;
pub(crate) use metrics::NullSystemMetricsCollector;
pub(crate) use snapshot::{NullSnapshotProvider, NullStateStoreProvider};
pub(crate) use sync::{NullLockProvider, NullSyncProvider};
