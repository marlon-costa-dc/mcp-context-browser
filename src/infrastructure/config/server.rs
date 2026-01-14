//! # Server Configuration
//!
//! HTTP server and MCP protocol configuration.
//! Defines listen addresses, TLS settings, and request handling parameters.

use serde::{Deserialize, Serialize};
use validator::Validate;

/// HTTP server configuration
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ServerConfig {
    /// Host address to bind the server to
    #[validate(length(min = 1))]
    pub host: String,
    /// Port number to listen on
    #[validate(range(min = 1))]
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
        }
    }
}
