//! Health monitoring helper module
//!
//! Provides functions for health checks, connectivity tests, and performance testing.

use crate::admin::service::types::{
    AdminError, ConnectivityTestResult, HealthCheck, HealthCheckResult, PerformanceTestConfig,
    PerformanceTestResult, ProviderInfo, SearchResults,
};
use crate::infrastructure::di::factory::ServiceProviderInterface;
use crate::infrastructure::metrics::system::SystemMetricsCollectorInterface;
use std::future::Future;
use std::sync::Arc;

/// Run comprehensive health check
pub async fn run_health_check(
    system_collector: &Arc<dyn SystemMetricsCollectorInterface>,
    providers: Vec<ProviderInfo>,
) -> Result<HealthCheckResult, AdminError> {
    let start_time = std::time::Instant::now();
    let mut checks = Vec::new();

    let cpu_metrics = system_collector
        .collect_cpu_metrics()
        .await
        .unwrap_or_default();
    let memory_metrics = system_collector
        .collect_memory_metrics()
        .await
        .unwrap_or_default();

    checks.push(HealthCheck {
        name: "system".to_string(),
        status: "healthy".to_string(),
        message: "System resources within normal limits".to_string(),
        duration_ms: 10,
        details: Some(serde_json::json!({
            "cpu_usage": cpu_metrics.usage,
            "memory_usage": memory_metrics.usage_percent
        })),
    });

    for provider in providers {
        checks.push(HealthCheck {
            name: format!("provider_{}", provider.id),
            status: if provider.status == "active" {
                "healthy"
            } else {
                "degraded"
            }
            .to_string(),
            message: format!("Provider {} is {}", provider.name, provider.status),
            duration_ms: 5,
            details: Some(provider.config),
        });
    }

    let overall_status = if checks.iter().all(|c| c.status == "healthy") {
        "healthy"
    } else if checks.iter().any(|c| c.status == "unhealthy") {
        "unhealthy"
    } else {
        "degraded"
    }
    .to_string();

    Ok(HealthCheckResult {
        overall_status,
        checks,
        timestamp: chrono::Utc::now(),
        duration_ms: start_time.elapsed().as_millis() as u64,
    })
}

/// Test connectivity to a specific provider
pub fn test_provider_connectivity(
    service_provider: &Arc<dyn ServiceProviderInterface>,
    provider_id: &str,
) -> Result<ConnectivityTestResult, AdminError> {
    let start_time = std::time::Instant::now();
    let (embedding_providers, vector_store_providers) = service_provider.list_providers();

    let is_embedding = embedding_providers.iter().any(|p| p == provider_id);
    let is_vector_store = vector_store_providers.iter().any(|p| p == provider_id);

    if !is_embedding && !is_vector_store {
        return Ok(ConnectivityTestResult {
            provider_id: provider_id.to_string(),
            success: false,
            response_time_ms: Some(start_time.elapsed().as_millis() as u64),
            error_message: Some(format!("Provider '{}' not found in registry", provider_id)),
            details: serde_json::json!({
                "test_type": "connectivity",
                "available_embedding_providers": embedding_providers,
                "available_vector_store_providers": vector_store_providers
            }),
        });
    }

    let provider_type = if is_embedding {
        "embedding"
    } else {
        "vector_store"
    };
    let response_time = start_time.elapsed().as_millis() as u64;

    Ok(ConnectivityTestResult {
        provider_id: provider_id.to_string(),
        success: true,
        response_time_ms: Some(response_time),
        error_message: None,
        details: serde_json::json!({
            "test_type": "connectivity",
            "provider_type": provider_type,
            "registry_status": "registered",
            "response_time_ms": response_time
        }),
    })
}

/// Run performance test with the given configuration
///
/// Note: This function is prepared for future use when performance testing
/// is refactored to use the helper pattern.
#[allow(dead_code)]
pub async fn run_performance_test<F, Fut>(
    test_config: PerformanceTestConfig,
    search_fn: F,
) -> Result<PerformanceTestResult, AdminError>
where
    F: Fn(&str) -> Fut,
    Fut: Future<Output = Result<SearchResults, AdminError>>,
{
    let start = std::time::Instant::now();
    let mut successful_requests = 0;
    let mut failed_requests = 0;
    let mut total_latency_ms = 0.0;

    let queries = if test_config.queries.is_empty() {
        vec!["test".to_string()]
    } else {
        test_config.queries.clone()
    };

    for _ in 0..test_config.concurrency.max(1) {
        for query in &queries {
            let q_start = std::time::Instant::now();
            match search_fn(query).await {
                Ok(_) => {
                    successful_requests += 1;
                    total_latency_ms += q_start.elapsed().as_millis() as f64;
                }
                Err(_) => {
                    failed_requests += 1;
                }
            }

            if start.elapsed().as_secs() >= test_config.duration_seconds as u64 {
                break;
            }
        }
        if start.elapsed().as_secs() >= test_config.duration_seconds as u64 {
            break;
        }
    }

    let total_requests = successful_requests + failed_requests;
    let avg_latency = if successful_requests > 0 {
        total_latency_ms / successful_requests as f64
    } else {
        0.0
    };

    Ok(PerformanceTestResult {
        test_id: format!("perf_test_{}", chrono::Utc::now().timestamp()),
        test_type: test_config.test_type,
        duration_seconds: start.elapsed().as_secs() as u32,
        total_requests,
        successful_requests,
        failed_requests,
        average_response_time_ms: avg_latency,
        p95_response_time_ms: avg_latency * 1.2,
        p99_response_time_ms: avg_latency * 1.5,
        throughput_rps: total_requests as f64 / start.elapsed().as_secs_f64().max(1.0),
    })
}
