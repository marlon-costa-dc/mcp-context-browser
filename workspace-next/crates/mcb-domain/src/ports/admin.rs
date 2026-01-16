//! Admin Service Domain Ports
//!
//! Defines the port interfaces for admin and monitoring services.
//! These traits break the circular dependency where infrastructure/di
//! previously imported from server layer.

use serde::{Deserialize, Serialize};
use shaku::Interface;
use std::collections::HashMap;

// ============================================================================
// Performance Metrics Types
// ============================================================================

/// Performance metrics data
///
/// This type is defined in domain to allow the trait to be used
/// without circular dependencies on server layer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetricsData {
    /// Total Queries
    pub total_queries: u64,
    /// Successful Queries
    pub successful_queries: u64,
    /// Failed Queries
    pub failed_queries: u64,
    /// Average Response Time Ms
    pub average_response_time_ms: f64,
    /// Cache Hit Rate
    pub cache_hit_rate: f64,
    /// Active Connections
    pub active_connections: u32,
    /// Uptime Seconds
    pub uptime_seconds: u64,
}

// ============================================================================
// Performance Metrics Interface
// ============================================================================

/// Real-time performance metrics tracking interface
///
/// Domain port for tracking server performance metrics including
/// queries, response times, cache hits, and active connections.
///
/// # Example
///
/// ```ignore
/// use mcb_domain::ports::admin::PerformanceMetricsInterface;
///
/// // Record a successful query with 50ms response time (cache miss)
/// metrics.record_query(50, true, false);
///
/// // Track active connections
/// metrics.update_active_connections(1);  // connection opened
/// metrics.update_active_connections(-1); // connection closed
///
/// // Get current metrics snapshot
/// let stats = metrics.get_performance_metrics();
/// println!("Uptime: {}s, Queries: {}", stats.uptime_seconds, stats.total_queries);
/// ```
pub trait PerformanceMetricsInterface: Interface + Send + Sync {
    /// Get server uptime in seconds
    fn uptime_secs(&self) -> u64;

    /// Record a query with its metrics
    fn record_query(&self, response_time_ms: u64, success: bool, cache_hit: bool);

    /// Update active connection count (positive to add, negative to remove)
    fn update_active_connections(&self, delta: i64);

    /// Get current performance metrics snapshot
    fn get_performance_metrics(&self) -> PerformanceMetricsData;
}

// ============================================================================
// Indexing Operations Types
// ============================================================================

/// Tracks ongoing indexing operations
#[derive(Debug, Clone)]
pub struct IndexingOperation {
    /// Operation ID
    pub id: String,
    /// Collection being indexed
    pub collection: String,
    /// Current file being processed
    pub current_file: Option<String>,
    /// Total files to process
    pub total_files: usize,
    /// Files processed so far
    pub processed_files: usize,
    /// Operation start timestamp (Unix timestamp)
    pub start_timestamp: u64,
}

// ============================================================================
// Indexing Operations Interface
// ============================================================================

/// Interface for indexing operations tracking
///
/// Domain port for tracking ongoing indexing operations in the MCP server.
///
/// # Example
///
/// ```ignore
/// use mcb_domain::ports::admin::IndexingOperationsInterface;
///
/// // Get all active indexing operations
/// let operations = tracker.get_operations();
/// for (id, op) in operations {
///     println!("Operation {}: {}/{} files in {}",
///         id, op.processed_files, op.total_files, op.collection);
/// }
/// ```
pub trait IndexingOperationsInterface: Interface + Send + Sync {
    /// Get the map of ongoing indexing operations
    fn get_operations(&self) -> HashMap<String, IndexingOperation>;
}
