//! Tests for Tokio-based event bus
//!
//! Migrated from src/infrastructure/events/tokio_impl.rs

use mcp_context_browser::infrastructure::events::{EventBus, EventBusProvider, SystemEvent};
use std::sync::Arc;

#[tokio::test]
async fn test_event_bus_publish_subscribe() {
    let bus = EventBus::new(10);
    let mut receiver = bus.subscribe_sync();

    // Send event directly without unnecessary clone
    let _ = bus.publish_sync(SystemEvent::Reload);

    let received = receiver.recv().await.unwrap();
    assert!(matches!(received, SystemEvent::Reload));
}

#[tokio::test]
async fn test_event_bus_provider_trait() {
    let bus: Arc<dyn EventBusProvider> = Arc::new(EventBus::new(10));

    // Subscribe BEFORE publishing to avoid race condition
    let mut receiver = bus.subscribe().await.unwrap();

    let event = SystemEvent::Shutdown;
    let _ = bus.publish(event).await;

    let received = receiver.recv().await.unwrap();
    assert!(matches!(received, SystemEvent::Shutdown));
}
