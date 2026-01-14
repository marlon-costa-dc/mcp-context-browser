//! View models for web templates - bridge between service layer and presentation
//!
//! These DTOs are specifically designed for template rendering, with:
//! - Pre-computed CSS classes for UI styling
//! - Pre-formatted strings for display
//! - Flat structures optimized for Tera template access

use serde::Serialize;

use crate::infrastructure::utils::{css, FormattingUtils, StatusUtils, StringUtils};

// =============================================================================
// Dashboard View Models
// =============================================================================

/// Complete dashboard view model - aggregates data from multiple service calls
#[derive(Debug, Clone, Serialize)]
pub struct DashboardViewModel {
    /// Page
    pub page: &'static str,
    /// Metrics
    pub metrics: MetricsViewModel,
    /// Providers
    pub providers: ProvidersViewModel,
    /// Indexes
    pub indexes: IndexesSummaryViewModel,
    /// Collection of activities items
    pub activities: Vec<ActivityViewModel>,
    /// System Health
    pub system_health: HealthViewModel,
}

/// System performance metrics
#[derive(Debug, Clone, Serialize)]
pub struct MetricsViewModel {
    /// Cpu Usage
    pub cpu_usage: f64,
    /// Cpu Usage Formatted
    pub cpu_usage_formatted: String,
    /// Memory Usage
    pub memory_usage: f64,
    /// Memory Usage Formatted
    pub memory_usage_formatted: String,
    /// Total Queries
    pub total_queries: u64,
    /// Total Queries Formatted
    pub total_queries_formatted: String,
    /// Avg Latency Ms
    pub avg_latency_ms: f64,
    /// Avg Latency Formatted
    pub avg_latency_formatted: String,
}

impl MetricsViewModel {
    pub fn new(cpu_usage: f64, memory_usage: f64, total_queries: u64, avg_latency_ms: f64) -> Self {
        Self {
            cpu_usage,
            cpu_usage_formatted: FormattingUtils::format_percentage_raw(cpu_usage),
            memory_usage,
            memory_usage_formatted: FormattingUtils::format_percentage_raw(memory_usage),
            total_queries,
            total_queries_formatted: FormattingUtils::format_number(total_queries),
            avg_latency_ms,
            avg_latency_formatted: format!("{:.1}ms", avg_latency_ms),
        }
    }
}

// =============================================================================
// Providers View Models
// =============================================================================

/// Provider list view model with summary counts
#[derive(Debug, Clone, Serialize)]
pub struct ProvidersViewModel {
    /// Page
    pub page: &'static str,
    /// Active Count
    pub active_count: usize,
    /// Total Count
    pub total_count: usize,
    /// Collection of providers items
    pub providers: Vec<ProviderViewModel>,
}

impl ProvidersViewModel {
    pub fn new(providers: Vec<ProviderViewModel>) -> Self {
        let active_count = providers.iter().filter(|p| p.is_active).count();
        let total_count = providers.len();
        Self {
            page: "providers",
            active_count,
            total_count,
            providers,
        }
    }
}

/// Individual provider view model
#[derive(Debug, Clone, Serialize)]
pub struct ProviderViewModel {
    /// Id
    pub id: String,
    /// Name
    pub name: String,
    /// Provider Type
    pub provider_type: String,
    /// Provider Type Display
    pub provider_type_display: String,
    /// Status
    pub status: String,
    /// Status Display
    pub status_display: String,
    /// Status Class
    pub status_class: &'static str,
    /// Is Active
    pub is_active: bool,
}

impl ProviderViewModel {
    pub fn new(id: String, name: String, provider_type: String, status: String) -> Self {
        let is_active = StatusUtils::is_healthy(&status);
        let status_class = css::badge_for_status(&status);
        let provider_type_display = StringUtils::to_title_case(&provider_type);
        let status_display = StringUtils::capitalize_first(&status);

        Self {
            id,
            name,
            provider_type,
            provider_type_display,
            status,
            status_display,
            status_class,
            is_active,
        }
    }
}

// =============================================================================
// Indexes View Models
// =============================================================================

/// Index list view model for indexes page
#[derive(Debug, Clone, Serialize)]
pub struct IndexesViewModel {
    /// Page
    pub page: &'static str,
    /// Collection of indexes items
    pub indexes: Vec<IndexViewModel>,
    /// Total Documents
    pub total_documents: u64,
    /// Total Documents Formatted
    pub total_documents_formatted: String,
    /// Active Count
    pub active_count: usize,
}

impl IndexesViewModel {
    pub fn new(indexes: Vec<IndexViewModel>, total_documents: u64) -> Self {
        let active_count = indexes.iter().filter(|i| i.is_active).count();
        Self {
            page: "indexes",
            indexes,
            total_documents,
            total_documents_formatted: FormattingUtils::format_number(total_documents),
            active_count,
        }
    }
}

/// Summary view model for dashboard
#[derive(Debug, Clone, Serialize)]
pub struct IndexesSummaryViewModel {
    /// Active Count
    pub active_count: usize,
    /// Total Documents
    pub total_documents: u64,
    /// Total Documents Formatted
    pub total_documents_formatted: String,
    /// Is Indexing
    pub is_indexing: bool,
}

/// Individual index view model
#[derive(Debug, Clone, Serialize)]
pub struct IndexViewModel {
    /// Id
    pub id: String,
    /// Name
    pub name: String,
    /// Status
    pub status: String,
    /// Status Display
    pub status_display: String,
    /// Status Class
    pub status_class: &'static str,
    /// Is Active
    pub is_active: bool,
    /// Is Indexing
    pub is_indexing: bool,
    /// Document Count
    pub document_count: u64,
    /// Document Count Formatted
    pub document_count_formatted: String,
    /// Created At
    pub created_at: u64,
    /// Updated At
    pub updated_at: u64,
    /// Age Display
    pub age_display: String,
}

