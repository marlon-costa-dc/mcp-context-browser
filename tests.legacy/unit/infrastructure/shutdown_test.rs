//! Shutdown coordinator unit tests

use mcp_context_browser::infrastructure::shutdown::ShutdownCoordinator;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_coordinator_basic() {
    let coordinator = ShutdownCoordinator::new();
    assert!(!coordinator.is_shutting_down());
    assert_eq!(coordinator.active_tasks(), 0);
}

#[tokio::test]
async fn test_spawn_and_track() {
    let coordinator = ShutdownCoordinator::new();
    let counter = Arc::new(AtomicUsize::new(0));

    let counter_clone = counter.clone();
    coordinator.spawn("test_task", async move {
        counter_clone.fetch_add(1, Ordering::SeqCst);
    });

    // Wait briefly for task to complete
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert_eq!(counter.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_cancellable_task() {
    let coordinator = ShutdownCoordinator::new();
    let completed = Arc::new(AtomicUsize::new(0));

    let completed_clone = completed.clone();
    coordinator.spawn_cancellable("cancellable", |token| async move {
        loop {
            tokio::select! {
                _ = token.cancelled() => {
                    completed_clone.fetch_add(1, Ordering::SeqCst);
                    break;
                }
                _ = tokio::time::sleep(Duration::from_secs(10)) => {}
            }
        }
    });

    // Give task time to start
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert_eq!(completed.load(Ordering::SeqCst), 0);

    // Shutdown should trigger cancellation
    let success = coordinator.shutdown(Duration::from_secs(1)).await;
    assert!(success);
    assert_eq!(completed.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_child_token_hierarchy() {
    let coordinator = ShutdownCoordinator::new();
    let child = coordinator.child_token();

    assert!(!child.is_cancelled());
    coordinator.shutdown(Duration::from_millis(10)).await;
    assert!(child.is_cancelled());
}

#[tokio::test]
async fn test_graceful_shutdown() {
    let coordinator = ShutdownCoordinator::new();

    // Spawn a task that completes quickly
    coordinator.spawn("quick", async {
        tokio::time::sleep(Duration::from_millis(10)).await;
    });

    // Shutdown should complete successfully
    let success = coordinator.shutdown(Duration::from_secs(1)).await;
    assert!(success);
    assert!(coordinator.is_shutting_down());
}
