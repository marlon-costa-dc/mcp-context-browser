//! Admin request handlers
//!
//! HTTP handlers for admin API endpoints including health checks,
//! performance metrics, and indexing status.

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mcb_domain::ports::admin::{IndexingOperationsInterface, PerformanceMetricsInterface};
use serde::Serialize;
use std::sync::Arc;

/// Admin handler state containing shared service references
#[derive(Clone)]
pub struct AdminState {
    /// Performance metrics tracker
    pub metrics: Arc<dyn PerformanceMetricsInterface>,
    /// Indexing operations tracker
    pub indexing: Arc<dyn IndexingOperationsInterface>,
}

/// Health check response for admin API
#[derive(Serialize)]
pub struct AdminHealthResponse {
    /// Server status
    pub status: &'static str,
    /// Server uptime in seconds
    pub uptime_seconds: u64,
    /// Number of active indexing operations
    pub active_indexing_operations: usize,
}

/// Health check endpoint
pub async fn health_check(State(state): State<AdminState>) -> impl IntoResponse {
    let metrics = state.metrics.get_performance_metrics();
    let operations = state.indexing.get_operations();

    Json(AdminHealthResponse {
        status: "healthy",
        uptime_seconds: metrics.uptime_seconds,
        active_indexing_operations: operations.len(),
    })
}

/// Get performance metrics endpoint
pub async fn get_metrics(State(state): State<AdminState>) -> impl IntoResponse {
    let metrics = state.metrics.get_performance_metrics();
    Json(metrics)
}

/// Indexing status response
#[derive(Serialize)]
pub struct IndexingStatusResponse {
    /// Whether indexing is currently active
    pub is_indexing: bool,
    /// Number of active operations
    pub active_operations: usize,
    /// Details of each operation
    pub operations: Vec<IndexingOperationStatus>,
}

/// Individual indexing operation status
#[derive(Serialize)]
pub struct IndexingOperationStatus {
    /// Operation ID
    pub id: String,
    /// Collection being indexed
    pub collection: String,
    /// Current file being processed
    pub current_file: Option<String>,
    /// Progress as percentage
    pub progress_percent: f32,
    /// Files processed
    pub processed_files: usize,
    /// Total files
    pub total_files: usize,
}

/// Get indexing status endpoint
pub async fn get_indexing_status(State(state): State<AdminState>) -> impl IntoResponse {
    let operations = state.indexing.get_operations();

    let operation_statuses: Vec<IndexingOperationStatus> = operations
        .values()
        .map(|op| {
            let progress = if op.total_files > 0 {
                (op.processed_files as f32 / op.total_files as f32) * 100.0
            } else {
                0.0
            };

            IndexingOperationStatus {
                id: op.id.clone(),
                collection: op.collection.clone(),
                current_file: op.current_file.clone(),
                progress_percent: progress,
                processed_files: op.processed_files,
                total_files: op.total_files,
            }
        })
        .collect();

    Json(IndexingStatusResponse {
        is_indexing: !operation_statuses.is_empty(),
        active_operations: operation_statuses.len(),
        operations: operation_statuses,
    })
}

/// Readiness check endpoint (for k8s/docker health checks)
pub async fn readiness_check(State(state): State<AdminState>) -> impl IntoResponse {
    let metrics = state.metrics.get_performance_metrics();

    // Consider ready if server has been up for at least 1 second
    if metrics.uptime_seconds >= 1 {
        (StatusCode::OK, Json(serde_json::json!({ "ready": true })))
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({ "ready": false })),
        )
    }
}

/// Liveness check endpoint (for k8s/docker health checks)
pub async fn liveness_check() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "alive": true })))
}
