# ADR 024: Simplified Dependency Injection

## Status

**Proposed** (v0.1.2)

> Planned replacement for [ADR 002: Dependency Injection with Shaku](002-dependency-injection-shaku.md) as part of the refatoração and simplification initiative.

## Context

The current dependency injection system uses Shaku (version 0.6), a compile-time DI container that provides trait-based dependency resolution. While effective, this approach introduces substantial complexity that impacts development velocity and maintainability.

### Current Shaku Implementation

The current system uses a two-layer DI approach (ADR 012) with:

1. **Shaku Components**: Services registered with `#[derive(Component)]` and `#[shaku(interface = dyn Trait)]`
2. **Runtime Factories**: Production providers created via factory functions outside Shaku
3. **Module Composition**: Services organized in Shaku modules with macro-generated wiring

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
1. **Macro complexity**: `#[derive(Component)]`, `#[shaku(interface = ...)]`, `#[shaku(inject)]` everywhere
2. **Build time impact**: Extensive macro expansion slows compilation
3. **Learning curve**: Shaku API is complex for new team members
4. **Debugging difficulty**: DI resolution happens through macro-generated code

#### Maintenance Issues
1. **Module sync**: Manual maintenance of module definitions as services change
2. **Trait bounds**: Complex trait bounds on component implementations
3. **Testing overhead**: Need to understand Shaku to write unit tests
4. **Refactoring friction**: Changes require updating multiple macro annotations

#### Architectural Issues
1. **Over-engineering**: DI container complexity exceeds project needs
2. **Runtime opacity**: Service resolution happens through generated code
3. **Limited flexibility**: Hard to customize service creation per environment

### Constructor Injection as Alternative

Constructor injection with manual composition provides a simpler approach:

**Key Benefits:**
1. **Explicit dependencies**: Constructor parameters make dependencies clear
2. **Easy testing**: Direct constructor calls enable straightforward mocking
3. **Fast compilation**: No macro expansion overhead
4. **Simple debugging**: Direct object construction is easy to trace
5. **Framework independence**: No DI framework lock-in

**Constructor injection pattern:**
```rust
// Service with explicit dependencies
pub struct EmbeddingService {
    provider: Arc<dyn EmbeddingProvider>,
    cache: Arc<dyn CacheProvider>,
}

impl EmbeddingService {
    // Dependencies are explicit constructor parameters
    pub fn new(
        provider: Arc<dyn EmbeddingProvider>,
        cache: Arc<dyn CacheProvider>,
    ) -> Self {
        Self { provider, cache }
    }
}

// Manual composition in bootstrap
let provider = Arc::new(OllamaProvider::new(config));
let cache = Arc::new(MokaCache::new(config));
let service = Arc::new(EmbeddingService::new(provider, cache));
```

### Comparative Analysis

| Aspect | Shaku (Current) | Constructor Injection |
|--------|----------------|----------------------|
| **API Complexity** | High (macros, modules, components) | Low (constructors, Arc<T>) |
| **Build Time** | Slow (extensive macro expansion) | Fast (no macros) |
| **Learning Curve** | Steep (Shaku-specific knowledge) | Shallow (standard Rust patterns) |
| **Testability** | Good (but requires Shaku setup) | Excellent (direct constructor calls) |
| **Debugging** | Difficult (macro-generated code) | Easy (direct object construction) |
| **Flexibility** | Limited (container-managed) | High (manual composition) |
| **Maintenance** | High (module definitions) | Low (constructor changes) |

## Decision

We will replace Shaku-based DI with a simplified constructor injection pattern:

1. **Remove Shaku dependency** from all crates
2. **Use constructor injection** for all service dependencies
3. **Maintain trait-based abstraction** for testability
4. **Use manual service composition** in the application bootstrap
5. **Keep dependency inversion** through trait objects (`Arc<dyn Trait>`)

### Implementation Pattern

**Before (Shaku):**
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
```

**After (Constructor Injection):**
```rust
pub struct MyServiceImpl {
    dependency: Arc<dyn OtherService>,
}

impl MyServiceImpl {
    pub fn new(dependency: Arc<dyn OtherService>) -> Self {
        Self { dependency }
    }
}

// Manual composition in bootstrap
let other_service = Arc::new(OtherServiceImpl::new());
let my_service = Arc::new(MyServiceImpl::new(other_service));
```

### Bootstrap Pattern

Service composition will be handled in `mcb-infrastructure/src/di/bootstrap.rs`:

```rust
pub struct ServiceContainer {
    pub my_service: Arc<dyn MyService>,
    pub other_service: Arc<dyn OtherService>,
}

impl ServiceContainer {
    pub fn new(config: &AppConfig) -> Result<Self> {
        let other_service = Arc::new(OtherServiceImpl::new(config));
        let my_service = Arc::new(MyServiceImpl::new(other_service.clone()));

        Ok(Self {
            my_service,
            other_service,
        })
    }
}
```

## Consequences

### Positive
- **Reduced complexity**: Eliminates macro-heavy DI infrastructure
- **Better readability**: Clear dependency chains through constructors
- **Faster compilation**: Less macro expansion overhead
- **Easier debugging**: Direct object construction is straightforward
- **Simplified testing**: Constructor injection still enables mocking
- **Smaller dependency tree**: Removes Shaku and related macro dependencies

### Negative
- **Manual composition**: Service wiring becomes explicit code
- **Runtime errors**: Missing dependencies caught at runtime, not compile-time
- **Boilerplate**: More verbose service instantiation
- **Refactoring impact**: Changes to dependencies require updating multiple constructors

### Risks
- **Service creation errors**: Runtime failures if dependencies are missing
- **Constructor parameter creep**: Large constructors with many dependencies
- **Testing complexity**: Need to manually create all dependency chains in tests

## Migration Strategy

### Phase 1: Preparation
1. Create new constructor injection implementations alongside Shaku code
2. Add integration tests to verify both approaches work
3. Update documentation and examples

### Phase 2: Gradual Migration
1. Migrate infrastructure services first (least dependent)
2. Migrate application services
3. Migrate server bootstrap code
4. Keep both systems running in parallel during transition

### Phase 3: Cleanup
1. Remove Shaku dependencies
2. Delete old Shaku module definitions
3. Update all documentation
4. Run comprehensive testing

## Validation Criteria

- [ ] All services can be instantiated through constructor injection
- [ ] Test mocking still works with trait objects
- [ ] Application startup time improves or stays the same
- [ ] Compile time reduces significantly
- [ ] All integration tests pass
- [ ] Binary size remains stable or decreases
- [ ] Code coverage maintained

## Related ADRs

- [ADR 002: Dependency Injection with Shaku](002-dependency-injection-shaku.md) - **SUPERSEDED** by this ADR
- [ADR 012: Two-Layer DI Strategy](012-di-strategy-two-layer-approach.md) - Related DI layering approach
- [ADR 013: Clean Architecture Crate Separation](013-clean-architecture-crate-separation.md) - Multi-crate organization