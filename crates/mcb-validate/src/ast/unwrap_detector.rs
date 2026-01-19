//! AST-based Unwrap Detector
//!
//! Uses Tree-sitter to detect `.unwrap()` and `.expect()` calls in Rust code,
//! replacing the Regex-based approach in quality.rs.
//!
//! This module implements Phase 2 deliverable:
//! "QUAL001 (no-unwrap) detects `.unwrap()` calls via AST"

use std::path::Path;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

use crate::{Result, ValidationError};

/// Detection result for unwrap/expect usage
#[derive(Debug, Clone)]
pub struct UnwrapDetection {
    /// File where the detection occurred
    pub file: String,
    /// Line number (1-based)
    pub line: usize,
    /// Column number (1-based)
    pub column: usize,
    /// The specific method detected ("unwrap", "expect", "unwrap_or", etc.)
    pub method: String,
    /// Whether this is in a test module
    pub in_test: bool,
    /// The source text of the method call
    pub context: String,
}

/// AST-based unwrap detector using Tree-sitter
pub struct UnwrapDetector {
    parser: Parser,
    unwrap_query: Query,
    test_module_query: Query,
}

impl UnwrapDetector {
    /// Create a new unwrap detector
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_rust::LANGUAGE.into())
            .map_err(|e| ValidationError::Config(format!("Failed to set Rust language: {e}")))?;

        // Query for method calls like .unwrap(), .expect(), etc.
        // This matches field_expression -> call_expression patterns
        //
        // Tree-sitter Rust structure for `x.unwrap()`:
        // (call_expression
        //   function: (field_expression
        //     field: (field_identifier) @method_name)
        //   arguments: (arguments))
        let unwrap_query = Query::new(
            &tree_sitter_rust::LANGUAGE.into(),
            r#"
            (call_expression
              function: (field_expression
                field: (field_identifier) @method_name)
              arguments: (arguments)) @call
            "#,
        )
        .map_err(|e| ValidationError::Config(format!("Failed to compile unwrap query: {e}")))?;

        // Query to detect test modules - simpler pattern
        // We look for mod items and check their attributes separately
        let test_module_query = Query::new(
            &tree_sitter_rust::LANGUAGE.into(),
            r#"
            (mod_item
              name: (identifier) @mod_name) @test_mod
            "#,
        )
        .map_err(|e| {
            ValidationError::Config(format!("Failed to compile test module query: {e}"))
        })?;

        Ok(Self {
            parser,
            unwrap_query,
            test_module_query,
        })
    }

    /// Detect unwrap/expect calls in Rust source code
    pub fn detect_in_content(&mut self, content: &str, filename: &str) -> Result<Vec<UnwrapDetection>> {
        let tree = self.parser.parse(content, None).ok_or_else(|| {
            ValidationError::Parse {
                file: filename.into(),
                message: "Failed to parse Rust code".into(),
            }
        })?;

        let root = tree.root_node();
        let source_bytes = content.as_bytes();

        // First, find all test module ranges
        let test_ranges = self.find_test_module_ranges(&root, source_bytes);

        // Then, find all method calls
        let mut detections = Vec::new();
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&self.unwrap_query, root, source_bytes);

        // Find the capture indices
        let method_name_idx = self
            .unwrap_query
            .capture_index_for_name("method_name")
            .expect("method_name capture should exist");
        let call_idx = self
            .unwrap_query
            .capture_index_for_name("call")
            .expect("call capture should exist");

        // Use StreamingIterator pattern for tree-sitter 0.26+
        while let Some(match_) = matches.next() {
            let mut method_name = None;
            let mut call_node = None;

            for capture in match_.captures {
                if capture.index == method_name_idx {
                    method_name = Some(capture.node);
                } else if capture.index == call_idx {
                    call_node = Some(capture.node);
                }
            }

            if let (Some(method_node), Some(call)) = (method_name, call_node) {
                let method = method_node
                    .utf8_text(source_bytes)
                    .unwrap_or("")
                    .to_string();

                // Check if this is an unwrap-family method
                if self.is_target_method(&method) {
                    let start_pos = call.start_position();
                    let line = start_pos.row + 1; // 1-based
                    let column = start_pos.column + 1;

                    // Check if inside a test module
                    let byte_offset = call.start_byte();
                    let in_test = test_ranges.iter().any(|(start, end)| {
                        byte_offset >= *start && byte_offset < *end
                    });

                    // Get context (the source text of the call)
                    let context = call
                        .utf8_text(source_bytes)
                        .unwrap_or("")
                        .lines()
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();

                    detections.push(UnwrapDetection {
                        file: filename.to_string(),
                        line,
                        column,
                        method,
                        in_test,
                        context,
                    });
                }
            }
        }

        Ok(detections)
    }

    /// Detect unwrap/expect calls in a file
    pub fn detect_in_file(&mut self, path: &Path) -> Result<Vec<UnwrapDetection>> {
        let content = std::fs::read_to_string(path)?;
        self.detect_in_content(&content, &path.to_string_lossy())
    }

    /// Check if a method name is one we want to detect
    fn is_target_method(&self, method: &str) -> bool {
        // Target methods that indicate potential panics
        matches!(method, "unwrap" | "expect")
    }

    /// Check if a method is a safe alternative
    #[allow(dead_code)]
    fn is_safe_alternative(&self, method: &str) -> bool {
        matches!(
            method,
            "unwrap_or" | "unwrap_or_else" | "unwrap_or_default"
        )
    }

    /// Find byte ranges of test modules
    fn find_test_module_ranges(
        &self,
        root: &tree_sitter::Node,
        source_bytes: &[u8],
    ) -> Vec<(usize, usize)> {
        let mut ranges = Vec::new();
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&self.test_module_query, *root, source_bytes);

        let test_mod_idx = self
            .test_module_query
            .capture_index_for_name("test_mod")
            .expect("test_mod capture should exist");

        // Use StreamingIterator pattern for tree-sitter 0.26+
        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                if capture.index == test_mod_idx {
                    let mod_node = capture.node;
                    // Check if this module has #[cfg(test)] by looking at its source
                    // We look backward from the mod declaration for #[cfg(test)]
                    let mod_start = mod_node.start_byte();

                    // Check if there's a #[cfg(test)] attribute before this mod
                    // by looking at the previous sibling
                    if let Some(prev) = mod_node.prev_sibling() {
                        let prev_text = prev.utf8_text(source_bytes).unwrap_or("");
                        if prev_text.contains("#[cfg(test)]") {
                            ranges.push((mod_node.start_byte(), mod_node.end_byte()));
                            continue;
                        }
                    }

                    // Also check the text before the mod for inline attributes
                    if mod_start > 20 {
                        let before = std::str::from_utf8(&source_bytes[mod_start.saturating_sub(50)..mod_start])
                            .unwrap_or("");
                        if before.contains("#[cfg(test)]") {
                            ranges.push((mod_node.start_byte(), mod_node.end_byte()));
                        }
                    }
                }
            }
        }

        ranges
    }
}

