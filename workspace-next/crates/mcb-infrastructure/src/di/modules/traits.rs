//! Module Trait Interfaces - Shaku Strict Pattern
//!
//! These traits define the interfaces for domain-specific DI modules.
//! Each trait represents a bounded context in the Clean Architecture.
//!
//! ## Shaku Module Trait Pattern
//!
//! Module traits extend `HasComponent<dyn Trait>` for services they provide.
//! This enables compile-time dependency checking and runtime resolution.
//!
//! ## Domain Separation
//!
//! - `InfrastructureModule`: Core infrastructure (cache, crypto, health)
//! - `ServerModule`: MCP server components (metrics, indexing)
//! - `AdaptersModule`: External integrations (providers, repositories)
//! - `ApplicationModule`: Business logic services
//! - `AdminModule`: Administrative services (depends on all)

use shaku::HasComponent;

// ============================================================================
// Infrastructure Module (Core Services)
// ============================================================================

/// Infrastructure module trait - core infrastructure services.
///
/// Provides fundamental services that other modules depend on.
/// No external dependencies - this is the foundation layer.
pub trait InfrastructureModule:
    HasComponent<dyn crate::cache::provider::CacheProvider>
    + HasComponent<dyn crate::crypto::CryptoService>
    + HasComponent<dyn crate::health::HealthRegistry>
    + HasComponent<dyn crate::infrastructure::auth::AuthServiceInterface>
    + HasComponent<dyn crate::infrastructure::events::EventBusProvider>
    + HasComponent<dyn crate::infrastructure::metrics::system::SystemMetricsCollectorInterface>
    + HasComponent<dyn crate::infrastructure::snapshot::SnapshotProvider>
    + HasComponent<dyn crate::infrastructure::sync::SyncProvider>
{
}

// ============================================================================
// Server Module (MCP Server Components)
// ============================================================================

/// Server module trait - MCP server-specific components.
///
/// Provides server-side services like performance monitoring
/// and indexing operations. No external dependencies.
pub trait ServerModule:
    HasComponent<dyn mcb_domain::ports::admin::PerformanceMetricsInterface>
    + HasComponent<dyn mcb_domain::ports::admin::IndexingOperationsInterface>
{
}

// ============================================================================
// Adapters Module (External Integrations)
// ============================================================================

/// Adapters module trait - external service integrations.
///
/// Provides adapters for external systems: HTTP clients, AI providers,
/// vector stores, and repositories. No external dependencies.
pub trait AdaptersModule:
    HasComponent<dyn crate::adapters::http_client::HttpClientProvider>
    + HasComponent<dyn mcb_domain::ports::providers::EmbeddingProvider>
    + HasComponent<dyn mcb_domain::ports::providers::VectorStoreProvider>
    + HasComponent<dyn crate::adapters::repository::ChunkRepository>
    + HasComponent<dyn crate::adapters::repository::SearchRepository>
{
}

// ============================================================================
// Application Module (Business Logic)
// ============================================================================

/// Application module trait - business logic services.
///
/// Provides domain services for context management, search, and indexing.
/// Depends on adapters for data access.
pub trait ApplicationModule:
    HasComponent<dyn mcb_domain::domain_services::search::ContextServiceInterface>
    + HasComponent<dyn mcb_domain::domain_services::search::SearchServiceInterface>
    + HasComponent<dyn mcb_domain::domain_services::search::IndexingServiceInterface>
    + HasComponent<dyn mcb_domain::ports::ChunkingOrchestratorInterface>
    + HasComponent<dyn mcb_domain::ports::CodeChunker>
{
}

// ============================================================================
// Admin Module (Administrative Services)
// ============================================================================

/// Admin module trait - administrative services.
///
/// Provides admin services that depend on all other modules.
/// This is the composition root for administrative functionality.
pub trait AdminModule:
    HasComponent<dyn crate::application::admin::AdminService>
{
}

// ============================================================================
// Future Module Traits (v0.3.0+)
// ============================================================================

/// Analysis module trait - code complexity and technical debt detection.
///
/// Placeholder trait for future analysis capabilities including:
/// - Code complexity metrics (cyclomatic, cognitive)
/// - Technical debt detection
/// - Self-Admitted Technical Debt (SATD) identification
#[cfg(feature = "analysis")]
pub trait AnalysisModule: Send + Sync {}

/// Quality module trait - quality gates and assessment.
///
/// Placeholder trait for future quality capabilities including:
/// - Quality gate definitions and enforcement
/// - Code quality metrics
/// - Quality trend analysis
#[cfg(feature = "quality")]
pub trait QualityModule: Send + Sync {}

/// Git module trait - git operations and repository analysis.
///
/// Placeholder trait for future git integration including:
/// - Repository operations
/// - Commit history analysis
/// - Branch management
#[cfg(feature = "git")]
pub trait GitModule: Send + Sync {}