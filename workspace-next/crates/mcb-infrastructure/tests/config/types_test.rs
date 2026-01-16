//! Configuration Types Tests

use mcb_infrastructure::config::types::{ConfigKey, ConfigProfile, ValidationResult};

#[test]
fn test_config_key_utilities() {
    assert_eq!(ConfigKey::join(&["server", "port"]), "server.port");
    assert_eq!(ConfigKey::split("server.port"), vec!["server", "port"]);
    assert!(ConfigKey::is_under_prefix("server.port", "server"));
    assert_eq!(ConfigKey::parent("server.port"), Some("server".to_string()));
    assert_eq!(ConfigKey::last_component("server.port"), "port");
}

#[test]
fn test_validation_result() {
    let valid = ValidationResult::valid();
    assert!(valid.is_valid);
    assert!(!valid.has_errors());

    let invalid = ValidationResult::invalid(vec!["error1".to_string()]).with_warning("warning1");
    assert!(!invalid.is_valid);
    assert!(invalid.has_errors());
    assert!(invalid.has_warnings());
}

#[test]
fn test_config_profile() {
    let dev_profile = ConfigProfile::Development;
    let overrides = dev_profile.overrides();
    assert!(overrides.contains_key("logging.level"));

    let prod_profile = ConfigProfile::Production;
    let prod_overrides = prod_profile.overrides();
    assert_eq!(
        prod_overrides.get("logging.level").unwrap(),
        &serde_json::Value::String("info".to_string())
    );
}
