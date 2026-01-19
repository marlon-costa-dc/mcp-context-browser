//! Dependency Injection System - Simple Container Architecture
//!
//! This module implements dependency injection using simple Arc-based containers,
//! following Clean Architecture and Domain-Driven Design principles.
//!
//! ## Key Principles
//!
//! - **Trait-based DI**: All dependencies injected as `Arc<dyn Trait>`
//! - **Composition Root**: Services composed in bootstrap.rs init_app()
//! - **Domain Separation**: Infrastructure concerns separate from business logic
//! - **Testability**: Null providers enable isolated testing

pub mod bootstrap;
pub mod dispatch;
pub mod modules;
pub mod resolver;

pub use bootstrap::*;
pub use dispatch::*;
pub use modules::{DomainServicesContainer, DomainServicesFactory, ServiceDependencies};
pub use resolver::{resolve_providers, ResolvedProviders};
