//! Tests for DI container bootstrap
//!
//! Migrated from src/infrastructure/di/bootstrap.rs

use std::sync::Arc;

use mcp_context_browser::adapters::http_client::HttpClientProvider;
use mcp_context_browser::domain::ports::{
    ChunkRepository, EmbeddingProvider, SearchRepository, VectorStoreProvider,
};
use mcp_context_browser::infrastructure::auth::AuthServiceInterface;
use mcp_context_browser::infrastructure::di::bootstrap::DiContainer;
use mcp_context_browser::infrastructure::di::factory::ServiceProviderInterface;
use mcp_context_browser::infrastructure::events::EventBusProvider;
use mcp_context_browser::infrastructure::metrics::system::SystemMetricsCollectorInterface;
use mcp_context_browser::server::metrics::PerformanceMetricsInterface;
use mcp_context_browser::server::operations::IndexingOperationsInterface;

#[tokio::test]
async fn test_di_container_resolves_http_client() {
    let container = DiContainer::build().expect("DiContainer should build");
    let http_client: Arc<dyn HttpClientProvider> = container.resolve();
    assert!(Arc::strong_count(&http_client) >= 1);
}

#[tokio::test]
async fn test_di_container_resolves_performance_metrics() {
    let container = DiContainer::build().expect("DiContainer should build");
    let metrics: Arc<dyn PerformanceMetricsInterface> = container.resolve();
    assert!(Arc::strong_count(&metrics) >= 1);
}

#[tokio::test]
async fn test_di_container_resolves_indexing_operations() {
    let container = DiContainer::build().expect("DiContainer should build");
    let ops: Arc<dyn IndexingOperationsInterface> = container.resolve();
    assert!(Arc::strong_count(&ops) >= 1);
}

#[tokio::test]
async fn test_di_container_resolves_service_provider() {
    let container = DiContainer::build().expect("DiContainer should build");
    let provider: Arc<dyn ServiceProviderInterface> = container.resolve();
    assert!(Arc::strong_count(&provider) >= 1);
}

#[tokio::test]
async fn test_di_container_resolves_system_collector() {
    let container = DiContainer::build().expect("DiContainer should build");
    let collector: Arc<dyn SystemMetricsCollectorInterface> = container.resolve();
    assert!(Arc::strong_count(&collector) >= 1);
}

#[tokio::test]
async fn test_di_container_resolves_event_bus() {
    let container = DiContainer::build().expect("DiContainer should build");
    let event_bus: Arc<dyn EventBusProvider> = container.resolve();
    assert!(Arc::strong_count(&event_bus) >= 1);
}

#[tokio::test]
async fn test_di_container_resolves_auth_service() {
    let container = DiContainer::build().expect("DiContainer should build");
    let auth_service: Arc<dyn AuthServiceInterface> = container.resolve();
    assert!(Arc::strong_count(&auth_service) >= 1);
}

#[tokio::test]
async fn test_di_container_resolves_embedding_provider() {
    let container = DiContainer::build().expect("DiContainer should build");
    let provider: Arc<dyn EmbeddingProvider> = container.resolve();
    assert!(Arc::strong_count(&provider) >= 1);
    // Verify we got a null provider
    assert_eq!(provider.provider_name(), "null");
}

#[tokio::test]
async fn test_di_container_resolves_vector_store_provider() {
    let container = DiContainer::build().expect("DiContainer should build");
    let provider: Arc<dyn VectorStoreProvider> = container.resolve();
    assert!(Arc::strong_count(&provider) >= 1);
    // Verify we got a null provider
    assert_eq!(provider.provider_name(), "null");
}

#[tokio::test]
async fn test_di_container_resolves_chunk_repository() {
    let container = DiContainer::build().expect("DiContainer should build");
    let repository: Arc<dyn ChunkRepository> = container.resolve();
    assert!(Arc::strong_count(&repository) >= 1);
}

#[tokio::test]
async fn test_di_container_resolves_search_repository() {
    let container = DiContainer::build().expect("DiContainer should build");
    let repository: Arc<dyn SearchRepository> = container.resolve();
    assert!(Arc::strong_count(&repository) >= 1);
}

// NOTE: AdminService resolution requires complex runtime dependencies
// which is not yet complete. This will be enabled in Phase 3.
// #[tokio::test]
// async fn test_di_container_resolves_admin_service() {
//     let container = DiContainer::build().expect("DiContainer should build");
//     let admin_service: Arc<dyn AdminService> = container.resolve();
//     assert!(Arc::strong_count(&admin_service) >= 1);
// }
