# Services Module

**Source**: `src/services/`

Orchestrates the semantic code search workflow - from codebase ingestion to search results.

## Overview

The services module contains core business logic that powers the semantic code search platform. Each service encapsulates specific capabilities that work together to deliver code intelligence.

## Services

### ContextService

Coordinates embedding generation and vector storage operations.

**Responsibilities**:

-   Generate embeddings via AI providers
-   Store and retrieve vectors
-   Handle batch processing
-   Collect performance metrics

**Related**: [providers/embedding](./providers.md), [core/types](./core.md)

### IndexingService

Processes codebases and creates searchable vector indexes.

**Responsibilities**:

-   Repository scanning and file discovery
-   Language detection and AST parsing
-   Incremental indexing with change detection
-   Chunk generation and metadata extraction

**Related**: [chunking module](../../src/chunking/), [core/types](./core.md)

### SearchService

Executes semantic similarity searches across indexed codebases.

**Responsibilities**:

-   Query processing and embedding generation
-   Vector similarity search execution
-   Result ranking and filtering
-   Response caching and optimization

**Related**: [providers/vector_store](./providers.md), [core/hybrid_search](./core.md)

## Integration Points

### AI Providers

-   OpenAI, Ollama, Gemini, VoyageAI
-   Intelligent routing with failover
-   See [providers module](./providers.md)

### Vector Storage

-   Milvus (production), InMemory (development)
-   See [providers module](./providers.md)

### MCP Protocol

-   Standardized interface with AI assistants
-   See [server module](./server.md)

## Key Exports

```rust
pub use context::ContextService;
pub use indexing::IndexingService;
pub use search::SearchService;
```

## File Structure

```text
src/services/
├── context.rs    # Embedding and vector operations
├── indexing.rs   # Codebase ingestion and processing
├── mod.rs        # Module coordination
└── search.rs     # Query processing and ranking
```

## Testing

214 tests cover services functionality. See [tests/](../../tests/).

## Cross-References

-   **Architecture**: [ARCHITECTURE.md](../architecture/ARCHITECTURE.md)
-   **Core Types**: [core.md](./core.md)
-   **Providers**: [providers.md](./providers.md)
-   **Server**: [server.md](./server.md)
