//! Tests for the server module

// Note: handlers tests are in handlers.bak/ pending API updates

#[path = "server/rate_limit_middleware.rs"]
mod rate_limit_middleware;

#[path = "server/security.rs"]
mod security;

#[path = "server/transport.rs"]
mod transport;
