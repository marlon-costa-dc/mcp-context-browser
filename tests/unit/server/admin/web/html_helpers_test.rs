//! Unit tests for HTML response generation helpers
//!
//! Tests for reusable HTML fragments for common UI patterns in admin responses.

use mcp_context_browser::server::admin::web::html_helpers::{
    html_error, html_info, html_success, html_warning, htmx_error, htmx_loading, htmx_success,
};

#[test]
fn test_html_error() {
    let result = html_error("test error");
    assert!(result.0.contains("text-red-600"));
    assert!(result.0.contains("Error: test error"));
}

#[test]
fn test_html_success() {
    let result = html_success("test success");
    assert!(result.0.contains("text-green-600"));
    assert!(result.0.contains("test success"));
}

#[test]
fn test_html_warning() {
    let result = html_warning("test warning");
    assert!(result.0.contains("text-yellow-600"));
    assert!(result.0.contains("test warning"));
}

#[test]
fn test_html_info() {
    let result = html_info("test info");
    assert!(result.0.contains("text-blue-600"));
    assert!(result.0.contains("test info"));
}

#[test]
fn test_htmx_error() {
    let result = htmx_error("Failed to load metrics");
    assert!(result.0.contains("text-red-500"));
    assert!(result.0.contains("dark:text-red-400"));
    assert!(result.0.contains("Failed to load metrics"));
}

#[test]
fn test_htmx_loading() {
    let result = htmx_loading();
    assert!(result.0.contains("text-gray-500"));
    assert!(result.0.contains("dark:text-gray-400"));
    assert!(result.0.contains("Loading..."));
}

#[test]
fn test_htmx_success() {
    let result = htmx_success("Updated successfully");
    assert!(result.0.contains("text-green-500"));
    assert!(result.0.contains("dark:text-green-400"));
    assert!(result.0.contains("Updated successfully"));
}
