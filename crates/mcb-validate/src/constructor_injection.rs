//! Constructor Injection Validator
//!
//! Validates migration from Shaku DI to constructor injection.
//! Ensures services use explicit constructor parameters instead of DI macros.

use crate::violation_trait::{Violation, ViolationCategory};
use crate::{Result, ValidationConfig};
use regex::Regex;
use std::path::{Path, PathBuf};

/// Violation for Shaku DI usage that should be migrated to constructor injection
#[derive(Debug, Clone)]
pub struct ConstructorInjectionViolation {
    pub file: PathBuf,
    pub line: usize,
    pub violation_type: ConstructorInjectionViolationType,
    pub context: String,
}

#[derive(Debug, Clone)]
pub enum ConstructorInjectionViolationType {
    /// Still using #[derive(Component)] macro
    ShakuComponentDerive,
    /// Still using #[shaku(interface = ...)] attribute
    ShakuInterfaceAttribute,
    /// Still using #[shaku(inject)] attribute
    ShakuInjectAttribute,
    /// Still using module! macro
    ShakuModuleMacro,
    /// Service implementation missing constructor with Arc<dyn Trait> parameters
    MissingConstructorInjection,
    /// Manual service composition missing from bootstrap
    MissingManualComposition,
    /// Mixed usage: both Shaku and constructor injection in same file
    MixedUsage,
    /// Fallback pattern: old Shaku code still present
    FallbackPattern,
    /// Container.resolve() usage that should be manual instantiation
    ContainerResolveUsage,
    /// Missing shaku import removal
    MissingShakuRemoval,
    /// Service without proper trait bounds
    MissingTraitBounds,
    /// Constructor injection without proper error handling
    UnsafeConstructorInjection,
}

