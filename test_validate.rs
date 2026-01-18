//! Test script to validate current codebase with new migration validators

use mcb_validate::{ArchitectureValidator, ValidationConfig};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Testing MCP Context Browser Architecture Validation");
    println!("====================================================");

    // Create validation config for the workspace
    let workspace_root = PathBuf::from(".");
    let config = ValidationConfig::new(workspace_root);

    // Create validator
    let mut validator = ArchitectureValidator::with_config(config);

    // Run comprehensive validation
    println!("🔍 Running comprehensive architecture validation...");
    let (legacy_report, registry_report) = validator.validate_comprehensive()?;

    // Print summary
    println!("\n📊 Validation Summary:");
    println!("======================");

    println!("Legacy Validators:");
    println!("  Total violations: {}", legacy_report.summary.total_violations);
    println!("  Dependency issues: {}", legacy_report.summary.dependency_count);
    println!("  Quality issues: {}", legacy_report.summary.quality_count);
    println!("  Shaku issues: {}", legacy_report.summary.shaku_count);

    println!("\nRegistry Validators:");
    println!("  Total violations: {}", registry_report.summary.total_violations);

    // Check for migration-specific violations
    let mut migration_violations = Vec::new();

    // Add legacy report violations
    migration_violations.extend(legacy_report.shaku_violations);

    // Add registry report violations
    migration_violations.extend(registry_report.violations.into_iter().map(|v| v.boxed()));

    println!("\n🔄 Migration-Specific Issues:");
    println!("==============================");

    let mut inventory_issues = 0;
    let mut shaku_issues = 0;
    let mut config_issues = 0;
    let mut rocket_issues = 0;

    for violation in &migration_violations {
        let id = violation.id();
        match &id[0..4] {
            "LINK" => inventory_issues += 1,
            "CTOR" => shaku_issues += 1,
            "FIGM" => config_issues += 1,
            "ROCK" => rocket_issues += 1,
            "SHAK" => shaku_issues += 1,
            _ => {}
        }
    }

    println!("  Inventory → Linkme: {} issues", inventory_issues);
    println!("  Shaku → Constructor Injection: {} issues", shaku_issues);
    println!("  Config → Figment: {} issues", config_issues);
    println!("  Axum → Rocket: {} issues", rocket_issues);

    // Show top issues by category
    println!("\n🚨 Top Migration Issues:");
    println!("========================");

    let mut sorted_violations = migration_violations;
    sorted_violations.sort_by(|a, b| {
        // Sort by severity (Error > Warning > Info), then by ID
        let severity_cmp = b.severity().cmp(&a.severity());
        if severity_cmp != std::cmp::Ordering::Equal {
            severity_cmp
        } else {
            a.id().cmp(b.id())
        }
    });

    for (i, violation) in sorted_violations.iter().take(20).enumerate() {
        println!("  {}. {}: {}", i + 1, violation.id(), violation.message());
        if let Some(suggestion) = violation.suggestion() {
            println!("     💡 {}", suggestion);
        }
        println!();
    }

    // Recommendations based on findings
    println!("📋 Migration Recommendations:");
    println!("============================");

    let total_migration_issues = inventory_issues + shaku_issues + config_issues + rocket_issues;

    if total_migration_issues == 0 {
        println!("✅ No migration issues found! Codebase is ready for v0.1.2.");
    } else {
        println!("🔄 Migration needed. Recommended order:");
        println!("  1. Fix Inventory issues ({} issues) - LINKME*", inventory_issues);
        println!("  2. Fix Shaku issues ({} issues) - CTOR*/SHAKU*", shaku_issues);
        println!("  3. Fix Config issues ({} issues) - FIGMENT*", config_issues);
        println!("  4. Fix Rocket issues ({} issues) - ROCKET*", rocket_issues);
        println!("\n💪 Total migration issues to fix: {}", total_migration_issues);
    }

    println!("\n✨ Validation complete!");

    Ok(())
}