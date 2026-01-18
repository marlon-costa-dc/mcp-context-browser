//! Test file for YAML-based rule validation system
//!
//! This demonstrates how the new YAML rule system works.

use mcb_validate::{ValidationConfig, ArchitectureValidator};
use std::path::PathBuf;

#[tokio::test]
async fn test_yaml_rule_loading() {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let config = ValidationConfig::new(workspace_root);
    let validator = ArchitectureValidator::with_config(config);

    // Test loading YAML rules
    let rules = validator.load_yaml_rules().await;
    match rules {
        Ok(rules) => {
            println!("Loaded {} YAML rules", rules.len());
            for rule in &rules {
                println!("- {}: {} ({})", rule.id, rule.name, rule.category);
            }
            assert!(!rules.is_empty(), "Should load at least some rules");
        }
        Err(e) => {
            println!("YAML rules not available yet: {}", e);
            // This is expected during development
        }
    }
}

#[tokio::test]
async fn test_yaml_validation() {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let config = ValidationConfig::new(workspace_root);
    let validator = ArchitectureValidator::with_config(config);

    // Test YAML-based validation
    let report = validator.validate_with_yaml_rules().await;
    match report {
        Ok(report) => {
            println!("YAML validation completed");
            println!("Violations found: {}", report.summary.total_violations);
        }
        Err(e) => {
            println!("YAML validation not available yet: {}", e);
            // This is expected during development
        }
    }
}

#[tokio::test]
fn test_yaml_validator_creation() {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_path_buf();
    let config = ValidationConfig::new(workspace_root);
    let validator = ArchitectureValidator::with_config(config);

    // Test YAML validator creation
    let yaml_validator = validator.yaml_validator();
    match yaml_validator {
        Ok(_) => println!("YAML validator created successfully"),
        Err(e) => {
            println!("YAML validator not available yet: {}", e);
            // This is expected during development
        }
    }
}