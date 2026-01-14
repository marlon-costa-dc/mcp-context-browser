//! Analysis domain port traits
//!
//! Defines abstract interfaces for code analysis capabilities.
//! Implementations will be added in v0.3.0+

use async_trait::async_trait;
use std::path::Path;
use crate::domain::types::Result;

use super::types::{ComplexityReport, TdgGrade, TdgReport, TdgComparison};

/// Complexity analysis interface
///
/// Analyzes code complexity using multiple metrics:
/// - Cyclomatic complexity (decision points)
/// - Cognitive complexity (understandability)
#[async_trait]
pub trait ComplexityAnalysisInterface: Send + Sync {
    /// Analyze complexity of a single file
    async fn analyze_file(&self, path: &Path) -> Result<ComplexityReport>;

    /// Analyze complexity across an entire codebase
    async fn analyze_codebase(&self, root: &Path) -> Result<Vec<ComplexityReport>>;
}

/// Technical Debt Analysis interface
///
/// Analyzes and grades technical debt using multiple dimensions:
/// - Code complexity
/// - Test coverage
/// - Documentation completeness
/// - Code duplication
#[async_trait]
pub trait TechnicalDebtInterface: Send + Sync {
    /// Calculate Technical Debt Grade (TDG) for a codebase
    ///
    /// Returns a grade from A+ (excellent) to F (critical debt)
    async fn calculate_tdg_grade(&self, codebase: &Path) -> Result<TdgGrade>;

    /// Compare TDG against a baseline
    async fn compare_tdg(&self, baseline: &TdgReport, current: &TdgReport) -> Result<TdgComparison>;

    /// Get detailed TDG report
    async fn generate_tdg_report(&self, codebase: &Path) -> Result<TdgReport>;
}

/// Self-Admitted Technical Debt interface (v0.3.0+)
///
/// Detects SATD markers (TODO, FIXME, HACK, etc.) in code
#[async_trait]
pub trait SATDDetectionInterface: Send + Sync {
    /// Detect SATD markers in a file
    async fn detect_satd(&self, path: &Path) -> Result<Vec<SATDMarker>>;

    /// Scan entire codebase for SATD
    async fn scan_codebase(&self, root: &Path) -> Result<SATDReport>;
}

/// SATD marker found in code
#[derive(Debug, Clone)]
pub struct SATDMarker {
    /// Line number
    pub line: usize,
    /// Marker type (TODO, FIXME, HACK, etc.)
    pub marker_type: String,
    /// Associated comment
    pub comment: String,
}

/// SATD scan report
#[derive(Debug, Clone)]
pub struct SATDReport {
    /// Total markers found
    pub total_markers: usize,
    /// Markers by type
    pub by_type: std::collections::HashMap<String, usize>,
}
