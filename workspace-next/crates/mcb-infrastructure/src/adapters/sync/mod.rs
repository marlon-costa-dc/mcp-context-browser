//! Sync Infrastructure Adapters
//!
//! Concrete implementations of sync coordinator and provider interfaces.
//! Provides file sync coordination with debouncing and change detection.

mod coordinator;
mod null;
mod provider;

pub use coordinator::FileSyncCoordinator;
pub use null::{NullSyncCoordinator, NullSyncProvider};
pub use provider::DefaultSyncProvider;
