//! Tests for ShutdownCoordinator
//!
//! Tests shutdown coordination and signaling functionality.

use mcb_domain::ports::admin::ShutdownCoordinator;
use mcb_infrastructure::adapters::admin::shutdown::DefaultShutdownCoordinator;

#[test]
fn test_shutdown_coordinator_creation() {
    let coordinator = DefaultShutdownCoordinator::new();
    assert!(!coordinator.is_shutting_down());
}

#[test]
fn test_shutdown_signal() {
    let coordinator = DefaultShutdownCoordinator::new();

    assert!(!coordinator.is_shutting_down());
    coordinator.signal_shutdown();
    assert!(coordinator.is_shutting_down());
}

#[test]
fn test_shutdown_signal_idempotent() {
    let coordinator = DefaultShutdownCoordinator::new();

    // Multiple signals should not panic
    coordinator.signal_shutdown();
    coordinator.signal_shutdown();
    coordinator.signal_shutdown();

    assert!(coordinator.is_shutting_down());
}

#[tokio::test]
async fn test_shutdown_subscribe() {
    let coordinator = DefaultShutdownCoordinator::new();
    let mut rx = coordinator.subscribe();

    // Spawn a task to signal shutdown
    let coord_clone = coordinator.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        coord_clone.signal_shutdown();
    });

    // Wait for signal
    let result = tokio::time::timeout(
        tokio::time::Duration::from_millis(100),
        rx.recv(),
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_wait_for_shutdown_timeout_signal() {
    let coordinator = DefaultShutdownCoordinator::new();

    // Spawn a task to signal shutdown
    let coord_clone = coordinator.clone();
    tokio::spawn(async move {
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        coord_clone.signal_shutdown();
    });

    let received = coordinator
        .wait_for_shutdown_timeout(tokio::time::Duration::from_millis(100))
        .await;

    assert!(received);
}

#[tokio::test]
async fn test_wait_for_shutdown_timeout_expires() {
    let coordinator = DefaultShutdownCoordinator::new();

    // No signal will be sent
    let received = coordinator
        .wait_for_shutdown_timeout(tokio::time::Duration::from_millis(10))
        .await;

    assert!(!received);
}

#[test]
fn test_shutdown_coordinator_clone() {
    let coord1 = DefaultShutdownCoordinator::new();
    let coord2 = coord1.clone();

    // Signal on one should affect the other
    coord1.signal_shutdown();
    assert!(coord2.is_shutting_down());
}
