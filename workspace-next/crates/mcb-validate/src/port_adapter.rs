//! Port/Adapter Compliance Validation
//!
//! Validates Clean Architecture port/adapter patterns:
//! - Adapters should implement port traits
//! - Adapters should not call other adapters directly
//! - Port traits should follow Interface Segregation Principle (ISP)
//! - Port traits should be in correct location (mcb-application/ports/)

use crate::define_violations;
use crate::violation_trait::{Severity, Violation, ViolationCategory};
use crate::{Result, ValidationConfig};
use regex::Regex;
use std::path::PathBuf;
use walkdir::WalkDir;

define_violations! {
    ViolationCategory::Architecture,
    pub enum PortAdapterViolation {
        #[violation(
            id = "PORT001",
            severity = Warning,
            message = "Adapter {adapter_name} doesn't implement a port trait in {file}",
            suggestion = "Adapter structs should implement a trait from mcb-application/ports/"
        )]
        AdapterMissingPortImpl {
            adapter_name: String,
            file: PathBuf,
            line: usize,
        },

        #[violation(
            id = "PORT002",
            severity = Warning,
            message = "Adapter {adapter_name} directly uses another adapter {other_adapter}",
            suggestion = "Adapters should depend on port traits, not concrete adapter implementations"
        )]
        AdapterUsesAdapter {
            adapter_name: String,
            other_adapter: String,
            file: PathBuf,
            line: usize,
        },

        #[violation(
            id = "PORT003",
            severity = Info,
            message = "Port trait {trait_name} has {method_count} methods (recommended: 3-10 for ISP)",
            suggestion = "Consider splitting large port traits into smaller, more focused interfaces"
        )]
        PortTooLarge {
            trait_name: String,
            method_count: usize,
            file: PathBuf,
            line: usize,
        },

        #[violation(
            id = "PORT004",
            severity = Info,
            message = "Port trait {trait_name} has only {method_count} method(s)",
            suggestion = "Very small port traits may indicate over-fragmentation"
        )]
        PortTooSmall {
            trait_name: String,
            method_count: usize,
            file: PathBuf,
            line: usize,
        },

        #[violation(
            id = "PORT005",
            severity = Warning,
            message = "Port trait {trait_name} defined outside ports directory in {file}",
            suggestion = "Move port trait to mcb-application/src/ports/"
        )]
        PortWrongLocation {
            trait_name: String,
            file: PathBuf,
            line: usize,
        },
    }
}

/// Port/Adapter Compliance Validator
pub struct PortAdapterValidator;

impl Default for PortAdapterValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl PortAdapterValidator {
    /// Create a new port/adapter validator
    pub fn new() -> Self {
        Self
    }

    /// Validate all port/adapter patterns
    pub fn validate(&self, config: &ValidationConfig) -> Result<Vec<PortAdapterViolation>> {
        let mut violations = Vec::new();

        violations.extend(self.check_port_trait_sizes(config)?);
        violations.extend(self.check_adapter_direct_usage(config)?);

        Ok(violations)
    }

