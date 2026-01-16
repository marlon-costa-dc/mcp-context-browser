//! HTTP Transport for MCP
//!
//! Implements MCP protocol over HTTP using Server-Sent Events (SSE).
//! This transport allows web clients to connect to the MCP server.

use super::types::{McpRequest, McpResponse};
use crate::constants::JSONRPC_METHOD_NOT_FOUND;
use crate::McpServer;
use axum::{
    extract::State,
    response::{sse::Event, IntoResponse, Sse},
    routing::{get, post},
    Json, Router,
};
use futures::stream::Stream;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tracing::info;

/// HTTP transport configuration
#[derive(Debug, Clone)]
pub struct HttpTransportConfig {
    /// Host to bind to
    pub host: String,
    /// Port to listen on
    pub port: u16,
    /// Enable CORS for browser access
    pub enable_cors: bool,
}

impl Default for HttpTransportConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            enable_cors: true,
        }
    }
}

impl HttpTransportConfig {
    /// Create config for localhost with specified port
    pub fn localhost(port: u16) -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port,
            enable_cors: true,
        }
    }

    /// Get the socket address
    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .unwrap_or_else(|_| SocketAddr::from(([127, 0, 0, 1], self.port)))
    }
}

/// Shared state for HTTP transport
#[derive(Clone)]
pub struct HttpTransportState {
    /// Broadcast channel for SSE events
    pub event_tx: broadcast::Sender<String>,
    /// MCP server reference (for handling requests)
    #[allow(dead_code)]
    server: Arc<McpServer>,
}

/// HTTP transport server
pub struct HttpTransport {
    config: HttpTransportConfig,
    state: HttpTransportState,
}

impl HttpTransport {
    /// Create a new HTTP transport
    pub fn new(config: HttpTransportConfig, server: Arc<McpServer>) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        Self {
            config,
            state: HttpTransportState { event_tx, server },
        }
    }

    /// Create the HTTP router
    pub fn router(&self) -> Router {
        let state = self.state.clone();

        let router = Router::new()
            .route("/mcp", post(handle_mcp_request))
            .route("/events", get(handle_sse))
            .with_state(state);

        if self.config.enable_cors {
            // Add CORS headers
            router.layer(
                tower_http::cors::CorsLayer::new()
                    .allow_origin(tower_http::cors::Any)
                    .allow_methods([axum::http::Method::GET, axum::http::Method::POST])
                    .allow_headers(tower_http::cors::Any),
            )
        } else {
            router
        }
    }

    /// Start the HTTP transport server
    pub async fn start(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = self.config.socket_addr();
        let listener = TcpListener::bind(addr).await?;

        info!("HTTP transport listening on {}", addr);

        let router = self.router();
        axum::serve(listener, router).await?;

        Ok(())
    }

    /// Start with graceful shutdown
    pub async fn start_with_shutdown(
        self,
        shutdown_signal: impl std::future::Future<Output = ()> + Send + 'static,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = self.config.socket_addr();
        let listener = TcpListener::bind(addr).await?;

        info!("HTTP transport listening on {}", addr);

        let router = self.router();
        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_signal)
            .await?;

        Ok(())
    }
}

/// Handle MCP request via HTTP POST
async fn handle_mcp_request(
    State(_state): State<HttpTransportState>,
    Json(request): Json<McpRequest>,
) -> impl IntoResponse {
    // NOTE: HTTP routing is a placeholder pending full MCP protocol implementation
    // For now, return method not found for unimplemented methods
    let response = McpResponse::error(
        request.id,
        JSONRPC_METHOD_NOT_FOUND,
        format!("Method '{}' not yet implemented over HTTP", request.method),
    );

    Json(response)
}

/// Handle SSE connection for server-to-client events
async fn handle_sse(
    State(state): State<HttpTransportState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.event_tx.subscribe();

    let stream = async_stream::stream! {
        let mut rx = rx;
        while let Ok(data) = rx.recv().await {
            yield Ok(Event::default().data(data));
        }
    };

    Sse::new(stream)
}
