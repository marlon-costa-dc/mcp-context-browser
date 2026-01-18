//! Linkme Migration Validator
//!
//! Validates migration from inventory to linkme for plugin registration.
//! Ensures all providers use linkme distributed slices instead of inventory.

use crate::violation_trait::{Violation, ViolationCategory};
use crate::{Result, ValidationConfig};
use regex::Regex;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Violation for inventory crate usage that should be migrated to linkme
#[derive(Debug, Clone)]
pub struct LinkmeViolation {
    pub file: PathBuf,
    pub line: usize,
    pub violation_type: LinkmeViolationType,
    pub context: String,
}

#[derive(Debug, Clone)]
pub enum LinkmeViolationType {
    /// Still using inventory::submit! macro
    InventorySubmitUsage,
    /// Still using inventory::collect! macro
    InventoryCollectUsage,
    /// Missing #[linkme::distributed_slice] declaration
    MissingLinkmeSliceDeclaration,
    /// Missing #[linkme::distributed_slice(NAME)] attribute on static
    MissingLinkmeSliceAttribute,
    /// Inventory dependency still present in Cargo.toml
    InventoryDependencyPresent,
    /// Mixed usage: both inventory and linkme in same file
    MixedUsage,
    /// Fallback pattern detected (old code not migrated)
    FallbackPattern,
    /// Registry access pattern needs updating
    RegistryAccessPattern,
    /// Provider registration without proper error handling
    UnsafeRegistration,
    /// Missing linkme import
    MissingLinkmeImport,
}

impl LinkmeViolation {
    pub fn new(
        file: impl Into<PathBuf>,
        line: usize,
        violation_type: LinkmeViolationType,
        context: impl Into<String>,
    ) -> Self {
        Self {
            file: file.into(),
            line,
            violation_type,
            context: context.into(),
        }
    }
}

impl Violation for LinkmeViolation {
    fn id(&self) -> &str {
        match self.violation_type {
            LinkmeViolationType::InventorySubmitUsage => "LINKME001",
            LinkmeViolationType::InventoryCollectUsage => "LINKME002",
            LinkmeViolationType::MissingLinkmeSliceDeclaration => "LINKME003",
            LinkmeViolationType::MissingLinkmeSliceAttribute => "LINKME004",
            LinkmeViolationType::InventoryDependencyPresent => "LINKME005",
        }
    }

    fn category(&self) -> ViolationCategory {
        ViolationCategory::DependencyInjection
    }

    fn file(&self) -> Option<&std::path::PathBuf> {
        Some(&self.file)
    }

    fn line(&self) -> Option<usize> {
        Some(self.line)
    }

    fn message(&self) -> String {
        match self.violation_type {
            LinkmeViolationType::InventorySubmitUsage => {
                "Found inventory::submit! usage that should be migrated to linkme distributed slices".to_string()
            }
            LinkmeViolationType::InventoryCollectUsage => {
                "Found inventory::collect! usage that should be migrated to linkme distributed slices".to_string()
            }
            LinkmeViolationType::MissingLinkmeSliceDeclaration => {
                "Missing #[linkme::distributed_slice] declaration for provider registry".to_string()
            }
            LinkmeViolationType::MissingLinkmeSliceAttribute => {
                "Provider registration missing #[linkme::distributed_slice(NAME)] attribute".to_string()
            }
            LinkmeViolationType::InventoryDependencyPresent => {
                "Inventory dependency still present - should be replaced with linkme".to_string()
            }
        }
    }

    fn suggestion(&self) -> Option<String> {
        match self.violation_type {
            LinkmeViolationType::InventorySubmitUsage => {
                Some("Replace inventory::submit! with #[linkme::distributed_slice(NAME)] static item".to_string())
            }
            LinkmeViolationType::InventoryCollectUsage => {
                Some("Replace inventory::collect! with #[linkme::distributed_slice] declaration".to_string())
            }
            LinkmeViolationType::MissingLinkmeSliceDeclaration => {
                Some("Add #[linkme::distributed_slice] above the static slice declaration".to_string())
            }
            LinkmeViolationType::MissingLinkmeSliceAttribute => {
                Some("Add #[linkme::distributed_slice(SLICE_NAME)] above the static provider registration".to_string())
            }
            LinkmeViolationType::InventoryDependencyPresent => {
                Some("Remove 'inventory' dependency and add 'linkme' dependency in Cargo.toml".to_string())
            }
        }
    }

