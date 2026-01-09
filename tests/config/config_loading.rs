use mcp_context_browser::config::ConfigLoader;
use std::env;
use std::io::Write;
use tempfile::Builder;

#[tokio::test]
async fn test_config_loader_priority() {
    // Set some env vars
    unsafe {
        env::set_var("MCP__SERVER__PORT", "4000");
        env::set_var("MCP__METRICS__PORT", "4001");
    }

    // Create a temp config file with .toml extension
    let mut file = Builder::new().suffix(".toml").tempfile().unwrap();

    writeln!(
        file,
        r#"
[server]
port = 5000
host = "0.0.0.0"

[metrics]
port = 5001
enabled = true
"#
    )
    .unwrap();

    // Load config
    let loader = ConfigLoader::new();
    let config = loader.load_with_file(file.path()).await.unwrap();

    assert_eq!(config.server.port, 4000); // Env priority
    assert_eq!(config.server.host, "0.0.0.0"); // File fallback
    assert_eq!(config.metrics.port, 4001); // Env priority
}
