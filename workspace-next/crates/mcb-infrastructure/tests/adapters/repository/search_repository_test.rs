//! Search Repository Tests
//!
//! Tests for the VectorStoreSearchRepository implementation.

use mcb_infrastructure::adapters::repository::SearchStatsTracker;
use std::sync::atomic::Ordering;

#[test]
fn test_stats_tracker_default() {
    let tracker = SearchStatsTracker::default();
    assert_eq!(tracker.total_queries.load(Ordering::Relaxed), 0);
    assert_eq!(tracker.indexed_documents.load(Ordering::Relaxed), 0);
}

#[test]
fn test_stats_tracker_increment() {
    let tracker = SearchStatsTracker::default();

    tracker.total_queries.fetch_add(1, Ordering::Relaxed);
    tracker.indexed_documents.fetch_add(100, Ordering::Relaxed);

    assert_eq!(tracker.total_queries.load(Ordering::Relaxed), 1);
    assert_eq!(tracker.indexed_documents.load(Ordering::Relaxed), 100);
}
