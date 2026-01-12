//! Event Bus System for Decoupled Communication
//!
//! Provides a publish/subscribe event system using tokio::sync::broadcast (default)
//! or NATS JetStream (when enabled) for decoupling components like AdminService from core logic.
//!
//! # Features
//!
//! - Abstract EventBusProvider trait for multiple backends
//! - Default tokio::sync::broadcast implementation (in-process, no persistence)
//! - Optional NATS backend (cross-process, persistent, at-least-once delivery)
//! - Zero-downtime migration between backends via feature flags
//! - Backward compatible (defaults to tokio if NATS not enabled)

use std::sync::Arc;
use tokio::sync::broadcast::{self, Receiver, Sender};

/// System-wide events for internal communication
#[derive(Debug, Clone)]
pub enum SystemEvent {
    /// Request to clear all caches
    CacheClear {
        /// Optional namespace to clear (None = all)
        namespace: Option<String>,
    },
    /// Request to create a backup
    BackupCreate {
        /// Target path for backup
        path: String,
    },
    /// Request to restore a backup
    BackupRestore {
        /// Path to backup to restore
        path: String,
    },
    /// Request to rebuild the index
    IndexRebuild {
        /// Collection to rebuild (None = all)
        collection: Option<String>,
    },
    /// Request to clear an index
    IndexClear {
        /// Collection to clear (None = all)
        collection: Option<String>,
    },
    /// Request to optimize an index
    IndexOptimize {
        /// Collection to optimize (None = all)
        collection: Option<String>,
    },
    /// Configuration was reloaded
    ConfigReloaded,
    /// Configuration has been changed by an administrator
    ConfigurationChanged {
        /// User who made the change
        user: String,
        /// List of changes that were applied
        changes: Vec<String>,
        /// Whether restart is required to apply all changes
        requires_restart: bool,
        /// When the change was made
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    /// Server is shutting down
    Shutdown,
    /// Request to reload configuration (SIGHUP)
    Reload,
    /// Request to respawn the server binary (SIGUSR1)
    Respawn,
    /// Binary file was updated, prepare for respawn
    BinaryUpdated {
        /// New binary path
        path: String,
    },
    /// Sync operation completed
    SyncCompleted {
        /// Path that was synced
        path: String,
        /// Number of files that changed
        files_changed: i32,
    },

    // === Subsystem Control Events (ADR-007) ===
    /// Request to restart a provider
    ProviderRestart {
        /// Type of provider (embedding, vector_store, etc.)
        provider_type: String,
        /// Unique identifier for the provider instance
        provider_id: String,
    },
    /// Request to reconfigure a provider without full restart
    ProviderReconfigure {
        /// Type of provider (embedding, vector_store, etc.)
        provider_type: String,
        /// New configuration to apply
        config: serde_json::Value,
    },
    /// Request a health check on a specific subsystem
    SubsystemHealthCheck {
        /// Subsystem identifier to check
        subsystem_id: String,
    },
    /// Request to reload router configuration
    RouterReload,

    // === Recovery Events ===
    /// Provider has been successfully restarted
    ProviderRestarted {
        /// Type of provider (embedding, vector_store, etc.)
        provider_type: String,
        /// Unique identifier for the provider instance
        provider_id: String,
    },
    /// Recovery process has started for a subsystem
    RecoveryStarted {
        /// Subsystem identifier being recovered
        subsystem_id: String,
        /// Current retry attempt number
        retry_attempt: u32,
    },
    /// Recovery process completed for a subsystem
    RecoveryCompleted {
        /// Subsystem identifier
        subsystem_id: String,
        /// Whether the recovery was successful
        success: bool,
        /// Message describing the outcome
        message: String,
    },
    /// Recovery exhausted all retries for a subsystem
    RecoveryExhausted {
        /// Subsystem identifier
        subsystem_id: String,
        /// Total retries attempted
        total_retries: u32,
        /// Last error message if any
        last_error: Option<String>,
    },
}

/// Abstract trait for event receiver implementations
#[async_trait::async_trait]
pub trait EventReceiver: Send {
    /// Receive the next event from the event bus
    async fn recv(&mut self) -> crate::domain::error::Result<SystemEvent>;
}

/// Abstract trait for event bus provider implementations
///
/// Allows pluggable backends (tokio broadcast, NATS, etc.)
#[async_trait::async_trait]
pub trait EventBusProvider: Send + Sync {
    /// Publish an event to all subscribers
    ///
    /// Returns the number of subscribers that received the event
    async fn publish(&self, event: SystemEvent) -> crate::domain::error::Result<usize>;

