//! Figment Configuration Validator
//!
//! Validates migration from config crate to Figment for configuration loading.
//! Ensures all configuration uses Figment's provider system.

use crate::violation_trait::{Violation, ViolationCategory};
use crate::{Result, ValidationConfig};
use regex::Regex;
use std::path::{Path, PathBuf};

/// Violation for config crate usage that should be migrated to Figment
#[derive(Debug, Clone)]
pub struct FigmentViolation {
    pub file: PathBuf,
    pub line: usize,
    pub violation_type: FigmentViolationType,
    pub context: String,
}

#[derive(Debug, Clone)]
pub enum FigmentViolationType {
    /// Still using config::Config::builder() pattern
    ConfigBuilderUsage,
    /// Still using config::Environment provider
    ConfigEnvironmentUsage,
    /// Still using config::File provider
    ConfigFileUsage,
    /// Missing Figment::new().merge() pattern
    MissingFigmentPattern,
    /// Missing profile support in configuration
    MissingProfileSupport,
    /// Config crate dependency still present
    ConfigDependencyPresent,
    /// Mixed usage: both config crate and Figment in same file
    MixedUsage,
    /// Fallback pattern: old config crate code still present
    FallbackPattern,
    /// Manual config parsing that should use Figment providers
    ManualConfigParsing,
    /// Missing error handling in Figment usage
    UnsafeFigmentUsage,
    /// Missing figment import
    MissingFigmentImport,
    /// Profile configuration without proper validation
    InvalidProfileUsage,
}

