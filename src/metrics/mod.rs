//! System metrics collection and HTTP API for MCP Context Browser v0.0.3
//!
//! This module provides comprehensive system monitoring capabilities including:
//! - CPU, memory, disk, and network metrics collection
//! - Query performance and cache metrics tracking
//! - HTTP REST API server on port 3001
//! - Web dashboard for real-time metrics visualization

pub mod http_server;
pub mod performance;
pub mod system;

pub use http_server::MetricsApiServer;
pub use performance::{CacheMetrics, PerformanceMetrics, QueryPerformanceMetrics};
pub use system::{
    CpuMetrics, DiskMetrics, MemoryMetrics, NetworkMetrics, ProcessMetrics, SystemMetricsCollector,
};

/// Global metrics collector singleton
use std::sync::Arc;
use tokio::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref GLOBAL_METRICS_COLLECTOR: Arc<Mutex<PerformanceMetrics>> = Arc::new(Mutex::new(PerformanceMetrics::new()));
}

/// Get the global metrics collector instance
pub fn global_metrics_collector() -> Arc<Mutex<PerformanceMetrics>> {
    Arc::clone(&GLOBAL_METRICS_COLLECTOR)
}
