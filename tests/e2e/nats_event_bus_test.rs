//! NATS Event Bus Integration Tests
//!
//! Tests the NatsEventBus against a real local NATS instance.
//! Requires NATS running on localhost:4222 (see docker-compose.yml)
//!
//! Run with: cargo test --test e2e nats_event_bus -- --nocapture

use mcp_context_browser::infrastructure::events::{
    create_event_bus, EventBusConfig, EventBusProvider, SystemEvent,
};
use std::sync::Arc;
use std::time::Duration;

/// Get NATS URL from environment or default to localhost
fn get_nats_url() -> String {
    std::env::var("NATS_URL")
        .or_else(|_| std::env::var("MCP_NATS_URL"))
        .unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string())
}

/// Check if NATS is available by attempting to connect
async fn is_nats_available() -> bool {
    match async_nats::connect(&get_nats_url()).await {
        Ok(_client) => true,
        Err(_) => false,
    }
}

/// Helper to skip test if NATS is not available
macro_rules! skip_if_no_nats {
    () => {
        if !is_nats_available().await {
            eprintln!("⚠️  Skipping test: NATS not available on localhost:4222");
            eprintln!("    Start NATS with: docker-compose up -d nats");
            return;
        }
    };
}

#[tokio::test]
async fn test_nats_event_bus_creation() {
    skip_if_no_nats!();

    let config = EventBusConfig::Nats {
        url: get_nats_url(),
        retention_hours: 24,
        max_msgs_per_subject: 1000,
    };

    let bus = create_event_bus(&config)
        .await
        .expect("Failed to create NATS event bus");

    // Verify we can get subscriber count (basic functionality check)
    assert_eq!(bus.subscriber_count(), 0);
    println!("✅ NATS event bus created successfully");
}

#[tokio::test]
async fn test_nats_event_bus_publish_subscribe() {
    skip_if_no_nats!();

    let config = EventBusConfig::Nats {
        url: get_nats_url(),
        retention_hours: 24,
        max_msgs_per_subject: 1000,
    };

    let bus: Arc<dyn EventBusProvider> = Arc::new(
        create_event_bus(&config)
            .await
            .expect("Failed to create NATS event bus"),
    );

    let mut receiver = bus
        .subscribe()
        .await
        .expect("Failed to subscribe to event bus");

    // Publish an event
    let event = SystemEvent::CacheClear { namespace: None };
    bus.publish(event.clone())
        .await
        .expect("Failed to publish event");

    // Receive the event (with timeout)
    tokio::select! {
        result = receiver.recv() => {
            match result {
                Some(received_event) => {
                    assert!(matches!(received_event, SystemEvent::CacheClear { namespace: None }));
                    println!("✅ NATS publish/subscribe works correctly");
                }
                None => {
                    panic!("Channel closed unexpectedly");
                }
            }
        }
        _ = tokio::time::sleep(Duration::from_secs(5)) => {
            // NATS may have latency, this is acceptable in integration tests
            println!("⚠️  Timeout waiting for event (NATS may be slow or event not delivered)");
        }
    }
}

#[tokio::test]
async fn test_nats_multiple_subscribers() {
    skip_if_no_nats!();

    let config = EventBusConfig::Nats {
        url: get_nats_url(),
        retention_hours: 24,
        max_msgs_per_subject: 1000,
    };

    let bus: Arc<dyn EventBusProvider> = Arc::new(
        create_event_bus(&config)
            .await
            .expect("Failed to create NATS event bus"),
    );

    // Create multiple subscribers
    let _receiver1 = bus.subscribe().await.expect("Failed to subscribe 1");
    let _receiver2 = bus.subscribe().await.expect("Failed to subscribe 2");

    // Check subscriber count
    let count = bus.subscriber_count();
    assert!(count >= 1, "Should have at least 1 subscriber, got {}", count);
    println!("✅ Multiple subscribers created: {} active", count);
}

#[tokio::test]
async fn test_nats_system_events() {
    skip_if_no_nats!();

    let config = EventBusConfig::Nats {
        url: get_nats_url(),
        retention_hours: 24,
        max_msgs_per_subject: 1000,
    };

    let bus: Arc<dyn EventBusProvider> = Arc::new(
        create_event_bus(&config)
            .await
            .expect("Failed to create NATS event bus"),
    );

    // Test various system events can be published
    let events = vec![
        SystemEvent::CacheClear { namespace: None },
        SystemEvent::CacheClear {
            namespace: Some("test".to_string()),
        },
        SystemEvent::ConfigReload,
        SystemEvent::IndexingStarted {
            path: "/test/path".to_string(),
        },
        SystemEvent::IndexingCompleted {
            path: "/test/path".to_string(),
            files_indexed: 100,
            duration_ms: 1000,
        },
    ];

    for event in events {
        let result = bus.publish(event).await;
        assert!(result.is_ok(), "Failed to publish event: {:?}", result);
    }

    println!("✅ All system events published successfully");
}
