//! Admin Web UI Module
//!
//! Provides an HTMX-powered web interface for the admin panel.
//! Templates are embedded at compile time for zero-dependency deployment.
//!
//! ## Pages
//!
//! - `/` or `/ui` - Dashboard with real-time metrics
//! - `/ui/config` - Configuration editor with live reload
//! - `/ui/health` - Health status and dependency monitoring
//! - `/ui/indexing` - Indexing operation progress

pub mod handlers;
pub mod router;

// Re-export public functions
pub use handlers::{config_page, dashboard, favicon, health_page, indexing_page};
pub use router::web_router;