    fn severity(&self) -> crate::Severity {
        match self.violation_type {
            LinkmeViolationType::InventorySubmitUsage => crate::Severity::Error,
            LinkmeViolationType::InventoryCollectUsage => crate::Severity::Error,
            LinkmeViolationType::MissingLinkmeSliceDeclaration => crate::Severity::Error,
            LinkmeViolationType::MissingLinkmeSliceAttribute => crate::Severity::Error,
            LinkmeViolationType::InventoryDependencyPresent => crate::Severity::Error,
            LinkmeViolationType::MixedUsage => crate::Severity::Error,
            LinkmeViolationType::FallbackPattern => crate::Severity::Warning,
            LinkmeViolationType::RegistryAccessPattern => crate::Severity::Warning,
            LinkmeViolationType::UnsafeRegistration => crate::Severity::Warning,
            LinkmeViolationType::MissingLinkmeImport => crate::Severity::Info,
        }
    }
}

/// Linkme migration validator
#[derive(Debug)]
pub struct LinkmeValidator {
    config: ValidationConfig,
}

impl LinkmeValidator {
    pub fn with_config(config: ValidationConfig) -> Self {
        Self { config }
    }

    pub fn validate_all(&self) -> Result<Vec<LinkmeViolation>> {
        let mut violations = Vec::new();

        // Check Cargo.toml files for inventory dependency
        violations.extend(self.check_cargo_dependencies()?);

        // Check Rust source files for inventory usage
        violations.extend(self.check_source_files()?);

        Ok(violations)
    }

    fn check_cargo_dependencies(&self) -> Result<Vec<LinkmeViolation>> {
        let mut violations = Vec::new();

        // Find all Cargo.toml files
        let cargo_files = self.find_cargo_files()?;

        for cargo_path in cargo_files {
            if let Ok(content) = std::fs::read_to_string(&cargo_path) {
                // Check for inventory dependency
                if content.contains("inventory =") || content.contains("inventory = {") {
                    violations.push(LinkmeViolation::new(
                        cargo_path.to_string_lossy(),
                        0, // Line number not critical for Cargo.toml
                        LinkmeViolationType::InventoryDependencyPresent,
                        "Found inventory dependency in Cargo.toml".to_string(),
                    ));
                }

                // Check that linkme is present (basic check)
                if !content.contains("linkme =") && !content.contains("linkme = {") {
                    // Only flag this if we're in a crate that should have linkme
                    if self.is_provider_crate(&cargo_path) {
                        violations.push(LinkmeViolation::new(
                            cargo_path.to_string_lossy(),
                            0,
                            LinkmeViolationType::MissingLinkmeSliceDeclaration,
                            "Provider crate missing linkme dependency".to_string(),
                        ));
                    }
                }
            }
        }

        Ok(violations)
    }

    fn check_source_files(&self) -> Result<Vec<LinkmeViolation>> {
        let mut violations = Vec::new();

        let source_files = self.find_source_files()?;

        for file_path in source_files {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                violations.extend(self.check_file_content(&file_path, &content));
            }
        }

