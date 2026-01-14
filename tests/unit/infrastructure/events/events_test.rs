//! Tests for event bus module
//!
//! Migrated from src/infrastructure/events/mod.rs

use mcp_context_browser::infrastructure::events::EventBusConfig;
use serial_test::serial;

#[test]
fn test_event_bus_config_default() {
    let config = EventBusConfig::default();
    match config {
        EventBusConfig::Tokio { capacity } => assert_eq!(capacity, 100),
        _ => panic!("Expected Tokio config"),
    }
}

#[test]
#[serial]
fn test_event_bus_config_from_env_tokio() {
    // Clean environment before test
    std::env::remove_var("MCP_EVENT_BUS_TYPE");
    std::env::remove_var("MCP_NATS_URL");

    std::env::set_var("MCP_EVENT_BUS_TYPE", "tokio");
    let config = EventBusConfig::from_env();

    // Clean up after test
    std::env::remove_var("MCP_EVENT_BUS_TYPE");

    match config {
        EventBusConfig::Tokio { capacity } => assert!(capacity > 0),
        _ => panic!("Expected Tokio config"),
    }
}

#[test]
#[serial]
fn test_event_bus_config_from_env_nats() {
    // Clean environment before test
    std::env::remove_var("MCP_EVENT_BUS_TYPE");
    std::env::remove_var("MCP_NATS_URL");

    std::env::set_var("MCP_EVENT_BUS_TYPE", "nats");
    std::env::set_var("MCP_NATS_URL", "nats://test:4222");
    let config = EventBusConfig::from_env();

    // Clean up after test
    std::env::remove_var("MCP_EVENT_BUS_TYPE");
    std::env::remove_var("MCP_NATS_URL");

    match config {
        EventBusConfig::Nats { url, .. } => assert_eq!(url, "nats://test:4222"),
        _ => panic!("Expected NATS config"),
    }
}
