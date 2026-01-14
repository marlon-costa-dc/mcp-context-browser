//! Server-level operations
//!
//! Re-exports operations implementations from the infrastructure layer
//! to maintain Clean Architecture boundaries.

// Re-export operations from infrastructure layer
pub use crate::infrastructure::operations::tracking::{
    IndexingOperation, IndexingOperationsInterface, McpIndexingOperations,
};
