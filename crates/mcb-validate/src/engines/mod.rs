//! Hybrid Rule Engines
//!
//! Provides a unified interface for multiple rule engines:
//! - rust-rule-engine: RETE-UL algorithm with GRL syntax
//! - rusty-rules: JSON DSL with composition (all/any/not)
//! - validator/garde: Field-level validations

pub mod hybrid_engine;
pub mod rust_rule_engine;
pub mod rusty_rules_engine;
pub mod validator_engine;

pub use hybrid_engine::{HybridRuleEngine, RuleEngineType};
pub use rust_rule_engine::RustRuleEngineWrapper;
pub use rusty_rules_engine::RustyRulesEngineWrapper;
pub use validator_engine::ValidatorEngine;