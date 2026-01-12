//! Handler tests for MCP server tools
//!
//! Tests for clear_index, search_code, index_codebase, and get_indexing_status handlers.

use async_trait::async_trait;
use rmcp::handler::server::wrapper::Parameters;
use std::collections::HashMap;
use std::sync::Arc;

use mcp_context_browser::adapters::hybrid_search::{HybridSearchAdapter, HybridSearchMessage};
use mcp_context_browser::admin::service::{
    AdminError, AdminService, BackupConfig, BackupInfo, BackupResult, CacheType, CleanupConfig,
    ConfigDiff, ConfigPersistResult, ConfigurationChange, ConfigurationData,
    ConfigurationUpdateResult, ConnectivityTestResult, DashboardData, HealthCheckResult,
    IndexingStatus, LogEntries, LogExportFormat, LogFilter, LogStats, MaintenanceResult,
    PerformanceMetricsData, PerformanceTestConfig, PerformanceTestResult, ProviderInfo,
    RestoreResult, RouteInfo, SearchResults, SignalResult, SubsystemInfo, SubsystemSignal,
    SystemInfo,
};
use mcp_context_browser::application::{ContextService, IndexingService, SearchService};
use mcp_context_browser::domain::ports::{
    EmbeddingProvider, HybridSearchProvider, VectorStoreProvider,
};
use mcp_context_browser::infrastructure::auth::{AuthConfig, AuthService};
use mcp_context_browser::infrastructure::limits::{ResourceLimits, ResourceLimitsConfig};
use mcp_context_browser::server::args::{
    ClearIndexArgs, GetIndexingStatusArgs, IndexCodebaseArgs, SearchCodeArgs,
};
use mcp_context_browser::server::auth::AuthHandler;
use mcp_context_browser::server::handlers::{
    ClearIndexHandler, GetIndexingStatusHandler, IndexCodebaseHandler, SearchCodeHandler,
};

// ============================================================================
// Test Provider Setup
// ============================================================================

fn create_test_providers() -> (
    Arc<dyn EmbeddingProvider>,
    Arc<dyn VectorStoreProvider>,
    Arc<dyn HybridSearchProvider>,
) {
    let embedding_provider = Arc::new(
        mcp_context_browser::adapters::providers::embedding::null::NullEmbeddingProvider::new(),
    );
    let vector_store_provider = Arc::new(
        mcp_context_browser::adapters::providers::vector_store::null::NullVectorStoreProvider::new(
        ),
    );
    let (sender, receiver) = tokio::sync::mpsc::channel(100);
    tokio::spawn(async move {
        let mut receiver = receiver;
        while let Some(msg) = receiver.recv().await {
            match msg {
                HybridSearchMessage::Search { respond_to, .. } => {
                    let _ = respond_to.send(Ok(Vec::new()));
                }
                HybridSearchMessage::GetStats { respond_to } => {
                    let _ = respond_to.send(std::collections::HashMap::new());
                }
                _ => {}
            }
        }
    });
    let hybrid_search_provider = Arc::new(HybridSearchAdapter::new(sender));
    (
        embedding_provider,
        vector_store_provider,
        hybrid_search_provider,
    )
}

// ============================================================================
// Mock Implementations
// ============================================================================

/// Mock AdminService for testing get_indexing_status handler
struct MockAdminService {
    system_info: SystemInfo,
    indexing_status: IndexingStatus,
    performance_metrics: PerformanceMetricsData,
}

impl MockAdminService {
    fn new() -> Self {
        Self {
            system_info: SystemInfo {
                version: "0.1.0-test".to_string(),
                uptime: 1000,
                pid: 12345,
            },
            indexing_status: IndexingStatus {
                is_indexing: false,
                total_documents: 100,
                indexed_documents: 100,
                failed_documents: 0,
                current_file: None,
                start_time: None,
                estimated_completion: None,
            },
            performance_metrics: PerformanceMetricsData {
                total_queries: 500,
                successful_queries: 498,
                failed_queries: 2,
                average_response_time_ms: 45.5,
                cache_hit_rate: 0.75,
                active_connections: 3,
                uptime_seconds: 1000,
            },
        }
    }

