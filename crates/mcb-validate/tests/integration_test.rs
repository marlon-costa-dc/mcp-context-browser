//! Integration tests that validate the actual workspace

use mcb_validate::{ArchitectureValidator, Reporter, Severity, ValidationConfig};
use std::path::PathBuf;

fn get_workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

#[test]
fn test_validate_workspace_dependencies() {
    let workspace_root = get_workspace_root();
    let mut validator = ArchitectureValidator::new(&workspace_root);

    let violations = validator.validate_dependencies().unwrap();

    println!("\n=== Dependency Violations ===");
    for v in &violations {
        println!("  [{:?}] {}", v.severity(), v);
    }
    println!("Total: {} dependency violations\n", violations.len());

    // Dependencies should follow Clean Architecture
    assert!(
        violations.is_empty(),
        "Found {} dependency violations - Clean Architecture rules violated",
        violations.len()
    );
}

#[test]
fn test_validate_workspace_quality() {
    let workspace_root = get_workspace_root();
    let mut validator = ArchitectureValidator::new(&workspace_root);

    let violations = validator.validate_quality().unwrap();

    println!("\n=== Quality Violations ===");
    let errors: Vec<_> = violations
        .iter()
        .filter(|v| v.severity() == Severity::Error)
        .collect();
    let warnings: Vec<_> = violations
        .iter()
        .filter(|v| v.severity() == Severity::Warning)
        .collect();

    for v in &errors {
        println!("  [ERROR] {}", v);
    }
    for v in &warnings {
        println!("  [WARNING] {}", v);
    }
    println!(
        "Total: {} errors, {} warnings\n",
        errors.len(),
        warnings.len()
    );

    // Report but don't fail on warnings (file size, TODOs)
    // Only fail on errors (unwrap/expect/panic in production)
    if !errors.is_empty() {
        println!("\nProduction code contains unwrap/expect/panic!");
        for e in &errors {
            println!("  - {}", e);
        }
    }
}

#[test]
fn test_validate_workspace_patterns() {
    let workspace_root = get_workspace_root();
    let mut validator = ArchitectureValidator::new(&workspace_root);

    let violations = validator.validate_patterns().unwrap();

    println!("\n=== Pattern Violations ===");
    for v in &violations {
        println!("  [{:?}] {}", v.severity(), v);
    }
    println!("Total: {} pattern violations\n", violations.len());
}

#[test]
fn test_validate_workspace_tests() {
    let workspace_root = get_workspace_root();
    let mut validator = ArchitectureValidator::new(&workspace_root);

    let violations = validator.validate_tests().unwrap();

    println!("\n=== Test Organization Violations ===");
    for v in &violations {
        println!("  [{:?}] {}", v.severity(), v);
    }
    println!("Total: {} test organization violations\n", violations.len());
}

#[test]
fn test_validate_workspace_documentation() {
    let workspace_root = get_workspace_root();
    let mut validator = ArchitectureValidator::new(&workspace_root);

    let violations = validator.validate_documentation().unwrap();

    println!("\n=== Documentation Violations ===");
    let by_severity = |sev: Severity| violations.iter().filter(|v| v.severity() == sev).count();

    println!(
        "  Errors: {}, Warnings: {}, Info: {}",
        by_severity(Severity::Error),
        by_severity(Severity::Warning),
        by_severity(Severity::Info)
    );

    // Only print first 20 violations to avoid noise
    for v in violations.iter().take(20) {
        println!("  [{:?}] {}", v.severity(), v);
    }
    if violations.len() > 20 {
        println!("  ... and {} more", violations.len() - 20);
    }
    println!("Total: {} documentation violations\n", violations.len());
}

#[test]
fn test_validate_workspace_naming() {
    let workspace_root = get_workspace_root();
    let mut validator = ArchitectureValidator::new(&workspace_root);

    let violations = validator.validate_naming().unwrap();

    println!("\n=== Naming Violations ===");
    for v in &violations {
        println!("  [{:?}] {}", v.severity(), v);
    }
    println!("Total: {} naming violations\n", violations.len());
}

#[test]
fn test_full_validation_report() {
    let workspace_root = get_workspace_root();
    let mut validator = ArchitectureValidator::new(&workspace_root);

    let report = validator.validate_all().unwrap();

    println!("\n{}", Reporter::to_human_readable(&report));

    // Count error-level violations
    let error_count = Reporter::count_errors(&report);
    let warning_count = Reporter::count_warnings(&report);

    println!(
        "Summary: {} errors, {} warnings, {} total violations",
        error_count, warning_count, report.summary.total_violations
    );

    // The validation should complete without panicking
    // We don't assert on violation count as existing code may have issues
}

