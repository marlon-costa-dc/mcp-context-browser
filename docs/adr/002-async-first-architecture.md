# ADR 002: Async-First Architecture

## Status

**Implemented**(v0.1.0)

> Fully implemented with Tokio async runtime. 480+ async functions across the codebase.
> All provider interfaces use async traits. Structured concurrency with Tokio::spawn and channels.

## Context

The MCP Context Browser handles AI operations (embedding generation, vector searches) and large codebase processing that require high performance and concurrency. The system needs to handle multiple concurrent users, process large codebases efficiently, and integrate with external APIs that may have high latency.

Key performance requirements:

\1-   Handle 1000+ concurrent users
\1-   Process codebases with 1000+ files efficiently
\1-   Maintain sub-500ms response times for queries
\1-   Support streaming and background processing
\1-   Integrate with external APIs (OpenAI, vector databases)

Traditional synchronous programming would create bottlenecks and poor resource utilization for these I/O-bound operations.

## Decision

Adopt an async-first architecture using Tokio as the async runtime throughout the entire system. All provider interfaces use async traits, and the application is designed for high concurrency from the ground up.

Key architectural decisions:

\1-   Tokio as the primary async runtime
\1-   Async traits for all provider interfaces
\1-   Structured concurrency with Tokio::spawn
\1-   Async channels for inter-task communication
\1-   Hyper for HTTP client operations
\1-   Futures and streams for data processing pipelines

## Consequences

Async-first architecture provides excellent performance and concurrency but requires careful error handling and increases code complexity.

### Positive Consequences

\1-  **High Performance**: Efficient handling of concurrent operations and I/O
\1-  **Scalability**: Support for thousands of concurrent users
\1-  **Resource Efficiency**: Better CPU and memory utilization
\1-  **Future-Proof**: Aligns with modern async programming patterns
\1-  **Integration**: Natural fit with async HTTP clients and databases

### Negative Consequences

\1-  **Complexity**: Async code is harder to reason about and debug
\1-  **Error Handling**: Async error propagation is more complex
\1-  **Testing**: Async tests require special handling
\1-  **Learning Curve**: Steeper learning curve for team members
\1-  **Debugging**: Stack traces are less informative in async contexts

## Alternatives Considered

### Alternative 1: Synchronous Architecture

\1-  **Description**: Traditional blocking I/O with thread pools for concurrency
\1-  **Pros**: Simpler code, easier debugging, familiar patterns
\1-  **Cons**: Poor performance for I/O operations, limited concurrency
\1-  **Rejection Reason**: Cannot meet performance requirements for AI operations and concurrent users

### Alternative 2: Mixed Sync/Async

\1-  **Description**: Sync core with async wrappers for external operations
\1-  **Pros**: Gradual adoption, less complexity
\1-  **Cons**: Inconsistent patterns, performance bottlenecks at boundaries
\1-  **Rejection Reason**: Creates architectural inconsistency and performance issues

### Alternative 3: Actor Model (Actix)

\1-  **Description**: Use Actix for actor-based concurrency instead of Tokio
\1-  **Pros**: High-level abstractions, built-in supervision
\1-  **Cons**: Additional complexity, less ecosystem support
\1-  **Rejection Reason**: Tokio has better ecosystem support and performance for our use case

## Implementation Notes

### Async Runtime Configuration

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Configure Tokio runtime for optimal performance
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(num_cpus::get())
        .thread_name("mcp-worker")
        .thread_stack_size(3 * 1024 * 1024) // 3MB stack
        .enable_io()
        .enable_time()
        .build()?;

    runtime.block_on(async_main())
}

async fn async_main() -> Result<()> {
    // Application logic here
    Ok(())
}
```

### Async Trait Pattern

```rust
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn embed(&self, text: &str) -> Result<Embedding>;

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Embedding>> {
        // Default implementation using streams for concurrency
        let futures = texts.iter().map(|text| self.embed(text));
        let results = futures_util::future::join_all(futures).await;

        results.into_iter().collect()
    }

    fn dimensions(&self) -> usize;
    fn provider_name(&self) -> &str;
}
```

### Structured Concurrency

```rust
pub async fn process_codebase(&self, path: &Path) -> Result<IndexingStats> {
    // Create a task scope for structured concurrency
    let (stats_tx, stats_rx) = tokio::sync::mpsc::channel(100);

    // Spawn background tasks
    let file_processing = tokio::spawn(async move {
        self.process_files_concurrently(path, stats_tx).await
    });

    let metadata_update = tokio::spawn(async move {
        self.update_metadata_concurrently(path).await
    });

    // Wait for all tasks to complete
    let (file_result, metadata_result) = tokio::try_join!(file_processing, metadata_update)?;

    file_result?;
    metadata_result?;

    // Collect final statistics
    let mut total_stats = IndexingStats::default();
    while let Some(stats) = stats_rx.recv().await {
        total_stats.merge(stats);
    }

    Ok(total_stats)
}
```

### Error Handling in Async Code

```rust
pub async fn handle_request(&self, request: Request) -> Result<Response> {
    // Use timeout for external operations
    let result = tokio::time::timeout(
        Duration::from_secs(30),
        self.process_request(request)
    ).await
    .map_err(|_| Error::timeout("Request processing timed out"))??;

    Ok(result)
}