    fn with_indexing_in_progress(mut self) -> Self {
        self.indexing_status.is_indexing = true;
        self.indexing_status.total_documents = 200;
        self.indexing_status.indexed_documents = 50;
        self.indexing_status.current_file = Some("src/main.rs".to_string());
        self
    }
}

#[async_trait]
impl AdminService for MockAdminService {
    async fn get_system_info(&self) -> std::result::Result<SystemInfo, AdminError> {
        Ok(self.system_info.clone())
    }

    async fn get_providers(&self) -> std::result::Result<Vec<ProviderInfo>, AdminError> {
        Ok(vec![])
    }

    async fn add_provider(
        &self,
        _provider_type: &str,
        _config: serde_json::Value,
    ) -> std::result::Result<ProviderInfo, AdminError> {
        Err(AdminError::InternalError("Not implemented".to_string()))
    }

    async fn remove_provider(&self, _provider_id: &str) -> std::result::Result<(), AdminError> {
        Ok(())
    }

    async fn search(
        &self,
        _query: &str,
        _collection: Option<&str>,
        _limit: Option<usize>,
    ) -> std::result::Result<SearchResults, AdminError> {
        Ok(SearchResults {
            query: "test".to_string(),
            results: vec![],
            total: 0,
            took_ms: 10,
        })
    }

    async fn get_indexing_status(&self) -> std::result::Result<IndexingStatus, AdminError> {
        Ok(self.indexing_status.clone())
    }

    async fn get_performance_metrics(
        &self,
    ) -> std::result::Result<PerformanceMetricsData, AdminError> {
        Ok(self.performance_metrics.clone())
    }

    async fn get_dashboard_data(&self) -> std::result::Result<DashboardData, AdminError> {
        Ok(DashboardData {
            system_info: self.system_info.clone(),
            active_providers: 2,
            total_providers: 3,
            active_indexes: 1,
            total_documents: 100,
            cpu_usage: 25.0,
            memory_usage: 40.0,
            performance: self.performance_metrics.clone(),
        })
    }

    async fn get_configuration(&self) -> std::result::Result<ConfigurationData, AdminError> {
        Err(AdminError::InternalError("Not implemented".to_string()))
    }

    async fn update_configuration(
        &self,
        _updates: HashMap<String, serde_json::Value>,
        _user: &str,
    ) -> std::result::Result<ConfigurationUpdateResult, AdminError> {
        Err(AdminError::InternalError("Not implemented".to_string()))
    }

    async fn validate_configuration(
        &self,
        _updates: &HashMap<String, serde_json::Value>,
    ) -> std::result::Result<Vec<String>, AdminError> {
        Ok(vec![])
    }

    async fn get_configuration_history(
        &self,
        _limit: Option<usize>,
    ) -> std::result::Result<Vec<ConfigurationChange>, AdminError> {
        Ok(vec![])
    }

    async fn get_logs(&self, _filter: LogFilter) -> std::result::Result<LogEntries, AdminError> {
        Ok(LogEntries {
            entries: vec![],
            total_count: 0,
            has_more: false,
        })
    }

    async fn export_logs(
        &self,
        _filter: LogFilter,
        _format: LogExportFormat,
    ) -> std::result::Result<String, AdminError> {
        Ok("".to_string())
    }

    async fn get_log_stats(&self) -> std::result::Result<LogStats, AdminError> {
        Ok(LogStats {
            total_entries: 0,
            entries_by_level: HashMap::new(),
            entries_by_module: HashMap::new(),
            oldest_entry: None,
            newest_entry: None,
        })
    }

    async fn clear_cache(
        &self,
        _cache_type: CacheType,
    ) -> std::result::Result<MaintenanceResult, AdminError> {
        Ok(MaintenanceResult {
            success: true,
            operation: "clear_cache".to_string(),
            message: "Cache cleared".to_string(),
            affected_items: 100,
            execution_time_ms: 50,
        })
    }

    async fn restart_provider(
        &self,
        _provider_id: &str,
    ) -> std::result::Result<MaintenanceResult, AdminError> {
        Ok(MaintenanceResult {
            success: true,
            operation: "restart_provider".to_string(),
            message: "Provider restarted".to_string(),
            affected_items: 1,
            execution_time_ms: 100,
        })
    }

