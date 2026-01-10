//! MCP Context Browser - A semantic code search server

pub mod adapters;
pub mod admin;
pub mod application;
pub mod chunking;
pub mod daemon;
pub mod domain;
pub mod infrastructure;
pub mod server;
pub mod snapshot;
pub mod sync;

// Re-export core types for public API
pub use domain::error::{Error, Result};
pub use domain::types::*;

// Re-export main entry points
pub use server::builder::McpServerBuilder;
pub use server::init::run_server;
pub use server::mcp_server::McpServer;
