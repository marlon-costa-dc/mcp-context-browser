//! Unit tests organized by Clean Architecture layer
//!
//! Structure mirrors src/ directory:
//! - domain/ - Domain layer tests
//! - application/ - Application layer tests
//! - adapters/ - Adapter layer tests
//! - infrastructure/ - Infrastructure layer tests
//! - server/ - Server layer tests

mod adapters;
mod application;
mod domain;
mod infrastructure;
mod server;

// Cross-cutting tests at root level
mod property_based_test;
mod security_test;
mod unit_test;
