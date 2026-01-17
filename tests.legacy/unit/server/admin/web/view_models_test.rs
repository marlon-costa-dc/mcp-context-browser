//! Unit tests for view models
//!
//! Tests for view models - bridge between service layer and presentation.

use mcp_context_browser::server::admin::web::view_models::{HealthViewModel, ProviderViewModel};

#[test]
fn test_provider_view_model() {
    let vm = ProviderViewModel::new(
        "openai-1".to_string(),
        "OpenAI".to_string(),
        "embedding".to_string(),
        "available".to_string(),
    );
    assert!(vm.is_active);
    assert_eq!(vm.provider_type_display, "Embedding");
    assert!(vm.status_class.contains("green"));
}

#[test]
fn test_health_view_model() {
    let vm = HealthViewModel::new("healthy", 3661, 12345);
    assert_eq!(vm.uptime_formatted, "1h 1m 1s");
    assert!(vm.status_class.contains("green"));
}
