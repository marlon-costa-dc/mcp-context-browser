//! Analysis Domain
//!
//! Future domain for code analysis capabilities (v0.3.0+)
//!
//! This domain will handle:
//! - Complexity analysis (cyclomatic, cognitive)
//! - Technical debt detection and grading
//! - SATD (Self-Admitted Technical Debt) detection
//! - Code quality metrics
//!
//! **v0.2.0**: Trait definitions only
//! **v0.3.0+**: Implementations added

pub mod ports;
pub mod types;

pub use ports::{ComplexityAnalysisInterface, TechnicalDebtInterface};
pub use types::{ComplexityReport, TdgGrade, TdgReport, TdgComparison};
