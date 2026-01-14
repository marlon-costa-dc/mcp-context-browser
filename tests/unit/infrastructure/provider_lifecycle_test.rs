//! Provider lifecycle manager unit tests

use mcp_context_browser::infrastructure::di::factory::ServiceProvider;
use mcp_context_browser::infrastructure::di::registry::{ProviderRegistry, ProviderRegistryTrait};
use mcp_context_browser::infrastructure::events::EventBus;
use mcp_context_browser::infrastructure::provider_lifecycle::ProviderLifecycleManager;
use std::sync::Arc;

#[test]
fn test_lifecycle_manager_creation() {
    // This is a basic smoke test - in a real scenario we'd mock the dependencies
    let registry: Arc<dyn ProviderRegistryTrait> = Arc::new(ProviderRegistry::new());
    let service_provider = Arc::new(ServiceProvider::new());
    let event_bus = Arc::new(EventBus::new(10));

    let lifecycle = ProviderLifecycleManager::new(service_provider, registry, event_bus);

    // Verify it has a connection tracker
    let _tracker = lifecycle.connection_tracker();

    // Just verify it can be created without panic
    let _ = lifecycle;
}
