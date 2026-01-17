//! Layer Event Flow Validation
//!
//! Validates that dependencies flow in correct Clean Architecture direction:
//! mcb-domain -> mcb-application -> mcb-providers -> mcb-infrastructure -> mcb-server
//!
//! # Forbidden Dependencies
//!
//! - mcb-domain -> ANY other crate (pure domain, zero external deps)
//! - mcb-application -> mcb-providers (ports don't depend on adapters)
//! - mcb-application -> mcb-infrastructure (application doesn't know infra)
//! - mcb-providers -> mcb-infrastructure (adapters don't depend on composition)
//! - mcb-providers -> mcb-server (adapters don't know presentation)

use crate::define_violations;
use crate::violation_trait::{Severity, Violation, ViolationCategory};
use crate::{Result, ValidationConfig};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use walkdir::WalkDir;

define_violations! {
    ViolationCategory::Architecture,
    pub enum LayerFlowViolation {
        #[violation(
            id = "LAYER001",
            severity = Error,
            message = "Forbidden import in {source_crate}: {import_path}",
            suggestion = "Remove dependency on {target_crate} from {source_crate} - violates Clean Architecture"
        )]
        ForbiddenDependency {
            source_crate: String,
            target_crate: String,
            import_path: String,
            location: PathBuf,
            line: usize,
        },

        #[violation(
            id = "LAYER002",
            severity = Error,
            message = "Circular dependency detected: {crate_a} <-> {crate_b}",
            suggestion = "Break circular dependency by extracting shared types to mcb-domain"
        )]
        CircularDependency {
            crate_a: String,
            crate_b: String,
            location: PathBuf,
            line: usize,
        },

        #[violation(
            id = "LAYER003",
            severity = Warning,
            message = "Domain crate {crate_name} imports external crate: {external_crate}",
            suggestion = "Domain layer should only depend on std and core serialization crates"
        )]
        DomainExternalDependency {
            crate_name: String,
            external_crate: String,
            location: PathBuf,
            line: usize,
        },
    }
}

/// Defines allowed dependencies between crates in Clean Architecture
struct LayerRules {
    /// Map from crate name to set of allowed dependencies
    allowed: HashMap<&'static str, HashSet<&'static str>>,
    /// Map from crate name to set of forbidden dependencies
    forbidden: HashMap<&'static str, HashSet<&'static str>>,
}

impl Default for LayerRules {
    fn default() -> Self {
        let mut allowed = HashMap::new();
        let mut forbidden = HashMap::new();

        // mcb-domain: can only depend on std, serde, thiserror (no workspace crates)
        allowed.insert("mcb-domain", HashSet::new());
        forbidden.insert(
            "mcb-domain",
            ["mcb-application", "mcb-providers", "mcb-infrastructure", "mcb-server"]
                .into_iter()
                .collect(),
        );

        // mcb-application: depends only on mcb-domain
        allowed.insert("mcb-application", ["mcb-domain"].into_iter().collect());
        forbidden.insert(
            "mcb-application",
            ["mcb-providers", "mcb-infrastructure", "mcb-server"]
                .into_iter()
                .collect(),
        );

        // mcb-providers: depends on mcb-domain and mcb-application
        allowed.insert(
            "mcb-providers",
            ["mcb-domain", "mcb-application"].into_iter().collect(),
        );
        forbidden.insert(
            "mcb-providers",
            ["mcb-infrastructure", "mcb-server"].into_iter().collect(),
        );

        // mcb-infrastructure: depends on mcb-domain, mcb-application, mcb-providers
        allowed.insert(
            "mcb-infrastructure",
            ["mcb-domain", "mcb-application", "mcb-providers"]
                .into_iter()
                .collect(),
        );
        forbidden.insert("mcb-infrastructure", ["mcb-server"].into_iter().collect());

        // mcb-server: depends on all (composition root)
        allowed.insert(
            "mcb-server",
            ["mcb-domain", "mcb-application", "mcb-infrastructure"]
                .into_iter()
                .collect(),
        );
        // Server should not import providers directly - go through infrastructure
        forbidden.insert("mcb-server", ["mcb-providers"].into_iter().collect());

        Self { allowed, forbidden }
    }
}

/// Layer Flow Validator
///
/// Validates Clean Architecture dependency rules between crates.
pub struct LayerFlowValidator {
    rules: LayerRules,
}

impl Default for LayerFlowValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl LayerFlowValidator {
    /// Create a new layer flow validator
    pub fn new() -> Self {
        Self {
            rules: LayerRules::default(),
        }
    }

    /// Validate all layer flow rules
    pub fn validate(&self, config: &ValidationConfig) -> Result<Vec<LayerFlowViolation>> {
        let mut violations = Vec::new();

        violations.extend(self.check_forbidden_imports(config)?);
        violations.extend(self.check_circular_dependencies(config)?);
        violations.extend(self.check_domain_external_deps(config)?);

        Ok(violations)
    }