    /// Subscribe to receive events
    async fn subscribe(&self) -> crate::domain::error::Result<Box<dyn EventReceiver>>;

    /// Get the number of active subscribers
    fn subscriber_count(&self) -> usize;
}

/// Tokio broadcast-based event receiver
pub struct TokioEventReceiver {
    receiver: Receiver<SystemEvent>,
}

#[async_trait::async_trait]
impl EventReceiver for TokioEventReceiver {
    async fn recv(&mut self) -> crate::domain::error::Result<SystemEvent> {
        self.receiver.recv().await.map_err(|e| {
            crate::domain::error::Error::Internal {
                message: format!("Event bus receiver error: {}", e),
            }
        })
    }
}

/// Event Bus for publishing and subscribing to system events
#[derive(Clone)]
pub struct EventBus {
    sender: Sender<SystemEvent>,
}

impl EventBus {
    /// Create a new EventBus with specified channel capacity
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    /// Create a new EventBus with default capacity (100)
    pub fn with_default_capacity() -> Self {
        Self::new(100)
    }

    /// Publish an event to all subscribers (synchronous wrapper for compatibility)
    pub fn publish(
        &self,
        event: SystemEvent,
    ) -> Result<usize, broadcast::error::SendError<SystemEvent>> {
        self.sender.send(event)
    }

    /// Subscribe to receive events (synchronous wrapper for compatibility)
    pub fn subscribe(&self) -> Receiver<SystemEvent> {
        self.sender.subscribe()
    }

    /// Get the number of active subscribers
    pub fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::with_default_capacity()
    }
}

#[async_trait::async_trait]
impl EventBusProvider for EventBus {
    async fn publish(&self, event: SystemEvent) -> crate::domain::error::Result<usize> {
        self.sender.send(event).map_err(|e| {
            crate::domain::error::Error::Internal {
                message: format!("Failed to publish event: {}", e),
            }
        })
    }

    async fn subscribe(&self) -> crate::domain::error::Result<Box<dyn EventReceiver>> {
        Ok(Box::new(TokioEventReceiver {
            receiver: self.sender.subscribe(),
        }))
    }

    fn subscriber_count(&self) -> usize {
        self.sender.receiver_count()
    }
}

/// Shared EventBus wrapped in Arc for thread-safe sharing
pub type SharedEventBus = Arc<EventBus>;

/// Shared trait object for event bus provider
pub type SharedEventBusProvider = Arc<dyn EventBusProvider>;

/// Create a shared EventBus
pub fn create_shared_event_bus() -> SharedEventBus {
    Arc::new(EventBus::default())
}

/// Create a shared EventBusProvider (tokio implementation)
pub fn create_shared_event_bus_provider() -> SharedEventBusProvider {
    Arc::new(EventBus::default()) as SharedEventBusProvider
}

// ============================================================================
// NATS JetStream Event Bus Provider
// ============================================================================

use async_nats::jetstream;

const NATS_STREAM_NAME: &str = "MCP_EVENTS";
const NATS_SUBJECT: &str = "mcp.events.>";
const NATS_CONSUMER_DURABLE: &str = "mcp-consumer";

/// NATS JetStream-based event receiver
pub struct NatsEventReceiver {
    subscription: jetstream::consumer::pull::Stream<jetstream::Message>,
}

