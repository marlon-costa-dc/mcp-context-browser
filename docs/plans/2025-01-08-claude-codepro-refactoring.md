# Claude CodePro Refactoring Plan

> **IMPORTANT:** Start with fresh context. Run `/clear` before `/implement`.

Created: 2025-01-08
Updated: 2026-01-08
Status: VERIFIED

> **Status Lifecycle:** PENDING → COMPLETE → VERIFIED
>
> -   PENDING: Initial state, awaiting implementation
> -   COMPLETE: All tasks implemented (set by /implement)
> -   VERIFIED: Rules supervisor passed (set automatically)

## Summary

**Goal:** Elevate the codebase to "Pro" production standards by removing all mocks, fixing disconnected wiring, optimizing performance, and enforcing **Dependency Injection (DI)** and **Clean Architecture**.

**Core Philosophy:**

-   **Minimal Changes:** Evolve existing structures; do not rewrite unless necessary.
-   **Continuous Quality:** Fix linter errors and update tests *immediately* after every change.
-   **Real Verification:** No mocks in tests; verify actual behavior (file creation, event propagation) before moving to the next task.
-   **Lean Code:** Remove dead code immediately; do not leave "TODOs" or commented-out blocks.

**Architecture:**

-   **Event Bus:** Decouple Admin Service from core logic using `tokio::sync::broadcast` (Standard, robust, async-first).
-   **Composition Root:** Use a **Builder Pattern** (`McpServerBuilder`) as the single Composition Root to assemble the application graph, ensuring pure Constructor Injection.
-   **Shared State:** Unify metrics and observability state using `Arc` (Reference Counting) for safe concurrent access.
-   **In-Memory Logging:** Custom `tracing-subscriber::Layer` for zero-allocation circular buffer logging.
-   **Backups:** Use `tar` and `flate2` crates for standard, portable stream-based compression.

**Tech Stack:**

-   **Core:** `tokio` (Async Runtime), `dashmap` (Concurrent Maps)
-   **Web:** `axum` (HTTP API)
-   **Observability:** `tracing`, `tracing-subscriber`
-   **Compression:** `tar`, `flate2` (Standard GZIP/Tar)
-   **DI Strategy:** Pure Constructor Injection (No DI frameworks to keep it lean/native).

## Scope

### In Scope

-   **Admin Service:** Replace all mocks with real implementations via Event Bus.
-   **Observability:** Implement in-memory log capture and shared metrics.
-   **Refactoring:** `McpServer` cleanup, `VoyageAI` optimization.
-   **Backups:** Implement real file-system backup/restore logic.
-   **Wiring:** Fix disconnected `MetricsApiServer`.

### Out of Scope

-   New user-facing features (strictly code quality/architecture focus).
-   UI changes (HTML/CSS remains as is, only data sources change).
-   Database migration (we stick to the current storage strategy).

## Prerequisites

-   `tokio` (already in dependencies)
-   `tracing-subscriber` (already in dependencies)
-   **Add:** `tar` and `flate2` to `Cargo.toml`.

## Context for Implementer

-   **Mocks to Kill:** `src/admin/service.rs` is currently 90% hardcoded data.
-   **Wiring Bug:** `init.rs` creates one `PerformanceMetrics`, but `McpServer` creates another. They are disconnected.
-   **Event Bus:** Use `tokio::sync::broadcast` for the internal event bus. Events should include `CacheClear`, `IndexRebuild`, `BackupCreate`.
-   **Clean Architecture:** Use "Ports and Adapters". `AdminService` is a Port (Trait). `AdminServiceImpl` is an Adapter. Use strict constructor injection.

## Feature Inventory (Refactoring)

### Files Being Refactored/Replaced

| Old File | Functions/Classes | Mapped to Task |
|----------|-------------------|----------------|
| `src/admin/service.rs` | `AdminServiceImpl`, `get_logs`, `clear_cache` (mocks) | Task 3, Task 4 |
| `src/server/server.rs` | `McpServer::new`, `initialize_services` | Task 2 |
| `src/server/init.rs` | `initialize_server_components` | Task 2 |
| `src/metrics/http_server.rs` | `MetricsApiServer::new` (disconnected metrics) | Task 2 |
| `src/providers/embedding/voyageai.rs` | `embed_batch` (new client per request) | Task 5 |

### Feature Mapping Verification

-   [x] All old files listed above
-   [x] All functions/classes identified
-   [x] Every feature has a task number
-   [x] No features accidentally omitted

## Progress Tracking

**MANDATORY: Update this checklist as tasks complete. Change `[ ]` to `[x]`.**

-   [x] Task 1: Core Infrastructure (Event Bus & Logging)
-   [x] Task 2: Server Refactoring & Wiring (Builder Pattern)
-   [x] Task 3: Admin Service - Read Operations (Real Data)
-   [x] Task 4: Admin Service - Write Operations (Event Bus)
-   [x] Task 5: Provider Optimization & Cleanup (VoyageAI + Final Polish)

**Total Tasks:** 5 | **Completed:** 5 | **Remaining:** 0

## Implementation Tasks

### Task 1: Core Infrastructure (Event Bus & Logging)

**Objective:** Create the foundational systems for decoupling (Event Bus) and observability (Ring Buffer Logger) with minimal footprint.

**Files:**

-   Create: `src/core/events.rs` (Event definitions and Bus implementation)
-   Create: `src/core/logging.rs` (In-memory ring buffer tracing layer)
-   Modify: `src/core/mod.rs` (Export new modules)
-   Test: `tests/core/events_test.rs`

**Implementation Steps:**

