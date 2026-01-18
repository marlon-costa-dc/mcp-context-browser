use mcb_validate::{ArchitectureValidator, ValidationConfig};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let workspace_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();

    println!("Checking test violations in: {}", workspace_root.display());

    let config = ValidationConfig::new(&workspace_root);
    let mut validator = ArchitectureValidator::with_config(config);

    // Check test violations
    let test_violations = validator.validate_tests()?;
    println!("\n=== Test Organization Violations ===");
    for v in &test_violations {
        println!("  [{:?}] {}", v.severity(), v);
    }
    println!("Total: {} test organization violations", test_violations.len());

    if test_violations.is_empty() {
        println!("\n✅ No test organization violations found!");
    } else {
        println!("\n❌ Found test organization violations that need to be fixed:");
        for v in &test_violations {
            if let Some(file) = v.file() {
                println!("  - {}: {}", file.display(), v);
            }
        }
    }

    Ok(())
}