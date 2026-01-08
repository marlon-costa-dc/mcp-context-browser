# Core Module

**Source**: `src/core/`

Foundational types, traits, and utilities used throughout the system.

## Overview

The core module establishes fundamental domain types and shared utilities that form the foundation of all operations. It defines essential types, error handling, and infrastructure utilities.

## Submodules

### Types (`types.rs`)

Core data structures for code intelligence.

-   `Embedding` - Vector representation of text/code
-   `CodeChunk` - Parsed code segment with metadata
-   `SearchResult` - Ranked search item with score
-   `Language` - Supported programming languages

### Error Handling (`error.rs`)

Comprehensive error types with `thiserror`.

-   `Error` - Main error enum with variants
-   `Result<T>` - Type alias for `Result<T, Error>`

### Authentication (`auth.rs`)

JWT-based identity and access management.

-   `AuthService` - Token validation and generation
-   `Claims` - JWT payload structure
-   `Permission` - Authorization controls

### Caching (`cache.rs`)

Multi-level caching with TTL and size limits.

-   `CacheManager` - Main cache interface
-   Configurable TTL and eviction policies

### Rate Limiting (`rate_limit.rs`)

Request throttling with multiple strategies.

-   `RateLimiter` - Token bucket implementation
-   Configurable limits per endpoint/user

### Hybrid Search (`hybrid_search.rs`)

Combined BM25 + semantic search.

-   `HybridSearchEngine` - Orchestrates dual ranking
-   `BM25Scorer` - Term frequency ranking
-   Configurable weighting between methods

### Other Utilities

-   `crypto.rs` - Encryption utilities (AES-GCM)
-   `database.rs` - Connection pooling
-   `http_client.rs` - HTTP client with retry
-   `limits.rs` - Resource quotas
-   `merkle.rs` - Data integrity verification

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

-   **Architecture**: [ARCHITECTURE.md](../architecture/ARCHITECTURE.md)
-   **Services**: [services.md](./services.md) (uses core types)
-   **Providers**: [providers.md](./providers.md) (implements traits)
-   **Server**: [server.md](./server.md) (uses auth/rate limiting)
