//! Tests for domain ports - event interfaces
//!
//! Migrated from src/domain/ports/events.rs inline tests.
//! Tests the EventPublisher trait and DomainEvent types.

use async_trait::async_trait;
use mcp_context_browser::domain::error::Result;
use mcp_context_browser::domain::ports::events::{DomainEvent, EventPublisher};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Mock event publisher for testing
struct MockEventPublisher {
    publish_count: AtomicUsize,
    has_subscribers: AtomicBool,
}

impl MockEventPublisher {
    fn new(has_subscribers: bool) -> Self {
        Self {
            publish_count: AtomicUsize::new(0),
            has_subscribers: AtomicBool::new(has_subscribers),
        }
    }

    fn get_publish_count(&self) -> usize {
        self.publish_count.load(Ordering::Relaxed)
    }
}

#[async_trait]
impl EventPublisher for MockEventPublisher {
    async fn publish(&self, _event: DomainEvent) -> Result<()> {
        self.publish_count.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    fn has_subscribers(&self) -> bool {
        self.has_subscribers.load(Ordering::Relaxed)
    }
}

#[tokio::test]
async fn test_event_publisher_publish() {
    let publisher = MockEventPublisher::new(true);

    let result = publisher
        .publish(DomainEvent::IndexRebuild { collection: None })
        .await;

    assert!(result.is_ok());
    assert_eq!(publisher.get_publish_count(), 1);
}

#[tokio::test]
async fn test_event_publisher_has_subscribers() {
    let publisher_with_subs = MockEventPublisher::new(true);
    let publisher_without_subs = MockEventPublisher::new(false);

    assert!(publisher_with_subs.has_subscribers());
    assert!(!publisher_without_subs.has_subscribers());
}

#[test]
fn test_domain_event_serialization() {
    let event = DomainEvent::SyncCompleted {
        path: "/test/path".to_string(),
        files_changed: 5,
    };

    let serialized = serde_json::to_string(&event).expect("Serialization failed");
    let deserialized: DomainEvent =
        serde_json::from_str(&serialized).expect("Deserialization failed");

    assert_eq!(event, deserialized);
}
