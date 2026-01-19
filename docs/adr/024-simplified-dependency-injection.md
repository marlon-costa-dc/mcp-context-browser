# ADR 024: Shaku to dill DI Migration

## Status

**Implemented** (v0.1.2)

> Replacement for [ADR 002: Dependency Injection with Shaku](002-dependency-injection-shaku.md) using the dill runtime DI framework.
>
> **Implementation Note (2026-01-18)**: The `#[component]` macro approach is incompatible with our domain error types (`mcb_domain::Error` vs dill's `InjectionError`). We use dill's `add_value` pattern instead, which achieves the same result without requiring the component macro.

## Context

The current dependency injection system uses Shaku (version 0.6), a compile-time DI container that provides trait-based dependency resolution. While effective, this approach introduces substantial complexity that impacts development velocity and maintainability.

### Current Shaku Implementation

The current system uses a two-layer DI approach (ADR 012) with:

1.  **Shaku Components**: Services registered with `#[derive(Component)]` and `#[shaku(interface = dyn Trait)]`
2.  **Runtime Factories**: Production providers created via factory functions outside Shaku
3.  **Module Composition**: Services organized in Shaku modules with macro-generated wiring

**Example of current complexity:**

```rust
// Component definition with multiple attributes
#[derive(Component)]
#[shaku(interface = dyn EmbeddingProvider)]
pub struct OllamaEmbeddingProvider {
    #[shaku(inject)]  // Runtime injection point
    config: Arc<dyn ConfigProvider>,
    #[shaku(inject)]
    http_client: Arc<dyn HttpClient>,
}

// Module definition with macro
module! {
    pub EmbeddingModuleImpl: EmbeddingModule {
        components = [OllamaEmbeddingProvider],
        providers = []
    }
}

// Runtime resolution
let provider: Arc<dyn EmbeddingProvider> = container.resolve();
```

### Problems with Current Approach

#### Developer Experience Issues

1.  **Macro complexity**: `#[derive(Component)]`, `#[shaku(interface = ...)]`, `#[shaku(inject)]` everywhere
2.  **Build time impact**: Extensive macro expansion slows compilation
3.  **Learning curve**: Shaku API is complex for new team members
4.  **Debugging difficulty**: DI resolution happens through macro-generated code

#### Maintenance Issues

1.  **Module sync**: Manual maintenance of module definitions as services change
2.  **Trait bounds**: Complex trait bounds on component implementations
3.  **Testing overhead**: Need to understand Shaku to write unit tests
4.  **Refactoring friction**: Changes require updating multiple macro annotations

#### Architectural Issues

1.  **Over-engineering**: DI container complexity exceeds project needs
2.  **Runtime opacity**: Service resolution happens through generated code
3.  **Limited flexibility**: Hard to customize service creation per environment

### DI Library Research

We evaluated modern Rust DI alternatives:

| Library | Type | Cross-Crate | Async | Verdict |
|---------|------|-------------|-------|---------|
| **Shaku** (current) | Compile-time | Yes | No | High boilerplate |
| **nject** | Compile-time | **NO** | No | Rejected (cross-crate limitation) |
| **dill** | Runtime | Yes | Tokio | **SELECTED** |
| **dependency_injector** | Runtime | Yes | Optional | Viable alternative |
| Manual injection | N/A | N/A | N/A | Step backwards |

**Critical Requirement**: Cross-crate compatibility is essential for the 8-crate workspace architecture.

### Why dill

The [dill](https://github.com/sergiimk/dill-rs) crate (version 0.15.0, as of January 2026) provides runtime DI designed specifically for Clean/Onion Architecture:

**Key Benefits:**

1.  **Clean Architecture alignment**: Explicitly designed for Onion Architecture patterns
2.  **Native `Arc<dyn Trait>`**: First-class support for trait-based DI
3.  **Cross-crate compatible**: Works seamlessly across all 8 workspace crates
4.  **Tokio-compatible**: Task-local scoping for async contexts
5.  **Production-proven**: Used in [kamu-cli](https://github.com/kamu-data/kamu-cli), a large-scale Rust project with similar architecture
6.  **Lower boilerplate**: Simple `#[component]` + `#[interface]` attributes

**dill Injection Specifications:**

-   `OneOf<T>` - Single implementation
-   `AllOf<T>` - Collection of all implementations
-   `Maybe<T>` - Optional dependency (returns None if missing)
-   `Lazy<T>` - Deferred resolution

**dill Scopes:**

-   `Transient` - New instance per call
-   `Singleton` - Reused after first creation
-   `Transaction` - Cached during transaction

## Decision

We will replace Shaku-based DI with dill runtime DI:

1.  **Replace Shaku with dill** across all crates
2.  **Use Catalog pattern** for service registration and resolution
3.  **Maintain trait-based abstraction** for testability
4.  **Keep dependency inversion** through trait objects (`Arc<dyn Trait>`)

### Implementation Pattern

**Before (Shaku - verbose macros):**

```rust
#[derive(Component)]
#[shaku(interface = dyn MyService)]
pub struct MyServiceImpl {
    #[shaku(inject)]
    dependency: Arc<dyn OtherService>,
}

module! {
    pub MyModule: MyModuleTrait {
        components = [MyServiceImpl],
        providers = []
    }
}

// Resolution
let service: Arc<dyn MyService> = container.resolve();
```

**After (dill - add_value pattern):**

```rust
use dill::{Catalog, CatalogBuilder};

// Note: #[component] macro is incompatible with our domain error types.
// We use add_value to register pre-instantiated Arc<dyn Trait> values.

pub struct MyServiceImpl;

impl MyService for MyServiceImpl {
    // ... trait implementation
}

// Catalog composition
let catalog = CatalogBuilder::new()
    .add_value(Arc::new(MyServiceImpl) as Arc<dyn MyService>)
    .add_value(Arc::new(OtherServiceImpl) as Arc<dyn OtherService>)
    .build();

// Resolution
let service: Arc<dyn MyService> = catalog.get_one().unwrap();
```

> **Why add_value instead of #[component]?** The dill `#[component]` macro generates code that uses `InjectionError` for error handling and creates a conflicting `new()` method. Our services use `mcb_domain::Error` and have existing constructors. The `add_value` pattern is simpler and compatible with our architecture.

### Bootstrap Pattern (Implemented)

Service composition is handled in `mcb-infrastructure/src/di/bootstrap.rs`:

```rust
use dill::{Catalog, CatalogBuilder};

/// Build the infrastructure Catalog with all services registered
fn build_infrastructure_catalog() -> Catalog {
    CatalogBuilder::new()
        // Infrastructure services
        .add_value(Arc::new(NullAuthService::new()) as Arc<dyn AuthServiceInterface>)
        .add_value(Arc::new(TokioBroadcastEventBus::new()) as Arc<dyn EventBusProvider>)
        .add_value(Arc::new(NullSystemMetricsCollector::new()) as Arc<dyn SystemMetricsCollectorInterface>)
        .add_value(Arc::new(NullSyncProvider::new()) as Arc<dyn SyncProvider>)
        .add_value(Arc::new(NullSnapshotProvider::new()) as Arc<dyn SnapshotProvider>)
        .add_value(Arc::new(DefaultShutdownCoordinator::new()) as Arc<dyn ShutdownCoordinator>)
        // Admin services
        .add_value(Arc::new(NullPerformanceMetrics) as Arc<dyn PerformanceMetricsInterface>)
        .add_value(Arc::new(NullIndexingOperations) as Arc<dyn IndexingOperationsInterface>)
        .build()
}

pub struct AppContext {
    pub config: AppConfig,
    pub providers: ResolvedProviders,  // External providers from linkme registry
    catalog: Catalog,  // dill Catalog with infrastructure services
}

impl AppContext {
    pub fn get<T: ?Sized + Send + Sync + 'static>(&self) -> Arc<T> {
        self.catalog.get_one().expect("Service not registered in catalog")
    }
}
```

> **Hybrid Architecture**: External providers (embedding, vector_store, cache, language) are resolved via the linkme-based registry system. Internal infrastructure services are registered in the dill Catalog. This separation allows dynamic provider selection at runtime while maintaining type-safe DI for infrastructure services.

### Comparative Analysis

| Aspect | Shaku (Current) | dill (Proposed) |
|--------|----------------|-----------------|
| **API Complexity** | High (macros, modules, components) | Low (#[component], Catalog) |
| **Build Time** | Slow (extensive macro expansion) | Faster (simpler macros) |
| **Learning Curve** | Steep (Shaku-specific) | Moderate (catalog pattern) |
| **Testability** | Good (but requires Shaku setup) | Excellent (catalog builder) |
| **Cross-Crate** | Yes | Yes |
| **Async Support** | No | Tokio task-local |
| **Production Use** | Many projects | kamu-cli (similar architecture) |

## Consequences

### Positive

-   **Reduced complexity**: Simpler attribute-based registration
-   **Better readability**: Clear catalog-based composition
-   **Faster compilation**: Less macro expansion overhead
-   **Easier debugging**: Direct catalog resolution
-   **Architecture alignment**: dill designed for Clean Architecture
-   **Maintained automation**: Unlike manual injection, keeps DI benefits
-   **Optional dependencies**: `Maybe<T>` pattern for optional services

### Negative

-   **New dependency**: Adds dill to the dependency tree
-   **Runtime resolution**: Dependencies resolved at runtime, not compile-time
-   **API change**: Different syntax from Shaku
-   **Learning curve**: Team needs to learn dill patterns

### Risks

-   **Runtime errors**: Missing dependencies caught at runtime
-   **Catalog misconfiguration**: Must register all components
-   **Version stability**: dill is relatively newer than Shaku

## Migration Strategy

### Phase 1: Preparation

1.  Add dill dependency alongside Shaku
2.  Create new dill-based implementations in parallel
3.  Add integration tests for both approaches

### Phase 2: Gradual Migration

1.  Migrate infrastructure services first (mcb-infrastructure)
2.  Migrate application services (mcb-application)
3.  Migrate server bootstrap (mcb-server)
4.  Keep both systems running during transition

### Phase 3: Cleanup

1.  Remove Shaku dependencies from all crates
2.  Delete old Shaku module definitions
3.  Update all documentation
4.  Run comprehensive testing

## Validation Criteria

-   [x] All infrastructure services registered in dill Catalog (8 services)
-   [x] External providers use linkme-based registry (embedding, vector_store, cache, language)
-   [x] Test mocking works with catalog.get_one()
-   [x] All crates compile successfully
-   [x] No Shaku references remain in production code
-   [ ] Binary size remains stable (not yet measured)

### Implementation Summary (2026-01-18)

| Component | Pattern | Status |
|-----------|---------|--------|
| Infrastructure services | dill `add_value` | ✅ Implemented |
| External providers | linkme distributed slices | ✅ Implemented |
| Provider factories | Function pointers (not closures) | ✅ Implemented |
| Shaku removal | All macros removed | ✅ Completed |
| `#[component]` macro | Incompatible with domain errors | ⚠️ Not used |

## Related ADRs

-   [ADR 002: Dependency Injection with Shaku](002-dependency-injection-shaku.md) - **SUPERSEDED** by this ADR
-   [ADR 012: Two-Layer DI Strategy](012-di-strategy-two-layer-approach.md) - **SUPERSEDED** (dill simplifies to single layer)
-   [ADR 013: Clean Architecture Crate Separation](013-clean-architecture-crate-separation.md) - Multi-crate organization

## References

-   [dill-rs GitHub](https://github.com/sergiimk/dill-rs) - dill source and documentation (v0.15.0)
-   [dill on crates.io](https://crates.io/crates/dill) - 56,675 downloads, actively maintained
-   [kamu-cli](https://github.com/kamu-data/kamu-cli) - Production example using dill
-   [Rust DI Libraries Comparison](https://users.rust-lang.org/t/comparing-dependency-injection-libraries-shaku-nject/102619) - Community discussion

## Migration Status (Completed 2026-01-18)

**Shaku Removal Summary:**

| Category | Before | After |
|----------|--------|-------|
| `#[derive(Component)]` | 1 file | 0 files |
| `module!` macro | 4 files | 0 files |
| `use shaku::Interface` | ~20 files | 0 files |
| `: Interface` trait bound | ~20 files | 0 files |
| Shaku in Cargo.toml | 2 crates | 0 crates |

**Current Architecture:**

| Layer | Pattern | Files |
|-------|---------|-------|
| External Providers | linkme distributed slices + function pointers | 14 files (embedding, vector_store, cache, language) |
| Infrastructure Services | dill Catalog + add_value | 1 file (bootstrap.rs) |
| Service Resolution | AppContext.get::<dyn Trait>() | Used throughout mcb-server |

**Total migration effort**: ~45 files changed, all Shaku references removed
