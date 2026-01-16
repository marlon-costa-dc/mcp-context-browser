//! DI/Shaku Pattern Validation
//!
//! Validates code against Dependency Injection and Shaku patterns:
//! - Direct ::new() calls for service types (should use DI)
//! - Service structs without #[derive(Component)]
//! - Null/Fake/Mock types in production code
//! - Dual initialization paths (both DI and manual)
//! - Services not wired to the system

use crate::{Result, Severity, ValidationConfig};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use walkdir::WalkDir;

/// DI/Shaku violation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShakuViolation {
    /// Direct ::new() call for type that should use DI
    DirectInstantiation {
        file: PathBuf,
        line: usize,
        type_name: String,
        suggestion: String,
        severity: Severity,
    },

    /// Service struct without #[derive(Component)]
    UnregisteredService {
        file: PathBuf,
        line: usize,
        service_name: String,
        severity: Severity,
    },

    /// Null/Fake/Mock type in production code
    FakeImplementation {
        file: PathBuf,
        line: usize,
        type_name: String,
        context: String,
        severity: Severity,
    },

    /// Dual initialization path (both DI and manual)
    DualInitializationPath {
        file: PathBuf,
        line: usize,
        context: String,
        severity: Severity,
    },

    /// File named null.rs in production code
    NullProviderFile {
        file: PathBuf,
        severity: Severity,
    },
}

impl ShakuViolation {
    pub fn severity(&self) -> Severity {
        match self {
            Self::DirectInstantiation { severity, .. } => *severity,
            Self::UnregisteredService { severity, .. } => *severity,
            Self::FakeImplementation { severity, .. } => *severity,
            Self::DualInitializationPath { severity, .. } => *severity,
            Self::NullProviderFile { severity, .. } => *severity,
        }
    }
}

impl std::fmt::Display for ShakuViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirectInstantiation {
                file,
                line,
                type_name,
                suggestion,
                ..
            } => {
                write!(
                    f,
                    "DI: Direct instantiation of {}: {}:{} - {}",
                    type_name,
                    file.display(),
                    line,
                    suggestion
                )
            }
            Self::UnregisteredService {
                file,
                line,
                service_name,
                ..
            } => {
                write!(
                    f,
                    "DI: Service {} not registered with DI container: {}:{}",
                    service_name,
                    file.display(),
                    line
                )
            }
            Self::FakeImplementation {
                file,
                line,
                type_name,
                context,
                ..
            } => {
                write!(
                    f,
                    "DI: Fake/Null implementation {} in production: {}:{} - {}",
                    type_name,
                    file.display(),
                    line,
                    context
                )
            }
            Self::DualInitializationPath {
                file,
                line,
                context,
                ..
            } => {
                write!(
                    f,
                    "DI: Dual initialization path at {}:{} - {}",
                    file.display(),
                    line,
                    context
                )
            }
            Self::NullProviderFile { file, .. } => {
                write!(
                    f,
                    "DI: Null provider file in production: {}",
                    file.display()
                )
            }
        }
    }
}

/// DI/Shaku validator
pub struct ShakuValidator {
    config: ValidationConfig,
}

impl ShakuValidator {
    /// Create a new Shaku validator
    pub fn new(workspace_root: impl Into<PathBuf>) -> Self {
        Self::with_config(ValidationConfig::new(workspace_root))
    }

    /// Create a validator with custom configuration for multi-directory support
    pub fn with_config(config: ValidationConfig) -> Self {
        Self { config }
    }

    /// Run all DI/Shaku validations
    pub fn validate_all(&self) -> Result<Vec<ShakuViolation>> {
        let mut violations = Vec::new();
        violations.extend(self.validate_direct_instantiation()?);
        violations.extend(self.validate_fake_implementations()?);
        violations.extend(self.validate_null_provider_files()?);
        violations.extend(self.validate_dual_initialization()?);
        Ok(violations)
    }