        Ok(violations)
    }

    fn check_file_content(&self, file_path: &Path, content: &str) -> Vec<LinkmeViolation> {
        let mut violations = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Check for inventory usage
        let inventory_submit_regex = Regex::new(r"inventory::submit!").unwrap();
        let inventory_collect_regex = Regex::new(r"inventory::collect!").unwrap();

        for (i, line) in lines.iter().enumerate() {
            let line_num = i + 1;

            // Check for inventory::submit! usage
            if inventory_submit_regex.is_match(line) {
                violations.push(LinkmeViolation::new(
                    file_path.to_string_lossy(),
                    line_num,
                    LinkmeViolationType::InventorySubmitUsage,
                    line.trim().to_string(),
                ));
            }

            // Check for inventory::collect! usage
            if inventory_collect_regex.is_match(line) {
                violations.push(LinkmeViolation::new(
                    file_path.to_string_lossy(),
                    line_num,
                    LinkmeViolationType::InventoryCollectUsage,
                    line.trim().to_string(),
                ));
            }
        }

        // Check for missing linkme usage in provider files
        if self.is_provider_file(file_path) {
            let has_linkme_slice = content.contains("#[linkme::distributed_slice");
            let has_linkme_slice_attr = content.contains("linkme::distributed_slice(");

            if !has_linkme_slice && self.contains_provider_registry(content) {
                violations.push(LinkmeViolation::new(
                    file_path.to_string_lossy(),
                    1, // File-level issue
                    LinkmeViolationType::MissingLinkmeSliceDeclaration,
                    "Provider registry file missing linkme distributed slice".to_string(),
                ));
            }

            if !has_linkme_slice_attr && self.contains_provider_registration(content) {
                violations.push(LinkmeViolation::new(
                    file_path.to_string_lossy(),
                    1, // File-level issue
                    LinkmeViolationType::MissingLinkmeSliceAttribute,
                    "Provider registration missing linkme attribute".to_string(),
                ));
            }
        }

        violations
    }

    fn find_cargo_files(&self) -> Result<Vec<std::path::PathBuf>> {
        let mut cargo_files = Vec::new();

        for dir in self.config.get_source_dirs()? {
            let cargo_path = dir.with_file_name("Cargo.toml");
            if cargo_path.exists() {
                cargo_files.push(cargo_path);
            }
        }

        Ok(cargo_files)
    }

    fn find_source_files(&self) -> Result<Vec<std::path::PathBuf>> {
        let mut source_files = Vec::new();

        for dir in self.config.get_scan_dirs()? {
            self.collect_source_files(&dir, &mut source_files)?;
        }

        Ok(source_files)
    }

    fn collect_source_files(&self, dir: &Path, files: &mut Vec<std::path::PathBuf>) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() && !self.config.should_exclude(&path) {
                self.collect_source_files(&path, files)?;
            } else if let Some(ext) = path.extension() {
                if ext == "rs" && !self.config.should_exclude(&path) {
                    files.push(path);
                }
            }
        }

        Ok(())
    }

    fn is_provider_crate(&self, cargo_path: &Path) -> bool {
        cargo_path.to_string_lossy().contains("mcb-providers")
    }

    fn is_provider_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy();
        path_str.contains("providers") ||
        path_str.contains("registry") ||
        path_str.contains("embedding") ||
        path_str.contains("vector_store") ||
        path_str.contains("cache") ||
        path_str.contains("language")
    }

    fn contains_provider_registry(&self, content: &str) -> bool {
        content.contains("inventory::collect!") ||
        content.contains("Vec<") && content.contains("Provider") ||
        content.contains("registry") && content.contains("static")
    }

    fn contains_provider_registration(&self, content: &str) -> bool {
        content.contains("inventory::submit!") ||
        (content.contains("static") && content.contains("Provider"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inventory_submit_detection() {
        let validator = LinkmeValidator::with_config(ValidationConfig::new("/tmp"));
        let content = r#"
        inventory::submit! {
            EmbeddingProviderEntry {
                name: "ollama",
                description: "Ollama provider",
                factory: |config| Ok(Arc::new(OllamaProvider::new(config))),
            }
        }
        "#;

        let violations = validator.check_file_content(Path::new("test.rs"), content);
        assert_eq!(violations.len(), 1);
        assert!(matches!(violations[0].violation_type, LinkmeViolationType::InventorySubmitUsage));
    }

    #[test]
    fn test_inventory_collect_detection() {
        let validator = LinkmeValidator::with_config(ValidationConfig::new("/tmp"));
        let content = r#"
        inventory::collect!(EmbeddingProviderEntry);
        "#;

        let violations = validator.check_file_content(Path::new("test.rs"), content);
        assert_eq!(violations.len(), 1);
        assert!(matches!(violations[0].violation_type, LinkmeViolationType::InventoryCollectUsage));
    }
}