impl FigmentViolation {
    pub fn new(
        file: impl Into<PathBuf>,
        line: usize,
        violation_type: FigmentViolationType,
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

impl Violation for FigmentViolation {
    fn id(&self) -> &str {
        match self.violation_type {
            FigmentViolationType::ConfigBuilderUsage => "FIGMENT001",
            FigmentViolationType::ConfigEnvironmentUsage => "FIGMENT002",
            FigmentViolationType::ConfigFileUsage => "FIGMENT003",
            FigmentViolationType::MissingFigmentPattern => "FIGMENT004",
            FigmentViolationType::MissingProfileSupport => "FIGMENT005",
            FigmentViolationType::ConfigDependencyPresent => "FIGMENT006",
        }
    }

    fn category(&self) -> ViolationCategory {
        ViolationCategory::Configuration
    }

    fn file(&self) -> Option<&std::path::PathBuf> {
        Some(&self.file)
    }

    fn line(&self) -> Option<usize> {
        Some(self.line)
    }

    fn message(&self) -> String {
        match self.violation_type {
            FigmentViolationType::ConfigBuilderUsage => {
                "Found config::Config::builder() usage that should be migrated to Figment".to_string()
            }
            FigmentViolationType::ConfigEnvironmentUsage => {
                "Found config::Environment usage that should be migrated to figment::providers::Env".to_string()
            }
            FigmentViolationType::ConfigFileUsage => {
                "Found config::File usage that should be migrated to Figment file providers".to_string()
            }
            FigmentViolationType::MissingFigmentPattern => {
                "Configuration loading missing Figment::new().merge() pattern".to_string()
            }
            FigmentViolationType::MissingProfileSupport => {
                "Configuration missing profile support (development/production)".to_string()
            }
            FigmentViolationType::ConfigDependencyPresent => {
                "Config crate dependency still present - should be replaced with figment".to_string()
            }
        }
    }

    fn suggestion(&self) -> Option<String> {
        match self.violation_type {
            FigmentViolationType::ConfigBuilderUsage => {
                Some("Replace Config::builder() with Figment::new().merge() pattern".to_string())
            }
            FigmentViolationType::ConfigEnvironmentUsage => {
                Some("Replace config::Environment with figment::providers::Env".to_string())
            }
            FigmentViolationType::ConfigFileUsage => {
                Some("Replace config::File with appropriate Figment provider (Toml, Json, etc.)".to_string())
            }
            FigmentViolationType::MissingFigmentPattern => {
                Some("Use Figment::new().merge(provider).extract() pattern for configuration loading".to_string())
            }
            FigmentViolationType::MissingProfileSupport => {
                Some("Add profile-based configuration with Profile::new() and .profile() method".to_string())
            }
            FigmentViolationType::ConfigDependencyPresent => {
                Some("Remove 'config' dependency and add 'figment' with appropriate features".to_string())
            }
        }
    }

    fn severity(&self) -> crate::Severity {
        match self.violation_type {
            FigmentViolationType::ConfigUsage => crate::Severity::Error,
            FigmentViolationType::MissingFigmentUsage => crate::Severity::Error,
            FigmentViolationType::MixedConfigUsage => crate::Severity::Error,
            FigmentViolationType::ConfigDependencyPresent => crate::Severity::Error,
            FigmentViolationType::FallbackPattern => crate::Severity::Warning,
            FigmentViolationType::UnsafeConfigLoading => crate::Severity::Warning,
        }
    }
}

/// Figment configuration validator
#[derive(Debug)]
pub struct FigmentValidator {
    config: ValidationConfig,
}

impl FigmentValidator {
    pub fn with_config(config: ValidationConfig) -> Self {
        Self { config }
    }

    pub fn validate_all(&self) -> Result<Vec<FigmentViolation>> {
        let mut violations = Vec::new();

        // Check Cargo.toml files for config dependency
        violations.extend(self.check_cargo_dependencies()?);

        // Check Rust source files for config crate usage
        violations.extend(self.check_source_files()?);

        Ok(violations)
    }

    fn check_cargo_dependencies(&self) -> Result<Vec<FigmentViolation>> {
        let mut violations = Vec::new();

        let cargo_files = self.find_cargo_files()?;

        for cargo_path in cargo_files {
            if let Ok(content) = std::fs::read_to_string(&cargo_path) {
                // Check for config dependency
                if content.contains("config =") || content.contains("config = {") {
                    violations.push(FigmentViolation::new(
                        cargo_path.to_string_lossy(),
                        0,
                        FigmentViolationType::ConfigDependencyPresent,
                        "Found config crate dependency in Cargo.toml".to_string(),
                    ));
                }

                // Check that figment is present for crates that need configuration
                if !content.contains("figment =") && !content.contains("figment = {") {
                    if self.is_config_crate(&cargo_path) {
                        violations.push(FigmentViolation::new(
                            cargo_path.to_string_lossy(),
                            0,
                            FigmentViolationType::MissingFigmentPattern,
                            "Configuration crate missing figment dependency".to_string(),
                        ));
                    }
                }
            }
        }

        Ok(violations)
    }

    fn check_source_files(&self) -> Result<Vec<FigmentViolation>> {
        let mut violations = Vec::new();

        let source_files = self.find_source_files()?;

        for file_path in source_files {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                violations.extend(self.check_file_content(&file_path, &content));
            }
        }

        Ok(violations)
    }

    fn check_file_content(&self, file_path: &Path, content: &str) -> Vec<FigmentViolation> {
        let mut violations = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Regex patterns for config crate usage
        let config_builder_regex = Regex::new(r"Config::builder\(\)").unwrap();
        let config_environment_regex = Regex::new(r"config::Environment").unwrap();
        let config_file_regex = Regex::new(r"config::File").unwrap();

        for (i, line) in lines.iter().enumerate() {
            let line_num = i + 1;

            // Check for Config::builder() usage
            if config_builder_regex.is_match(line) {
                violations.push(FigmentViolation::new(
                    file_path.to_string_lossy(),
                    line_num,
                    FigmentViolationType::ConfigBuilderUsage,
                    line.trim().to_string(),
                ));
            }

            // Check for config::Environment usage
            if config_environment_regex.is_match(line) {
                violations.push(FigmentViolation::new(
                    file_path.to_string_lossy(),
                    line_num,
                    FigmentViolationType::ConfigEnvironmentUsage,
                    line.trim().to_string(),
                ));
            }

            // Check for config::File usage
            if config_file_regex.is_match(line) {
                violations.push(FigmentViolation::new(
                    file_path.to_string_lossy(),
                    line_num,
                    FigmentViolationType::ConfigFileUsage,
                    line.trim().to_string(),
                ));
            }
        }

        // Check for missing Figment patterns in config files
        if self.is_config_file(file_path) {
            violations.extend(self.check_figment_patterns(file_path, content));
        }

        violations
    }

    fn check_figment_patterns(&self, file_path: &Path, content: &str) -> Vec<FigmentViolation> {
        let mut violations = Vec::new();

        let has_config_builder = content.contains("Config::builder()");
        let has_figment_new = content.contains("Figment::new()");
        let has_figment_merge = content.contains(".merge(");
        let has_figment_extract = content.contains(".extract()");

        // If file has configuration loading but uses old pattern
        if has_config_builder && !has_figment_new {
            violations.push(FigmentViolation::new(
                file_path.to_string_lossy(),
                1,
                FigmentViolationType::MissingFigmentPattern,
                "Configuration loading uses Config::builder() instead of Figment::new()".to_string(),
            ));
        }

        // If file has Figment but missing proper pattern
        if has_figment_new && (!has_figment_merge || !has_figment_extract) {
            violations.push(FigmentViolation::new(
                file_path.to_string_lossy(),
                1,
                FigmentViolationType::MissingFigmentPattern,
                "Figment usage missing proper merge() and extract() pattern".to_string(),
            ));
        }

        // Check for profile support in main config files
        if self.is_main_config_file(file_path) && has_figment_new {
            let has_profile_support = content.contains("Profile::") ||
                                    content.contains(".profile(") ||
                                    content.contains("development") ||
                                    content.contains("production");

            if !has_profile_support {
                violations.push(FigmentViolation::new(
                    file_path.to_string_lossy(),
                    1,
                    FigmentViolationType::MissingProfileSupport,
                    "Configuration missing profile support for development/production environments".to_string(),
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

    fn is_config_crate(&self, cargo_path: &Path) -> bool {
        cargo_path.to_string_lossy().contains("mcb-infrastructure")
    }

    fn is_config_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy();
        path_str.contains("config") ||
        path_str.contains("loader") ||
        path_str.contains("bootstrap") ||
        path_str.contains("init")
    }

    fn is_main_config_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy();
        path_str.contains("config/loader.rs") ||
        path_str.contains("config/mod.rs") ||
        path_str.contains("bootstrap.rs") ||
        path_str.contains("init.rs")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder_detection() {
        let validator = FigmentValidator::with_config(ValidationConfig::new("/tmp"));
        let content = r#"
        let mut builder = Config::builder();
        builder = builder.add_source(File::from(config_path));
        let config = builder.build()?.try_deserialize()?;
        "#;

        let violations = validator.check_file_content(Path::new("test.rs"), content);
        assert!(violations.iter().any(|v| matches!(v.violation_type, FigmentViolationType::ConfigBuilderUsage)));
        assert!(violations.iter().any(|v| matches!(v.violation_type, FigmentViolationType::ConfigFileUsage)));
    }

    #[test]
    fn test_config_environment_detection() {
        let validator = FigmentValidator::with_config(ValidationConfig::new("/tmp"));
        let content = r#"
        builder = builder.add_source(config::Environment::with_prefix("APP"));
        "#;

        let violations = validator.check_file_content(Path::new("test.rs"), content);
        assert!(violations.iter().any(|v| matches!(v.violation_type, FigmentViolationType::ConfigEnvironmentUsage)));
    }
}