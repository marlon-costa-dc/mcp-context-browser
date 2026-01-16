//! Snapshot Infrastructure Adapters
//!
//! Concrete implementations of snapshot provider interfaces.
//! Provides filesystem-based snapshot storage with content hashing.

mod filesystem;
mod null;

pub use filesystem::FilesystemSnapshotProvider;
pub use null::NullSnapshotProvider;
