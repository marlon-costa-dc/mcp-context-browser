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
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
}

/// CPU usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStats {
    pub usage_percent: f32,
    pub cores: usize,
}

/// Disk usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskStats {
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub usage_percent: f32,
}

/// Operation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationStats {
    pub active_indexing: usize,
    pub active_search: usize,
    pub active_embedding: usize,
    pub queued_operations: usize,
}

/// Resource limit violations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceViolation {
    MemoryLimitExceeded {
        current_percent: f32,
        limit_percent: f32,
    },
    CpuLimitExceeded {
        current_percent: f32,
        limit_percent: f32,
    },
    DiskLimitExceeded {
        current_percent: f32,
        limit_percent: f32,
    },
    DiskSpaceLow {
        available_bytes: u64,
        required_bytes: u64,
    },
    ConcurrencyLimitExceeded {
        operation_type: String,
        current: usize,
        limit: usize,
    },
}
