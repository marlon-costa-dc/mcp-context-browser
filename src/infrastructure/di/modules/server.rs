//! Server DI Module Implementation
//!
//! Contains MCP server metrics and indexing operations.

use shaku::module;

use super::traits::ServerModule;
use crate::infrastructure::operations::McpIndexingOperations;
use crate::server::metrics::McpPerformanceMetrics;

/// Implementation of the ServerModule trait providing server-specific components.
///
/// This module contains server-level components for monitoring and operations:
/// - McpPerformanceMetrics for tracking server performance statistics
/// - McpIndexingOperations for managing and tracking indexing operations
module! {
    pub ServerModuleImpl: ServerModule {
        components = [McpPerformanceMetrics, McpIndexingOperations],
        providers = []
    }
}
