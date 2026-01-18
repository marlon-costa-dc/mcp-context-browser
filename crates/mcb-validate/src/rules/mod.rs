//! Rule Registry System
//!
//! Provides declarative rule definitions and registry management.

pub mod registry;  // Legacy registry (kept for compatibility)
pub mod yaml_loader;
pub mod yaml_validator;
pub mod templates;

// Re-export legacy for compatibility
pub use registry::{clean_architecture_rules, layer_boundary_rules, Rule, RuleRegistry};

// Re-export YAML system
pub use yaml_loader::YamlRuleLoader;
pub use yaml_validator::YamlRuleValidator;
pub use templates::TemplateEngine;
