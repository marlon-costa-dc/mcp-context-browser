//! AST Types Module
//!
//! Additional AST-related types and violations.

use super::core::AstNode;

/// AST-based violation
#[derive(Debug, Clone)]
pub struct AstViolation {
    pub rule_id: String,
    pub file: String,
    pub node: AstNode,
    pub message: String,
    pub severity: String,
}
