//! Domain tests
//!
//! Tests for domain layer: types, validation, error handling, chunking.

// Core types
#[path = "domain/core_types.rs"]
mod core_types;

// Error handling
#[path = "domain/error_handling.rs"]
mod error_handling;

// Validation
#[path = "domain/validation_tests.rs"]
mod validation_tests;

// Chunking
#[path = "domain/chunking/chunking.rs"]
mod chunking;
