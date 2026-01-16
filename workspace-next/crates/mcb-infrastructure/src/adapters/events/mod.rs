//! Event Infrastructure Adapters
//!
//! Concrete implementations of domain event publishing interfaces.
//! Provides multiple event backends: in-process broadcast channel
//! and null provider for testing.

mod broadcaster;
mod null;

pub use broadcaster::TokioEventPublisher;
pub use null::NullEventPublisher;
