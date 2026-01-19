//! Admin API routes
//!
//! Route definitions for the admin API endpoints.
//!
//! Migrated from Axum to Rocket in v0.1.2 (ADR-026).

use rocket::{routes, Build, Rocket};

use super::config_handlers::{get_config, reload_config, update_config_section};
use super::handlers::{
    extended_health_check, get_cache_stats, get_indexing_status, get_metrics, health_check,
    liveness_check, readiness_check, shutdown, AdminState,
};
use super::lifecycle_handlers::{
    list_services, restart_service, services_health, start_service, stop_service,
};
use super::sse::events_stream;

/// Create the admin API rocket instance
///
/// Routes:
/// - GET /health - Health check with uptime and status
/// - GET /health/extended - Extended health check with dependency status
/// - GET /metrics - Performance metrics
/// - GET /indexing - Indexing operations status
/// - GET /ready - Kubernetes readiness probe
/// - GET /live - Kubernetes liveness probe
/// - POST /shutdown - Initiate graceful server shutdown
/// - GET /config - View current configuration (sanitized)
/// - POST /config/reload - Trigger configuration reload
/// - PATCH /config/:section - Update configuration section
/// - GET /events - SSE event stream for real-time updates
/// - GET /services - List registered services
/// - GET /services/health - Health check all services
/// - POST /services/:name/start - Start a service
/// - POST /services/:name/stop - Stop a service
/// - POST /services/:name/restart - Restart a service
/// - GET /cache/stats - Cache statistics
pub fn admin_rocket(state: AdminState) -> Rocket<Build> {
    rocket::build()
        .manage(state)
        .mount(
            "/",
            routes![
                // Health and monitoring
                health_check,
                extended_health_check,
                get_metrics,
                get_indexing_status,
                readiness_check,
                liveness_check,
                // Service control
                shutdown,
                // Configuration management
                get_config,
                reload_config,
                update_config_section,
                // SSE event stream
                events_stream,
                // Service lifecycle management
                list_services,
                services_health,
                start_service,
                stop_service,
                restart_service,
                // Cache management
                get_cache_stats,
            ],
        )
}
