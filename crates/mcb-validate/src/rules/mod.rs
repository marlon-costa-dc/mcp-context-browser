//! Rule Registry System
//!
//! Provides declarative rule definitions and registry management.

pub mod registry;  // Legacy registry (kept for compatibility)

// Re-export legacy for compatibility
pub use registry::{clean_architecture_rules, layer_boundary_rules, Rule, RuleRegistry};