impl ConstructorInjectionViolation {
    pub fn new(
        file: impl Into<PathBuf>,
        line: usize,
        violation_type: ConstructorInjectionViolationType,
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

impl Violation for ConstructorInjectionViolation {
    fn id(&self) -> &str {
        match self.violation_type {
            ConstructorInjectionViolationType::ShakuComponentDerive => "CTOR001",
            ConstructorInjectionViolationType::ShakuInterfaceAttribute => "CTOR002",
            ConstructorInjectionViolationType::ShakuInjectAttribute => "CTOR003",
            ConstructorInjectionViolationType::ShakuModuleMacro => "CTOR004",
            ConstructorInjectionViolationType::MissingConstructorInjection => "CTOR005",
            ConstructorInjectionViolationType::MissingManualComposition => "CTOR006",
            ConstructorInjectionViolationType::MixedUsage => "CTOR007",
            ConstructorInjectionViolationType::FallbackPattern => "CTOR008",
            ConstructorInjectionViolationType::ContainerResolveUsage => "CTOR009",
            ConstructorInjectionViolationType::MissingShakuRemoval => "CTOR010",
            ConstructorInjectionViolationType::MissingTraitBounds => "CTOR011",
            ConstructorInjectionViolationType::UnsafeConstructorInjection => "CTOR012",
        }
    }

    fn category(&self) -> ViolationCategory {
        ViolationCategory::DependencyInjection
    }

    fn severity(&self) -> crate::Severity {
        match self.violation_type {
            ConstructorInjectionViolationType::ShakuComponentDerive => crate::Severity::Error,
            ConstructorInjectionViolationType::ShakuInterfaceAttribute => crate::Severity::Error,
            ConstructorInjectionViolationType::ShakuInjectAttribute => crate::Severity::Error,
            ConstructorInjectionViolationType::ShakuModuleMacro => crate::Severity::Error,
            ConstructorInjectionViolationType::ContainerResolveUsage => crate::Severity::Error,
            ConstructorInjectionViolationType::MixedUsage => crate::Severity::Error,
            ConstructorInjectionViolationType::MissingConstructorInjection => crate::Severity::Warning,
            ConstructorInjectionViolationType::MissingManualComposition => crate::Severity::Warning,
            ConstructorInjectionViolationType::FallbackPattern => crate::Severity::Warning,
            ConstructorInjectionViolationType::UnsafeConstructorInjection => crate::Severity::Warning,
            ConstructorInjectionViolationType::MissingShakuRemoval => crate::Severity::Info,
            ConstructorInjectionViolationType::MissingTraitBounds => crate::Severity::Info,
        }
    }

    fn file(&self) -> Option<&std::path::PathBuf> {
        Some(&self.file)
    }

    fn line(&self) -> Option<usize> {
        Some(self.line)
    }

    fn message(&self) -> String {
        match self.violation_type {
            ConstructorInjectionViolationType::ShakuComponentDerive => {
                "Found #[derive(Component)] that should be removed for constructor injection".to_string()
            }
            ConstructorInjectionViolationType::ShakuInterfaceAttribute => {
                "Found #[shaku(interface = ...)] attribute that should be removed".to_string()
            }
            ConstructorInjectionViolationType::ShakuInjectAttribute => {
                "Found #[shaku(inject)] attribute that should be removed".to_string()
            }
            ConstructorInjectionViolationType::ShakuModuleMacro => {
                "Found module! macro that should be replaced with manual composition".to_string()
            }
            ConstructorInjectionViolationType::MissingConstructorInjection => {
                "Service implementation missing constructor with Arc<dyn Trait> parameters".to_string()
            }
            ConstructorInjectionViolationType::MissingManualComposition => {
                "Missing manual service composition in bootstrap/container code".to_string()
            }
            ConstructorInjectionViolationType::MixedUsage => {
                "Mixed usage of Shaku and constructor injection in same file".to_string()
            }
            ConstructorInjectionViolationType::FallbackPattern => {
                "Fallback pattern detected - old Shaku code not fully migrated".to_string()
            }
            ConstructorInjectionViolationType::ContainerResolveUsage => {
                "Found container.resolve() that should be manual instantiation".to_string()
            }
            ConstructorInjectionViolationType::MissingShakuRemoval => {
                "Shaku import still present - should be removed after migration".to_string()
            }
            ConstructorInjectionViolationType::MissingTraitBounds => {
                "Service missing proper trait bounds for constructor injection".to_string()
            }
            ConstructorInjectionViolationType::UnsafeConstructorInjection => {
                "Constructor injection without proper error handling".to_string()
            }
        }
    }

    fn suggestion(&self) -> Option<String> {
        match self.violation_type {
            ConstructorInjectionViolationType::ShakuComponentDerive => {
                Some("Remove #[derive(Component)] and implement constructor injection manually".to_string())
            }
            ConstructorInjectionViolationType::ShakuInterfaceAttribute => {
                Some("Remove #[shaku(interface = ...)] attribute".to_string())
            }
            ConstructorInjectionViolationType::ShakuInjectAttribute => {
                Some("Remove #[shaku(inject)] and add parameter to constructor".to_string())
            }
            ConstructorInjectionViolationType::ShakuModuleMacro => {
                Some("Replace module! macro with manual service instantiation in bootstrap".to_string())
            }
            ConstructorInjectionViolationType::MissingConstructorInjection => {
                Some("Add constructor method that accepts Arc<dyn Trait> parameters".to_string())
            }
            ConstructorInjectionViolationType::MissingManualComposition => {
                Some("Add manual service composition in bootstrap/container code".to_string())
            }
            ConstructorInjectionViolationType::MixedUsage => {
                Some("Remove all Shaku usage and use constructor injection consistently".to_string())
            }
            ConstructorInjectionViolationType::FallbackPattern => {
                Some("Remove fallback code and use constructor injection directly".to_string())
            }
            ConstructorInjectionViolationType::ContainerResolveUsage => {
                Some("Replace container.resolve() with manual Arc<T> instantiation".to_string())
            }
            ConstructorInjectionViolationType::MissingShakuRemoval => {
                Some("Remove 'use shaku::*;' imports after migration".to_string())
            }
            ConstructorInjectionViolationType::MissingTraitBounds => {
                Some("Add proper trait bounds (Send + Sync + 'static) for Arc<dyn Trait>".to_string())
            }
            ConstructorInjectionViolationType::UnsafeConstructorInjection => {
                Some("Add proper error handling with Result<T, E> in constructor".to_string())
            }
        }
    }
}

/// Constructor injection validator
#[derive(Debug)]
pub struct ConstructorInjectionValidator {
    config: ValidationConfig,
}

impl ConstructorInjectionValidator {
    pub fn with_config(config: ValidationConfig) -> Self {
        Self { config }
    }

    pub fn validate_all(&self) -> Result<Vec<ConstructorInjectionViolation>> {
        let mut violations = Vec::new();

        // Check Cargo.toml files for shaku dependency
        violations.extend(self.check_cargo_dependencies()?);

        // Check Rust source files for Shaku usage
        violations.extend(self.check_source_files()?);

        Ok(violations)
    }

    fn check_cargo_dependencies(&self) -> Result<Vec<ConstructorInjectionViolation>> {
        let mut violations = Vec::new();

        let cargo_files = self.find_cargo_files()?;

        for cargo_path in cargo_files {
            if let Ok(content) = std::fs::read_to_string(&cargo_path) {
                // Check for shaku dependency
                if content.contains("shaku =") || content.contains("shaku = {") {
                    violations.push(ConstructorInjectionViolation::new(
                        cargo_path.to_string_lossy(),
                        0,
                        ConstructorInjectionViolationType::ShakuComponentDerive,
                        "Found shaku dependency in Cargo.toml - should be removed for constructor injection".to_string(),
                    ));
                }
            }
        }

        Ok(violations)
    }

    fn check_source_files(&self) -> Result<Vec<ConstructorInjectionViolation>> {
        let mut violations = Vec::new();

        let source_files = self.find_source_files()?;

        for file_path in source_files {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                violations.extend(self.check_file_content(&file_path, &content));
            }
        }

        Ok(violations)
    }

    fn check_file_content(&self, file_path: &Path, content: &str) -> Vec<ConstructorInjectionViolation> {
        let mut violations = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Track usage patterns
        let mut has_shaku_usage = false;
        let mut has_constructor_injection = false;
        let mut has_container_resolve = false;
        let mut has_shaku_import = false;

        // Regex patterns for Shaku usage
        let component_derive_regex = Regex::new(r"#\[derive\(Component\)\]").unwrap();
        let interface_attr_regex = Regex::new(r"#\[shaku\(interface\s*=\s*").unwrap();
        let inject_attr_regex = Regex::new(r"#\[shaku\(inject\)\]").unwrap();
        let module_macro_regex = Regex::new(r"module!\s*\{").unwrap();
        let container_resolve_regex = Regex::new(r"container\.resolve\(\)").unwrap();
        let shaku_import_regex = Regex::new(r"use\s+shaku").unwrap();
        let arc_dyn_trait_regex = Regex::new(r"Arc<dyn\s+\w+>").unwrap();

        for (i, line) in lines.iter().enumerate() {
            let line_num = i + 1;

            // Check for #[derive(Component)]
            if component_derive_regex.is_match(line) {
                has_shaku_usage = true;
                violations.push(ConstructorInjectionViolation::new(
                    file_path.to_string_lossy().into(),
                    line_num,
                    ConstructorInjectionViolationType::ShakuComponentDerive,
                    line.trim().to_string(),
                ));
            }

            // Check for #[shaku(interface = ...)]
            if interface_attr_regex.is_match(line) {
                has_shaku_usage = true;
                violations.push(ConstructorInjectionViolation::new(
                    file_path.to_string_lossy().into(),
                    line_num,
                    ConstructorInjectionViolationType::ShakuInterfaceAttribute,
                    line.trim().to_string(),
                ));
            }

            // Check for #[shaku(inject)]
            if inject_attr_regex.is_match(line) {
                has_shaku_usage = true;
                violations.push(ConstructorInjectionViolation::new(
                    file_path.to_string_lossy().into(),
                    line_num,
                    ConstructorInjectionViolationType::ShakuInjectAttribute,
                    line.trim().to_string(),
                ));
            }

            // Check for module! macro
            if module_macro_regex.is_match(line) {
                has_shaku_usage = true;
                violations.push(ConstructorInjectionViolation::new(
                    file_path.to_string_lossy().into(),
                    line_num,
                    ConstructorInjectionViolationType::ShakuModuleMacro,
                    "Found module! macro definition".to_string(),
                ));
            }

            // Check for container.resolve() usage
            if container_resolve_regex.is_match(line) {
                has_container_resolve = true;
                violations.push(ConstructorInjectionViolation::new(
                    file_path.to_string_lossy().into(),
                    line_num,
                    ConstructorInjectionViolationType::ContainerResolveUsage,
                    line.trim().to_string(),
                ));
            }

            // Check for shaku import
            if shaku_import_regex.is_match(line) {
                has_shaku_import = true;
            }

            // Check for constructor injection patterns
            if arc_dyn_trait_regex.is_match(line) && line.contains("fn new(") {
                has_constructor_injection = true;
            }
        }

        // Check for mixed usage
        if has_shaku_usage && has_constructor_injection {
            violations.push(ConstructorInjectionViolation::new(
                file_path.to_string_lossy().into(),
                1,
                ConstructorInjectionViolationType::MixedUsage,
                "File contains both Shaku and constructor injection patterns".to_string(),
            ));
        }

        // Check for fallback patterns
        if has_shaku_usage && content.contains("#[cfg") && content.contains("shaku") {
            violations.push(ConstructorInjectionViolation::new(
                file_path.to_string_lossy().into(),
                1,
                ConstructorInjectionViolationType::FallbackPattern,
                "Conditional compilation with Shaku detected - potential fallback".to_string(),
            ));
        }

        // Check for missing shaku removal
        if has_shaku_import && !has_shaku_usage && has_constructor_injection {
            violations.push(ConstructorInjectionViolation::new(
                file_path.to_string_lossy().into(),
                1,
                ConstructorInjectionViolationType::MissingShakuRemoval,
                "Shaku import still present after migration to constructor injection".to_string(),
            ));
        }

        // Check for missing constructor injection in service files
        if self.is_service_file(file_path) {
            violations.extend(self.check_constructor_injection(file_path, content));
        }

        // Check for missing manual composition in bootstrap files
        if self.is_bootstrap_file(file_path) {
            violations.extend(self.check_manual_composition(file_path, content));
        }

        violations
    }

    fn check_constructor_injection(&self, file_path: &Path, content: &str) -> Vec<ConstructorInjectionViolation> {
        let mut violations = Vec::new();

        // If file has struct/impl but no constructor with Arc<dyn Trait>, flag it
        if content.contains("pub struct") && content.contains("impl") {
            let has_shaku_inject = content.contains("#[shaku(inject)]");
            let has_arc_dyn_trait_constructor = content.contains("Arc<dyn") && content.contains("fn new(");

            // If it has Shaku inject but no proper constructor, it's being migrated
            if has_shaku_inject && !has_arc_dyn_trait_constructor {
                violations.push(ConstructorInjectionViolation::new(
                    file_path.to_string_lossy(),
                    1,
                    ConstructorInjectionViolationType::MissingConstructorInjection,
                    "Service has #[shaku(inject)] but missing constructor with Arc<dyn Trait> parameters".to_string(),
                ));
            }

            // If it's a service but has neither Shaku nor constructor injection
            if !has_shaku_inject && !has_arc_dyn_trait_constructor && self.is_service_impl(content) {
                violations.push(ConstructorInjectionViolation::new(
                    file_path.to_string_lossy(),
                    1,
                    ConstructorInjectionViolationType::MissingConstructorInjection,
                    "Service implementation missing constructor injection pattern".to_string(),
                ));
            }
        }

        violations
    }

    fn check_manual_composition(&self, file_path: &Path, content: &str) -> Vec<ConstructorInjectionViolation> {
        let mut violations = Vec::new();

        // Bootstrap files should have manual service instantiation
        if content.contains("bootstrap") || content.contains("container") || content.contains("init") {
            let has_manual_instantiation = content.contains("Arc::new(") && content.contains("::new(");
            let has_shaku_resolve = content.contains("container.resolve()") || content.contains("resolve()");

            if has_shaku_resolve && !has_manual_instantiation {
                violations.push(ConstructorInjectionViolation::new(
                    file_path.to_string_lossy(),
                    1,
                    ConstructorInjectionViolationType::MissingManualComposition,
                    "Bootstrap code still uses container.resolve() instead of manual composition".to_string(),
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

    fn is_service_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy();
        path_str.contains("service") ||
        path_str.contains("provider") ||
        path_str.contains("handler") ||
        path_str.contains("controller") ||
        path_str.contains("usecase") ||
        path_str.contains("interactor")
    }

    fn is_bootstrap_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy();
        path_str.contains("bootstrap") ||
        path_str.contains("container") ||
        path_str.contains("init") ||
        path_str.contains("main") ||
        path_str.contains("di")
    }

    fn is_service_impl(&self, content: &str) -> bool {
        // Check if this looks like a service implementation
        content.contains("impl") &&
        (content.contains("Service") ||
         content.contains("Provider") ||
         content.contains("Handler") ||
         content.contains("Controller"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shaku_component_derive_detection() {
        let validator = ConstructorInjectionValidator::with_config(ValidationConfig::new("/tmp"));
        let content = r#"
        #[derive(Component)]
        #[shaku(interface = dyn MyService)]
        pub struct MyServiceImpl {
            #[shaku(inject)]
            dependency: Arc<dyn OtherService>,
        }
        "#;

        let violations = validator.check_file_content(Path::new("test.rs"), content);
        assert!(violations.iter().any(|v| matches!(v.violation_type, ConstructorInjectionViolationType::ShakuComponentDerive)));
        assert!(violations.iter().any(|v| matches!(v.violation_type, ConstructorInjectionViolationType::ShakuInterfaceAttribute)));
        assert!(violations.iter().any(|v| matches!(v.violation_type, ConstructorInjectionViolationType::ShakuInjectAttribute)));
    }

    #[test]
    fn test_module_macro_detection() {
        let validator = ConstructorInjectionValidator::with_config(ValidationConfig::new("/tmp"));
        let content = r#"
        module! {
            pub MyModuleImpl: MyModule {
                components = [MyServiceImpl],
                providers = []
            }
        }
        "#;

        let violations = validator.check_file_content(Path::new("test.rs"), content);
        assert!(violations.iter().any(|v| matches!(v.violation_type, ConstructorInjectionViolationType::ShakuModuleMacro)));
    }
}