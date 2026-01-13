# domain Module

**Source**: `src/domain/`
**Files**: 13 (9 port files + 4 core files)
**Lines of Code**: ~2,500
**Traits**: 14
**Structs**: 20+
**Enums**: 6

## Overview

The domain module defines the core business entities and port interfaces following Clean Architecture principles. All domain logic is technology-agnostic, with external concerns abstracted behind port traits.

## Key Exports

### Port Traits (14 total)

All traits extend `shaku::Interface` for DI compatibility:

**Provider Ports:**
- `EmbeddingProvider` - Text-to-vector conversion
- `VectorStoreProvider` - Vector storage and retrieval
- `HybridSearchProvider` - Combined BM25 + semantic search
- `CodeChunker` - AST-based code chunking

**Infrastructure Ports:**
- `SyncProvider` - Low-level sync operations
- `SnapshotProvider` - Codebase snapshot management
- `SyncCoordinator` - File synchronization with debouncing
- `EventPublisher` - Domain event publishing

**Repository Ports:**
- `ChunkRepository` - Code chunk persistence
- `SearchRepository` - Search operations

**Service Ports:**
- `ContextServiceInterface` - High-level code intelligence
- `SearchServiceInterface` - Semantic search
- `IndexingServiceInterface` - Codebase indexing
- `ChunkingOrchestratorInterface` - Batch chunking coordination

### Core Types
- `CodeChunk` - Semantic code unit
- `Embedding` - Vector representation
- `SearchResult` - Search result with score
- `Language` - Programming language enum

### Events
- `DomainEvent` - Domain-level events (IndexRebuild, SyncCompleted, etc.)

## File Structure

```text
ports/
├── chunking.rs         # CodeChunker trait (326 lines)
├── embedding.rs        # EmbeddingProvider trait
├── events.rs           # EventPublisher trait, DomainEvent
├── hybrid_search.rs    # HybridSearchProvider trait
├── infrastructure.rs   # SyncProvider, SnapshotProvider traits
├── repository.rs       # ChunkRepository, SearchRepository traits
├── services.rs         # Application service interfaces
├── sync.rs             # SyncCoordinator trait
└── vector_store.rs     # VectorStoreProvider trait

error.rs                # Domain error types
types.rs                # Core domain types (CodeChunk, Embedding, etc.)
validation.rs           # Input validation rules
ports.rs                # Module re-exports
```

## Port/Adapter Mappings

| Port | Implementation | Location |
|------|---------------|----------|
| `EmbeddingProvider` | OpenAI, VoyageAI, Ollama, Gemini, FastEmbed, Null | `src/adapters/providers/embedding/` |
| `VectorStoreProvider` | Milvus, EdgeVec, In-Memory, Filesystem, Null | `src/adapters/providers/vector_store/` |
| `HybridSearchProvider` | HybridSearchAdapter | `src/adapters/hybrid_search/` |
| `CodeChunker` | IntelligentChunker | `src/domain/chunking/engine.rs` |
| `EventPublisher` | EventBus | `src/infrastructure/events/tokio_impl.rs` |
| `SyncCoordinator` | SyncManager | `src/infrastructure/sync/manager.rs` |
| `SnapshotProvider` | SnapshotManager | `src/infrastructure/snapshot/manager.rs` |
| `ChunkRepository` | ChunkRepositoryImpl | `src/adapters/repository/` |
| `SearchRepository` | SearchRepositoryImpl | `src/adapters/repository/` |
| `ContextServiceInterface` | ContextService | `src/application/context.rs` |
| `SearchServiceInterface` | SearchService | `src/application/search.rs` |
| `IndexingServiceInterface` | IndexingService | `src/application/indexing/service.rs` |
| `ChunkingOrchestratorInterface` | ChunkingOrchestrator | `src/application/indexing/chunking_orchestrator.rs` |

---

*Updated 2026-01-13 - Reflects full port/adapter wiring*
