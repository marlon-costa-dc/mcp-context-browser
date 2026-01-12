//! Helper modules for AdminService implementation
//!
//! Splits the AdminService implementation into focused domains:
//! - `logging` - Log retrieval, filtering, export, and statistics
//! - `maintenance` - Cache clearing, provider restart, index rebuild, cleanup
//! - `health` - Health checks, connectivity tests, performance tests
//! - `backup` - Backup creation, listing, and restoration

pub mod backup;
pub mod health;
pub mod logging;
pub mod maintenance;
