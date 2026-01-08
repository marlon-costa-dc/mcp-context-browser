use shaku::module;

use crate::admin::service::AdminServiceImpl;
use crate::server::server::{McpPerformanceMetrics, McpIndexingOperations};
use crate::metrics::system::SystemMetricsCollector;
use crate::di::factory::ServiceProvider;

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
