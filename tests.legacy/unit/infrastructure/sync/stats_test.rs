//! Tests for sync statistics service
//!
//! Migrated from src/infrastructure/sync/stats.rs

use mcp_context_browser::infrastructure::sync::stats::SyncStatsCollector;

#[test]
fn test_new_collector() {
    let collector = SyncStatsCollector::new();
    let stats = collector.snapshot();
    assert_eq!(stats.total_attempts, 0);
    assert_eq!(stats.successful, 0);
    assert_eq!(stats.skipped, 0);
    assert_eq!(stats.failed, 0);
}

#[test]
fn test_record_operations() {
    let collector = SyncStatsCollector::new();

    collector.record_attempt();
    collector.record_attempt();
    collector.record_success();
    collector.record_skip();

    let stats = collector.snapshot();
    assert_eq!(stats.total_attempts, 2);
    assert_eq!(stats.successful, 1);
    assert_eq!(stats.skipped, 1);
    assert_eq!(stats.failed, 0);
}

#[test]
fn test_skipped_rate() {
    let collector = SyncStatsCollector::new();

    // 4 attempts, 2 skipped = 50%
    for _ in 0..4 {
        collector.record_attempt();
    }
    collector.record_skip();
    collector.record_skip();

    let stats = collector.snapshot();
    assert!((stats.skipped_rate - 50.0).abs() < 0.01);
}

#[test]
fn test_reset() {
    let collector = SyncStatsCollector::new();

    collector.record_attempt();
    collector.record_success();
    collector.reset();

    let stats = collector.snapshot();
    assert_eq!(stats.total_attempts, 0);
    assert_eq!(stats.successful, 0);
}