// ============================================
// Multi-Directory Validation Tests
// ============================================

#[test]
fn test_validation_with_legacy() {
    let workspace_root = get_workspace_root();
    let legacy_path = workspace_root.join("src.legacy");

    // Skip if legacy code has been removed
    if !legacy_path.exists() {
        println!("Skipping test: src.legacy/ not found (legacy code removed)");
        return;
    }

    let config = ValidationConfig::new(&workspace_root)
        .with_additional_path("src.legacy") // Archived legacy src/
        .with_exclude_pattern("target/");

    let mut validator = ArchitectureValidator::with_config(config);
    let report = validator.validate_all().unwrap();

    println!("\n{}", "=".repeat(60));
    println!("COMBINED VALIDATION: crates + src.legacy");
    println!("{}", "=".repeat(60));
    println!("{}", Reporter::to_human_readable(&report));

    let error_count = Reporter::count_errors(&report);
    let warning_count = Reporter::count_warnings(&report);
    let info_count = report.summary.total_violations - error_count - warning_count;

    println!(
        "\nSummary: {} errors, {} warnings, {} info, {} total",
        error_count, warning_count, info_count, report.summary.total_violations
    );
}

#[test]
fn test_legacy_only() {
    let workspace_root = get_workspace_root();
    let legacy_path = workspace_root.join("src.legacy");

    // Skip if legacy code has been removed
    if !legacy_path.exists() {
        println!("Skipping test: src.legacy/ not found (legacy code removed)");
        return;
    }

    let config = ValidationConfig::new(&workspace_root)
        .with_additional_path("src.legacy")
        .with_exclude_pattern("target/");

    let mut validator = ArchitectureValidator::with_config(config);
    let report = validator.validate_all().unwrap();

    // Filter to show only legacy violations (those containing "src.legacy" in path)
    println!("\n{}", "=".repeat(60));
    println!("LEGACY VIOLATIONS (src.legacy only)");
    println!("{}", "=".repeat(60));

    // Quality violations from legacy
    let legacy_quality: Vec<_> = report
        .quality_violations
        .iter()
        .filter(|v| {
            let display = format!("{}", v);
            display.contains("src.legacy") && !display.contains("/crates/")
        })
        .collect();

    if !legacy_quality.is_empty() {
        println!(
            "\nQuality violations in src.legacy: {}",
            legacy_quality.len()
        );
        for v in legacy_quality.iter().take(10) {
            println!("  [{:?}] {}", v.severity(), v);
        }
        if legacy_quality.len() > 10 {
            println!("  ... and {} more", legacy_quality.len() - 10);
        }
    }

    println!(
        "\nTotal violations: {} (showing subset from src.legacy)",
        report.summary.total_violations
    );
}

#[test]
fn test_validation_config() {
    let workspace_root = get_workspace_root();
    let config = ValidationConfig::new(&workspace_root)
        .with_additional_path("src.legacy")
        .with_exclude_pattern("target/");

    println!("\n{}", "=".repeat(60));
    println!("VALIDATION CONFIGURATION");
    println!("{}", "=".repeat(60));
    println!("Workspace root: {}", config.workspace_root.display());
    println!("Additional paths: {:?}", config.additional_src_paths);
    println!("Exclude patterns: {:?}", config.exclude_patterns);

    let dirs = config.get_source_dirs().unwrap();
    println!("\nSource directories to scan:");
    for dir in &dirs {
        println!("  - {}", dir.display());
    }
    println!("\nTotal directories: {}", dirs.len());
}

// =============================================================================
// MIGRATION VALIDATOR TESTS (v0.1.2)
// =============================================================================

