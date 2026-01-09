use shaku::module;

use crate::admin::service::AdminServiceImpl;
use crate::infrastructure::di::factory::ServiceProvider;
use crate::infrastructure::metrics::system::SystemMetricsCollector;
use crate::server::mcp_server::{McpIndexingOperations, McpPerformanceMetrics};

module! {
    pub McpModule {
        components = [
            AdminServiceImpl,
            McpPerformanceMetrics,
            McpIndexingOperations,
            SystemMetricsCollector,
            ServiceProvider
        ],
        providers = []
    }
}