    /// Check that port traits follow ISP (3-10 methods recommended)
    fn check_port_trait_sizes(
        &self,
        config: &ValidationConfig,
    ) -> Result<Vec<PortAdapterViolation>> {
        let mut violations = Vec::new();
        let ports_dir = config.workspace_root.join("crates/mcb-application/src/ports");

        if !ports_dir.exists() {
            return Ok(violations);
        }

        let trait_start_re = Regex::new(r"pub\s+trait\s+(\w+)").expect("Invalid regex");
        let fn_re = Regex::new(r"^\s*(?:async\s+)?fn\s+\w+").expect("Invalid regex");

        for entry in WalkDir::new(&ports_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.extension().is_some_and(|e| e == "rs") {
                continue;
            }

            let content = std::fs::read_to_string(path)?;
            let lines: Vec<&str> = content.lines().collect();

            let mut current_trait: Option<(String, usize, usize)> = None; // (name, start_line, method_count)
            let mut brace_depth = 0;
            let mut in_trait = false;

            for (line_num, line) in lines.iter().enumerate() {
                // Look for trait definition
                if let Some(captures) = trait_start_re.captures(line) {
                    let trait_name = captures.get(1).map(|m| m.as_str().to_string()).unwrap();
                    current_trait = Some((trait_name, line_num + 1, 0));
                    in_trait = true;
                }

                // Track brace depth for trait scope
                if in_trait {
                    brace_depth += line.matches('{').count();
                    brace_depth -= line.matches('}').count();

                    // Count methods
                    if fn_re.is_match(line) {
                        if let Some((_, _, ref mut count)) = current_trait {
                            *count += 1;
                        }
                    }

                    // End of trait
                    if brace_depth == 0 && current_trait.is_some() {
                        let (trait_name, start_line, method_count) = current_trait.take().unwrap();
                        in_trait = false;

                        // ISP check: 3-10 methods recommended
                        if method_count > 10 {
                            violations.push(PortAdapterViolation::PortTooLarge {
                                trait_name,
                                method_count,
                                file: path.to_path_buf(),
                                line: start_line,
                            });
                        } else if method_count < 2 && method_count > 0 {
                            violations.push(PortAdapterViolation::PortTooSmall {
                                trait_name,
                                method_count,
                                file: path.to_path_buf(),
                                line: start_line,
                            });
                        }
                    }
                }
            }
        }

        Ok(violations)
    }

    /// Check that adapters don't directly use other adapters
    fn check_adapter_direct_usage(
        &self,
        config: &ValidationConfig,
    ) -> Result<Vec<PortAdapterViolation>> {
        let mut violations = Vec::new();
        let providers_dir = config.workspace_root.join("crates/mcb-providers/src");

        if !providers_dir.exists() {
            return Ok(violations);
        }

        // Common adapter suffixes
        let adapter_suffixes = ["Provider", "Repository", "Adapter", "Client", "Service"];
        let adapter_import_re = Regex::new(r"use\s+(?:crate|super)::(?:\w+::)*(\w+(?:Provider|Repository|Adapter|Client))").expect("Invalid regex");

        for entry in WalkDir::new(&providers_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.extension().is_some_and(|e| e == "rs") {
                continue;
            }

            // Skip mod.rs and lib.rs
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if file_name == "mod.rs" || file_name == "lib.rs" {
                continue;
            }

            let content = std::fs::read_to_string(path)?;

            // Determine current adapter name from file
            let current_adapter = file_name.trim_end_matches(".rs");

            for (line_num, line) in content.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with("//") {
                    continue;
                }

                // Check for imports of other adapters
                if let Some(captures) = adapter_import_re.captures(line) {
                    let imported = captures.get(1).map(|m| m.as_str()).unwrap_or("");

                    // Skip self-imports and trait imports
                    if imported.to_lowercase().contains(current_adapter) {
                        continue;
                    }

                    // Check if imported type looks like an adapter (not a trait)
                    for suffix in &adapter_suffixes {
                        if imported.ends_with(suffix) && !imported.starts_with("dyn") {
                            // This might be a concrete adapter import
                            violations.push(PortAdapterViolation::AdapterUsesAdapter {
                                adapter_name: current_adapter.to_string(),
                                other_adapter: imported.to_string(),
                                file: path.to_path_buf(),
                                line: line_num + 1,
                            });
                            break;
                        }
                    }
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
    fn test_trait_pattern() {
        let re = Regex::new(r"pub\s+trait\s+(\w+)").unwrap();
        assert!(re.is_match("pub trait EmbeddingProvider {"));
        assert!(re.is_match("pub trait CacheProvider: Interface + Send + Sync {"));

        let captures = re.captures("pub trait EmbeddingProvider {").unwrap();
        assert_eq!(captures.get(1).unwrap().as_str(), "EmbeddingProvider");
    }

    #[test]
    fn test_fn_pattern() {
        let re = Regex::new(r"^\s*(?:async\s+)?fn\s+\w+").unwrap();
        assert!(re.is_match("    fn get_name(&self) -> &str;"));
        assert!(re.is_match("    async fn embed(&self, text: &str) -> Result<Vec<f32>>;"));
        assert!(!re.is_match("// fn commented_out()"));
    }
}
