//! Operations Tracking Module
//!
//! Infrastructure implementations for tracking system operations like indexing.
//!
//! ## Architecture
//!
//! - Traits defined in `domain::ports::admin`
//! - Implementations provided here for DI container registration

pub mod tracking;

pub use tracking::{IndexingOperation, IndexingOperationsInterface, McpIndexingOperations};
