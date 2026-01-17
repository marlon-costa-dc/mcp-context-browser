//! Analysis adapters
//!
//! Thin wrappers around PMAT algorithms to integrate with MCB architecture
//!
//! **Pattern**:
//! ```
//! PMAT Algorithm (libs/code-metrics/)
//!     ↓
//! Adapter (src/adapters/analyzers/)
//!     ↓
//! MCB Service (src/application/analysis/)
//! ```
//!
//! **v0.2.0**: Trait definitions only
//! **v0.3.0**: Implementations added

use async_trait::async_trait;
use std::path::Path;
use crate::domain::types::Result;

/// Generic adapter trait for analysis tools
///
/// Adapters provide thin type conversion wrappers around PMAT algorithms
/// while keeping the original PMAT code unchanged (100% reusable).
#[async_trait]
pub trait AnalysisAdapter: Send + Sync {
    /// PMAT's output type (from libs/code-metrics)
    type PmatType;

    /// MCB's domain type (from src/domain/analysis)
    type McbType;

    /// Execute analysis, converting PMAT type → MCB type
    async fn execute(&self, input: &Path) -> Result<Self::McbType>;
}

// Future adapter implementations (v0.3.0+)
//
// pub struct ComplexityAnalyzerAdapter;
// impl AnalysisAdapter for ComplexityAnalyzerAdapter {
//     type PmatType = /* PMAT ComplexityMetrics */;
//     type McbType = ComplexityReport;
//     async fn execute(&self, path: &Path) -> Result<ComplexityReport> { /* ... */ }
// }
//
// pub struct TdgScorerAdapter;
// impl AnalysisAdapter for TdgScorerAdapter {
//     type PmatType = /* PMAT TdgScore */;
//     type McbType = TdgReport;
//     async fn execute(&self, path: &Path) -> Result<TdgReport> { /* ... */ }
// }
//
// pub struct SatdDetectorAdapter;
// impl AnalysisAdapter for SatdDetectorAdapter {
//     type PmatType = Vec</* PMAT SATDMarker */>;
//     type McbType = Vec<SATDMarker>;
//     async fn execute(&self, path: &Path) -> Result<Vec<SATDMarker>> { /* ... */ }
// }
