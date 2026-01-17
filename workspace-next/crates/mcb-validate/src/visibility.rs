//! Module Visibility Validation
//!
//! Validates proper use of pub(crate), pub, and private visibility:
//! - Internal helpers should use pub(crate), not pub
//! - Only port traits and public API should be pub
//! - Domain layer should minimize pub exports

use crate::define_violations;
use crate::violation_trait::{Severity, Violation, ViolationCategory};
use crate::{Result, ValidationConfig};
use regex::Regex;
use std::path::PathBuf;
use walkdir::WalkDir;

define_violations! {
    ViolationCategory::Organization,
    pub enum VisibilityViolation {
        #[violation(
            id = "VIS001",
            severity = Info,
            message = "Internal helper {item_name} is pub in {file} - consider pub(crate)",
            suggestion = "Use pub(crate) for internal helpers to prevent accidental external usage"
        )]
        InternalHelperTooPublic {
            item_name: String,
            file: PathBuf,
            line: usize,
        },

        #[violation(
            id = "VIS002",
            severity = Warning,
            message = "Domain type {type_name} has pub(crate) - should be pub for external use",
            suggestion = "Domain types are typically part of public API and should use pub"
        )]
        DomainTypeTooRestricted {
            type_name: String,
            file: PathBuf,
            line: usize,
        },

        #[violation(
            id = "VIS003",
            severity = Info,
            message = "Utility module {module_name} has pub items - consider pub(crate)",
            suggestion = "Utility modules are usually internal and should use pub(crate)"
        )]
        UtilityModuleTooPublic {
            module_name: String,
            file: PathBuf,
            line: usize,
        },
    }
}

/// Visibility Validator
pub struct VisibilityValidator;

impl Default for VisibilityValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl VisibilityValidator {
    /// Create a new visibility validator
    pub fn new() -> Self {
        Self
    }

    /// Validate all visibility patterns
    pub fn validate(&self, config: &ValidationConfig) -> Result<Vec<VisibilityViolation>> {
        let mut violations = Vec::new();

        violations.extend(self.check_internal_helpers(config)?);
        violations.extend(self.check_utility_modules(config)?);

        Ok(violations)
    }

    /// Check that internal helpers use pub(crate)
    fn check_internal_helpers(
        &self,
        config: &ValidationConfig,
    ) -> Result<Vec<VisibilityViolation>> {
        let mut violations = Vec::new();

        // Directories that typically contain internal helpers
        let internal_dirs = [
            "crates/mcb-infrastructure/src/utils",
            "crates/mcb-providers/src/utils",
            "crates/mcb-server/src/utils",
            "crates/mcb-application/src/utils",
        ];

        let pub_item_re = Regex::new(r"^pub\s+(fn|struct|enum|type|const|static)\s+(\w+)").expect("Invalid regex");
        let pub_crate_re = Regex::new(r"^pub\(crate\)").expect("Invalid regex");

        for dir_path in &internal_dirs {
            let full_path = config.workspace_root.join(dir_path);
            if !full_path.exists() {
                continue;
            }

            for entry in WalkDir::new(&full_path)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if !path.extension().is_some_and(|e| e == "rs") {
                    continue;
                }

                let content = std::fs::read_to_string(path)?;

                for (line_num, line) in content.lines().enumerate() {
                    let trimmed = line.trim();

                    // Skip if already pub(crate)
                    if pub_crate_re.is_match(trimmed) {
                        continue;
                    }

                    // Check for pub items
                    if let Some(captures) = pub_item_re.captures(trimmed) {
                        let item_name = captures.get(2).map(|m| m.as_str()).unwrap_or("unknown");

                        // Skip common public items
                        if item_name.starts_with("Error") || item_name == "Result" {
                            continue;
                        }

                        violations.push(VisibilityViolation::InternalHelperTooPublic {
                            item_name: item_name.to_string(),
                            file: path.to_path_buf(),
                            line: line_num + 1,
                        });
                    }
                }
            }
        }

        Ok(violations)
    }

    /// Check utility modules for excessive public visibility
    fn check_utility_modules(&self, config: &ValidationConfig) -> Result<Vec<VisibilityViolation>> {
        let mut violations = Vec::new();

        // Files that are typically internal utilities
        let utility_patterns = ["common.rs", "helpers.rs", "internal.rs", "compat.rs"];

        let pub_item_re = Regex::new(r"^pub\s+(fn|struct|enum|type)\s+(\w+)").expect("Invalid regex");
        let pub_crate_re = Regex::new(r"^pub\(crate\)").expect("Invalid regex");

        for crate_name in ["mcb-infrastructure", "mcb-providers", "mcb-server", "mcb-application"] {
            let crate_src = config.workspace_root.join("crates").join(crate_name).join("src");
            if !crate_src.exists() {
                continue;
            }

            for entry in WalkDir::new(&crate_src)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if !path.extension().is_some_and(|e| e == "rs") {
                    continue;
                }

                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                // Check if this is a utility file
                if !utility_patterns.iter().any(|p| file_name == *p) {
                    continue;
                }

                let content = std::fs::read_to_string(path)?;
                let mut pub_count = 0;

                for line in content.lines() {
                    let trimmed = line.trim();

                    // Skip pub(crate)
                    if pub_crate_re.is_match(trimmed) {
                        continue;
                    }

                    if pub_item_re.is_match(trimmed) {
                        pub_count += 1;
                    }
                }

                // If more than 3 pub items in a utility file, flag it
                if pub_count > 3 {
                    violations.push(VisibilityViolation::UtilityModuleTooPublic {
                        module_name: file_name.trim_end_matches(".rs").to_string(),
                        file: path.to_path_buf(),
                        line: 1,
                    });
                }
            }
        }

        Ok(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pub_item_pattern() {
        let re = Regex::new(r"^pub\s+(fn|struct|enum|type|const|static)\s+(\w+)").unwrap();

        assert!(re.is_match("pub fn helper() {}"));
        assert!(re.is_match("pub struct Config {}"));
        assert!(re.is_match("pub enum Status {}"));
        assert!(!re.is_match("pub(crate) fn internal() {}"));
        assert!(!re.is_match("fn private() {}"));
    }

    #[test]
    fn test_pub_crate_pattern() {
        let re = Regex::new(r"^pub\(crate\)").unwrap();

        assert!(re.is_match("pub(crate) fn internal() {}"));
        assert!(re.is_match("pub(crate) struct Helper {}"));
        assert!(!re.is_match("pub fn external() {}"));
    }
}
