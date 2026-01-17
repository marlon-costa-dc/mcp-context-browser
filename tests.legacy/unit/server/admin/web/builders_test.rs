//! Unit tests for ViewModelBuilder
//!
//! Tests for the view model builder that bridges service layer and presentation.

use mcp_context_browser::infrastructure::utils::css;
use mcp_context_browser::server::admin::web::builders::ViewModelBuilder;
use mcp_context_browser::server::admin::web::view_models::*;
use tera::{Context, Tera};

#[test]
fn test_error_view_model() {
    let vm = ViewModelBuilder::build_error("Test Error", "Something went wrong", None);
    assert_eq!(vm.title, "Test Error");
    assert_eq!(vm.message, "Something went wrong");
    assert!(vm.details.is_none());

    let vm =
        ViewModelBuilder::build_error("Another Error", "Details below", Some("Stack trace here"));
    assert_eq!(vm.details, Some("Stack trace here".to_string()));
}

// ==========================================================================
// TEMPLATE RENDERING TESTS - Validate ALL templates with REAL data
// ==========================================================================

fn create_test_tera() -> Tera {
    let mut tera = Tera::default();
    tera.add_raw_template(
        "icons.html",
        include_str!("../../../../../src/server/admin/web/templates/icons.html"),
    )
    .expect("Failed to load icons.html");
    tera.add_raw_template(
        "base.html",
        include_str!("../../../../../src/server/admin/web/templates/base.html"),
    )
    .expect("Failed to load base.html");
    tera.add_raw_template(
        "dashboard.html",
        include_str!("../../../../../src/server/admin/web/templates/dashboard.html"),
    )
    .expect("Failed to load dashboard.html");
    tera.add_raw_template(
        "providers.html",
        include_str!("../../../../../src/server/admin/web/templates/providers.html"),
    )
    .expect("Failed to load providers.html");
    tera.add_raw_template(
        "indexes.html",
        include_str!("../../../../../src/server/admin/web/templates/indexes.html"),
    )
    .expect("Failed to load indexes.html");
    tera.add_raw_template(
        "configuration.html",
        include_str!("../../../../../src/server/admin/web/templates/configuration.html"),
    )
    .expect("Failed to load configuration.html");
    tera.add_raw_template(
        "logs.html",
        include_str!("../../../../../src/server/admin/web/templates/logs.html"),
    )
    .expect("Failed to load logs.html");
    tera.add_raw_template(
        "error.html",
        include_str!("../../../../../src/server/admin/web/templates/error.html"),
    )
    .expect("Failed to load error.html");
    tera.add_raw_template(
        "htmx/dashboard_metrics.html",
        include_str!("../../../../../src/server/admin/web/templates/htmx/dashboard_metrics.html"),
    )
    .expect("Failed to load htmx/dashboard_metrics.html");
    tera.add_raw_template(
        "htmx/providers_list.html",
        include_str!("../../../../../src/server/admin/web/templates/htmx/providers_list.html"),
    )
    .expect("Failed to load htmx/providers_list.html");
    tera.add_raw_template(
        "htmx/indexes_list.html",
        include_str!("../../../../../src/server/admin/web/templates/htmx/indexes_list.html"),
    )
    .expect("Failed to load htmx/indexes_list.html");
    tera
}

fn create_dashboard_view_model() -> DashboardViewModel {
    DashboardViewModel {
        page: "dashboard",
        metrics: MetricsViewModel::new(45.5, 62.3, 1234, 15.7),
        providers: ProvidersViewModel::new(vec![
            ProviderViewModel::new(
                "openai-1".to_string(),
                "OpenAI GPT".to_string(),
                "embedding".to_string(),
                "available".to_string(),
            ),
            ProviderViewModel::new(
                "ollama-1".to_string(),
                "Ollama Local".to_string(),
                "embedding".to_string(),
                "unavailable".to_string(),
            ),
        ]),
        indexes: IndexesSummaryViewModel {
            active_count: 1,
            total_documents: 5000,
            total_documents_formatted: "5,000".to_string(),
            is_indexing: false,
        },
        activities: vec![
            ActivityViewModel::new(
                "act-1".to_string(),
                "Index completed successfully".to_string(),
                chrono::Utc::now(),
                "success",
                "indexing".to_string(),
            ),
            ActivityViewModel::new(
                "act-2".to_string(),
                "Provider health check failed".to_string(),
                chrono::Utc::now(),
                "error",
                "health".to_string(),
            ),
        ],
        system_health: HealthViewModel::new("healthy", 3661, 12345),
    }
}

#[test]
fn test_dashboard_template_renders() {
    let tera = create_test_tera();
    let vm = create_dashboard_view_model();
    let vm_json = serde_json::to_string(&vm).expect("Failed to serialize view model");

    let mut context = Context::new();
    context.insert("vm", &vm);
    context.insert("vm_json", &vm_json);
    context.insert("page", &vm.page);

    let result = tera.render("dashboard.html", &context);
    assert!(
        result.is_ok(),
        "Dashboard template failed to render: {:?}",
        result.err()
    );

    let html = result.unwrap();
    assert!(
        html.contains("System Dashboard"),
        "Dashboard should contain title"
    );
    assert!(
        html.len() > 1000,
        "Dashboard should have substantial content"
    );
}