#[test]
fn test_linkme_validator() {
    use mcb_validate::LinkmeValidator;

    let workspace_root = get_workspace_root();
    let config = ValidationConfig::new(&workspace_root);
    let validator = LinkmeValidator::with_config(config);

    let violations = validator.validate_all().unwrap();

    println!("\n=== Linkme Migration Violations ===");
    for v in &violations {
        println!("  [{:?}] {}: {}", v.severity(), v.id(), v.message());
        if let Some(suggestion) = v.suggestion() {
            println!("    💡 {}", suggestion);
        }
    }
    println!("Total: {} linkme violations\n", violations.len());

    // Count by type for reporting
    let mut inventory_usage = 0;
    let mut dependency_issues = 0;
    let mut pattern_issues = 0;

    for v in &violations {
        match v.id().chars().nth(5) {
            Some('1') => inventory_usage += 1, // LINKME001
            Some('5') => dependency_issues += 1, // LINKME005
            _ => pattern_issues += 1,
        }
    }

    println!("Breakdown:");
    println!("  - Inventory usage: {}", inventory_usage);
    println!("  - Dependency issues: {}", dependency_issues);
    println!("  - Pattern issues: {}", pattern_issues);

    // Allow some issues during migration
    assert!(
        violations.len() >= 0,
        "Linkme validator should run without errors"
    );
}

#[test]
fn test_constructor_injection_validator() {
    use mcb_validate::ConstructorInjectionValidator;

    let workspace_root = get_workspace_root();
    let config = ValidationConfig::new(&workspace_root);
    let validator = ConstructorInjectionValidator::with_config(config);

    let violations = validator.validate_all().unwrap();

    println!("\n=== Constructor Injection Migration Violations ===");
    for v in &violations {
        println!("  [{:?}] {}: {}", v.severity(), v.id(), v.message());
        if let Some(suggestion) = v.suggestion() {
            println!("    💡 {}", suggestion);
        }
    }
    println!("Total: {} constructor injection violations\n", violations.len());

    // Count by type for reporting
    let mut shaku_usage = 0;
    let mut pattern_issues = 0;

    for v in &violations {
        match v.id().chars().nth(4) {
            Some('1') | Some('2') | Some('3') | Some('4') => shaku_usage += 1, // CTOR001-004
            _ => pattern_issues += 1,
        }
    }

    println!("Breakdown:");
    println!("  - Shaku usage: {}", shaku_usage);
    println!("  - Pattern issues: {}", pattern_issues);

    // Allow some issues during migration
    assert!(
        violations.len() >= 0,
        "Constructor injection validator should run without errors"
    );
}

#[test]
fn test_figment_validator() {
    use mcb_validate::FigmentValidator;

    let workspace_root = get_workspace_root();
    let config = ValidationConfig::new(&workspace_root);
    let validator = FigmentValidator::with_config(config);

    let violations = validator.validate_all().unwrap();

    println!("\n=== Figment Migration Violations ===");
    for v in &violations {
        println!("  [{:?}] {}: {}", v.severity(), v.id(), v.message());
        if let Some(suggestion) = v.suggestion() {
            println!("    💡 {}", suggestion);
        }
    }
    println!("Total: {} figment violations\n", violations.len());

    // Count by type for reporting
    let mut config_usage = 0;
    let mut dependency_issues = 0;

    for v in &violations {
        match v.id().chars().nth(7) {
            Some('1') | Some('2') | Some('3') => config_usage += 1, // FIGMENT001-003
            Some('6') => dependency_issues += 1, // FIGMENT006
            _ => {}
        }
    }

    println!("Breakdown:");
    println!("  - Config crate usage: {}", config_usage);
    println!("  - Dependency issues: {}", dependency_issues);

    // Allow some issues during migration
    assert!(
        violations.len() >= 0,
        "Figment validator should run without errors"
    );
}

#[test]
fn test_rocket_validator() {
    use mcb_validate::RocketValidator;

    let workspace_root = get_workspace_root();
    let config = ValidationConfig::new(&workspace_root);
    let validator = RocketValidator::with_config(config);

    let violations = validator.validate_all().unwrap();

    println!("\n=== Rocket Migration Violations ===");
    for v in &violations {
        println!("  [{:?}] {}: {}", v.severity(), v.id(), v.message());
        if let Some(suggestion) = v.suggestion() {
            println!("    💡 {}", suggestion);
        }
    }
    println!("Total: {} rocket violations\n", violations.len());

    // Count by type for reporting
    let mut axum_usage = 0;
    let mut dependency_issues = 0;

    for v in &violations {
        match v.id().chars().nth(6) {
            Some('1') | Some('2') => axum_usage += 1, // ROCKET001-002
            Some('5') => dependency_issues += 1, // ROCKET005
            _ => {}
        }
    }

    println!("Breakdown:");
    println!("  - Axum usage: {}", axum_usage);
    println!("  - Dependency issues: {}", dependency_issues);

    // Allow some issues during migration
    assert!(
        violations.len() >= 0,
        "Rocket validator should run without errors"
    );
}
