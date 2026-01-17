//! Infrastructure Adapters
//!
//! Concrete implementations of domain ports and services.
//! Following Clean Architecture: adapters implement domain interfaces.
//!
//! **ARCHITECTURE**:
//! - admin/    → Admin service implementations (metrics, indexing, shutdown)
//! - services/ → Domain service implementations (future)
//!
//! Provider implementations are in mcb-providers crate, NOT here.

pub mod admin;
