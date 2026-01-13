use clap::Parser;
use mcp_context_browser::server::run_server;

#[derive(Parser, Debug)]
#[command(name = "mcp-context-browser")]
#[command(about = "MCP Context Browser - Semantic Code Search Server")]
#[command(version)]
struct Cli {
    /// Path to configuration file
    #[arg(short, long)]
    config: Option<std::path::PathBuf>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    run_server(cli.config.as_deref()).await
}
