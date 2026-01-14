//! Analysis domain types
//!
//! Defines data structures for code analysis results

use std::path::PathBuf;

/// Complexity analysis report for a file
#[derive(Debug, Clone)]
pub struct ComplexityReport {
    /// File path analyzed
    pub path: PathBuf,
    /// Cyclomatic complexity (number of decision points)
    pub cyclomatic_complexity: f64,
    /// Cognitive complexity (understandability difficulty)
    pub cognitive_complexity: f64,
    /// Lines of code
    pub loc: usize,
    /// Nested level depth
    pub max_nesting_depth: usize,
}

/// Technical Debt Grade (A+ to F)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TdgGrade {
    /// Excellent (90-100%)
    APlus,
    /// Very Good (85-89%)
    A,
    /// Good (80-84%)
    BMinus,
    /// Above Average (70-79%)
    B,
    /// Average (60-69%)
    C,
    /// Below Average (50-59%)
    D,
    /// Poor (<50%)
    F,
}

impl std::fmt::Display for TdgGrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::APlus => "A+",
                Self::A => "A",
                Self::BMinus => "B-",
                Self::B => "B",
                Self::C => "C",
                Self::D => "D",
                Self::F => "F",
            }
        )
    }
}

/// Technical Debt Report
#[derive(Debug, Clone)]
pub struct TdgReport {
    /// Root path analyzed
    pub root: PathBuf,
    /// Overall TDG grade
    pub grade: TdgGrade,
    /// Grade percentage (0-100)
    pub score: f64,
    /// Average complexity across codebase
    pub avg_complexity: f64,
    /// Test coverage percentage
    pub test_coverage: f64,
    /// Documentation ratio
    pub documentation_ratio: f64,
    /// Code duplication percentage
    pub duplication_ratio: f64,
}

/// Comparison between TDG reports
#[derive(Debug, Clone)]
pub struct TdgComparison {
    /// Grade change (e.g., "A → B-")
    pub grade_change: String,
    /// Score change (delta)
    pub score_delta: f64,
    /// Complexity trend
    pub complexity_trend: String,
    /// Coverage trend
    pub coverage_trend: String,
}
