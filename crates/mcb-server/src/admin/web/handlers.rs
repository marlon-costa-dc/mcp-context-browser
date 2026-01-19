//! Web Handlers Module
//!
//! HTTP handlers for the admin web interface.

use axum::{
    http::{header, StatusCode},
    response::{Html, IntoResponse},
};

// Embed templates at compile time
const INDEX_HTML: &str = include_str!("templates/index.html");
const CONFIG_HTML: &str = include_str!("templates/config.html");
const HEALTH_HTML: &str = include_str!("templates/health.html");
const INDEXING_HTML: &str = include_str!("templates/indexing.html");

/// Dashboard page handler
pub async fn dashboard() -> Html<&'static str> {
    Html(INDEX_HTML)
}

/// Configuration page handler
pub async fn config_page() -> Html<&'static str> {
    Html(CONFIG_HTML)
}

/// Health status page handler
pub async fn health_page() -> Html<&'static str> {
    Html(HEALTH_HTML)
}

/// Indexing status page handler
pub async fn indexing_page() -> Html<&'static str> {
    Html(INDEXING_HTML)
}

/// Favicon handler - returns a simple SVG icon
pub async fn favicon() -> impl IntoResponse {
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "image/svg+xml")],
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><text y=".9em" font-size="90">📊</text></svg>"#,
    )
}
