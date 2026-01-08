# Server Module

**Source**: `src/server/`

MCP protocol server and HTTP API endpoints for AI assistant integration.

## Overview

The server module implements the interface between AI assistants (like Claude Desktop) and the semantic code search platform. It handles MCP protocol communication, request routing, and response formatting.

## Components

### McpServer

Main server coordinator handling MCP protocol messages.

**Responsibilities**:

-   Protocol message parsing and routing
-   Tool discovery and capability advertisement
-   Request/response lifecycle management
-   Error handling and graceful degradation

### Tool Handlers

Business logic for MCP tools.

| Handler | Tool | Purpose |
|---------|------|---------|
| `index_codebase` | Codebase ingestion | Process and index source files |
| `search_code` | Semantic search | Natural language code queries |
| `get_indexing_status` | Status monitoring | Check indexing progress |
| `clear_index` | Index management | Reset indexed data |

### Authentication (`auth.rs`)

JWT-based security for API access.

-   Token validation and generation
-   Role-based permissions
-   Request authentication middleware

### Rate Limiting (`rate_limit_middleware.rs`)

Request throttling to prevent abuse.

-   Token bucket implementation
-   Per-user/per-endpoint limits
-   Configurable thresholds

## MCP Protocol

Implements [Model Context Protocol](https://modelcontextprotocol.io/) specification.

**Transport**: stdio (standard input/output)

**Message Types**:

-   `initialize` - Capability negotiation
-   `tools/list` - Tool discovery
-   `tools/call` - Tool execution
-   `shutdown` - Graceful termination

## File Structure

```text
src/server/
├── args.rs              # Request argument types
├── auth.rs              # Authentication logic
├── builder.rs           # Server builder pattern
├── formatter.rs         # Response formatting
├── handlers/
│   ├── clear_index.rs
│   ├── get_indexing_status.rs
│   ├── index_codebase.rs
│   ├── mod.rs
│   └── search_code.rs
├── init.rs              # Server initialization
├── mod.rs               # Module exports
├── rate_limit_middleware.rs
├── security.rs          # Security controls
└── server.rs            # Main server implementation
```

## Key Exports

```rust
pub use server::McpServer;
pub use auth::AuthHandler;
pub use handlers::*;
pub use args::{IndexCodebaseArgs, SearchCodeArgs};
```

## Testing

15 MCP protocol tests plus 13 integration tests. See [tests/](../../tests/).

## Cross-References

-   **Architecture**: [ARCHITECTURE.md](../architecture/ARCHITECTURE.md)
-   **Services**: [services.md](./services.md) (business logic)
-   **Core**: [core.md](./core.md) (auth, rate limiting)
-   **Providers**: [providers.md](./providers.md) (AI providers)