#[async_trait::async_trait]
impl EventReceiver for NatsEventReceiver {
    async fn recv(&mut self) -> crate::domain::error::Result<SystemEvent> {
        use std::any::type_name_of_val;
        use tracing::debug;

        match self.subscription.next().await {
            Some(msg) => {
                match serde_json::from_slice::<SystemEvent>(&msg.payload) {
                    Ok(event) => {
                        debug!("Received event from NATS: {:?}", type_name_of_val(&event));
                        let _ = msg.ack().await;
                        Ok(event)
                    }
                    Err(e) => {
                        tracing::error!("Failed to deserialize NATS event: {}", e);
                        Err(crate::domain::error::Error::Internal {
                            message: format!("Failed to deserialize NATS event: {}", e),
                        })
                    }
                }
            }
            None => {
                tracing::error!("NATS subscription closed");
                Err(crate::domain::error::Error::Internal {
                    message: "NATS subscription closed unexpectedly".to_string(),
                })
            }
        }
    }
}

/// NATS JetStream-based Event Bus Provider
///
/// Provides persistent event distribution with at-least-once delivery semantics.
/// Events are retained for 1 hour or up to 10,000 messages, whichever comes first.
pub struct NatsEventBus {
    client: async_nats::Client,
    jetstream: jetstream::Context,
    stream_name: String,
}

impl NatsEventBus {
    /// Create a new NATS event bus, connecting to the specified server
    pub async fn new(server_url: &str) -> crate::domain::error::Result<Self> {
        use tracing::debug;

        debug!("Connecting to NATS server: {}", server_url);

        // Connect to NATS
        let client = async_nats::connect(server_url).await.map_err(|e| {
            crate::domain::error::Error::Internal {
                message: format!("Failed to connect to NATS: {}", e),
            }
        })?;

        debug!("Connected to NATS, creating JetStream context");

        // Create JetStream context
        let jetstream_ctx = jetstream::new(client.clone());

        // Ensure stream exists
        Self::ensure_stream_exists(&jetstream_ctx).await?;

        debug!("NATS event bus initialized successfully");

        Ok(Self {
            client,
            jetstream: jetstream_ctx,
            stream_name: NATS_STREAM_NAME.to_string(),
        })
    }

    /// Create the JetStream stream if it doesn't exist
    async fn ensure_stream_exists(jetstream_ctx: &jetstream::Context) -> crate::domain::error::Result<()> {
        use tracing::{debug, error};
        use std::time::Duration;

        match jetstream_ctx
            .get_or_create_stream(jetstream::stream::Config {
                name: NATS_STREAM_NAME.to_string(),
                subjects: vec![NATS_SUBJECT.to_string()],
                retention: jetstream::stream::RetentionPolicy::Limits,
                max_msgs: 10000,
                max_age: Duration::from_secs(3600),
                discard: jetstream::stream::DiscardPolicy::OldestPerSubject,
                ..Default::default()
            })
            .await
        {
            Ok(_) => {
                debug!("JetStream stream '{}' ready", NATS_STREAM_NAME);
                Ok(())
            }
            Err(e) => {
                error!("Failed to create JetStream stream: {}", e);
                Err(crate::domain::error::Error::Internal {
                    message: format!("Failed to create JetStream stream: {}", e),
                })
            }
        }
    }
}

#[async_trait::async_trait]
impl EventBusProvider for NatsEventBus {
    async fn publish(&self, event: SystemEvent) -> crate::domain::error::Result<usize> {
        use tracing::debug;

        // Serialize the event
        let payload = serde_json::to_vec(&event).map_err(|e| crate::domain::error::Error::Internal {
            message: format!("Failed to serialize event: {}", e),
        })?;

        // Publish to NATS JetStream
        match self
            .jetstream
            .publish(NATS_SUBJECT.to_string(), payload.into())
            .await
        {
            Ok(ack) => {
                debug!("Published event to NATS (sequence: {})", ack.sequence);
                Ok(1)
            }
            Err(e) => {
                tracing::error!("Failed to publish event to NATS: {}", e);
                Err(crate::domain::error::Error::Internal {
                    message: format!("Failed to publish event: {}", e),
                })
            }
        }
    }

