# Admin Module

**Source**: `src/admin/`

Web-based administration and monitoring interface.

## Overview

The admin module provides a comprehensive administration platform for MCP Context Browser. It includes configuration management, real-time monitoring, and log investigation capabilities.

## Components

### AdminService (`service.rs`)

Core administration business logic.

**Capabilities**:

-   Configuration management (read/update)
-   Performance metrics retrieval
-   Log access and filtering
-   Cache operations (clear, stats)
-   Backup management

### Admin Router (`routes.rs`)

HTTP routes for administration API.

**Endpoints**:

| Endpoint | Method | Purpose |
|----------|--------|---------|
| `/admin/config` | GET/POST | Configuration management |
| `/admin/metrics` | GET | Performance metrics |
| `/admin/logs` | GET | Log retrieval |
| `/admin/cache/clear` | POST | Clear cache |
| `/admin/backup` | POST | Create backup |

### Authentication (`auth.rs`)

Admin-specific authentication.

-   Admin role verification
-   Session management
-   Audit logging

### Web Interface (`web.rs`)

HTML dashboard for browser access.

-   Real-time metrics display
-   Configuration editor
-   Log viewer with filters

## File Structure

```text
src/admin/
├── api.rs       # REST API handlers
├── auth.rs      # Admin authentication
├── handlers.rs  # Request handlers
├── mod.rs       # Module exports
├── models.rs    # Data models
├── routes.rs    # Route definitions
├── service.rs   # Business logic
└── web.rs       # Web dashboard
```

## Key Exports

```rust
pub use routes::create_admin_router;
pub use service::AdminService;
```

## Configuration

Default admin port: 3002

Environment variables:

-   `MCP_ADMIN_ENABLED=true` - Enable admin interface
-   `MCP_ADMIN_PORT=3002` - Admin API port

## Cross-References

-   **Architecture**: [ARCHITECTURE.md](../architecture/ARCHITECTURE.md)
-   **Metrics**: [metrics.md](./metrics.md) (metrics source)
-   **Server**: [server.md](./server.md) (main server)
-   **Core**: [core.md](./core.md) (auth, logging)