async fn process_request(&self, request: Request) -> Result<Response> {
    // Handle cancellation gracefully
    let mut operation = self.start_operation(request);

    tokio::select! {
        result = operation.wait() => {
            result
        }
        _ = tokio::signal::ctrl_c() => {
            operation.cancel().await?;
            Err(Error::cancelled("Operation was cancelled"))
        }
    }
}
```

### Testing Async Code

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;

    #[tokio::test]
    async fn test_embedding_provider() {
        let provider = MockEmbeddingProvider::new();

        // Test async operation
        let embedding = provider.embed("test text").await.unwrap();
        assert_eq!(embedding.dimensions, 128);
    }

    #[tokio::test]
    async fn test_concurrent_operations() {
        let provider = Arc::new(MockEmbeddingProvider::new());

        // Test concurrent embedding
        let texts = vec!["text1".to_string(), "text2".to_string(), "text3".to_string()];
        let embeddings = provider.embed_batch(&texts).await.unwrap();

        assert_eq!(embeddings.len(), 3);
        assert!(embeddings.iter().all(|e| e.dimensions == 128));
    }
}
```

## Update for v0.2.0: Hybrid Parallelization with Rayon

**Date**: 2026-01-14

As MCB evolves to include CPU-intensive code analysis features (v0.3.0+), the async-first design has been extended to support hybrid parallelization:

### Updated Strategy

- **Tokio**: I/O-bound operations (file reads, network calls, database queries, vector search)
- **Rayon**: CPU-bound operations (AST parsing, complexity calculation, graph analysis)
- **Pattern**: Wrap Rayon in `tokio::task::spawn_blocking` to bridge sync CPU work with async I/O

### Rationale

1. **Tokio for I/O**: Tokio's event-driven architecture is optimal for I/O-bound work
2. **Rayon for Compute**: Rayon's work-stealing scheduler is proven for CPU-bound parallelism
3. **PMAT Integration**: Upcoming PMAT analysis code uses Rayon extensively with proven performance
4. **No Conflicts**: Tokio and Rayon are complementary and don't interfere with each other

### Implementation Pattern

```rust
#[async_trait]
pub trait CodeAnalyzer: Send + Sync {
    async fn analyze_complexity(&self, path: &Path) -> Result<ComplexityReport> {
        // 1. Read file (I/O - Tokio)
        let content = tokio::fs::read_to_string(path).await?;

        // 2. Compute complexity (CPU - Rayon, wrapped in spawn_blocking)
        let report = tokio::task::spawn_blocking(move || {
            // Rayon parallelism for AST analysis
            self.compute_complexity(&content)
        }).await??;

        Ok(report)
    }
}

fn compute_complexity(content: &str) -> Result<ComplexityReport> {
    // Rayon for parallel AST node processing
    let nodes = parse_ast(content)?;

    let metrics: Vec<_> = nodes
        .par_iter()  // Rayon's parallel iterator
        .map(|node| calculate_node_complexity(node))
        .collect();

    Ok(ComplexityReport { metrics })
}
```

### Benefits

- ✅ Tokio remains the primary runtime for all async coordination
- ✅ Rayon's work-stealing keeps CPU cores busy during analysis
- ✅ No context switching between runtimes
- ✅ Straightforward to test and reason about
- ✅ Maintains clean async/sync boundaries

### Performance Implications

- **I/O Operations**: Unchanged (Tokio handles efficiently)
- **CPU Operations**: Improved parallelism (Rayon fully utilizes CPU cores)
- **Context Switching**: Minimal (spawn_blocking reuses Tokio's worker threads)
- **Memory**: Slight increase for Rayon work-stealing queues (negligible)

## References

\1-   [Tokio Documentation](https://tokio.rs/)
\1-   [Async Programming in Rust](https://rust-lang.github.io/async-book/)
\1-   [Structured Concurrency](https://vorpus.org/blog/notes-on-structured-concurrency-or-go-statement-considered-harmful/)
\1-   [Rayon: Data Parallelism](https://docs.rs/rayon/latest/rayon/)
\1-   [Tokio spawn_blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)
