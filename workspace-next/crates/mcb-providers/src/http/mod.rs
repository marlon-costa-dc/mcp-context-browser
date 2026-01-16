//! HTTP Client Module
//!
//! Provides HTTP client abstractions for API-based providers. This module defines
//! the interface (trait) that embedding and other providers depend on.
//!
//! ## Clean Architecture
//!
//! Following Dependency Inversion Principle:
//! - **Interface** (`HttpClientProvider` trait): Defined here in mcb-providers
//! - **Implementation** (`HttpClientPool`): Provided by mcb-infrastructure
//!
//! This allows providers to declare their HTTP requirements without depending
//! on concrete implementations, enabling testing with mocks.
//!
//! ## Components
//!
//! - [`HttpClientProvider`]: Trait for HTTP client operations
//! - [`HttpClientConfig`]: Configuration for HTTP client behavior
//! - [`HttpResponseUtils`]: Utilities for parsing API responses

mod config;
mod provider;
mod response;

pub use config::HttpClientConfig;
pub use provider::HttpClientProvider;
pub use response::HttpResponseUtils;
