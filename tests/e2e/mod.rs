//! End-to-End Integration Tests
//!
//! Full system integration tests including MCP protocol,
//! Docker containers, and external service integration.

mod admin_e2e_test;
mod docker_test;
mod integration_docker_test;
mod integration_logic_test;
mod mcp_e2e_test;
mod mcp_full_integration_test;
mod mcp_protocol_test;
// mod nats_event_bus_test; // Disabled: API incompatibility with SystemEvent variants
mod ollama_test;
