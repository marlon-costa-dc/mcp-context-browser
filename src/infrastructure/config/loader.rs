//! # Configuration Loader
//!
//! Unified configuration loading from files, environment, and defaults.
//! Implements layered configuration with precedence rules.

use crate::domain::error::{Error, Result};
use config::{Config as ConfigBuilder, Environment, FileFormat};
use std::path::Path;
use validator::Validate;

use super::types::Config;

/// Embedded default configuration from config/default.toml
/// This is the single source of truth for default values in the binary.
/// Works from any working directory because it's compiled into the binary.
const DEFAULT_CONFIG_TOML: &str = include_str!("../../../config/default.toml");

/// Returns the embedded default config TOML for testing purposes
///
/// This function is exposed for external tests to verify configuration parsing.
pub fn get_default_config_toml() -> &'static str {
    DEFAULT_CONFIG_TOML
}

/// Load only embedded defaults without user config or environment variables.
/// Useful for testing that embedded defaults are correctly set.
///
/// This function is exposed for external tests to verify configuration defaults.
pub async fn load_embedded_defaults_only() -> Result<Config> {
    let config = ConfigBuilder::builder()
        .add_source(config::File::from_str(
            DEFAULT_CONFIG_TOML,
            FileFormat::Toml,
        ))
        .build()
        .map_err(|e| Error::config(format!("Failed to build configuration: {}", e)))?;

    let config: Config = config
        .try_deserialize()
        .map_err(|e| Error::config(format!("Failed to deserialize configuration: {}", e)))?;

    config
        .validate()
        .map_err(|e| Error::config(format!("Configuration validation failed: {}", e)))?;

    Ok(config)
}

/// Configuration loader for TOML-based application settings
///
/// Handles loading configuration from various sources including embedded defaults,
/// environment variables, and configuration files with support for hot-reload.
#[derive(Debug, Clone, Copy)]
pub struct ConfigLoader;

impl Default for ConfigLoader {
    /// Create a default configuration loader instance
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigLoader {
    /// Create a new configuration loader instance with default settings
    pub fn new() -> Self {
        Self
    }

    /// Load configuration from embedded defaults and environment variables
    ///
    /// Merges configuration sources in order:
    /// 1. Embedded TOML defaults (source of truth)
    /// 2. Environment variables (override defaults)
    ///
    /// # Returns
    /// Complete configuration struct or error if loading fails
    pub async fn load(&self) -> Result<Config> {
        // Start with embedded default config (source of truth for defaults)
        let mut builder = ConfigBuilder::builder().add_source(config::File::from_str(
            DEFAULT_CONFIG_TOML,
            FileFormat::Toml,
        ));

        // Layer 2: User configuration from XDG standard location (if exists)
        let config_dir = dirs::config_dir();
        if let Some(dir) = config_dir {
            let user_config_path = dir.join("mcp-context-browser").join("config.toml");
            if user_config_path.exists() {
                builder = builder.add_source(config::File::from(user_config_path).required(false));
            }
        }

        // Layer 3: Environment variables (highest priority)
        builder = builder.add_source(
            Environment::with_prefix("MCP")
                .separator("__")
                .try_parsing(true),
        );

        let config = builder
            .build()
            .map_err(|e| Error::config(format!("Failed to build configuration: {}", e)))?;

        let config: Config = config
            .try_deserialize()
            .map_err(|e| Error::config(format!("Failed to deserialize configuration: {}", e)))?;

        config
            .validate()
            .map_err(|e| Error::config(format!("Configuration validation failed: {}", e)))?;

        Ok(config)
    }

    /// Load configuration from a specific TOML file
    ///
    /// Merges configuration sources in order:
    /// 1. Embedded TOML defaults (source of truth)
    /// 2. Specified configuration file
    /// 3. Environment variables (highest priority)
    ///
    /// # Arguments
    /// * `path` - Path to the TOML configuration file
    ///
    /// Load configuration from embedded defaults, environment variables, and a specific file
    ///
    /// This method loads configuration in the following priority order:
    /// 1. Embedded default configuration
    /// 2. Specified configuration file (if exists)
    /// 3. Environment variables (highest priority)
    ///
    /// # Arguments
    /// * `path` - Path to the configuration file to load
    ///
    /// # Returns
    /// Complete configuration struct or error if loading fails
    pub async fn load_with_file(&self, path: &Path) -> Result<Config> {
        // Start with embedded default config
        let mut builder = ConfigBuilder::builder()
            .add_source(config::File::from_str(
                DEFAULT_CONFIG_TOML,
                FileFormat::Toml,
            ))
            // Override with specified file
            .add_source(config::File::from(path).required(false));

        // Environment variables still have highest priority
        builder = builder.add_source(
            Environment::with_prefix("MCP")
                .separator("__")
                .try_parsing(true),
        );

        let config = builder
            .build()
            .map_err(|e| Error::config(format!("Failed to build configuration: {}", e)))?;

        let config: Config = config
            .try_deserialize()
            .map_err(|e| Error::config(format!("Failed to deserialize configuration: {}", e)))?;

        config
            .validate()
            .map_err(|e| Error::config(format!("Configuration validation failed: {}", e)))?;

        Ok(config)
    }
}
