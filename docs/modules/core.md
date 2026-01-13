# Core Module

**Source**: `src/core/`

Foundational types, traits, and utilities used throughout the system.

## Overview

The core module establishes fundamental domain types and shared utilities that form the foundation of all operations. It defines essential types, error handling, and infrastructure utilities.

## Submodules

### Types (`types.rs`)

Core data structures for code intelligence.

\1-   `Embedding` - Vector representation of text/code
\1-   `CodeChunk` - Parsed code segment with metadata
\1-   `SearchResult` - Ranked search item with score
\1-   `Language` - Supported programming languages

### Error Handling (`error.rs`)

Comprehensive error types with `thiserror`.

\1-   `Error` - Main error enum with variants
\1-   `Result<T>` - Type alias for `Result<T, Error>`

### Authentication (`auth.rs`)

JWT-based identity and access management.

\1-   `AuthService` - Token validation and generation
\1-   `Claims` - JWT payload structure
\1-   `Permission` - Authorization controls

### Caching (`cache.rs`)

Multi-level caching with TTL and size limits.

\1-   `CacheManager` - Main cache interface
\1-   Configurable TTL and eviction policies

### Rate Limiting (`rate_limit.rs`)

Request throttling with multiple strategies.

\1-   `RateLimiter` - Token bucket implementation
\1-   Configurable limits per endpoint/user

### Hybrid Search (`hybrid_search.rs`)

Combined BM25 + semantic search.

\1-   `HybridSearchEngine` - Orchestrates dual ranking
\1-   `BM25Scorer` - Term frequency ranking
\1-   Configurable weighting between methods

### Other Utilities

\1-   `crypto.rs` - Encryption utilities (AES-GCM)
\1-   `database.rs` - Connection pooling
\1-   `http_client.rs` - HTTP client with retry
\1-   `limits.rs` - Resource quotas
\1-   `merkle.rs` - Data integrity verification

## Key Exports

```rust
// Domain types
pub use types::{Embedding, CodeChunk, SearchResult, Language};
pub use error::{Error, Result};

// Security
pub use auth::{AuthService, Permission, Claims};
pub use crypto::*;

// Infrastructure
pub use cache::CacheManager;
pub use rate_limit::RateLimiter;
pub use hybrid_search::HybridSearchEngine;
```

## File Structure

```text
src/core/
├── auth.rs          # JWT authentication
├── cache.rs         # Multi-level caching
├── crypto.rs        # Encryption utilities
├── database.rs      # Database connectivity
├── error.rs         # Error types
├── http_client.rs   # HTTP client
├── hybrid_search.rs # BM25 + semantic search
├── limits.rs        # Resource quotas
├── merkle.rs        # Data integrity
├── mod.rs           # Module exports
├── rate_limit.rs    # Request throttling
└── types.rs         # Domain types
```

## Testing

Core types have 18 dedicated tests. See [tests/core_types.rs](../../tests/core_types.rs).

## Cross-References

\1-  **Architecture**: [ARCHITECTURE.md](../architecture/ARCHITECTURE.md)
\1-  **Services**: [services.md](./services.md) (uses core types)
\1-  **Providers**: [providers.md](./providers.md) (implements traits)
\1-  **Server**: [server.md](./server.md) (uses auth/rate limiting)