impl Default for UnwrapDetector {
    fn default() -> Self {
        Self::new().expect("Failed to create UnwrapDetector")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_creation() {
        let detector = UnwrapDetector::new();
        assert!(detector.is_ok(), "Should create detector successfully");
    }

    #[test]
    fn test_detect_unwrap_simple() {
        let mut detector = UnwrapDetector::new().expect("Should create detector");
        let code = r#"
fn main() {
    let x = Some(1).unwrap();
}
"#;

        let detections = detector
            .detect_in_content(code, "test.rs")
            .expect("Should detect");
        assert_eq!(detections.len(), 1, "Should find one unwrap");
        assert_eq!(detections[0].method, "unwrap");
        assert!(!detections[0].in_test, "Should not be in test module");
    }

    #[test]
    fn test_detect_expect() {
        let mut detector = UnwrapDetector::new().expect("Should create detector");
        let code = r#"
fn process() {
    let x = Some(1).expect("should have value");
}
"#;

        let detections = detector
            .detect_in_content(code, "test.rs")
            .expect("Should detect");
        assert_eq!(detections.len(), 1, "Should find one expect");
        assert_eq!(detections[0].method, "expect");
    }

    #[test]
    fn test_detect_multiple() {
        let mut detector = UnwrapDetector::new().expect("Should create detector");
        let code = r#"
fn risky() {
    let a = Some(1).unwrap();
    let b = "test".parse::<i32>().unwrap();
    let c = std::env::var("TEST").expect("TEST must be set");
}
"#;

        let detections = detector
            .detect_in_content(code, "test.rs")
            .expect("Should detect");
        assert_eq!(detections.len(), 3, "Should find three detections");
    }

    #[test]
    fn test_ignore_safe_alternatives() {
        let mut detector = UnwrapDetector::new().expect("Should create detector");
        let code = r#"
fn safe() {
    let a = Some(1).unwrap_or(0);
    let b = Some(2).unwrap_or_else(|| 0);
    let c = Some(3).unwrap_or_default();
}
"#;

        let detections = detector
            .detect_in_content(code, "test.rs")
            .expect("Should detect");
        assert_eq!(detections.len(), 0, "Should not detect safe alternatives");
    }

    #[test]
    fn test_detect_in_test_module() {
        let mut detector = UnwrapDetector::new().expect("Should create detector");
        let code = r#"
fn production() {
    let x = Some(1).unwrap(); // Should detect
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        let x = Some(1).unwrap(); // Should mark as in_test
    }
}
"#;

        let detections = detector
            .detect_in_content(code, "test.rs")
            .expect("Should detect");

        // Should find both, but one is in test
        let production: Vec<_> = detections.iter().filter(|d| !d.in_test).collect();
        let test_code: Vec<_> = detections.iter().filter(|d| d.in_test).collect();

        assert_eq!(production.len(), 1, "Should find one production unwrap");
        assert_eq!(test_code.len(), 1, "Should find one test unwrap");
    }

    #[test]
    fn test_line_numbers_are_correct() {
        let mut detector = UnwrapDetector::new().expect("Should create detector");
        let code = "fn main() {\n    let x = Some(1).unwrap();\n}\n";

        let detections = detector
            .detect_in_content(code, "test.rs")
            .expect("Should detect");

        assert_eq!(detections.len(), 1);
        assert_eq!(detections[0].line, 2, "Should be on line 2");
    }
}
