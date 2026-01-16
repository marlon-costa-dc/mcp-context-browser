//! Admin Adapter Implementations
//!
//! Infrastructure implementations for admin domain ports including
//! performance metrics tracking and indexing operations monitoring.

pub mod indexing;
pub mod metrics;

pub use indexing::DefaultIndexingOperations;
pub use metrics::AtomicPerformanceMetrics;