    /// Check for direct ::new() calls on service types
    pub fn validate_direct_instantiation(&self) -> Result<Vec<ShakuViolation>> {
        let mut violations = Vec::new();
        // Pattern: SomeService::new() or SomeProvider::new() or SomeRepository::new()
        let new_pattern =
            Regex::new(r"([A-Z][a-zA-Z0-9_]*(?:Service|Provider|Repository|Handler|Manager))::new\s*\(")
                .expect("Invalid regex");

        for src_dir in self.config.get_scan_dirs()? {
            // Skip mcb-validate itself
            if src_dir.to_string_lossy().contains("mcb-validate") {
                continue;
            }

            for entry in WalkDir::new(&src_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
            {
                let path = entry.path();

                // Skip test files and directories
                if path.to_string_lossy().contains("/tests/")
                    || path.to_string_lossy().contains("_test.rs")
                    || path.to_string_lossy().contains("/test_")
                {
                    continue;
                }

                // Skip DI bootstrap files (they're allowed to instantiate)
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if file_name == "bootstrap.rs"
                    || file_name == "module.rs"
                    || file_name == "container.rs"
                {
                    continue;
                }

                let content = std::fs::read_to_string(path)?;
                let lines: Vec<&str> = content.lines().collect();

                // Track test modules to skip
                let mut in_test_module = false;
                let mut test_brace_depth: i32 = 0;
                let mut brace_depth: i32 = 0;

                for (line_num, line) in lines.iter().enumerate() {
                    let trimmed = line.trim();

                    // Track test module boundaries
                    if trimmed.contains("#[cfg(test)]") {
                        in_test_module = true;
                        test_brace_depth = brace_depth;
                    }

                    // Skip comments
                    if trimmed.starts_with("//") {
                        continue;
                    }

                    // Track brace depth
                    brace_depth += line.chars().filter(|c| *c == '{').count() as i32;
                    brace_depth -= line.chars().filter(|c| *c == '}').count() as i32;

                    // Exit test module when braces close
                    if in_test_module && brace_depth < test_brace_depth {
                        in_test_module = false;
                    }

                    // Skip test modules
                    if in_test_module {
                        continue;
                    }

                    // Check for direct instantiation
                    if let Some(cap) = new_pattern.captures(line) {
                        let type_name = cap.get(1).map(|m| m.as_str()).unwrap_or("");

                        // Skip if it's in a builder or factory method
                        if trimmed.contains("fn new") || trimmed.contains("fn build") {
                            continue;
                        }

                        violations.push(ShakuViolation::DirectInstantiation {
                            file: path.to_path_buf(),
                            line: line_num + 1,
                            type_name: type_name.to_string(),
                            suggestion: "Use DI container.resolve() instead".to_string(),
                            severity: Severity::Warning,
                        });
                    }
                }
            }
        }

        Ok(violations)
    }

    /// Check for Null/Fake/Mock types in production code
    pub fn validate_fake_implementations(&self) -> Result<Vec<ShakuViolation>> {
        let mut violations = Vec::new();
        // Pattern: NullXxx, FakeXxx, MockXxx, DummyXxx
        let fake_pattern =
            Regex::new(r"\b(Null|Fake|Mock|Dummy|Stub)([A-Z][a-zA-Z0-9_]*)")
                .expect("Invalid regex");

        for src_dir in self.config.get_scan_dirs()? {
            // Skip mcb-validate itself
            if src_dir.to_string_lossy().contains("mcb-validate") {
                continue;
            }

            for entry in WalkDir::new(&src_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
            {
                let path = entry.path();
                let path_str = path.to_string_lossy();

                // Skip test files and directories
                if path_str.contains("/tests/")
                    || path_str.contains("_test.rs")
                    || path_str.contains("/test_")
                {
                    continue;
                }

                // Skip null.rs files (handled separately)
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if file_name == "null.rs"
                    || file_name == "mock.rs"
                    || file_name == "fake.rs"
                    || file_name == "stub.rs"
                {
                    continue;
                }

                let content = std::fs::read_to_string(path)?;
                let lines: Vec<&str> = content.lines().collect();

                // Track test modules to skip
                let mut in_test_module = false;
                let mut test_brace_depth: i32 = 0;
                let mut brace_depth: i32 = 0;

                for (line_num, line) in lines.iter().enumerate() {
                    let trimmed = line.trim();

                    // Track test module boundaries
                    if trimmed.contains("#[cfg(test)]") {
                        in_test_module = true;
                        test_brace_depth = brace_depth;
                    }

                    // Skip comments
                    if trimmed.starts_with("//") {
                        continue;
                    }

                    // Track brace depth
                    brace_depth += line.chars().filter(|c| *c == '{').count() as i32;
                    brace_depth -= line.chars().filter(|c| *c == '}').count() as i32;

                    // Exit test module when braces close (use < not <= to avoid premature exit)
                    if in_test_module && brace_depth < test_brace_depth {
                        in_test_module = false;
                    }

                    // Skip test modules
                    if in_test_module {
                        continue;
                    }

                    // Check for fake implementations
                    if let Some(cap) = fake_pattern.captures(line) {
                        let prefix = cap.get(1).map(|m| m.as_str()).unwrap_or("");
                        let name = cap.get(2).map(|m| m.as_str()).unwrap_or("");
                        let full_name = format!("{}{}", prefix, name);

                        // Skip if it's in an import statement (definitions are elsewhere)
                        if !trimmed.starts_with("use ") && !trimmed.starts_with("mod ") {
                            violations.push(ShakuViolation::FakeImplementation {
                                file: path.to_path_buf(),
                                line: line_num + 1,
                                type_name: full_name,
                                context: trimmed.chars().take(60).collect(),
                                severity: Severity::Info,
                            });
                        }
                    }
                }
            }
        }

        Ok(violations)
    }

    /// Check for null.rs files in production code
    pub fn validate_null_provider_files(&self) -> Result<Vec<ShakuViolation>> {
        let mut violations = Vec::new();

        for src_dir in self.config.get_scan_dirs()? {
            // Skip mcb-validate itself
            if src_dir.to_string_lossy().contains("mcb-validate") {
                continue;
            }

            for entry in WalkDir::new(&src_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
            {
                let path = entry.path();
                let path_str = path.to_string_lossy();

                // Skip test directories
                if path_str.contains("/tests/") {
                    continue;
                }

                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

                // Check for null.rs, fake.rs, mock.rs in production directories
                if file_name == "null.rs" || file_name == "fake.rs" || file_name == "mock.rs" {
                    violations.push(ShakuViolation::NullProviderFile {
                        file: path.to_path_buf(),
                        severity: Severity::Info,
                    });
                }
            }
        }

        Ok(violations)
    }

    /// Check for dual initialization paths
    pub fn validate_dual_initialization(&self) -> Result<Vec<ShakuViolation>> {
        let mut violations = Vec::new();
        // Pattern: Both Arc::new(SomeService::new()) AND container.resolve()
        let arc_new_pattern =
            Regex::new(r"Arc::new\s*\(\s*[A-Z][a-zA-Z0-9_]*::new").expect("Invalid regex");
        let resolve_pattern = Regex::new(r"\.resolve\s*[:<]").expect("Invalid regex");

        for src_dir in self.config.get_scan_dirs()? {
            // Skip mcb-validate itself
            if src_dir.to_string_lossy().contains("mcb-validate") {
                continue;
            }

            for entry in WalkDir::new(&src_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
            {
                let path = entry.path();
                let path_str = path.to_string_lossy();

                // Skip test files
                if path_str.contains("/tests/") || path_str.contains("_test.rs") {
                    continue;
                }

                let content = std::fs::read_to_string(path)?;

                // Check if file has both patterns (indicates dual initialization)
                let has_arc_new = arc_new_pattern.is_match(&content);
                let has_resolve = resolve_pattern.is_match(&content);

                if has_arc_new && has_resolve {
                    // Find the Arc::new lines for reporting
                    let lines: Vec<&str> = content.lines().collect();

                    for (line_num, line) in lines.iter().enumerate() {
                        if arc_new_pattern.is_match(line) {
                            violations.push(ShakuViolation::DualInitializationPath {
                                file: path.to_path_buf(),
                                line: line_num + 1,
                                context: "Both manual Arc::new() and DI resolve() in same file"
                                    .to_string(),
                                severity: Severity::Warning,
                            });
                            break; // Only report once per file
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
    use std::fs;
    use tempfile::TempDir;

    fn create_test_crate(temp: &TempDir, name: &str, content: &str) {
        let crate_dir = temp.path().join("crates").join(name).join("src");
        fs::create_dir_all(&crate_dir).unwrap();
        fs::write(crate_dir.join("lib.rs"), content).unwrap();

        let cargo_dir = temp.path().join("crates").join(name);
        fs::write(
            cargo_dir.join("Cargo.toml"),
            format!(
                r#"
[package]
name = "{}"
version = "0.1.0"
"#,
                name
            ),
        )
        .unwrap();
    }

    #[test]
    fn test_direct_instantiation() {
        let temp = TempDir::new().unwrap();
        create_test_crate(
            &temp,
            "mcb-test",
            r#"
pub fn setup() {
    let service = MyService::new();
    let provider = EmbeddingProvider::new();
}
"#,
        );

        let validator = ShakuValidator::new(temp.path());
        let violations = validator.validate_direct_instantiation().unwrap();

        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_fake_implementation() {
        let temp = TempDir::new().unwrap();
        create_test_crate(
            &temp,
            "mcb-test",
            r#"
pub fn setup() {
    let provider: Arc<dyn Provider> = Arc::new(NullProvider::new());
    let mock = MockService::new();
}
"#,
        );

        let validator = ShakuValidator::new(temp.path());
        let violations = validator.validate_fake_implementations().unwrap();

        assert_eq!(violations.len(), 2);
    }

    #[test]
    fn test_no_violations_in_tests() {
        let temp = TempDir::new().unwrap();
        create_test_crate(
            &temp,
            "mcb-test",
            r#"
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let mock = MockService::new();
        let null = NullProvider::new();
    }
}
"#,
        );

        let validator = ShakuValidator::new(temp.path());
        let violations = validator.validate_all().unwrap();

        assert!(violations.is_empty(), "Test code should not trigger violations");
    }
}