    async fn rebuild_index(
        &self,
        _index_id: &str,
    ) -> std::result::Result<MaintenanceResult, AdminError> {
        Ok(MaintenanceResult {
            success: true,
            operation: "rebuild_index".to_string(),
            message: "Index rebuilt".to_string(),
            affected_items: 100,
            execution_time_ms: 5000,
        })
    }

    async fn cleanup_data(
        &self,
        _cleanup_config: CleanupConfig,
    ) -> std::result::Result<MaintenanceResult, AdminError> {
        Ok(MaintenanceResult {
            success: true,
            operation: "cleanup".to_string(),
            message: "Data cleaned".to_string(),
            affected_items: 50,
            execution_time_ms: 200,
        })
    }

    async fn run_health_check(&self) -> std::result::Result<HealthCheckResult, AdminError> {
        Ok(HealthCheckResult {
            overall_status: "healthy".to_string(),
            checks: vec![],
            timestamp: chrono::Utc::now(),
            duration_ms: 100,
        })
    }

    async fn test_provider_connectivity(
        &self,
        provider_id: &str,
    ) -> std::result::Result<ConnectivityTestResult, AdminError> {
        Ok(ConnectivityTestResult {
            provider_id: provider_id.to_string(),
            success: true,
            response_time_ms: Some(50),
            error_message: None,
            details: serde_json::json!({}),
        })
    }

    async fn run_performance_test(
        &self,
        _test_config: PerformanceTestConfig,
    ) -> std::result::Result<PerformanceTestResult, AdminError> {
        Ok(PerformanceTestResult {
            test_id: "test-1".to_string(),
            test_type: "search".to_string(),
            duration_seconds: 60,
            total_requests: 1000,
            successful_requests: 990,
            failed_requests: 10,
            average_response_time_ms: 45.0,
            p95_response_time_ms: 100.0,
            p99_response_time_ms: 150.0,
            throughput_rps: 16.5,
        })
    }

    async fn create_backup(
        &self,
        backup_config: BackupConfig,
    ) -> std::result::Result<BackupResult, AdminError> {
        Ok(BackupResult {
            backup_id: "backup-1".to_string(),
            name: backup_config.name,
            size_bytes: 1024000,
            created_at: chrono::Utc::now(),
            path: "/backups/backup-1.tar.gz".to_string(),
        })
    }

    async fn list_backups(&self) -> std::result::Result<Vec<BackupInfo>, AdminError> {
        Ok(vec![])
    }

    async fn restore_backup(
        &self,
        backup_id: &str,
    ) -> std::result::Result<RestoreResult, AdminError> {
        Ok(RestoreResult {
            success: true,
            backup_id: backup_id.to_string(),
            restored_items: 100,
            errors: vec![],
        })
    }

    // === Subsystem Control Methods (ADR-007) ===

    async fn get_subsystems(&self) -> std::result::Result<Vec<SubsystemInfo>, AdminError> {
        Ok(vec![])
    }

    async fn send_subsystem_signal(
        &self,
        subsystem_id: &str,
        _signal: SubsystemSignal,
    ) -> std::result::Result<SignalResult, AdminError> {
        Ok(SignalResult {
            success: true,
            subsystem_id: subsystem_id.to_string(),
            signal: "test".to_string(),
            message: "Signal sent".to_string(),
        })
    }

    async fn get_routes(&self) -> std::result::Result<Vec<RouteInfo>, AdminError> {
        Ok(vec![])
    }

    async fn reload_routes(&self) -> std::result::Result<MaintenanceResult, AdminError> {
        Ok(MaintenanceResult {
            success: true,
            operation: "reload_routes".to_string(),
            message: "Routes reloaded".to_string(),
            affected_items: 0,
            execution_time_ms: 0,
        })
    }

    async fn persist_configuration(&self) -> std::result::Result<ConfigPersistResult, AdminError> {
        Ok(ConfigPersistResult {
            success: true,
            path: "/tmp/config.toml".to_string(),
            warnings: vec![],
        })
    }

    async fn get_config_diff(&self) -> std::result::Result<ConfigDiff, AdminError> {
        Ok(ConfigDiff {
            has_changes: false,
            runtime_only: HashMap::new(),
            file_only: HashMap::new(),
        })
    }
}

