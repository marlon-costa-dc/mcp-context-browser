//! DI Container Bootstrap - Shaku Strict Pattern
//!
//! Provides the composition root for the Shaku-based dependency injection system.
//! This follows the strict Shaku hierarchical module pattern with no manual wiring.
//!
//! ## Shaku Strict Architecture
//!
//! - **Module Hierarchy**: Uses `module!` macro with `use dyn ModuleTrait` for composition
//! - **No Manual Wiring**: All dependencies resolved through module interfaces
//! - **Provider Overrides**: Runtime component overrides for production configuration
//! - **Trait-based DI**: All dependencies injected as `Arc<dyn Trait>`
//!
//! ## Construction Pattern
//!
//! ```rust,ignore
//! // 1. Build leaf modules (no dependencies)
//! let infrastructure = Arc::new(InfrastructureModuleImpl::builder().build());
//! let server = Arc::new(ServerModuleImpl::builder().build());
//! let adapters = Arc::new(AdaptersModuleImpl::builder().build());
//!
//! // 2. Build application module (depends on adapters)
//! let application = Arc::new(ApplicationModuleImpl::builder(adapters.clone()).build());
//!
//! // 3. Build admin module (depends on all)
//! let admin = Arc::new(AdminModuleImpl::builder(
//!     infrastructure.clone(),
//!     server.clone(),
//!     adapters.clone(),
//!     application.clone()
//! ).build());
//!
//! // 4. Build root container
//! let container = McpModule::builder(infrastructure, server, adapters, application, admin).build();
//! ```

use crate::config::AppConfig;
use mcb_domain::error::Result;
use std::sync::Arc;
use tracing::info;

// Import all module implementations and traits
use super::modules::{
    AdaptersModuleImpl, AdminModuleImpl, ApplicationModuleImpl, InfrastructureModuleImpl,
    McpModule, ServerModuleImpl,
    traits::{AdaptersModule, AdminModule, ApplicationModule, InfrastructureModule, ServerModule},
};

// Import factories for provider overrides (production configuration)
use super::factory::{EmbeddingProviderFactory, VectorStoreProviderFactory};

/// Shaku-based DI Container following strict hierarchical pattern.
///
/// This container holds the root McpModule and provides access to all services
/// through the module resolution system. No manual component management.
///
/// ## Usage
///
/// ```rust,ignore
/// // Create container with production config
/// let container = DiContainer::build_with_config(config, http_client).await?;
///
/// // Resolve any service through trait-based access
/// let search_service: Arc<dyn SearchServiceInterface> = container.resolve();
/// let embedding_provider: Arc<dyn EmbeddingProvider> = container.resolve();
/// ```
pub type DiContainer = McpModule;

/// Provider configuration overrides for production setup.
///
/// This struct provides methods to create configured providers
/// that can be injected into the module hierarchy at runtime.
pub struct ProviderOverrides;

impl ProviderOverrides {
    /// Create embedding provider from configuration
    pub fn create_embedding_provider(config: &AppConfig) -> Result<Box<dyn mcb_domain::ports::providers::EmbeddingProvider>> {
        if let Some((name, embedding_config)) = config.providers.embedding.iter().next() {
            info!(provider = name, "Creating embedding provider from config");
            EmbeddingProviderFactory::create(embedding_config, None)
        } else {
            info!("No embedding provider configured, using null provider");
            Ok(EmbeddingProviderFactory::create_null())
        }
    }

    /// Create vector store provider from configuration
    pub fn create_vector_store_provider(
        config: &AppConfig,
        crypto: &CryptoService,
    ) -> Result<Box<dyn mcb_domain::ports::providers::VectorStoreProvider>> {
        if let Some((name, vector_config)) = config.providers.vector_store.iter().next() {
            info!(
                provider = name,
                "Creating vector store provider from config"
            );
            // Wrap CryptoService as Arc<dyn CryptoProvider> for DI
            let crypto_provider: Arc<dyn mcb_domain::ports::providers::CryptoProvider> = Arc::new(crypto.clone());
            VectorStoreProviderFactory::create(vector_config, Some(crypto_provider))
        } else {
            info!("No vector store provider configured, using in-memory provider");
            Ok(VectorStoreProviderFactory::create_in_memory())
        }
    }
}

