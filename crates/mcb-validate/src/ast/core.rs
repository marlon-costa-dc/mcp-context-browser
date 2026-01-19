//! AST Core Types Module
//!
//! Core data structures for representing AST nodes and parsing results.

use std::collections::HashMap;

/// Unified AST node representation across all languages
#[derive(Debug, Clone, PartialEq)]
pub struct AstNode {
    /// Node type (function, class, variable, etc.)
    pub kind: String,
    /// Node name (function name, variable name, etc.)
    pub name: Option<String>,
    /// Source code span (start/end positions)
    pub span: Span,
    /// Child nodes
    pub children: Vec<AstNode>,
    /// Additional metadata (language-specific)
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Source code position span
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

/// Position in source code
#[derive(Debug, Clone, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub byte_offset: usize,
}

/// AST parsing result
#[derive(Debug)]
pub struct AstParseResult {
    pub root: AstNode,
    pub errors: Vec<String>,
}
