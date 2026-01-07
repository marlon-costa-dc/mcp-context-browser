//! Tool argument types for MCP server
//!
//! This module contains all the argument types used by the MCP tools.
//! These are extracted to improve code organization and maintainability.

use serde::Deserialize;
use schemars::JsonSchema;

/// Arguments for the index_codebase tool
#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Parameters for indexing a codebase directory")]
pub struct IndexCodebaseArgs {
    /// Path to the codebase directory to index
    #[schemars(
        description = "Absolute or relative path to the directory containing code to index"
    )]
    pub path: String,
    /// Optional JWT token for authentication
    #[schemars(description = "JWT token for authenticated requests")]
    pub token: Option<String>,
}

/// Arguments for the search_code tool
#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Parameters for searching code using natural language")]
pub struct SearchCodeArgs {
    /// Natural language query to search for
    #[schemars(
        description = "The search query in natural language (e.g., 'find functions that handle authentication')"
    )]
    pub query: String,
    /// Maximum number of results to return (default: 10)
    #[schemars(description = "Maximum number of search results to return")]
    #[serde(default = "default_limit")]
    pub limit: usize,
    /// Optional JWT token for authentication
    #[schemars(description = "JWT token for authenticated requests")]
    pub token: Option<String>,
}

/// Arguments for getting indexing status
#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Parameters for checking indexing status")]
pub struct GetIndexingStatusArgs {
    /// Collection name (default: 'default')
    #[schemars(description = "Name of the collection to check status for")]
    #[serde(default = "default_collection")]
    pub collection: String,
}

/// Arguments for clearing an index
#[derive(Debug, Deserialize, JsonSchema)]
#[schemars(description = "Parameters for clearing an index")]
pub struct ClearIndexArgs {
    /// Collection name to clear (default: 'default')
    #[schemars(description = "Name of the collection to clear")]
    #[serde(default = "default_collection")]
    pub collection: String,
}

fn default_limit() -> usize {
    10
}

fn default_collection() -> String {
    "default".to_string()
}