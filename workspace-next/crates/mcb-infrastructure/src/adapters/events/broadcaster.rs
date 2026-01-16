//! Tokio Broadcast Event Publisher
//!
//! Event publisher implementation using tokio broadcast channels for
//! in-process event distribution.

use async_trait::async_trait;
use mcb_domain::error::Result;
use mcb_domain::events::domain_events::{DomainEvent, EventPublisher};
use std::sync::Arc;
use tokio::sync::broadcast;

/// Default channel capacity
const DEFAULT_CAPACITY: usize = 100;

/// Event publisher using tokio broadcast channels
///
/// Provides in-process event distribution with multiple subscribers.
/// Events are broadcast to all active subscribers without persistence.
pub struct TokioEventPublisher {
    /// Broadcast sender for publishing events
    sender: broadcast::Sender<DomainEvent>,
}

impl TokioEventPublisher {
    /// Create a new tokio event publisher
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(DEFAULT_CAPACITY);
        Self { sender }
    }

    /// Create with custom capacity
    pub fn with_capacity(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    /// Get a subscriber receiver for listening to events
    pub fn subscribe(&self) -> broadcast::Receiver<DomainEvent> {
        self.sender.subscribe()
    }

    /// Create as Arc for sharing
    pub fn new_shared() -> Arc<Self> {
        Arc::new(Self::new())
    }
}

impl Default for TokioEventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl EventPublisher for TokioEventPublisher {
    async fn publish(&self, event: DomainEvent) -> Result<()> {
        // Broadcast to all subscribers, ignore errors if no subscribers
        let _ = self.sender.send(event);
        Ok(())
    }

    fn has_subscribers(&self) -> bool {
        self.sender.receiver_count() > 0
    }
}