    /// Check for forbidden imports between crates
    fn check_forbidden_imports(
        &self,
        config: &ValidationConfig,
    ) -> Result<Vec<LayerFlowViolation>> {
        let mut violations = Vec::new();
        let crates_dir = config.workspace_root.join("crates");

        if !crates_dir.exists() {
            return Ok(violations);
        }

        // Build regex patterns for each mcb crate
        let import_pattern = Regex::new(r"use\s+(mcb_\w+)").expect("Invalid regex");

        for crate_name in self.rules.forbidden.keys() {
            let crate_dir = crates_dir.join(crate_name).join("src");
            if !crate_dir.exists() {
                continue;
            }

            let forbidden_deps = &self.rules.forbidden[crate_name];
            let crate_name_underscored = crate_name.replace('-', "_");

            for entry in WalkDir::new(&crate_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path();
                if !path.extension().is_some_and(|e| e == "rs") {
                    continue;
                }

                let content = std::fs::read_to_string(path)?;

                for (line_num, line) in content.lines().enumerate() {
                    // Skip comments
                    let trimmed = line.trim();
                    if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                        continue;
                    }

                    for captures in import_pattern.captures_iter(line) {
                        let imported_crate = captures.get(1).map(|m| m.as_str()).unwrap_or("");
                        let imported_crate_dashed = imported_crate.replace('_', "-");

                        // Skip self-imports
                        if imported_crate == crate_name_underscored {
                            continue;
                        }

                        // Check if this is a forbidden dependency
                        if forbidden_deps.contains(imported_crate_dashed.as_str()) {
                            violations.push(LayerFlowViolation::ForbiddenDependency {
                                source_crate: crate_name.to_string(),
                                target_crate: imported_crate_dashed,
                                import_path: line.trim().to_string(),
                                location: path.to_path_buf(),
                                line: line_num + 1,
                            });
                        }
                    }
                }
            }
        }

        Ok(violations)
    }

    /// Check for circular dependencies between crates
    fn check_circular_dependencies(
        &self,
        config: &ValidationConfig,
    ) -> Result<Vec<LayerFlowViolation>> {
        let mut violations = Vec::new();
        let crates_dir = config.workspace_root.join("crates");

        if !crates_dir.exists() {
            return Ok(violations);
        }

        // Build dependency graph from Cargo.toml files
        let mut deps: HashMap<String, HashSet<String>> = HashMap::new();

        for crate_name in ["mcb-domain", "mcb-application", "mcb-providers", "mcb-infrastructure", "mcb-server"] {
            let cargo_toml = crates_dir.join(crate_name).join("Cargo.toml");
            if !cargo_toml.exists() {
                continue;
            }

            let content = std::fs::read_to_string(&cargo_toml)?;
            let mut crate_deps = HashSet::new();

            for line in content.lines() {
                for dep_crate in ["mcb-domain", "mcb-application", "mcb-providers", "mcb-infrastructure", "mcb-server"] {
                    if dep_crate != crate_name && line.contains(dep_crate) {
                        crate_deps.insert(dep_crate.to_string());
                    }
                }
            }

            deps.insert(crate_name.to_string(), crate_deps);
        }

        // Check for bidirectional dependencies (simple circular check)
        let crate_list: Vec<_> = deps.keys().cloned().collect();
        for (i, crate_a) in crate_list.iter().enumerate() {
            for crate_b in crate_list.iter().skip(i + 1) {
                let a_deps_b = deps.get(crate_a).is_some_and(|d| d.contains(crate_b));
                let b_deps_a = deps.get(crate_b).is_some_and(|d| d.contains(crate_a));

                if a_deps_b && b_deps_a {
                    violations.push(LayerFlowViolation::CircularDependency {
                        crate_a: crate_a.clone(),
                        crate_b: crate_b.clone(),
                        file: crates_dir.join(crate_a).join("Cargo.toml"),
                        line: 1,
                    });
                }
            }
        }

        Ok(violations)
    }

    /// Check that domain layer doesn't depend on external crates
    fn check_domain_external_deps(
        &self,
        config: &ValidationConfig,
    ) -> Result<Vec<LayerFlowViolation>> {
        let mut violations = Vec::new();
        let domain_crate = config.workspace_root.join("crates/mcb-domain/src");

        if !domain_crate.exists() {
            return Ok(violations);
        }

        // Allowed external crates for domain layer
        let allowed_external = [
            "std", "core", "alloc", "serde", "serde_json", "thiserror", "async_trait",
            "shaku", "uuid", "chrono",
        ];

        let import_pattern = Regex::new(r"use\s+(\w+)::").expect("Invalid regex");

        for entry in WalkDir::new(&domain_crate)
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
                if trimmed.starts_with("//") || trimmed.starts_with("/*") {
                    continue;
                }

                for captures in import_pattern.captures_iter(line) {
                    let crate_name = captures.get(1).map(|m| m.as_str()).unwrap_or("");

                    // Skip self, super, crate keywords
                    if crate_name == "self" || crate_name == "super" || crate_name == "crate" {
                        continue;
                    }

                    // Skip mcb crates (handled by forbidden imports check)
                    if crate_name.starts_with("mcb_") {
                        continue;
                    }

                    // Check if external crate is allowed
                    if !allowed_external.contains(&crate_name) {
                        violations.push(LayerFlowViolation::DomainExternalDependency {
                            crate_name: "mcb-domain".to_string(),
                            external_crate: crate_name.to_string(),
                            location: path.to_path_buf(),
                            line: line_num + 1,
                        });
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
    fn test_layer_rules_default() {
        let rules = LayerRules::default();

        // Domain should have no allowed workspace deps
        assert!(rules.allowed["mcb-domain"].is_empty());

        // Application can only depend on domain
        assert!(rules.allowed["mcb-application"].contains("mcb-domain"));
        assert!(!rules.allowed["mcb-application"].contains("mcb-providers"));

        // Server can depend on infrastructure but not providers directly
        assert!(rules.allowed["mcb-server"].contains("mcb-infrastructure"));
        assert!(rules.forbidden["mcb-server"].contains("mcb-providers"));
    }

    #[test]
    fn test_import_pattern() {
        let pattern = Regex::new(r"use\s+(mcb_\w+)").unwrap();

        assert!(pattern.is_match("use mcb_domain::entities;"));
        assert!(pattern.is_match("use mcb_providers::embedding;"));
        assert!(!pattern.is_match("use std::collections;"));
    }
}