impl IndexViewModel {
    pub fn new(
        id: String,
        name: String,
        status: String,
        document_count: u64,
        created_at: u64,
        updated_at: u64,
    ) -> Self {
        let is_indexing = status == "indexing";
        let is_active = StatusUtils::is_healthy(&status);
        let status_class = css::badge_for_status(&status);
        let age_display = FormattingUtils::format_age(created_at);

        Self {
            id,
            name,
            status_display: StringUtils::capitalize_first(&status),
            status,
            status_class,
            is_active,
            is_indexing,
            document_count,
            document_count_formatted: FormattingUtils::format_number(document_count),
            created_at,
            updated_at,
            age_display,
        }
    }
}

// =============================================================================
// Activity View Models
// =============================================================================

/// Activity item view model for activity feed
#[derive(Debug, Clone, Serialize)]
pub struct ActivityViewModel {
    /// Id
    pub id: String,
    /// Message
    pub message: String,
    /// Timestamp
    pub timestamp: String,
    /// Timestamp Relative
    pub timestamp_relative: String,
    /// Level
    pub level: String,
    /// Level Class
    pub level_class: &'static str,
    /// Indicator Class
    pub indicator_class: &'static str,
    /// Category
    pub category: String,
}

impl ActivityViewModel {
    pub fn new(
        id: String,
        message: String,
        timestamp: chrono::DateTime<chrono::Utc>,
        level: &str,
        category: String,
    ) -> Self {
        let level_class = css::badge_for_level(level);
        let indicator_class = css::indicator_for_level(level);
        let timestamp_str = timestamp.format("%H:%M:%S").to_string();
        let timestamp_relative = StringUtils::format_relative_time(timestamp);

        Self {
            id,
            message,
            timestamp: timestamp_str,
            timestamp_relative,
            level: level.to_string(),
            level_class,
            indicator_class,
            category,
        }
    }
}

// =============================================================================
// Health View Models
// =============================================================================

/// System health view model
#[derive(Debug, Clone, Serialize)]
pub struct HealthViewModel {
    /// Status
    pub status: String,
    /// Status Display
    pub status_display: String,
    /// Status Class
    pub status_class: &'static str,
    /// Indicator Class
    pub indicator_class: &'static str,
    /// Uptime Seconds
    pub uptime_seconds: u64,
    /// Uptime Formatted
    pub uptime_formatted: String,
    /// Pid
    pub pid: u32,
}

impl HealthViewModel {
    pub fn new(status: &str, uptime_seconds: u64, pid: u32) -> Self {
        let status_class = css::badge_for_status(status);
        let indicator_class = css::indicator_for_status(status);

        Self {
            status: status.to_string(),
            status_display: StringUtils::capitalize_first(status),
            status_class,
            indicator_class,
            uptime_seconds,
            uptime_formatted: FormattingUtils::format_duration(uptime_seconds),
            pid,
        }
    }
}

// =============================================================================
// Configuration View Models
// =============================================================================

/// Configuration page view model
#[derive(Debug, Clone, Serialize)]
pub struct ConfigurationViewModel {
    /// Page
    pub page: &'static str,
    /// Page Description
    pub page_description: &'static str,
    /// Collection of categories items
    pub categories: Vec<ConfigCategoryViewModel>,
}

/// Configuration category view model
#[derive(Debug, Clone, Serialize)]
pub struct ConfigCategoryViewModel {
    /// Name
    pub name: String,
    /// Description
    pub description: String,
    /// Collection of settings items
    pub settings: Vec<ConfigSettingViewModel>,
}

/// Individual configuration setting view model
#[derive(Debug, Clone, Serialize)]
pub struct ConfigSettingViewModel {
    /// Key
    pub key: String,
    /// Label
    pub label: String,
    /// Value
    pub value: serde_json::Value,
    /// Value Display
    pub value_display: String,
    /// Setting Type
    pub setting_type: &'static str,
    /// Description
    pub description: String,
    /// Editable
    pub editable: bool,
}

// =============================================================================
// Logs View Models
// =============================================================================

/// Logs page view model
#[derive(Debug, Clone, Serialize)]
pub struct LogsViewModel {
    /// Page
    pub page: &'static str,
    /// Page Description
    pub page_description: &'static str,
    /// Collection of entries items
    pub entries: Vec<LogEntryViewModel>,
    /// Total Count
    pub total_count: u64,
    /// Stats
    pub stats: LogStatsViewModel,
}

/// Log entry view model
#[derive(Debug, Clone, Serialize)]
pub struct LogEntryViewModel {
    /// Timestamp
    pub timestamp: String,
    /// Level
    pub level: String,
    /// Level Class
    pub level_class: &'static str,
    /// Message
    pub message: String,
    /// Source
    pub source: String,
}

/// Log statistics view model
#[derive(Debug, Clone, Serialize)]
pub struct LogStatsViewModel {
    /// Total
    pub total: u64,
    /// Errors
    pub errors: u64,
    /// Warnings
    pub warnings: u64,
    /// Info
    pub info: u64,
}

// =============================================================================
// Error View Model
// =============================================================================

/// Error page view model
#[derive(Debug, Clone, Serialize)]
pub struct ErrorViewModel {
    /// Title
    pub title: String,
    /// Message
    pub message: String,
    /// Optional details value
    pub details: Option<String>,
    /// Back Url
    pub back_url: &'static str,
}

impl ErrorViewModel {
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            message: message.into(),
            details: None,
            back_url: "/dashboard",
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}
