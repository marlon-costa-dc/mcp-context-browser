//! HTML response generation helpers
//!
//! Provides reusable HTML fragments for common UI patterns in admin responses.
//! These helpers ensure consistent styling and markup across the admin interface.
//!
//! # Example
//!
//! ```rust
//! use mcp_context_browser::server::admin::web::html_helpers::*;
//!
//! // Generate styled messages
//! let error = html_error("Failed to update");
//! let success = html_success("Configuration saved");
//! let warning = html_warning("Operation in progress");
//! let info = html_info("Click 'Add' to create");
//!
//! // HTMX responses for AJAX
//! let htmx_err = htmx_error("Load failed");
//! let htmx_ok = htmx_success("Updated");
//! let loading = htmx_loading();
//! ```

use axum::response::Html;

/// Generate an HTML error message with red styling
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::server::admin::web::html_helpers::html_error;
///
/// let response = html_error("Failed to update configuration");
/// assert!(response.0.contains("text-red-600"));
/// assert!(response.0.contains("Error:"));
/// ```
#[inline]
pub fn html_error(message: impl AsRef<str>) -> Html<String> {
    Html(format!(
        r#"<div class="text-red-600 dark:text-red-400 mt-2">Error: {}</div>"#,
        message.as_ref()
    ))
}

/// Generate an HTML success message with green styling
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::server::admin::web::html_helpers::html_success;
///
/// let response = html_success("Configuration updated successfully");
/// assert!(response.0.contains("text-green-600"));
/// ```
#[inline]
pub fn html_success(message: impl AsRef<str>) -> Html<String> {
    Html(format!(
        r#"<div class="text-green-600 dark:text-green-400 mt-2">{}</div>"#,
        message.as_ref()
    ))
}

/// Generate an HTML warning message with yellow styling
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::server::admin::web::html_helpers::html_warning;
///
/// let response = html_warning("This operation may take some time");
/// assert!(response.0.contains("text-yellow-600"));
/// ```
#[inline]
pub fn html_warning(message: impl AsRef<str>) -> Html<String> {
    Html(format!(
        r#"<div class="text-yellow-600 dark:text-yellow-400 mt-2">{}</div>"#,
        message.as_ref()
    ))
}

/// Generate an HTML info message with blue styling
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::server::admin::web::html_helpers::html_info;
///
/// let response = html_info("No providers found. Click 'Add Provider' to create one.");
/// assert!(response.0.contains("text-blue-600"));
/// ```
#[inline]
pub fn html_info(message: impl AsRef<str>) -> Html<String> {
    Html(format!(
        r#"<div class="text-blue-600 dark:text-blue-400 mt-2">{}</div>"#,
        message.as_ref()
    ))
}

// =============================================================================
// HTMX-Specific Helpers - For AJAX responses (used in HTMX handlers)
// =============================================================================

/// Generate HTMX-compatible error message HTML
///
/// Uses Tailwind classes with dark mode support for AJAX error responses.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::server::admin::web::html_helpers::htmx_error;
///
/// let response = htmx_error("Failed to load metrics");
/// assert!(response.0.contains("text-red-500"));
/// ```
#[inline]
pub fn htmx_error(message: impl AsRef<str>) -> Html<String> {
    Html(format!(
        r#"<div class="text-red-500 dark:text-red-400">{}</div>"#,
        message.as_ref()
    ))
}

/// Generate HTMX-compatible loading message HTML
///
/// Uses gray color to indicate loading state.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::server::admin::web::html_helpers::htmx_loading;
///
/// let response = htmx_loading();
/// assert!(response.0.contains("Loading..."));
/// ```
#[inline]
pub fn htmx_loading() -> Html<String> {
    Html(r#"<div class="text-gray-500 dark:text-gray-400">Loading...</div>"#.to_string())
}

/// Generate HTMX-compatible success message HTML
///
/// Uses green color for success state.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::server::admin::web::html_helpers::htmx_success;
///
/// let response = htmx_success("Metrics updated successfully");
/// assert!(response.0.contains("text-green-500"));
/// ```
#[inline]
pub fn htmx_success(message: impl AsRef<str>) -> Html<String> {
    Html(format!(
        r#"<div class="text-green-500 dark:text-green-400">{}</div>"#,
        message.as_ref()
    ))
}
