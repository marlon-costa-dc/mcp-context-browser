//! Service helpers unit tests

use mcp_context_browser::infrastructure::service_helpers::{
    IteratorHelpers, SafeMetrics, TimedOperation, ValidationBuilder,
};
use std::time::Duration;

#[test]
fn test_timed_operation() {
    let timer = TimedOperation::start();
    std::thread::sleep(Duration::from_millis(10));
    assert!(timer.elapsed_ms() >= 10);
}

#[test]
fn test_timed_operation_remaining() {
    let timer = TimedOperation::start();
    let remaining = timer.remaining(Duration::from_secs(10));
    assert!(remaining.is_some());
    assert!(remaining.unwrap() > Duration::from_secs(9));
}

#[tokio::test]
async fn test_safe_metrics_collect_success() {
    let result: i32 =
        SafeMetrics::collect::<_, i32, String>(async { Ok::<i32, String>(42) }, 0).await;
    assert_eq!(result, 42);
}

#[tokio::test]
async fn test_safe_metrics_collect_failure() {
    let result: i32 = SafeMetrics::collect::<_, i32, String>(
        async { Err::<i32, String>("error".to_string()) },
        99,
    )
    .await;
    assert_eq!(result, 99);
}

#[test]
fn test_validation_builder_range_ok() {
    let result = ValidationBuilder::new("test")
        .check_range("value", 50, 1, 100, "must be 1-100")
        .finalize();
    assert!(result.is_ok());
}

#[test]
fn test_validation_builder_range_fail() {
    let result = ValidationBuilder::new("test")
        .check_range("value", 150, 1, 100, "must be 1-100")
        .finalize();
    assert!(result.is_err());
}

#[test]
fn test_validation_builder_positive() {
    let result = ValidationBuilder::new("test")
        .check_positive("timeout", -1, "must be positive")
        .finalize();
    assert!(result.is_err());
}

#[test]
fn test_validation_builder_string() {
    let result = ValidationBuilder::new("test")
        .check_string_not_empty("username", "")
        .finalize();
    assert!(result.is_err());
}

#[test]
fn test_iterator_helpers_take_collect() {
    let vec = vec![1, 2, 3, 4, 5];
    let result = IteratorHelpers::take_collect(vec, 3);
    assert_eq!(result, vec![1, 2, 3]);
}
