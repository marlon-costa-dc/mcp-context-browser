//! Provider connection tracker unit tests

use mcp_context_browser::infrastructure::provider_connection_tracker::ProviderConnectionTracker;
use std::time::Duration;

#[tokio::test]
async fn test_connection_tracking() {
    let tracker = ProviderConnectionTracker::new();

    // Initially no connections
    assert_eq!(tracker.active_count("provider1"), 0);

    // Track a connection
    let _guard = tracker.track_connection("provider1");
    assert_eq!(tracker.active_count("provider1"), 1);

    // Drop guard to decrement
    drop(_guard);
    assert_eq!(tracker.active_count("provider1"), 0);
}

#[tokio::test]
async fn test_wait_for_drain_immediate() {
    let tracker = ProviderConnectionTracker::new();

    // No connections, should drain immediately
    let drained = tracker
        .wait_for_drain("provider1", Duration::from_secs(1))
        .await;
    assert!(drained);
}

#[tokio::test]
async fn test_wait_for_drain_timeout() {
    let tracker = ProviderConnectionTracker::new();

    // Create a connection that won't drain
    let _guard = tracker.track_connection("provider1");

    // Should timeout
    let drained = tracker
        .wait_for_drain("provider1", Duration::from_millis(200))
        .await;
    assert!(!drained);

    // Clean up
    drop(_guard);
}

#[tokio::test]
async fn test_wait_for_drain_with_completion() {
    let tracker = ProviderConnectionTracker::new();
    let tracker_clone = tracker.clone();

    let tracker_for_task = tracker.clone();
    tokio::spawn(async move {
        // Hold connection for 100ms
        let _guard = tracker_for_task.track_connection("provider1");
        tokio::time::sleep(Duration::from_millis(100)).await;
    });

    // Wait for drain with sufficient timeout
    let drained = tracker_clone
        .wait_for_drain("provider1", Duration::from_secs(1))
        .await;
    assert!(drained);
}
