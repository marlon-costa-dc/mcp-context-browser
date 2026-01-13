# Module Structure

This document shows the hierarchical structure of modules in the MCP Context Browser.

## Module Tree (Clean Architecture)

```
mcp-context-browser/src/
├── main.rs (entry point)
├── lib.rs (library exports)
├── domain/ (core business logic)
│   ├── ports/ (14 port traits)
│   │   ├── embedding.rs
│   │   ├── vector_store.rs
│   │   ├── hybrid_search.rs
│   │   ├── chunking.rs
│   │   ├── repository.rs
│   │   ├── services.rs
│   │   ├── sync.rs
│   │   ├── events.rs
│   │   └── infrastructure.rs
│   ├── chunking/ (12 language processors)
│   │   └── languages/
│   ├── types.rs
│   ├── error.rs
│   └── validation.rs
├── application/ (business services)
│   ├── context.rs
│   ├── search.rs
│   └── indexing/
│       ├── service.rs
│       └── chunking_orchestrator.rs
├── adapters/ (external integrations)
│   ├── providers/
│   │   ├── embedding/ (6 providers)
│   │   ├── vector_store/ (6 providers)
│   │   └── routing/ (circuit breaker, health, failover)
│   ├── hybrid_search/
│   └── repository/
├── infrastructure/ (shared systems)
│   ├── di/ (Shaku dependency injection)
│   ├── auth/ (JWT, rate limiting)
│   ├── config/
│   ├── cache.rs
│   ├── events/ (Tokio, NATS)
│   ├── sync/ (file synchronization)
│   ├── snapshot/ (change tracking)
│   ├── daemon/
│   └── metrics/
└── server/ (MCP protocol)
    ├── mcp_server.rs
    ├── handlers/
    └── admin/
```

## Architecture Layers

| Layer | Purpose | Key Components |
|-------|---------|----------------|
| **Domain** | Business entities and rules | Ports, types, validation |
| **Application** | Use case orchestration | ContextService, IndexingService, SearchService |
| **Adapters** | External service integration | Embedding providers, vector stores, repositories |
| **Infrastructure** | Technical services | DI, auth, cache, events, sync |
| **Server** | Protocol implementation | MCP handlers, admin API |

*Updated: 2026-01-13 - Reflects Clean Architecture refactoring*
