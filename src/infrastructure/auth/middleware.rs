//! Authentication middleware for Axum
//!
//! Provides JWT validation middleware and request extractors with rate limiting.

use super::claims::Claims;
use super::rate_limit::{extract_client_id, AuthRateLimiter, RateLimitError};
use super::service::AuthService;
use axum::{
    extract::{ConnectInfo, Request},
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::net::SocketAddr;
use std::sync::Arc;

/// Authentication middleware that validates JWT tokens with rate limiting
///
/// Extracts Bearer token from Authorization header and validates it.
/// On success, injects Claims into request extensions for handlers.
///
/// # Rate Limiting
///
/// When a `RateLimiterState` is provided, the middleware enforces rate limits
/// before processing authentication. This protects against brute-force attacks.
///
/// # Bypass Paths
///
/// Certain paths can be configured to bypass authentication (e.g., health checks).
/// These are configured in `AuthConfig::bypass_paths`.
pub async fn auth_middleware(
    auth_service: Arc<AuthService>,
    rate_limiter: Option<Arc<AuthRateLimiter>>,
    mut req: Request,
    next: Next,
) -> std::result::Result<Response, Response> {
    // Skip auth if disabled
    if !auth_service.is_enabled() {
        return Ok(next.run(req).await);
    }

    // Check if path should bypass authentication
    let path = req.uri().path();
    if auth_service.should_bypass(path) {
        return Ok(next.run(req).await);
    }

    // Apply rate limiting if enabled
    if let Some(limiter) = &rate_limiter {
        // Extract client ID from connection info
        let client_id = req
            .extensions()
            .get::<ConnectInfo<SocketAddr>>()
            .map(|ci| extract_client_id(Some(ci)))
            .unwrap_or_else(|| extract_client_id(None));

        // Check rate limit
        if let Err(retry_after) = limiter.check_request(&client_id) {
            return Err(RateLimitError { retry_after }.into_response());
        }
    }

    // Extract token from Authorization header
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(auth) if auth.starts_with("Bearer ") => auth.trim_start_matches("Bearer "),
        _ => {
            // Record failed attempt for rate limiting
            if let Some(limiter) = &rate_limiter {
                let client_id = req
                    .extensions()
                    .get::<ConnectInfo<SocketAddr>>()
                    .map(|ci| extract_client_id(Some(ci)))
                    .unwrap_or_else(|| extract_client_id(None));
                limiter.record_failed_attempt(&client_id);
            }
            return Err(StatusCode::UNAUTHORIZED.into_response());
        }
    };

    // Validate token
    match auth_service.validate_token(token) {
        Ok(claims) => {
            // Record successful auth for rate limiting
            if let Some(limiter) = &rate_limiter {
                let client_id = req
                    .extensions()
                    .get::<ConnectInfo<SocketAddr>>()
                    .map(|ci| extract_client_id(Some(ci)))
                    .unwrap_or_else(|| extract_client_id(None));
                limiter.record_success(&client_id);
            }
            // Insert claims into request extensions for handlers to use
            req.extensions_mut().insert(claims);
            Ok(next.run(req).await)
        }
        Err(_) => {
            // Record failed attempt for rate limiting
            if let Some(limiter) = &rate_limiter {
                let client_id = req
                    .extensions()
                    .get::<ConnectInfo<SocketAddr>>()
                    .map(|ci| extract_client_id(Some(ci)))
                    .unwrap_or_else(|| extract_client_id(None));
                limiter.record_failed_attempt(&client_id);
            }
            Err(StatusCode::UNAUTHORIZED.into_response())
        }
    }
}

/// Authentication middleware without rate limiting (for backwards compatibility)
///
/// Use `auth_middleware` with `rate_limiter: None` for new code.
pub async fn auth_middleware_simple(
    auth_service: Arc<AuthService>,
    req: Request,
    next: Next,
) -> std::result::Result<Response, Response> {
    auth_middleware(auth_service, None, req, next).await
}

/// Claims extractor for Axum handlers
///
/// Extracts authenticated user claims from request extensions.
/// Used as an Axum extractor to get the authenticated user's claims
/// in handler functions. The middleware must run before this extractor
/// can be used.
///
/// # Handler Usage
///
/// When used with `auth_middleware`, the claims are automatically
/// injected into the request extensions and can be extracted in handlers:
///
/// ```rust
/// use mcp_context_browser::infrastructure::auth::ClaimsExtractor;
///
/// // In an Axum handler (simplified - would need auth middleware)
/// fn example_handler(claims: ClaimsExtractor) {
///     println!("User: {}", claims.sub);
///     println!("Role: {:?}", claims.role);
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ClaimsExtractor(pub Claims);

impl std::ops::Deref for ClaimsExtractor {
    type Target = Claims;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> axum::extract::FromRequestParts<S> for ClaimsExtractor
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(ClaimsExtractor)
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

/// Optional claims extractor (for endpoints that work with or without auth)
#[derive(Debug, Clone)]
pub struct OptionalClaimsExtractor(pub Option<Claims>);

impl std::ops::Deref for OptionalClaimsExtractor {
    type Target = Option<Claims>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S> axum::extract::FromRequestParts<S> for OptionalClaimsExtractor
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(OptionalClaimsExtractor(
            parts.extensions.get::<Claims>().cloned(),
        ))
    }
}

/// Permission guard for handlers
///
/// Validates that the authenticated user has a specific permission.
/// Use [`RequirePermission::check`] to verify permissions in handlers.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::infrastructure::auth::{RequirePermission, Claims, Permission, UserRole};
///
/// // Create claims for testing
/// let claims = Claims::new(
///     "user-123".to_string(),
///     "admin@example.com".to_string(),
///     UserRole::Admin,
///     "test-issuer".to_string(),
///     3600,
/// );
///
/// // Check if user has permission
/// assert!(RequirePermission::check(&claims, &Permission::ManageUsers));
/// ```
pub struct RequirePermission {
    pub permission: super::roles::Permission,
    pub claims: Claims,
}

impl RequirePermission {
    /// Check if the user has the required permission
    pub fn check(claims: &Claims, permission: &super::roles::Permission) -> bool {
        claims.role.has_permission(permission)
    }
}
