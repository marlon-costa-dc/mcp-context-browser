//! Indexing operations tracking for MCP server
//!
//! Provides implementations for tracking ongoing indexing operations
//! in the MCP server.
//!
//! The trait and types are defined in `domain::ports::admin` to avoid circular dependencies.

use dashmap::DashMap;

// Re-export from domain for backward compatibility
pub use crate::domain::ports::admin::{IndexingOperation, IndexingOperationsInterface};

/// Concrete implementation of indexing operations tracking
#[derive(Debug, Default, shaku::Component)]
#[shaku(interface = IndexingOperationsInterface)]
pub struct McpIndexingOperations {
    #[shaku(default)]
    pub map: DashMap<String, IndexingOperation>,
}

impl IndexingOperationsInterface for McpIndexingOperations {
    fn get_map(&self) -> &DashMap<String, IndexingOperation> {
        &self.map
    }
}
