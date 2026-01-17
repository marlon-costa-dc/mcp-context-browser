//! Tests for ConfigPropagator
//!
//! Tests configuration propagation and callback functionality.

use mcb_server::admin::propagation::{ConfigPropagator, PropagatorHandle};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

#[tokio::test]
async fn test_config_propagator_creation() {
    let propagator = ConfigPropagator::new();
    assert!(propagator.callbacks.is_empty());
}

#[tokio::test]
async fn test_config_propagator_with_callback() {
    let call_count = Arc::new(AtomicUsize::new(0));
    let call_count_clone = Arc::clone(&call_count);

    let propagator = ConfigPropagator::new().on_config_change(Box::new(move |_config| {
        call_count_clone.fetch_add(1, Ordering::SeqCst);
    }));

    assert_eq!(propagator.callbacks.len(), 1);
}

#[test]
fn test_propagator_handle_is_running() {
    // Test that the handle properly tracks task state
    let handle = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            let handle = tokio::spawn(async {
                // Simulate some work
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            });
            PropagatorHandle { handle }
        });

    // The task should either be running or have completed
    // We just verify the method doesn't panic
    let _ = handle.is_running();
}
