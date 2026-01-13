# Services Module

**Source**: `src/services/`

Orchestrates the semantic code search workflow - from codebase ingestion to search results.

## Overview

The services module contains core business logic that powers the semantic code search platform. Each service encapsulates specific capabilities that work together to deliver code intelligence.

## Services

### ContextService

Coordinates embedding generation and vector storage operations.

**Responsibilities**:

\1-   Generate embeddings via AI providers
\1-   Store and retrieve vectors
\1-   Handle batch processing
\1-   Collect performance metrics

**Related**: [providers/embedding](./providers.md), [core/types](./core.md)

### IndexingService

Processes codebases and creates searchable vector indexes.

**Responsibilities**:

\1-   Repository scanning and file discovery
\1-   Language detection and AST parsing
\1-   Incremental indexing with change detection
\1-   Chunk generation and metadata extraction

**Related**: [chunking module](../../src/chunking/), [core/types](./core.md)

### SearchService

Executes semantic similarity searches across indexed codebases.

**Responsibilities**:

\1-   Query processing and embedding generation
\1-   Vector similarity search execution
\1-   Result ranking and filtering
\1-   Response caching and optimization

**Related**: [providers/vector_store](./providers.md), [core/hybrid_search](./core.md)

## Integration Points

### AI Providers

\1-   OpenAI, Ollama, Gemini, VoyageAI
\1-   Intelligent routing with failover
\1-   See [providers module](./providers.md)

### Vector Storage

\1-   Milvus (production), InMemory (development)
\1-   See [providers module](./providers.md)

### MCP Protocol

\1-   Standardized interface with AI assistants
\1-   See [server module](./server.md)

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

\1-  **Architecture**: [ARCHITECTURE.md](../architecture/ARCHITECTURE.md)
\1-  **Core Types**: [core.md](./core.md)
\1-  **Providers**: [providers.md](./providers.md)
\1-  **Server**: [server.md](./server.md)
