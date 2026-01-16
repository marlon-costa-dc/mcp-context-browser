//! Integration Tests for mcb-server
//!
//! This module aggregates all integration tests for the mcb-server crate.
//! Tests are organized by category:
//! - admin: Admin API HTTP endpoint tests
//! - tools: Tool registry and router tests
//! - e2e_flow: End-to-end workflow tests
//! - di_integration: Dependency injection container tests

mod admin;
mod di_integration_test;
mod e2e_flow_test;
mod tools;
