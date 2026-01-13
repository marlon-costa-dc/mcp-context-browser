use crate::infrastructure::cache::{create_cache_provider, SharedCacheProvider};
use crate::infrastructure::config::Config;
use crate::infrastructure::di::factory::ServiceProviderInterface;
use crate::infrastructure::di::DiContainer;
use crate::infrastructure::events::SharedEventBusProvider;
use crate::infrastructure::limits::ResourceLimits;
use crate::infrastructure::logging::SharedLogBuffer;
use crate::infrastructure::metrics::system::SystemMetricsCollectorInterface;
use crate::server::admin::service::AdminService;
use crate::server::mcp_server::McpServer;
use crate::server::metrics::PerformanceMetricsInterface;
use crate::server::operations::IndexingOperationsInterface;
use arc_swap::ArcSwap;
use std::sync::Arc;

/// Builder for McpServer to handle complex dependency injection
#[derive(Default)]
pub struct McpServerBuilder {
    config: Option<Arc<ArcSwap<Config>>>,
    cache_provider: Option<SharedCacheProvider>,
    event_bus: Option<SharedEventBusProvider>,
    log_buffer: Option<SharedLogBuffer>,
    performance_metrics: Option<Arc<dyn PerformanceMetricsInterface>>,
    indexing_operations: Option<Arc<dyn IndexingOperationsInterface>>,
    service_provider: Option<Arc<dyn ServiceProviderInterface>>,
    system_collector: Option<Arc<dyn SystemMetricsCollectorInterface>>,
    resource_limits: Option<Arc<ResourceLimits>>,
    http_client: Option<Arc<dyn crate::adapters::http_client::HttpClientProvider>>,
}

impl McpServerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(mut self, config: Arc<ArcSwap<Config>>) -> Self {
        self.config = Some(config);
        self
    }

    pub fn with_cache_provider(mut self, cache_provider: Option<SharedCacheProvider>) -> Self {
        self.cache_provider = cache_provider;
        self
    }

    pub fn with_event_bus(mut self, event_bus: SharedEventBusProvider) -> Self {
        self.event_bus = Some(event_bus);
        self
    }

    pub fn with_log_buffer(mut self, log_buffer: SharedLogBuffer) -> Self {
        self.log_buffer = Some(log_buffer);
        self
    }

    pub fn with_performance_metrics(
        mut self,
        metrics: Arc<dyn PerformanceMetricsInterface>,
    ) -> Self {
        self.performance_metrics = Some(metrics);
        self
    }

    pub fn with_indexing_operations(mut self, ops: Arc<dyn IndexingOperationsInterface>) -> Self {
        self.indexing_operations = Some(ops);
        self
    }

    pub fn with_service_provider(mut self, provider: Arc<dyn ServiceProviderInterface>) -> Self {
        self.service_provider = Some(provider);
        self
    }

    pub fn with_system_collector(
        mut self,
        collector: Arc<dyn SystemMetricsCollectorInterface>,
    ) -> Self {
        self.system_collector = Some(collector);
        self
    }

    pub fn with_resource_limits(mut self, limits: Arc<ResourceLimits>) -> Self {
        self.resource_limits = Some(limits);
        self
    }

    pub fn with_http_client(
        mut self,
        http_client: Arc<dyn crate::adapters::http_client::HttpClientProvider>,
    ) -> Self {
        self.http_client = Some(http_client);
        self
    }

    pub async fn build(self) -> Result<McpServer, Box<dyn std::error::Error>> {
        // Load configuration if not provided
        let config_arc = if let Some(c) = self.config {
            c
        } else {
            let loader = crate::infrastructure::config::ConfigLoader::new();
            let home_dir = dirs::home_dir().ok_or("Cannot determine home directory")?;
            let config_path = home_dir.join(".context").join("config.toml");
            let config = loader.load_with_file(&config_path).await?;
            Arc::new(ArcSwap::from_pointee(config))
        };

        // Build DI container for component resolution
        // Components provided via builder take precedence over DI resolution
        let container = DiContainer::build()
            .map_err(|e| anyhow::anyhow!("Failed to build DI container: {}", e))?;

        // Resolve EventBus from DI container if not explicitly provided
        let event_bus: crate::infrastructure::events::SharedEventBusProvider = self
            .event_bus
            .unwrap_or_else(|| container.resolve());
        let log_buffer = self
            .log_buffer
            .unwrap_or_else(|| crate::infrastructure::logging::create_shared_log_buffer(1000));

        // Resolve from DI container if not explicitly provided
        let performance_metrics: Arc<dyn PerformanceMetricsInterface> = self
            .performance_metrics
            .unwrap_or_else(|| container.resolve());

        let indexing_operations: Arc<dyn IndexingOperationsInterface> = self
            .indexing_operations
            .unwrap_or_else(|| container.resolve());

        let service_provider: Arc<dyn ServiceProviderInterface> = self
            .service_provider
            .unwrap_or_else(|| container.resolve());

        let system_collector: Arc<dyn SystemMetricsCollectorInterface> = self
            .system_collector
            .unwrap_or_else(|| container.resolve());

        // Initialize resource limits from config if not provided
        // ResourceLimits not yet in DI modules
        let resource_limits = if let Some(rl) = self.resource_limits {
            rl
        } else {
            let config = config_arc.load();
            Arc::new(ResourceLimits::new(config.resource_limits.clone()))
        };

        // Resolve HTTP client from DI container if not provided
        let http_client: Arc<dyn crate::adapters::http_client::HttpClientProvider> =
            self.http_client.unwrap_or_else(|| container.resolve());

        // Initialize cache provider if not provided
        let cache_provider = match self.cache_provider {
            Some(cp) => cp,
            None => {
                let config = config_arc.load().cache.clone();
                create_cache_provider(&config).await?
            }
        };

        // Resolve admin service from DI container
        // The DI-resolved AdminService doesn't have all dependencies wired yet,
        // so we need to manually construct for now until Phase 3 completes
        let event_bus_trait: SharedEventBusProvider = event_bus.clone() as SharedEventBusProvider;
        let deps = crate::server::admin::service::AdminServiceDependencies {
            performance_metrics: Arc::clone(&performance_metrics),
            indexing_operations: Arc::clone(&indexing_operations),
            service_provider: Arc::clone(&service_provider),
            system_collector: Arc::clone(&system_collector),
            http_client: Arc::clone(&http_client),
            event_bus: event_bus_trait,
            log_buffer: log_buffer.clone(),
            config: Arc::clone(&config_arc),
        };
        let admin_service: Arc<dyn AdminService> =
            Arc::new(crate::server::admin::service::AdminServiceImpl::new(deps));

        // Use from_components to assemble the server
        McpServer::from_components(crate::server::mcp_server::ServerComponents {
            config: config_arc,
            cache_provider: Some(cache_provider),
            performance_metrics,
            indexing_operations,
            admin_service,
            service_provider,
            resource_limits,
            http_client,
            event_bus,
            log_buffer,
            system_collector,
        })
        .await
    }
}
