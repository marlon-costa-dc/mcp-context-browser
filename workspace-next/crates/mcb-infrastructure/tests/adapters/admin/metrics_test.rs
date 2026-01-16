//! AtomicPerformanceMetrics Tests
//!
//! Comprehensive tests for the atomic performance metrics tracker.

use mcb_domain::ports::admin::PerformanceMetricsInterface;
use mcb_infrastructure::adapters::admin::AtomicPerformanceMetrics;
use std::sync::Arc;
use std::thread;

#[test]
fn test_metrics_record_successful_query() {
    let metrics = AtomicPerformanceMetrics::new();

    metrics.record_query(100, true, false);
    metrics.record_query(200, true, false);

    let data = metrics.get_performance_metrics();
    assert_eq!(data.total_queries, 2);
    assert_eq!(data.successful_queries, 2);
    assert_eq!(data.failed_queries, 0);
}

#[test]
fn test_metrics_record_failed_query() {
    let metrics = AtomicPerformanceMetrics::new();

    metrics.record_query(50, false, false);
    metrics.record_query(75, false, false);
    metrics.record_query(100, true, false);

    let data = metrics.get_performance_metrics();
    assert_eq!(data.total_queries, 3);
    assert_eq!(data.successful_queries, 1);
    assert_eq!(data.failed_queries, 2);
}

#[test]
fn test_metrics_cache_hit_tracking() {
    let metrics = AtomicPerformanceMetrics::new();

    // 3 queries: 2 cache hits, 1 miss
    metrics.record_query(10, true, true);
    metrics.record_query(20, true, true);
    metrics.record_query(30, true, false);

    let data = metrics.get_performance_metrics();
    assert_eq!(data.total_queries, 3);
    assert!((data.cache_hit_rate - 0.6666).abs() < 0.01); // ~66.7% cache hit rate
}

#[test]
fn test_metrics_average_response_time() {
    let metrics = AtomicPerformanceMetrics::new();

    // Response times: 100, 200, 300 = average 200
    metrics.record_query(100, true, false);
    metrics.record_query(200, true, false);
    metrics.record_query(300, true, false);

    let data = metrics.get_performance_metrics();
    assert!((data.average_response_time_ms - 200.0).abs() < 0.01);
}

#[test]
fn test_metrics_average_response_time_zero_queries() {
    let metrics = AtomicPerformanceMetrics::new();

    let data = metrics.get_performance_metrics();
    assert_eq!(data.average_response_time_ms, 0.0);
    assert_eq!(data.cache_hit_rate, 0.0);
}

#[test]
fn test_metrics_active_connections_increment() {
    let metrics = AtomicPerformanceMetrics::new();

    metrics.update_active_connections(1);
    metrics.update_active_connections(1);
    metrics.update_active_connections(1);

    let data = metrics.get_performance_metrics();
    assert_eq!(data.active_connections, 3);
}

#[test]
fn test_metrics_active_connections_decrement() {
    let metrics = AtomicPerformanceMetrics::new();

    metrics.update_active_connections(5);
    metrics.update_active_connections(-2);

    let data = metrics.get_performance_metrics();
    assert_eq!(data.active_connections, 3);
}

#[test]
fn test_metrics_active_connections_decrement_bounds() {
    let metrics = AtomicPerformanceMetrics::new();

    // Start with 2 connections
    metrics.update_active_connections(2);

    // Try to remove more than we have - should clamp to 0
    metrics.update_active_connections(-10);

    let data = metrics.get_performance_metrics();
    assert_eq!(data.active_connections, 0);
}

#[test]
fn test_metrics_uptime_calculation() {
    let metrics = AtomicPerformanceMetrics::new();

    // Small sleep to ensure uptime > 0
    std::thread::sleep(std::time::Duration::from_millis(10));

    let uptime = metrics.uptime_secs();
    // Uptime should be 0 or very small (test runs fast)
    assert!(uptime < 10, "Uptime should be small: got {}", uptime);
}

#[test]
fn test_metrics_concurrent_access() {
    let metrics = Arc::new(AtomicPerformanceMetrics::new());
    let mut handles = vec![];

    // Spawn 10 threads, each recording 100 queries
    for _ in 0..10 {
        let m = Arc::clone(&metrics);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                m.record_query(10, true, true);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    let data = metrics.get_performance_metrics();
    assert_eq!(data.total_queries, 1000);
    assert_eq!(data.successful_queries, 1000);
    assert_eq!(data.cache_hit_rate, 1.0);
}

#[test]
fn test_metrics_default_trait() {
    let metrics = AtomicPerformanceMetrics::default();
    let data = metrics.get_performance_metrics();

    assert_eq!(data.total_queries, 0);
    assert_eq!(data.successful_queries, 0);
    assert_eq!(data.failed_queries, 0);
    assert_eq!(data.active_connections, 0);
}

#[test]
fn test_metrics_new_shared() {
    let metrics = AtomicPerformanceMetrics::new_shared();

    metrics.record_query(50, true, false);

    let data = metrics.get_performance_metrics();
    assert_eq!(data.total_queries, 1);
}