// ============================================================================
// Test Utilities
// ============================================================================

fn create_auth_handler_disabled() -> AuthHandler {
    let config = AuthConfig {
        enabled: false,
        ..Default::default()
    };
    let auth_service = AuthService::new(config);
    AuthHandler::new(auth_service)
}

fn create_resource_limits_disabled() -> ResourceLimits {
    let config = ResourceLimitsConfig {
        enabled: false,
        ..Default::default()
    };
    ResourceLimits::new(config)
}

/// Helper to extract text from CallToolResult
fn extract_text(result: &rmcp::model::CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|c| {
            if let rmcp::model::RawContent::Text(text_content) = &c.raw {
                Some(text_content.text.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("")
}

// ============================================================================
// GetIndexingStatusHandler Tests
// ============================================================================

mod get_indexing_status_tests {
    use super::*;

    #[tokio::test]
    async fn test_get_indexing_status_ready() {
        let admin_service = Arc::new(MockAdminService::new()) as Arc<dyn AdminService>;
        let handler = GetIndexingStatusHandler::new(admin_service);

        let args = GetIndexingStatusArgs {
            collection: "default".to_string(),
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        assert!(text.contains("System Status"));
        assert!(text.contains("Ready for search"));
        assert!(text.contains("default"));
    }

    #[tokio::test]
    async fn test_get_indexing_status_in_progress() {
        let admin_service =
            Arc::new(MockAdminService::new().with_indexing_in_progress()) as Arc<dyn AdminService>;
        let handler = GetIndexingStatusHandler::new(admin_service);

        let args = GetIndexingStatusArgs {
            collection: "test-collection".to_string(),
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        assert!(text.contains("Indexing in progress"));
        assert!(text.contains("test-collection"));
    }

    #[tokio::test]
    async fn test_get_indexing_status_shows_metrics() {
        let admin_service = Arc::new(MockAdminService::new()) as Arc<dyn AdminService>;
        let handler = GetIndexingStatusHandler::new(admin_service);

        let args = GetIndexingStatusArgs {
            collection: "default".to_string(),
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        // Check metrics are included
        assert!(text.contains("Performance"));
        assert!(text.contains("Total Queries"));
        assert!(text.contains("Cache Hit Rate"));
    }
}

// ============================================================================
// ClearIndexHandler Tests
// ============================================================================

mod clear_index_tests {
    use super::*;

    async fn create_indexing_service() -> Arc<IndexingService> {
        let (embedding_provider, vector_store_provider, hybrid_search_provider) =
            create_test_providers();
        let context_service = Arc::new(ContextService::new_with_providers(
            embedding_provider,
            vector_store_provider,
            hybrid_search_provider,
        ));
        Arc::new(IndexingService::new(context_service, None).unwrap())
    }

    #[tokio::test]
    async fn test_clear_index_empty_collection_name() {
        let indexing_service = create_indexing_service().await;
        let handler = ClearIndexHandler::new(indexing_service);

        let args = ClearIndexArgs {
            collection: "   ".to_string(), // Empty after trim
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        assert!(text.contains("Error"));
        assert!(text.contains("cannot be empty"));
    }

    #[tokio::test]
    async fn test_clear_index_system_collection_blocked() {
        let indexing_service = create_indexing_service().await;
        let handler = ClearIndexHandler::new(indexing_service);

        let args = ClearIndexArgs {
            collection: "system".to_string(),
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        assert!(text.contains("Error"));
        assert!(text.contains("system collections"));
    }

    #[tokio::test]
    async fn test_clear_index_admin_collection_blocked() {
        let indexing_service = create_indexing_service().await;
        let handler = ClearIndexHandler::new(indexing_service);

        let args = ClearIndexArgs {
            collection: "admin".to_string(),
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        assert!(text.contains("Error"));
        assert!(text.contains("system collections"));
    }

    #[tokio::test]
    async fn test_clear_index_valid_collection() {
        let indexing_service = create_indexing_service().await;
        let handler = ClearIndexHandler::new(indexing_service);

        let args = ClearIndexArgs {
            collection: "test-collection".to_string(),
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        // Should either succeed or report an error (not validation error)
        assert!(
            text.contains("Completed Successfully") || text.contains("test-collection"),
            "Expected success message or collection reference, got: {}",
            text
        );
    }
}

// ============================================================================
// IndexCodebaseHandler Tests
// ============================================================================

mod index_codebase_tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    async fn create_indexing_service() -> Arc<IndexingService> {
        let (embedding_provider, vector_store_provider, hybrid_search_provider) =
            create_test_providers();
        let context_service = Arc::new(ContextService::new_with_providers(
            embedding_provider,
            vector_store_provider,
            hybrid_search_provider,
        ));
        Arc::new(IndexingService::new(context_service, None).unwrap())
    }

    #[tokio::test]
    async fn test_index_codebase_path_not_exists() {
        let indexing_service = create_indexing_service().await;
        let auth_handler = Arc::new(create_auth_handler_disabled());
        let resource_limits = Arc::new(create_resource_limits_disabled());
        let handler = IndexCodebaseHandler::new(indexing_service, auth_handler, resource_limits);

        let args = IndexCodebaseArgs {
            path: "/nonexistent/path/that/does/not/exist".to_string(),
            token: None,
            collection: None,
            extensions: None,
            ignore_patterns: None,
            max_file_size: None,
            follow_symlinks: None,
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        assert!(text.contains("Error"));
        assert!(text.contains("does not exist"));
    }

    #[tokio::test]
    async fn test_index_codebase_path_is_file() {
        let indexing_service = create_indexing_service().await;
        let auth_handler = Arc::new(create_auth_handler_disabled());
        let resource_limits = Arc::new(create_resource_limits_disabled());
        let handler = IndexCodebaseHandler::new(indexing_service, auth_handler, resource_limits);

        // Create a temporary file
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("test.rs");
        fs::write(&file_path, "fn main() {}").unwrap();

        let args = IndexCodebaseArgs {
            path: file_path.to_string_lossy().to_string(),
            token: None,
            collection: None,
            extensions: None,
            ignore_patterns: None,
            max_file_size: None,
            follow_symlinks: None,
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        assert!(text.contains("Error"));
        assert!(text.contains("not a directory"));
    }

    #[tokio::test]
    async fn test_index_codebase_valid_directory() {
        let indexing_service = create_indexing_service().await;
        let auth_handler = Arc::new(create_auth_handler_disabled());
        let resource_limits = Arc::new(create_resource_limits_disabled());
        let handler = IndexCodebaseHandler::new(indexing_service, auth_handler, resource_limits);

        // Create a temporary directory with source files
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("main.rs");
        fs::write(&file_path, "fn main() {\n    println!(\"Hello\");\n}").unwrap();

        let args = IndexCodebaseArgs {
            path: temp_dir.path().to_string_lossy().to_string(),
            token: None,
            collection: Some("test".to_string()),
            extensions: None,
            ignore_patterns: None,
            max_file_size: None,
            follow_symlinks: None,
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        // Should succeed or show indexing result
        assert!(
            text.contains("Completed") || text.contains("chunks"),
            "Expected indexing result, got: {}",
            text
        );
    }

    #[tokio::test]
    async fn test_index_codebase_uses_default_collection() {
        let indexing_service = create_indexing_service().await;
        let auth_handler = Arc::new(create_auth_handler_disabled());
        let resource_limits = Arc::new(create_resource_limits_disabled());
        let handler = IndexCodebaseHandler::new(indexing_service, auth_handler, resource_limits);

        let temp_dir = tempdir().unwrap();
        fs::write(temp_dir.path().join("test.rs"), "fn test() {}").unwrap();

        let args = IndexCodebaseArgs {
            path: temp_dir.path().to_string_lossy().to_string(),
            token: None,
            collection: None, // No collection specified
            extensions: None,
            ignore_patterns: None,
            max_file_size: None,
            follow_symlinks: None,
        };

        let result = handler.handle(Parameters(args)).await;

        // Should succeed - default collection "default" should be used
        assert!(result.is_ok());
    }
}

// ============================================================================
// SearchCodeHandler Tests
// ============================================================================

mod search_code_tests {
    use super::*;
    use mcp_context_browser::infrastructure::cache::{CacheConfig, CacheManager};

    async fn create_search_service() -> Arc<SearchService> {
        let (embedding_provider, vector_store_provider, hybrid_search_provider) =
            create_test_providers();
        let context_service = Arc::new(ContextService::new_with_providers(
            embedding_provider,
            vector_store_provider,
            hybrid_search_provider,
        ));
        Arc::new(SearchService::new(context_service))
    }

    async fn create_cache_manager() -> Arc<CacheManager> {
        let config = CacheConfig::default();
        Arc::new(CacheManager::new(config, None).await.unwrap())
    }

    #[tokio::test]
    async fn test_search_code_empty_query() {
        let search_service = create_search_service().await;
        let auth_handler = Arc::new(create_auth_handler_disabled());
        let resource_limits = Arc::new(create_resource_limits_disabled());
        let cache_manager = create_cache_manager().await;
        let handler =
            SearchCodeHandler::new(search_service, auth_handler, resource_limits, cache_manager);

        let args = SearchCodeArgs {
            query: "   ".to_string(), // Empty after trim
            limit: 10,
            token: None,
            filters: None,
            collection: None,
            extensions: None,
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        assert!(text.contains("Error"));
        assert!(text.contains("empty") || text.contains("cannot"));
    }

    #[tokio::test]
    async fn test_search_code_query_too_short() {
        let search_service = create_search_service().await;
        let auth_handler = Arc::new(create_auth_handler_disabled());
        let resource_limits = Arc::new(create_resource_limits_disabled());
        let cache_manager = create_cache_manager().await;
        let handler =
            SearchCodeHandler::new(search_service, auth_handler, resource_limits, cache_manager);

        let args = SearchCodeArgs {
            query: "ab".to_string(), // Less than 3 characters
            limit: 10,
            token: None,
            filters: None,
            collection: None,
            extensions: None,
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        assert!(text.contains("Error"));
        assert!(text.contains("short") || text.contains("3 characters"));
    }

    #[tokio::test]
    async fn test_search_code_valid_query() {
        let search_service = create_search_service().await;
        let auth_handler = Arc::new(create_auth_handler_disabled());
        let resource_limits = Arc::new(create_resource_limits_disabled());
        let cache_manager = create_cache_manager().await;
        let handler =
            SearchCodeHandler::new(search_service, auth_handler, resource_limits, cache_manager);

        let args = SearchCodeArgs {
            query: "find error handling functions".to_string(),
            limit: 10,
            token: None,
            filters: None,
            collection: None,
            extensions: None,
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        // Should show search results (even if empty)
        assert!(
            text.contains("Search") || text.contains("Results"),
            "Expected search response, got: {}",
            text
        );
    }

    #[tokio::test]
    async fn test_search_code_limit_clamped() {
        let search_service = create_search_service().await;
        let auth_handler = Arc::new(create_auth_handler_disabled());
        let resource_limits = Arc::new(create_resource_limits_disabled());
        let cache_manager = create_cache_manager().await;
        let handler =
            SearchCodeHandler::new(search_service, auth_handler, resource_limits, cache_manager);

        // Test with limit above maximum (should be clamped to 50)
        let args = SearchCodeArgs {
            query: "test query".to_string(),
            limit: 1000, // Above max
            token: None,
            filters: None,
            collection: None,
            extensions: None,
        };

        let result = handler.handle(Parameters(args)).await;

        // Should succeed - limit is silently clamped
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_search_code_query_too_long() {
        let search_service = create_search_service().await;
        let auth_handler = Arc::new(create_auth_handler_disabled());
        let resource_limits = Arc::new(create_resource_limits_disabled());
        let cache_manager = create_cache_manager().await;
        let handler =
            SearchCodeHandler::new(search_service, auth_handler, resource_limits, cache_manager);

        // Create a query longer than 1000 characters
        let long_query = "a".repeat(1001);
        let args = SearchCodeArgs {
            query: long_query,
            limit: 10,
            token: None,
            filters: None,
            collection: None,
            extensions: None,
        };

        let result = handler.handle(Parameters(args)).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        let text = extract_text(&response);

        assert!(text.contains("Error"));
        assert!(text.contains("long") || text.contains("1000"));
    }
}
