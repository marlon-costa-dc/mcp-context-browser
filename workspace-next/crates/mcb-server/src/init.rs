//! Server Initialization
//!
//! Handles server startup, dependency injection setup, and graceful shutdown.
//! Integrates with the infrastructure layer for configuration and DI container setup.

use std::path::Path;

use tracing::info;

use crate::transport::stdio::StdioServerExt;
use crate::{McpServerBuilder};

/// Run the MCP Context Browser server
///
/// This is the main entry point that initializes all components and starts the server.
/// It handles configuration loading, dependency injection, and MCP server startup.
pub async fn run_server(config_path: Option<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    let loader = match config_path {
        Some(path) => mcb_infrastructure::config::ConfigLoader::new().with_config_path(path),
        None => mcb_infrastructure::config::ConfigLoader::new(),
    };

    let config = loader.load()?;
    mcb_infrastructure::logging::init_logging(map_logging_config(&config.logging))?;

    info!("Starting MCP Context Browser server");

    let container = mcb_infrastructure::di::bootstrap::FullContainer::new(config).await?;

    let server = McpServerBuilder::new()
        .with_indexing_service(container.indexing_service())
        .with_context_service(container.context_service())
        .with_search_service(container.search_service())
        .build();

    info!("MCP server initialized successfully");
    server.serve_stdio().await
}

fn map_logging_config(
    config: &mcb_infrastructure::config::data::LoggingConfig,
) -> mcb_infrastructure::logging::LoggingConfig {
    mcb_infrastructure::logging::LoggingConfig {
        level: config.level.clone(),
        json_format: config.json_format,
        file_output: config.file_output.clone(),
        max_file_size: config.max_file_size,
        max_files: config.max_files,
    }
}
