//! Admin API routes
//!
//! Route definitions for the admin API endpoints.

use axum::{routing::get, Router};

use super::handlers::{
    get_indexing_status, get_metrics, health_check, liveness_check, readiness_check, AdminState,
};

/// Create the admin API router
///
/// Routes:
/// - GET /health - Health check with uptime and status
/// - GET /metrics - Performance metrics
/// - GET /indexing - Indexing operations status
/// - GET /ready - Kubernetes readiness probe
/// - GET /live - Kubernetes liveness probe
pub fn admin_router(state: AdminState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/metrics", get(get_metrics))
        .route("/indexing", get(get_indexing_status))
        .route("/ready", get(readiness_check))
        .route("/live", get(liveness_check))
        .with_state(state)
}

/// Create admin router with a prefix
///
/// This creates a nested router under the given prefix, e.g., `/admin`.
pub fn admin_router_with_prefix(prefix: &str, state: AdminState) -> Router {
    Router::new().nest(prefix, admin_router(state))
}
