//! Tests for the server module

#[path = "server/handlers_test.rs"]
mod handlers_test;

#[path = "server/rate_limit_middleware.rs"]
mod rate_limit_middleware;

#[path = "server/security.rs"]
mod security;

#[path = "server/transport.rs"]
mod transport;
