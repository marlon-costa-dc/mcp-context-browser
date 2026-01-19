//! Web Handlers Module
//!
//! HTTP handlers for the admin web interface.
//!
//! Migrated from Axum to Rocket in v0.1.2 (ADR-026).

use rocket::get;
use rocket::http::ContentType;
use rocket::response::content::RawHtml;

// Embed templates at compile time
const INDEX_HTML: &str = include_str!("templates/index.html");
const CONFIG_HTML: &str = include_str!("templates/config.html");
const HEALTH_HTML: &str = include_str!("templates/health.html");
const INDEXING_HTML: &str = include_str!("templates/indexing.html");

/// Dashboard page handler
#[get("/")]
pub fn dashboard() -> RawHtml<&'static str> {
    RawHtml(INDEX_HTML)
}

/// Dashboard page handler (alias)
#[get("/ui")]
pub fn dashboard_ui() -> RawHtml<&'static str> {
    RawHtml(INDEX_HTML)
}

/// Configuration page handler
#[get("/ui/config")]
pub fn config_page() -> RawHtml<&'static str> {
    RawHtml(CONFIG_HTML)
}

/// Health status page handler
#[get("/ui/health")]
pub fn health_page() -> RawHtml<&'static str> {
    RawHtml(HEALTH_HTML)
}

/// Indexing status page handler
#[get("/ui/indexing")]
pub fn indexing_page() -> RawHtml<&'static str> {
    RawHtml(INDEXING_HTML)
}

/// Favicon handler - returns a simple SVG icon
#[get("/favicon.ico")]
pub fn favicon() -> (ContentType, &'static str) {
    (
        ContentType::SVG,
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><text y=".9em" font-size="90">📊</text></svg>"#,
    )
}
