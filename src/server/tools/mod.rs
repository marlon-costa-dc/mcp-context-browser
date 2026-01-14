//! MCP Tools Module
//!
//! Organizes tool registration and request routing for the MCP server.
//!
//! ## Organization
//!
//! - **registry.rs** - Tool definitions and schema management
//! - **router.rs** - Request routing and tool dispatch

pub mod registry;
pub mod router;

pub use registry::{create_tool_list, ToolDefinitions};
pub use router::{route_tool_call, ToolHandlers};