#[test]
fn test_providers_template_renders() {
    let tera = create_test_tera();
    let vm = ProvidersViewModel::new(vec![ProviderViewModel::new(
        "openai-1".to_string(),
        "OpenAI".to_string(),
        "embedding".to_string(),
        "available".to_string(),
    )]);

    let mut context = Context::new();
    context.insert("vm", &vm);
    context.insert("page", &vm.page);

    let result = tera.render("providers.html", &context);
    assert!(
        result.is_ok(),
        "Providers template failed to render: {:?}",
        result.err()
    );
}

#[test]
fn test_indexes_template_renders() {
    let tera = create_test_tera();
    let vm = IndexesViewModel::new(
        vec![IndexViewModel::new(
            "main-index".to_string(),
            "Main Codebase Index".to_string(),
            "active".to_string(),
            5000,
            1704067200,
            1704153600,
        )],
        5000,
    );

    let mut context = Context::new();
    context.insert("vm", &vm);
    context.insert("page", &vm.page);

    let result = tera.render("indexes.html", &context);
    assert!(
        result.is_ok(),
        "Indexes template failed to render: {:?}",
        result.err()
    );
}

#[test]
fn test_configuration_template_renders() {
    let tera = create_test_tera();
    let vm = ConfigurationViewModel {
        page: "config",
        page_description: "Manage system settings",
        categories: vec![ConfigCategoryViewModel {
            name: "Indexing".to_string(),
            description: "Indexing settings".to_string(),
            settings: vec![ConfigSettingViewModel {
                key: "indexing.chunk_size".to_string(),
                label: "Chunk Size".to_string(),
                value: serde_json::json!(512),
                value_display: "512".to_string(),
                setting_type: "number",
                description: "Size of chunks".to_string(),
                editable: true,
            }],
        }],
    };

    let mut context = Context::new();
    context.insert("vm", &vm);
    // Use the &'static str directly without extra reference
    context.insert("page", vm.page);
    context.insert("page_description", vm.page_description);

    let result = tera.render("configuration.html", &context);
    assert!(
        result.is_ok(),
        "Configuration template failed to render: {:?}",
        result.err()
    );
}

#[test]
fn test_logs_template_renders() {
    let tera = create_test_tera();
    let vm = LogsViewModel {
        page: "logs",
        page_description: "View and filter system logs",
        entries: vec![LogEntryViewModel {
            timestamp: "2024-01-01 12:00:00".to_string(),
            level: "INFO".to_string(),
            level_class: css::badge::INFO,
            message: "Server started".to_string(),
            source: "main".to_string(),
        }],
        total_count: 1,
        stats: LogStatsViewModel {
            total: 100,
            errors: 5,
            warnings: 10,
            info: 85,
        },
    };

    let mut context = Context::new();
    context.insert("vm", &vm);
    // Use the &'static str directly without extra reference
    context.insert("page", vm.page);
    context.insert("page_description", vm.page_description);

    let result = tera.render("logs.html", &context);
    assert!(
        result.is_ok(),
        "Logs template failed to render: {:?}",
        result.err()
    );
}

#[test]
fn test_error_template_renders() {
    let tera = create_test_tera();
    let error_vm = ErrorViewModel::new("Test Error", "Something went wrong");

    let mut context = Context::new();
    context.insert("error", &error_vm);
    context.insert("page", "error");

    let result = tera.render("error.html", &context);
    assert!(
        result.is_ok(),
        "Error template failed to render: {:?}",
        result.err()
    );

    let html = result.unwrap();
    assert!(
        html.contains("Test Error"),
        "Error page should contain error title"
    );
}

#[test]
fn test_htmx_dashboard_metrics_renders() {
    let tera = create_test_tera();
    let vm = create_dashboard_view_model();

    let mut context = Context::new();
    context.insert("vm", &vm);

    let result = tera.render("htmx/dashboard_metrics.html", &context);
    assert!(
        result.is_ok(),
        "HTMX dashboard metrics failed to render: {:?}",
        result.err()
    );
}

#[test]
fn test_htmx_providers_list_renders() {
    let tera = create_test_tera();
    let providers = vec![ProviderViewModel::new(
        "openai-1".to_string(),
        "OpenAI".to_string(),
        "embedding".to_string(),
        "available".to_string(),
    )];

    let mut context = Context::new();
    context.insert("providers", &providers);

    let result = tera.render("htmx/providers_list.html", &context);
    assert!(
        result.is_ok(),
        "HTMX providers list failed to render: {:?}",
        result.err()
    );
}

#[test]
fn test_htmx_indexes_list_renders() {
    let tera = create_test_tera();
    let indexes = vec![IndexViewModel::new(
        "main-index".to_string(),
        "Main Index".to_string(),
        "active".to_string(),
        1000,
        1704067200,
        1704153600,
    )];

    let mut context = Context::new();
    context.insert("indexes", &indexes);

    let result = tera.render("htmx/indexes_list.html", &context);
    assert!(
        result.is_ok(),
        "HTMX indexes list failed to render: {:?}",
        result.err()
    );
}
