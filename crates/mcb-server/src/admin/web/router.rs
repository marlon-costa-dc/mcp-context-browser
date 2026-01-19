//! Web Router Module
//!
//! Router configuration for the admin web interface.

use axum::routing::get;
use axum::Router;

use super::handlers;

/// Create the admin web UI router
///
/// Routes:
/// - GET `/` - Dashboard
/// - GET `/ui` - Dashboard (alias)
/// - GET `/ui/config` - Configuration page
/// - GET `/ui/health` - Health status page
/// - GET `/ui/indexing` - Indexing status page
/// - GET `/favicon.ico` - Favicon
pub fn web_router() -> Router {
    Router::new()
        .route("/", get(handlers::dashboard))
        .route("/ui", get(handlers::dashboard))
        .route("/ui/config", get(handlers::config_page))
        .route("/ui/health", get(handlers::health_page))
        .route("/ui/indexing", get(handlers::indexing_page))
        .route("/favicon.ico", get(handlers::favicon))
}