    async fn subscribe(&self) -> crate::domain::error::Result<Box<dyn EventReceiver>> {
        use tracing::debug;
        use std::time::Duration;

        debug!("Creating NATS JetStream subscription");

        // Create or get durable consumer
        let consumer = self
            .jetstream
            .get_or_create_consumer(
                &self.stream_name,
                jetstream::consumer::pull::Config {
                    durable_name: Some(NATS_CONSUMER_DURABLE.to_string()),
                    deliver_policy: jetstream::consumer::DeliverPolicy::All,
                    ack_policy: jetstream::consumer::AckPolicy::Explicit,
                    ack_wait: Duration::from_secs(30),
                    max_deliver: 10,
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| crate::domain::error::Error::Internal {
                message: format!("Failed to create NATS consumer: {}", e),
            })?;

        // Start pulling messages
        let subscription = consumer
            .stream()
            .await
            .map_err(|e| crate::domain::error::Error::Internal {
                message: format!("Failed to create NATS subscription: {}", e),
            })?;

        debug!("NATS subscription created successfully");

        Ok(Box::new(NatsEventReceiver { subscription }))
    }

    fn subscriber_count(&self) -> usize {
        0 // NATS doesn't provide an easy way to get subscriber count
    }
}

// ============================================================================
// Event Bus Configuration and Factory
// ============================================================================

/// Configuration for event bus selection and behavior
#[derive(Debug, Clone)]
pub enum EventBusConfig {
    /// Use in-process tokio broadcast (default)
    Tokio { capacity: usize },
    /// Use NATS JetStream for cross-process communication
    Nats {
        url: String,
        retention_hours: u64,
        max_msgs_per_subject: i64,
    },
}

impl Default for EventBusConfig {
    fn default() -> Self {
        EventBusConfig::Tokio { capacity: 100 }
    }
}

impl EventBusConfig {
    /// Create from environment variables
    ///
    /// Respects:
    /// - `MCP_EVENT_BUS_TYPE` - "tokio" or "nats" (default: "tokio")
    /// - `MCP_NATS_URL` - NATS server URL (default: "nats://localhost:4222")
    /// - `MCP_NATS_RETENTION_HOURS` - Event retention (default: 1)
    /// - `MCP_EVENT_BUS_CAPACITY` - Tokio channel capacity (default: 100)
    pub fn from_env() -> Self {
        let bus_type = std::env::var("MCP_EVENT_BUS_TYPE").unwrap_or_else(|_| "tokio".to_string());

        match bus_type.as_str() {
            "nats" => {
                let url = std::env::var("MCP_NATS_URL")
                    .unwrap_or_else(|_| "nats://localhost:4222".to_string());
                let retention_hours = std::env::var("MCP_NATS_RETENTION_HOURS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(1);
                let max_msgs = std::env::var("MCP_NATS_MAX_MSGS")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(10000);

                EventBusConfig::Nats {
                    url,
                    retention_hours,
                    max_msgs_per_subject: max_msgs,
                }
            }
            _ => {
                let capacity = std::env::var("MCP_EVENT_BUS_CAPACITY")
                    .ok()
                    .and_then(|v| v.parse().ok())
                    .unwrap_or(100);

                EventBusConfig::Tokio { capacity }
            }
        }
    }
}

/// Create an event bus provider from configuration
pub async fn create_event_bus(config: &EventBusConfig) -> crate::domain::error::Result<SharedEventBusProvider> {
    match config {
        EventBusConfig::Tokio { capacity } => {
            tracing::info!("[EVENT_BUS] Using tokio broadcast backend (capacity: {})", capacity);
            Ok(Arc::new(EventBus::new(*capacity)) as SharedEventBusProvider)
        }
        EventBusConfig::Nats { url, .. } => {
            tracing::info!("[EVENT_BUS] Using NATS backend (url: {})", url);
            let bus = NatsEventBus::new(url).await?;
            Ok(Arc::new(bus) as SharedEventBusProvider)
        }
    }
}
