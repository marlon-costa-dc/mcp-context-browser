//! MCP Transport Layer
//!
//! Transport implementations for the MCP protocol.
//! Handles different transport mechanisms (stdio, HTTP, etc.).
//!
//! ## Available Transports
//!
//! | Transport | Description | Use Case |
//! |-----------|-------------|----------|
//! | [`stdio`] | Standard I/O streams | CLI tools, IDE integrations |
//! | [`http`] | HTTP with SSE | Web clients, REST APIs |
//!
//! ## Usage
//!
//! ```rust,ignore
//! use mcb_server::transport::{TransportConfig, TransportMode};
//! use mcb_server::McpServer;
//!
//! let server = McpServer::new(/* ... */);
//!
//! // Stdio transport (traditional MCP)
//! server.serve_stdio().await?;
//!
//! // HTTP transport (for web clients)
//! let http = HttpTransport::new(config, Arc::new(server));
//! http.start().await?;
//! ```

pub mod config;
pub mod http;
pub mod stdio;
pub mod types;

// Re-export transport types
pub use config::TransportConfig;
pub use http::{HttpTransport, HttpTransportConfig};
pub use stdio::StdioServerExt;
pub use types::{McpError, McpRequest, McpResponse};

// Re-export TransportMode from infrastructure config (single source of truth)
pub use mcb_infrastructure::config::TransportMode;
