//! Rocket Routing Validator
//!
//! Validates migration from Axum to Rocket for HTTP routing.
//! Ensures all HTTP handlers use Rocket's attribute-based routing.

use crate::violation_trait::{Violation, ViolationCategory};
use crate::{Result, ValidationConfig};
use regex::Regex;
use std::path::{Path, PathBuf};

/// Violation for Axum usage that should be migrated to Rocket
#[derive(Debug, Clone)]
pub struct RocketViolation {
    pub file: PathBuf,
    pub line: usize,
    pub violation_type: RocketViolationType,
    pub context: String,
}

#[derive(Debug, Clone)]
pub enum RocketViolationType {
    /// Still using axum::Router pattern
    AxumRouterUsage,
    /// Still using axum::routing::* imports
    AxumRoutingUsage,
    /// Missing Rocket attribute macros (#[get], #[post], etc.)
    MissingRocketAttributes,
    /// Routes not organized with routes![] macro
    MissingRoutesMacro,
    /// Axum dependency still present in Cargo.toml
    AxumDependencyPresent,
    /// Tower middleware usage that should be Rocket fairings
    TowerMiddlewareUsage,
}

impl RocketViolation {
    pub fn new(
        file: impl Into<PathBuf>,
        line: usize,
        violation_type: RocketViolationType,
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

impl Violation for RocketViolation {
    fn id(&self) -> &str {
        match self.violation_type {
            RocketViolationType::AxumRouterUsage => "ROCKET001",
            RocketViolationType::AxumRoutingUsage => "ROCKET002",
            RocketViolationType::MissingRocketAttributes => "ROCKET003",
            RocketViolationType::MissingRoutesMacro => "ROCKET004",
            RocketViolationType::AxumDependencyPresent => "ROCKET005",
            RocketViolationType::TowerMiddlewareUsage => "ROCKET006",
        }
    }

    fn category(&self) -> ViolationCategory {
        ViolationCategory::WebFramework
    }

    fn file(&self) -> Option<&std::path::PathBuf> {
        Some(&self.file)
    }

    fn line(&self) -> Option<usize> {
        Some(self.line)
    }

    fn message(&self) -> String {
        match self.violation_type {
            RocketViolationType::AxumRouterUsage => {
                "Found axum::Router usage that should be migrated to Rocket".to_string()
            }
            RocketViolationType::AxumRoutingUsage => {
                "Found axum::routing::* usage that should be migrated to Rocket attributes".to_string()
            }
            RocketViolationType::MissingRocketAttributes => {
                "HTTP handlers missing Rocket attribute macros (#[get], #[post], etc.)".to_string()
            }
            RocketViolationType::MissingRoutesMacro => {
                "Routes not organized with Rocket's routes![] macro".to_string()
            }
            RocketViolationType::AxumDependencyPresent => {
                "Axum dependency still present - should be replaced with rocket".to_string()
            }
            RocketViolationType::TowerMiddlewareUsage => {
                "Found Tower middleware usage that should be migrated to Rocket fairings".to_string()
            }
        }
    }

    fn suggestion(&self) -> Option<String> {
        match self.violation_type {
            RocketViolationType::AxumRouterUsage => {
                Some("Replace axum::Router with Rocket's attribute-based routing and rocket::build()".to_string())
            }
            RocketViolationType::AxumRoutingUsage => {
                Some("Replace axum::routing::* with Rocket attribute macros (#[get], #[post], etc.)".to_string())
            }
            RocketViolationType::MissingRocketAttributes => {
                Some("Add Rocket attribute macro (#[get], #[post], #[put], #[delete]) above handler function".to_string())
            }
            RocketViolationType::MissingRoutesMacro => {
                Some("Organize routes with Rocket's routes![] macro and mount with .mount()".to_string())
            }
            RocketViolationType::AxumDependencyPresent => {
                Some("Remove axum/axum-extra/tower dependencies and add rocket dependency".to_string())
            }
            RocketViolationType::TowerMiddlewareUsage => {
                Some("Replace Tower middleware with Rocket fairings using the Fairing trait".to_string())
            }
        }
    }

    fn severity(&self) -> crate::Severity {
        match self.violation_type {
            RocketViolationType::PoemUsage => crate::Severity::Error,
            RocketViolationType::TowerUsage => crate::Severity::Error,
            RocketViolationType::MixedRouting => crate::Severity::Error,
            RocketViolationType::MissingRocketMigration => crate::Severity::Error,
            RocketViolationType::TowerMiddlewareUsage => crate::Severity::Error,
            RocketViolationType::FallbackPattern => crate::Severity::Warning,
            RocketViolationType::UnsafeRocketSetup => crate::Severity::Warning,
        }
    }
}

/// Rocket routing validator
#[derive(Debug)]
pub struct RocketValidator {
    config: ValidationConfig,
}

impl RocketValidator {
    pub fn with_config(config: ValidationConfig) -> Self {
        Self { config }
    }

    pub fn validate_all(&self) -> Result<Vec<RocketViolation>> {
        let mut violations = Vec::new();

        // Check Cargo.toml files for axum dependencies
        violations.extend(self.check_cargo_dependencies()?);

        // Check Rust source files for Axum/Rocket usage
        violations.extend(self.check_source_files()?);

        Ok(violations)
    }

    fn check_cargo_dependencies(&self) -> Result<Vec<RocketViolation>> {
        let mut violations = Vec::new();

        let cargo_files = self.find_cargo_files()?;

        for cargo_path in cargo_files {
            if let Ok(content) = std::fs::read_to_string(&cargo_path) {
                // Check for axum dependencies
                if content.contains("axum =") || content.contains("axum = {") ||
                   content.contains("axum-extra =") || content.contains("tower =") ||
                   content.contains("tower-http =") {
                    violations.push(RocketViolation::new(
                        cargo_path.to_string_lossy(),
                        0,
                        RocketViolationType::AxumDependencyPresent,
                        "Found Axum/Tower dependencies in Cargo.toml".to_string(),
                    ));
                }

                // Check that rocket is present for server crates
                if !content.contains("rocket =") && !content.contains("rocket = {") {
                    if self.is_server_crate(&cargo_path) {
                        violations.push(RocketViolation::new(
                            cargo_path.to_string_lossy(),
                            0,
                            RocketViolationType::MissingRocketAttributes,
                            "Server crate missing rocket dependency".to_string(),
                        ));
                    }
                }
            }
        }

        Ok(violations)
    }

    fn check_source_files(&self) -> Result<Vec<RocketViolation>> {
        let mut violations = Vec::new();

        let source_files = self.find_source_files()?;

        for file_path in source_files {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                violations.extend(self.check_file_content(&file_path, &content));
            }
        }

        Ok(violations)
    }

    fn check_file_content(&self, file_path: &Path, content: &str) -> Vec<RocketViolation> {
        let mut violations = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Regex patterns for Axum usage
        let axum_router_regex = Regex::new(r"axum::Router").unwrap();
        let axum_routing_regex = Regex::new(r"axum::routing::").unwrap();
        let tower_middleware_regex = Regex::new(r"tower(|_http)::").unwrap();

        for (i, line) in lines.iter().enumerate() {
            let line_num = i + 1;

            // Check for axum::Router usage
            if axum_router_regex.is_match(line) {
                violations.push(RocketViolation::new(
                    file_path.to_string_lossy(),
                    line_num,
                    RocketViolationType::AxumRouterUsage,
                    line.trim().to_string(),
                ));
            }

            // Check for axum::routing::* usage
            if axum_routing_regex.is_match(line) {
                violations.push(RocketViolation::new(
                    file_path.to_string_lossy(),
                    line_num,
                    RocketViolationType::AxumRoutingUsage,
                    line.trim().to_string(),
                ));
            }

            // Check for Tower middleware usage
            if tower_middleware_regex.is_match(line) {
                violations.push(RocketViolation::new(
                    file_path.to_string_lossy(),
                    line_num,
                    RocketViolationType::TowerMiddlewareUsage,
                    line.trim().to_string(),
                ));
            }
        }

        // Check for missing Rocket patterns in handler files
        if self.is_handler_file(file_path) {
            violations.extend(self.check_rocket_patterns(file_path, content));
        }

        violations
    }

    fn check_rocket_patterns(&self, file_path: &Path, content: &str) -> Vec<RocketViolation> {
        let mut violations = Vec::new();

        let has_rocket_get = content.contains("#[get(");
        let has_rocket_post = content.contains("#[post(");
        let has_rocket_put = content.contains("#[put(");
        let has_rocket_delete = content.contains("#[delete(");
        let has_routes_macro = content.contains("routes![");
        let has_axum_handler = content.contains("axum::") && content.contains("fn ");

        // If file has HTTP handlers but uses Axum patterns
        if has_axum_handler && !has_rocket_get && !has_rocket_post && !has_rocket_put && !has_rocket_delete {
            violations.push(RocketViolation::new(
                file_path.to_string_lossy(),
                1,
                RocketViolationType::MissingRocketAttributes,
                "HTTP handlers use Axum patterns instead of Rocket attributes".to_string(),
            ));
        }

        // If file has Rocket handlers but missing routes![] macro
        if (has_rocket_get || has_rocket_post || has_rocket_put || has_rocket_delete) && !has_routes_macro {
            violations.push(RocketViolation::new(
                file_path.to_string_lossy(),
                1,
                RocketViolationType::MissingRoutesMacro,
                "Rocket handlers not organized with routes![] macro".to_string(),
            ));
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

    fn is_server_crate(&self, cargo_path: &Path) -> bool {
        cargo_path.to_string_lossy().contains("mcb-server")
    }

    fn is_handler_file(&self, file_path: &Path) -> bool {
        let path_str = file_path.to_string_lossy();
        path_str.contains("handler") ||
        path_str.contains("route") ||
        path_str.contains("api") ||
        path_str.contains("controller") ||
        path_str.contains("endpoint") ||
        path_str.contains("http") ||
        path_str.contains("web")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_axum_router_detection() {
        let validator = RocketValidator::with_config(ValidationConfig::new("/tmp"));
        let content = r#"
        use axum::{Router, routing::get};

        let router = Router::new()
            .route("/health", get(health_check))
            .route("/metrics", get(get_metrics));
        "#;

        let violations = validator.check_file_content(Path::new("test.rs"), content);
        assert!(violations.iter().any(|v| matches!(v.violation_type, RocketViolationType::AxumRouterUsage)));
        assert!(violations.iter().any(|v| matches!(v.violation_type, RocketViolationType::AxumRoutingUsage)));
    }

    #[test]
    fn test_tower_middleware_detection() {
        let validator = RocketValidator::with_config(ValidationConfig::new("/tmp"));
        let content = r#"
        use tower_http::{cors::CorsLayer, trace::TraceLayer};

        let app = Router::new()
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http());
        "#;

        let violations = validator.check_file_content(Path::new("test.rs"), content);
        assert!(violations.iter().any(|v| matches!(v.violation_type, RocketViolationType::TowerMiddlewareUsage)));
    }

    #[test]
    fn test_missing_rocket_attributes() {
        let validator = RocketValidator::with_config(ValidationConfig::new("/tmp"));
        let content = r#"
        use axum::Json;

        pub async fn health_check() -> Json<HealthStatus> {
            Json(HealthStatus { status: "ok".to_string() })
        }
        "#;

        let violations = validator.check_file_content(Path::new("handlers/health.rs"), content);
        assert!(violations.iter().any(|v| matches!(v.violation_type, RocketViolationType::MissingRocketAttributes)));
    }
}