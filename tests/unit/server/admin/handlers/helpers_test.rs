//! Unit tests for centralized handler helper functions
//!
//! Tests for error handling, JSON response wrapping, and service call patterns.

use axum::http::StatusCode;
use mcp_context_browser::server::admin::handlers::{
    error, error_to_status, internal_error, internal_error_with_log, result_to_json, success,
};
use serde::Serialize;

#[test]
fn test_internal_error() {
    let result = internal_error("some error");
    assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_internal_error_with_log() {
    let result = internal_error_with_log("some error");
    assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_success() {
    #[derive(Serialize)]
    struct TestData {
        value: i32,
    }

    let data = TestData { value: 42 };
    let result = success(data);
    assert!(result.is_ok());
}

#[test]
fn test_error() {
    #[derive(Serialize, Default)]
    struct TestData;

    let result = error::<TestData>("test error message");
    assert!(result.is_ok());
}

#[test]
fn test_result_to_json_success() {
    #[derive(Serialize)]
    struct TestData {
        value: i32,
    }

    let result: Result<_, String> = Ok(TestData { value: 42 });
    let response = result_to_json(result);
    assert!(response.is_ok());
}

#[test]
fn test_result_to_json_error() {
    let result: Result<String, String> = Err("test error".to_string());
    let response = result_to_json::<String, _>(result);
    assert!(response.is_err());
    assert_eq!(response.unwrap_err(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[test]
fn test_error_to_status() {
    let result = error_to_status("some error");
    assert_eq!(result, StatusCode::INTERNAL_SERVER_ERROR);
}
