//! # MCP Server Layer
//!
//! Model Context Protocol (MCP) server implementation with HTTP transport.
//!
//! This layer provides:
//!
//! - [`McpServer`] - Main server struct handling MCP protocol
//! - [`McpServerBuilder`] - Fluent builder for server configuration
//! - [`handlers`] - MCP tool handlers (index_codebase, search_code, etc.)
//! - [`admin`] - Admin HTTP API for monitoring and management
//! - [`transport`] - HTTP transport implementation
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use mcp_context_browser::server::run_server;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Start server with default configuration
//!     run_server(None).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Builder Pattern
//!
//! For custom configuration, use the builder:
//!
//! ```rust,no_run
//! use mcp_context_browser::server::McpServerBuilder;
//!
//! async fn build_server() {
//!     // Build server with custom settings
//!     let server = McpServerBuilder::new()
//!         // .with_config(config)
//!         // .with_cache(cache_provider)
//!         .build()
//!         .await;
//! }
//! ```
//!
//! ## MCP Tools
//!
//! The server exposes these MCP tools:
//!
//! | Tool | Description |
//! |------|-------------|
//! | `index_codebase` | Index a codebase for semantic search |
//! | `search_code` | Semantic search across indexed code |
//! | `get_indexing_status` | Check indexing progress |
//! | `clear_index` | Remove indexed data |
//!
//! [`McpServer`]: mcp_server::McpServer
//! [`McpServerBuilder`]: builder::McpServerBuilder
//! [`handlers`]: handlers

// Module declarations
pub mod admin;
pub mod args;
pub mod auth;
pub mod builder;
pub mod formatter;
pub mod handlers;
pub mod init;
pub mod mcp_server;
pub mod metrics;
pub mod operations;
pub mod rate_limit_middleware;
pub mod security;
pub mod transport;

// Re-exports for public API
pub use args::*;
pub use auth::AuthHandler;
pub use builder::McpServerBuilder;
pub use formatter::ResponseFormatter;
pub use handlers::*;
pub use init::run_server;
pub use mcp_server::McpServer;
pub use metrics::{McpPerformanceMetrics, PerformanceMetricsInterface};
pub use operations::{IndexingOperation, IndexingOperationsInterface, McpIndexingOperations};