/// Container builder for Shaku-based DI system.
///
/// Builds the hierarchical module structure following the strict Shaku pattern.
/// Provides both testing (null providers) and production (configured providers) setups.
pub struct DiContainerBuilder {
    config: Option<AppConfig>,
    embedding_override: Option<Box<dyn mcb_domain::ports::providers::EmbeddingProvider>>,
    vector_store_override: Option<Box<dyn mcb_domain::ports::providers::VectorStoreProvider>>,
}

impl DiContainerBuilder {
    /// Create a new container builder for testing (null providers)
    pub fn new() -> Self {
        Self {
            config: None,
            embedding_override: None,
            vector_store_override: None,
        }
    }

    /// Create a container builder with production configuration
    pub fn with_config(config: AppConfig) -> Self {
        Self {
            config: Some(config),
            embedding_override: None,
            vector_store_override: None,
        }
    }

    /// Override the embedding provider (for production configuration)
    pub fn with_embedding_provider(
        mut self,
        provider: Box<dyn mcb_domain::ports::providers::EmbeddingProvider>,
    ) -> Self {
        self.embedding_override = Some(provider);
        self
    }

    /// Override the vector store provider (for production configuration)
    pub fn with_vector_store_provider(
        mut self,
        provider: Box<dyn mcb_domain::ports::providers::VectorStoreProvider>,
    ) -> Self {
        self.vector_store_override = Some(provider);
        self
    }

    /// Build the DI container with hierarchical module composition
    pub async fn build(self) -> Result<DiContainer> {
        // 1. Build leaf modules (no dependencies)
        let infrastructure: Arc<dyn InfrastructureModule> =
            Arc::new(InfrastructureModuleImpl::builder().build());
        let server: Arc<dyn ServerModule> = Arc::new(ServerModuleImpl::builder().build());

        // 2. Build adapters module with provider overrides if configured
        let mut adapters_builder = AdaptersModuleImpl::builder();

        if let Some(embedding_provider) = self.embedding_override {
            adapters_builder = adapters_builder
                .with_component_override::<dyn mcb_domain::ports::providers::EmbeddingProvider>(embedding_provider);
        }

        if let Some(vector_store_provider) = self.vector_store_override {
            adapters_builder = adapters_builder
                .with_component_override::<dyn mcb_domain::ports::providers::VectorStoreProvider>(vector_store_provider);
        }

        let adapters: Arc<dyn AdaptersModule> = Arc::new(adapters_builder.build());

        // 3. Build application module (depends on adapters)
        let application: Arc<dyn ApplicationModule> =
            Arc::new(ApplicationModuleImpl::builder(Arc::clone(&adapters)).build());

        // 4. Build admin module (depends on all other modules)
        let admin: Arc<dyn AdminModule> = Arc::new(
            AdminModuleImpl::builder(
                Arc::clone(&infrastructure),
                Arc::clone(&server),
                Arc::clone(&adapters),
                Arc::clone(&application),
            )
            .build(),
        );

        // 5. Build root container with all modules
        Ok(McpModule::builder(infrastructure, server, adapters, application, admin).build())
    }
}

/// Convenience function to create DI container for testing
pub async fn create_test_container() -> Result<DiContainer> {
    DiContainerBuilder::new().build().await
}

/// Convenience function to create DI container with production configuration
pub async fn create_production_container(config: AppConfig) -> Result<DiContainer> {
    // Create configured providers
    let embedding_provider = ProviderOverrides::create_embedding_provider(&config)?;
    let crypto = crate::crypto::CryptoService::new(
        if config.auth.jwt.secret.len() >= 32 {
            config.auth.jwt.secret.as_bytes()[..32].to_vec()
        } else {
            crate::crypto::CryptoService::generate_master_key()
        }
    )?;
    let vector_store_provider = ProviderOverrides::create_vector_store_provider(&config, &crypto)?;

    DiContainerBuilder::with_config(config)
        .with_embedding_provider(embedding_provider)
        .with_vector_store_provider(vector_store_provider)
        .build()
        .await
}

/// Legacy compatibility - creates full container for gradual migration
///
/// **DEPRECATED**: Use `create_production_container()` instead.
/// This will be removed in v0.2.0 when migration to strict Shaku pattern is complete.
#[deprecated(
    since = "0.1.0",
    note = "Use `create_production_container()` instead. This function will be removed in v0.2.0."
)]
pub async fn create_full_container(config: AppConfig) -> Result<DiContainer> {
    create_production_container(config).await
}
