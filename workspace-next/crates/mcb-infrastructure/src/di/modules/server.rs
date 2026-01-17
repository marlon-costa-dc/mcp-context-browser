//! Server Module Implementation - MCP Server Components
//!
//! This module provides MCP server-specific components for performance monitoring
//! and indexing operations. It follows the Shaku strict pattern with no dependencies.
//!
//! ## Services Provided
//!
//! - Performance metrics for query/response monitoring
//! - Indexing operations for background processing tracking

use shaku::module;

// Import concrete implementations from mcb-providers (admin services)
use mcb_providers::admin::{AtomicPerformanceMetrics, DefaultIndexingOperations};

// Import traits
use super::traits::ServerModule;

/// Server module implementation following Shaku strict pattern.
///
/// This module provides MCP server components with no external dependencies.
/// All services are concrete implementations for server-side functionality.
///
/// ## Component Registration
///
/// Services are imported from `mcb_providers::admin` crate.
/// Uses `#[derive(Component)]` and `#[shaku(interface = ...)]` for registration.
///
/// ## Construction
///
/// ```rust,ignore
/// let server = ServerModuleImpl::builder().build();
/// ```
module! {
    pub ServerModuleImpl: ServerModule {
        components = [
            // MCP server components
            AtomicPerformanceMetrics,
            DefaultIndexingOperations
        ],
        providers = []
    }
}