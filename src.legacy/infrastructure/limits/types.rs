//! Resource limits types
//!
//! Defines statistics and violation types for resource monitoring.

use serde::{Deserialize, Serialize};

/// Resource usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceStats {
    /// Memory usage
    pub memory: MemoryStats,
    /// CPU usage
    pub cpu: CpuStats,
    /// Disk usage
    pub disk: DiskStats,
    /// Operation counts
    pub operations: OperationStats,
    /// Timestamp of measurement
    pub timestamp: u64,
}

/// Memory usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    /// Total
    pub total: u64,
    /// Used
    pub used: u64,
    /// Available
    pub available: u64,
    /// Usage Percent
    pub usage_percent: f32,
}

/// CPU usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStats {
    /// Usage Percent
    pub usage_percent: f32,
    /// Cores
    pub cores: usize,
}

/// Disk usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskStats {
    /// Total
    pub total: u64,
    /// Used
    pub used: u64,
    /// Available
    pub available: u64,
    /// Usage Percent
    pub usage_percent: f32,
}

/// Operation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationStats {
    /// Active Indexing
    pub active_indexing: usize,
    /// Active Search
    pub active_search: usize,
    /// Active Embedding
    pub active_embedding: usize,
    /// Queued Operations
    pub queued_operations: usize,
}

/// Resource limit violations with detailed information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceViolation {
    /// Memory usage exceeded configured limit
    MemoryLimitExceeded {
        /// Current memory usage as percentage
        current_percent: f32,
        /// Configured memory limit as percentage
        limit_percent: f32,
    },
    /// CPU usage exceeded configured limit
    CpuLimitExceeded {
        /// Current CPU usage as percentage
        current_percent: f32,
        /// Configured CPU limit as percentage
        limit_percent: f32,
    },
    /// Disk I/O exceeded configured limit
    DiskLimitExceeded {
        /// Current disk I/O usage as percentage
        current_percent: f32,
        /// Configured disk I/O limit as percentage
        limit_percent: f32,
    },
    /// Available disk space below required threshold
    DiskSpaceLow {
        /// Available disk space in bytes
        available_bytes: u64,
        /// Required minimum disk space in bytes
        required_bytes: u64,
    },
    /// Concurrent operations exceeded configured limit
    ConcurrencyLimitExceeded {
        /// Type of operation being limited
        operation_type: String,
        /// Current number of concurrent operations
        current: usize,
        /// Maximum allowed concurrent operations
        limit: usize,
    },
}