1.  **Event Bus:** Define `SystemEvent` enum (CacheClear, Backup, Rebuild). Implement `EventBus` struct using `tokio::sync::broadcast`.
2.  **Ring Buffer:** Implement `RingBufferLayer` for `tracing_subscriber` that stores `LogEntry` structs in a `VecDeque` with a Mutex.
3.  **TDD:** Write tests to verify events can be published/subscribed and logs are captured/rotated.
4.  **Verification:** Run `make test` and `make lint`. Ensure 0 warnings.

**Definition of Done:**

-   [x] `SystemEvent` and `EventBus` implemented.
-   [x] `RingBufferLayer` capturing logs.
-   [x] All new tests pass.
-   [x] No clippy warnings.

### Task 2: Server Refactoring & Wiring (Builder Pattern)

**Objective:** Clean up `McpServer` initialization using the Builder pattern and fix the disconnected metrics bug. *Change logic without rewriting the entire file.*

**Files:**

-   Create: `src/server/builder.rs` (New `McpServerBuilder`)
-   Modify: `src/server/server.rs` (Simplify `McpServer`, remove complex init logic)
-   Modify: `src/server/init.rs` (Use Builder, fix Metrics sharing)
-   Modify: `src/metrics/http_server.rs` (Accept external `PerformanceMetrics`)

**Implementation Steps:**

1.  **Builder:** Create `McpServerBuilder` that accumulates config, metrics, and event bus.
2.  **Wiring:** In `init.rs`, create `PerformanceMetrics` *once* and pass it to both the Builder and `MetricsApiServer`.
3.  **Injection:** Ensure `EventBus` and `RingBufferLogger` are created in `init.rs` and passed to the server/admin.
4.  **Verification:** Run the server locally (`make dev`). Check `/api/health` and ensuring metrics match `McpServer` activity.

**Definition of Done:**

-   [x] `McpServer` uses Constructor Injection via Builder.
-   [x] Metrics are shared (confirmed via manual or integration test).
-   [x] `make test` passes.
-   [x] No disconnected components.

### Task 3: Admin Service - Read Operations (Real Data)

**Objective:** Replace "Read" mocks in Admin Service with real system data (Logs, Metrics, Config).

**Files:**

-   Modify: `src/admin/service.rs`
-   Modify: `src/server/server.rs` (Expose necessary read-only state like config)

**Implementation Steps:**

1.  **Logs:** Update `get_logs` to read from the `RingBufferLogger` injected in Task 2.
2.  **Metrics:** Update `get_performance_metrics` to read from the real `PerformanceMetrics` Arc.
3.  **Config:** Update `get_configuration` to return the actual loaded config.
4.  **Verification:** Verify Admin API returns *real* logs and *real* metrics (not static numbers).

**Definition of Done:**

-   [x] `get_logs` returns actual runtime logs.
-   [x] `get_performance_metrics` matches system state.
-   [x] `get_configuration` returns loaded config.
-   [x] No mocks remain in these methods.

### Task 4: Admin Service - Write Operations (Event Bus)

**Objective:** Replace "Write" mocks (Clear Cache, Backup) by publishing events to the Event Bus.

**Files:**

-   Modify: `src/admin/service.rs` (Publish events instead of mocking success)
-   Modify: `src/core/cache.rs` (Subscribe to CacheClear events)
-   Modify: `src/services/indexing.rs` (Subscribe to Rebuild events)
-   Create: `src/core/backup.rs` (Real backup implementation listening to events)

**Implementation Steps:**

1.  **Publishing:** In `AdminServiceImpl`, replace `clear_cache` logic with `event_bus.publish(SystemEvent::ClearCache)`.
2.  **Subscribing:** In `CacheManager` and other services, spawn a background task to listen for relevant events and execute the actual logic.
3.  **Backups:** Implement `BackupManager` that listens for `CreateBackup` events and runs `tar` + `flate2` on the index directory.
4.  **Verification:** Trigger a backup via Admin API. Verify `.tar.gz` file exists on disk.

**Definition of Done:**

-   [x] `clear_cache` clears the actual cache.
-   [x] `create_backup` creates a real file.
-   [x] Tests verify event propagation.
-   [x] Zero mocks in `AdminServiceImpl`.

### Task 5: Provider Optimization & Cleanup

**Objective:** Optimize VoyageAI provider and perform final codebase cleanup (remove unused imports, comments).

**Files:**

-   Modify: `src/providers/embedding/voyageai.rs`
-   Global: Scan for `todo!()`, `unimplemented!()`, or mock comments.

**Implementation Steps:**

1.  **VoyageAI:** Inject `HttpClientPool` into `VoyageAIEmbeddingProvider` (similar to Gemini/OpenAI).
2.  **Cleanup:** Grep for "mock", "stub", "fake" and verify they are gone or justified.
3.  **Verification:** Run full test suite to ensure no regressions. Run `make lint` to ensure no unused imports.

**Definition of Done:**

-   [x] VoyageAI uses shared HTTP client.
-   [x] No "mock" or "fake" comments/code found.
-   [x] All tests pass.
-   [x] `make lint` is clean.

## Testing Strategy

-   **Unit tests:** Verify Event Bus routing and Logger rotation.
-   **Integration tests:** Trigger a backup via Admin Service and verify file creation. Verify logs appear in Admin API after activity.
-   **Gatekeeper:** Every task must pass `make test` and `make lint` before proceeding.

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Circular dependencies with Event Bus | Medium | High | Use weak references where needed, keep Event Bus structurally simple. |
| Locking contention in Logger | Low | Low | Use `RwLock` or lock-free structures if performance suffers (unlikely for dashboard logs). |
| Metric drift | Low | Medium | Single source of truth (Arc) strategy mitigates this completely. |

## Open Questions

-   None. Design decisions confirmed by user.

---
**USER: Please review this plan. Edit any section directly, then confirm to proceed.**
