//! Server Configuration Tests

use mcb_infrastructure::config::data::ServerConfig;
use mcb_infrastructure::config::server::{
    ServerConfigBuilder, ServerConfigPresets, ServerConfigUtils,
};
use mcb_infrastructure::constants::DEFAULT_HTTPS_PORT;
use std::net::SocketAddr;

#[test]
fn test_parse_address() {
    let config = ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        ..Default::default()
    };

    let address = ServerConfigUtils::parse_address(&config).unwrap();
    assert_eq!(address, SocketAddr::from(([127, 0, 0, 1], 8080)));
}

#[test]
fn test_server_url() {
    let http_config = ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
        https: false,
        ..Default::default()
    };

    let https_config = ServerConfig {
        host: "example.com".to_string(),
        port: 8443,
        https: true,
        ..Default::default()
    };

    assert_eq!(
        ServerConfigUtils::get_server_url(&http_config),
        "http://127.0.0.1:8080"
    );
    assert_eq!(
        ServerConfigUtils::get_server_url(&https_config),
        "https://example.com:8443"
    );
}

#[test]
fn test_server_config_builder() {
    let config = ServerConfigBuilder::new()
        .host("0.0.0.0")
        .port(9000)
        .https(true)
        .request_timeout(120)
        .cors(true, vec!["https://app.example.com".to_string()])
        .build();

    assert_eq!(config.host, "0.0.0.0");
    assert_eq!(config.port, 9000);
    assert!(config.https);
    assert_eq!(config.request_timeout_secs, 120);
    assert!(config.cors_enabled);
    assert_eq!(config.cors_origins, vec!["https://app.example.com"]);
}

#[test]
fn test_presets() {
    let dev_config = ServerConfigPresets::development();
    assert_eq!(dev_config.host, "127.0.0.1");
    assert_eq!(dev_config.port, 8080);
    assert!(!dev_config.https);

    let prod_config = ServerConfigPresets::production();
    assert_eq!(prod_config.host, "0.0.0.0");
    assert_eq!(prod_config.port, DEFAULT_HTTPS_PORT);
    assert!(prod_config.https);

    let test_config = ServerConfigPresets::testing();
    assert_eq!(test_config.port, 0); // Random port
    assert!(!test_config.https);
}
