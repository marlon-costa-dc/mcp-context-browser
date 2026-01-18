# Implementation Plan v0.1.2 - Refatoração & Simplification

## Executive Summary

**Goal**: Modernize MCP Context Browser by adopting simpler, more robust mechanisms that reduce infrastructure code and ensure better cross-platform compatibility.

**Scope**: Four targeted architectural migrations with automated validation and incremental rollout.

**Timeline**: 6 weeks (Weeks 1-4: Implementation, Weeks 5-6: Integration & Testing, Weeks 7-8: Monitoring)

**Validation**: Comprehensive automated checks via enhanced `mcb-validate` system with migration-specific validators.

**Risk Level**: Medium (incremental approach with rollback capabilities)

## Overview

This document outlines the implementation plan for MCP Context Browser v0.1.2, focusing on architectural improvements through targeted refactoring and simplification. The release introduces four key changes to modernize the codebase by adopting simpler and more robust mechanisms, reducing infrastructure code, and ensuring better cross-platform compatibility.

### Architectural Goals

1. **Plugin Registration**: Replace `inventory` crate with `linkme` for distributed slices
2. **Dependency Injection**: Replace `shaku` with explicit constructor injection
3. **Configuration**: Unify configuration loading with `figment`
4. **Web Routing**: Migrate from `axum` to `rocket` for attribute-based routing

### Key Benefits

- **Reduced Complexity**: Eliminate macro-heavy infrastructure code
- **Cross-Platform**: Better WASM and embedded platform support
- **Maintainability**: Simpler patterns with explicit dependencies
- **Performance**: Reduced compile-time overhead and runtime costs

### Technical References

#### Linkme (Plugin Registration)
- **Mechanism**: Allows registering static distributed elements via `#[linkme::distributed_slice]`
- **Collection**: Elements are collected by the linker into `&'static [T]` arrays
- **Advantages**: Eliminates boilerplate, WASM-compatible, easy iteration
- **Migration**: Replace `inventory::submit!` with `#[linkme::distributed_slice(NAME)]`

#### Figment (Configuration)
- **Mechanism**: Combines multiple configuration sources (TOML/JSON files, environment variables)
- **Extraction**: Type-safe extraction to typed structs with `extract<T>()`
- **Advantages**: Clear precedence rules, profile support, rich error messages
- **Migration**: Replace `config::Config::builder()` with `Figment::new().merge().extract()`

#### Constructor Injection (DI)
- **Mechanism**: Explicit dependency passing through constructor parameters
- **Advantages**: Clear dependencies, easy testing, no macro overhead
- **Trade-offs**: Runtime-only guarantees vs compile-time checking
- **Migration**: Replace `#[derive(Component)]` with manual `Arc<dyn Trait>` parameters

#### Rocket (Web Routing)
- **Mechanism**: Attribute-based routing with `#[get("/path")]`, `#[post("/path")]`
- **Advantages**: Familiar patterns, compile-time validation, built-in features
- **Migration**: Replace Axum route builders with Rocket attribute macros

## Timeline

**Total Duration**: 6 weeks
**Target Release Date**: 6 weeks from planning completion
**Release Manager**: Development Team

## Architectural Changes

### 1. Inventory to Linkme Migration (ADR 023)
**Duration**: 1 week
**Impact**: Provider registration system
**Risk Level**: Medium

#### Current State Analysis (mcb-validate results)
- ✅ **LinkmeValidator** criado: `make validate-linkme`
- 🔍 **Detection patterns**: `inventory::submit!`, `inventory::collect!`, `inventory::iter`
- 📊 **Expected violations**: ~15-20 provider registration sites
- 🎯 **Primary issues**: LINKME001 (inventory usage), LINKME005 (dependency)

#### Goals
- Replace inventory crate with linkme for plugin registration
- Reduce compile-time overhead and platform limitations
- Maintain backward compatibility for provider APIs

#### Implementation Steps

1. **Week 1, Day 1**: Run `make validate-linkme` to identify all `inventory::submit!` and `inventory::collect!` usage
2. **Week 1, Day 2**: Add `linkme` dependency to affected Cargo.toml files (mcb-providers, mcb-validate)
3. **Week 1, Day 3**: Declare distributed slices with `#[linkme::distributed_slice]`:
   ```rust
   #[linkme::distributed_slice]
   pub static EMBEDDING_PROVIDERS: [EmbeddingProviderEntry] = [..];
   ```
4. **Week 1, Day 4**: Replace `inventory::submit!` with slice registration:
   ```rust
   #[linkme::distributed_slice(EMBEDDING_PROVIDERS)]
   static OLLAMA_PROVIDER: EmbeddingProviderEntry = EmbeddingProviderEntry { ... };
   ```
5. **Week 1, Day 5**: Update collection code from `inventory::iter()` to direct slice iteration:
   ```rust
   // Before: for entry in inventory::iter::<EmbeddingProviderEntry>()
   // After:  for entry in EMBEDDING_PROVIDERS
   ```
6. **Week 1, Day 6**: Remove `inventory` dependency and unused code
7. **Week 1, Day 7**: Run `make validate-linkme` to verify completion (0 violations)

#### Validation
- ✅ `make validate-linkme` returns 0 violations
- ✅ All providers discoverable via new system
- ✅ Build succeeds on Linux, macOS, Windows
- ✅ WASM compatibility verified
- ✅ Performance benchmarks maintained

### 2. Simplified Dependency Injection (ADR 024)
**Duration**: 1 week
**Impact**: Core infrastructure
**Risk Level**: High

#### Current State Analysis (mcb-validate results)
- ✅ **ConstructorInjectionValidator** criado: `make validate-ctor`
- 🔍 **Detection patterns**: `#[derive(Component)]`, `#[shaku(inject)]`, `module!`, `container.resolve()`
- 📊 **Expected violations**: ~50-80 Shaku usage sites across infrastructure
- 🎯 **Primary issues**: CTOR001-004 (Shaku patterns), CTOR009 (container.resolve)

#### Goals
- Replace Shaku DI container with constructor injection
- Eliminate macro-heavy DI patterns
- Improve compile times and debugging experience

#### Implementation Steps

1. **Week 2, Day 1**: Run `make validate-ctor` to identify all `#[derive(Component)]`, `#[shaku(inject)]`, `module!` usage
2. **Week 2, Day 2**: Create manual container structure:
   ```rust
   pub struct ServiceContainer {
       pub embedding_svc: Arc<dyn EmbeddingService>,
       pub search_svc: Arc<dyn SearchService>,
       pub cache_svc: Arc<dyn CacheProvider>,
   }
   ```
3. **Week 2, Day 3**: Convert services to constructor injection:
   ```rust
   // Before: #[derive(Component)] #[shaku(interface = dyn MyService)]
   // After:  pub struct MyServiceImpl { dep: Arc<dyn DepTrait> }

   impl MyServiceImpl {
       pub fn new(dep: Arc<dyn DepTrait>) -> Self {
           Self { dep }
       }
   }
   ```
4. **Week 2, Day 4**: Implement manual container builder:
   ```rust
   impl ServiceContainer {
       pub async fn new(config: &AppConfig) -> Result<Self> {
           let cache_svc = Arc::new(MokaCacheProvider::new(&config.cache));
           let embedding_svc = Arc::new(EmbeddingServiceImpl::new(cache_svc.clone()));
           // ... build other services
           Ok(Self { embedding_svc, cache_svc, ... })
       }
   }
   ```
5. **Week 2, Day 5**: Update service instantiation throughout codebase
6. **Week 2, Day 6**: Remove `shaku` dependency and all Shaku-related code
7. **Week 2, Day 7**: Run `make validate-ctor` to verify completion (0 violations)

#### Validation
- ✅ `make validate-ctor` returns 0 violations
- ✅ All services instantiable via constructor injection
- ✅ Compile time reduced by >20%
- ✅ Test mocking still functional
- ✅ No runtime performance regression

### 3. Figment Configuration Migration (ADR 025)
**Duration**: 1 week
**Impact**: Configuration system
**Risk Level**: Medium

#### Current State Analysis (mcb-validate results)
- ✅ **FigmentValidator** criado: `make validate-figment`
- 🔍 **Detection patterns**: `Config::builder()`, `config::Environment`, `config::File`
- 📊 **Expected violations**: ~10-15 configuration loading sites
- 🎯 **Primary issues**: FIGMENT001-003 (config crate usage), FIGMENT006 (dependency)

#### Goals
- Replace config crate with Figment for unified configuration
- Add profile support (development/production)
- Improve error messages and validation

#### Implementation Steps

1. **Week 3, Day 1**: Run `make validate-figment` to identify all `Config::builder()`, `config::Environment` usage
2. **Week 3, Day 2**: Add `figment` dependency with required features:
   ```toml
   figment = { version = "0.10", features = ["toml", "env"] }
   ```
3. **Week 3, Day 3**: Migrate configuration loading to Figment pattern:
   ```rust
   use figment::{Figment, providers::{Toml, Env}};

   // Before: Config::builder().add_source(File::from(path)).add_source(Environment::with_prefix("APP"))
   // After:
   let figment = Figment::new()
       .merge(Toml::file("config/default.toml"))  // Base config
       .merge(Toml::file(config_path))           // User overrides
       .merge(Env::prefixed("APP_").split("_")); // Environment variables

   let config: AppConfig = figment.extract()?;
   ```
4. **Week 3, Day 4**: Add profile support:
   ```rust
   let profile = std::env::var("APP_PROFILE").unwrap_or_else(|_| "development".to_string());
   let config_file = format!("config/{}.toml", profile);

   let figment = Figment::new()
       .merge(Toml::file("config/default.toml"))
       .merge(Toml::file(config_file))  // Profile-specific config
       .merge(Env::prefixed("APP_"));
   ```
5. **Week 3, Day 5**: Update error handling for richer Figment errors
6. **Week 3, Day 6**: Remove `config` crate dependency and update documentation
7. **Week 3, Day 7**: Run `make validate-figment` to verify completion (0 violations)

#### Validation
- ✅ `make validate-figment` returns 0 violations
- ✅ All configuration sources load correctly
- ✅ Profile-based config works
- ✅ Error messages more helpful
- ✅ Backward compatibility maintained

### 4. API Routing Refactor - Rocket (ADR 026)
**Duration**: 1 week
**Impact**: HTTP server
**Risk Level**: High

#### Current State Analysis (mcb-validate results)
- ✅ **RocketValidator** criado: `make validate-rocket`
- 🔍 **Detection patterns**: `axum::Router`, `axum::routing::*`, Tower middleware
- 📊 **Expected violations**: ~20-30 routing and middleware sites
- 🎯 **Primary issues**: ROCKET001-002 (Axum usage), ROCKET005 (dependency)

#### Goals
- Migrate from Axum to Rocket for attribute-based routing
- Simplify route definitions and middleware
- Improve developer experience

#### Implementation Steps

1. **Week 4, Day 1**: Run `make validate-rocket` to identify all `axum::Router`, `axum::routing::*` usage
2. **Week 4, Day 2**: Add `rocket` dependency:
   ```toml
   rocket = { version = "0.5", features = ["json"] }
   ```
3. **Week 4, Day 3**: Convert Axum handlers to Rocket attribute-based routing:
   ```rust
   // Before (Axum):
   async fn health_check() -> Json<HealthResponse> { /* ... */ }
   let router = Router::new().route("/health", get(health_check));

   // After (Rocket):
   #[get("/health")]
   fn health_check() -> Json<HealthResponse> { /* ... */ }
   ```
4. **Week 4, Day 4**: Create route modules and register them:
   ```rust
   mod health_routes {
       use rocket::{get, routes};
       use rocket::serde::json::Json;

       #[get("/health")]
       fn check() -> Json<HealthStatus> { /* ... */ }

       pub fn routes() -> Vec<rocket::Route> {
           routes![check]
       }
   }

   // In main:
   rocket::build()
       .mount("/api/v1", health_routes::routes())
   ```
5. **Week 4, Day 5**: Convert middleware from Tower to Rocket fairings:
   ```rust
   // Before: tower_http::{CorsLayer, TraceLayer}
   // After:  Rocket fairings for CORS, logging, etc.
   ```
6. **Week 4, Day 6**: Update state management and error handling
7. **Week 4, Day 7**: Remove `axum`, `axum-extra`, `tower`, `tower-http` dependencies and run `make validate-rocket` (0 violations)

#### Validation
- ✅ `make validate-rocket` returns 0 violations
- ✅ All API endpoints functional
- ✅ Route definitions simplified
- ✅ Performance maintained or improved
- Documentation updated

## Integration & Testing Phase (Weeks 5-6)

### Week 5: Integration Testing
- **Day 1**: Run `make validate-migration` (all migration validators)
- **Day 2**: End-to-end testing with all changes integrated
- **Day 3**: Cross-platform compatibility testing (Linux/macOS/Windows)
- **Day 4**: Performance regression testing with benchmarks
- **Day 5**: Integration test suite execution

### Week 6: Validation & Documentation
- **Day 1**: Run `make validate` (complete architecture validation)
- **Day 2**: Run `make quality` (full quality gate)
- **Day 3**: Documentation updates and migration guides
- **Day 4**: Final integration testing and release preparation
- **Day 5**: Security audit and final review
- **Day 6-7**: Release candidate testing and deployment preparation

## Quality Gates

### Code Quality
- [ ] `make validate-migration` passes (0 violations for all migration validators)
- [ ] `make validate` passes (complete architecture validation)
- [ ] `make quality` passes (fmt + lint + test + doc)
- [ ] Clippy warnings eliminated
- [ ] Code coverage maintained >90%
- [ ] No new unwrap/expect usage

### Migration Validation
- [ ] `make validate-linkme` returns 0 violations
- [ ] `make validate-ctor` returns 0 violations
- [ ] `make validate-figment` returns 0 violations
- [ ] `make validate-rocket` returns 0 violations

### Performance
- [ ] Compile time improved or maintained
- [ ] Runtime performance not regressed
- [ ] Binary size stable or reduced
- [ ] Memory usage stable

### Compatibility
- [ ] All existing APIs functional
- [ ] Configuration backward compatible
- [ ] Provider interfaces stable
- [ ] Build succeeds on all target platforms

## Risk Mitigation

### Branching Strategy
- **Feature branches**: Create separate branches for each migration (`feature/linkme-migration`, `feature/di-simplification`, etc.)
- **Incremental merging**: Only merge after all tests pass and code review is complete
- **Independent validation**: Each migration can be validated independently
- **Quick rollback**: Individual branches can be reverted without affecting others

### CI/CD Integration
- **Pipeline triggers**: Run `make validate-migration` on every push to feature branches
- **Quality gates**: Block merges if any migration validator returns violations
- **Parallel testing**: Run migration-specific tests alongside general test suite
- **Benchmark monitoring**: Track performance regressions in CI pipeline

### Rollback Strategy
- **Individual reversion**: Can rollback specific migrations without affecting others
- **Compatibility preservation**: Old code patterns remain functional during transition
- **Gradual degradation**: System remains operational even with partial rollbacks

### Contingency Plans
- **Linkme issues**: Fallback to custom plugin registration
- **Rocket complexity**: Alternative Poem evaluation and migration
- **Performance regression**: Optimization phase before release

## Success Metrics

### Technical Metrics
- Compile time reduction: >15%
- Binary size change: <5% increase
- Test execution time: <10% increase
- Code lines changed: ~2000-3000 lines

### Quality Metrics
- Architecture violations: 0 (post-migration)
- Integration test pass rate: 100%
- Documentation completeness: 100%
- Developer satisfaction survey: >8/10

## Communication Plan

### Internal Communication
- **Weekly status updates**: Progress reports every Friday
- **Technical reviews**: Architecture changes reviewed by team
- **Risk updates**: Any blocking issues communicated immediately

### External Communication
- **Release notes**: Comprehensive changelog for v0.1.2
- **Migration guides**: Documentation for any breaking changes
- **Deprecation notices**: Clear communication of removed features

## Acceptance Criteria

### Linkme Migration (Phase 1)
- ✅ `make validate-linkme` returns 0 violations
- ✅ No usage of `inventory::submit!` or `inventory::collect!` in codebase
- ✅ All plugin registrations use `#[linkme::distributed_slice(NAME)]`
- ✅ Plugin discovery works correctly through slice iteration
- ✅ Build succeeds on Linux, macOS, Windows, and WASM targets
- ✅ `inventory` dependency completely removed from all Cargo.toml files

### Constructor Injection (Phase 2)
- ✅ `make validate-ctor` returns 0 violations
- ✅ No usage of `#[derive(Component)]`, `#[shaku(inject)]`, `module!` macros
- ✅ All services use constructor injection with `Arc<dyn Trait>` parameters
- ✅ Manual container provides all service dependencies
- ✅ Compile time improved (target: >15% reduction)
- ✅ `shaku` dependency completely removed

### Figment Configuration (Phase 3)
- ✅ `make validate-figment` returns 0 violations
- ✅ No usage of `config::Config::builder()` or `config::Environment`
- ✅ All configuration uses `Figment::new().merge().extract()` pattern
- ✅ Profile support implemented (development/production)
- ✅ Rich error messages with source attribution
- ✅ `config` dependency completely removed

### Rocket Routing (Phase 4)
- ✅ `make validate-rocket` returns 0 violations
- ✅ No usage of `axum::Router` or `axum::routing::*`
- ✅ All routes use Rocket attribute macros (`#[get]`, `#[post]`, etc.)
- ✅ Route organization uses `routes![]` macro and `.mount()`
- ✅ Middleware converted to Rocket fairings
- ✅ `axum`, `tower`, `tower-http` dependencies removed

## Dependencies

### Added Dependencies
- `linkme = "0.3"` - Distributed slice plugin registration
- `figment = { version = "0.10", features = ["toml", "env"] }` - Unified configuration
- `rocket = { version = "0.5", features = ["json"] }` - Web framework

### Removed Dependencies
- `inventory = "0.3"` - Replaced by linkme
- `shaku = "0.6"` - Replaced by constructor injection
- `config = "0.15"` - Replaced by figment
- `axum = "0.8"` - Replaced by rocket
- `axum-extra = "0.12"` - Replaced by rocket
- `tower = "0.5"` - Replaced by rocket
- `tower-http = "0.6"` - Replaced by rocket

## Post-Release Activities

### Week 7-8: Monitoring & Support
- **Production monitoring**: Performance and error tracking
- **User feedback**: Collect feedback on new architecture
- **Bug fixes**: Address any issues discovered in production
- **Documentation updates**: User guides and API documentation

## Post-Migration Cleanup

### Code Cleanup
- **Remove legacy code**: Eliminate any fallback patterns or compatibility layers
- **Update imports**: Clean up any unused imports across all crates
- **Documentation**: Update inline documentation and examples
- **Comments**: Remove outdated comments referencing old patterns

### Documentation Updates
- **README**: Update architecture overview and setup instructions
- **ADRs**: Mark migration ADRs as "Implemented" with completion dates
- **API docs**: Update examples to use new patterns
- **Migration guide**: Create guide for any external users

### Final Validation
- **Comprehensive testing**: Run full test suite on all platforms
- **Performance benchmarking**: Establish new performance baselines
- **Security audit**: Ensure no security regressions
- **Compatibility testing**: Verify with existing MCP clients

### Future Releases
- **v0.2.0**: Git-aware semantic indexing (original v0.2.0 features)
- **v0.3.0**: Multi-domain architecture expansion
- **v1.0.0**: Stable API and production hardening

## Summary

This implementation plan provides a structured approach to modernizing the MCP Context Browser through targeted architectural improvements. By following this phased approach with automated validation at each step, we ensure:

1. **Incremental Progress**: Each migration builds upon the previous, with clear validation checkpoints
2. **Risk Mitigation**: Independent branches and rollback capabilities minimize disruption
3. **Quality Assurance**: Automated validation prevents regressions and ensures consistency
4. **Knowledge Preservation**: Updated ADRs and documentation maintain architectural decisions

The four key migrations (Linkme, Constructor Injection, Figment, Rocket) collectively reduce infrastructure complexity, improve cross-platform compatibility, and establish clearer architectural patterns for future development.

**Success Metrics**:
- ✅ Zero migration violations in final codebase
- ✅ Improved compile times and runtime performance
- ✅ Cross-platform compatibility (WASM support)
- ✅ Simplified maintenance and testing
- ✅ Comprehensive documentation and examples

---

**Document Version**: 2.0
**Last Updated**: January 18, 2026
**Authors**: Development Team
**Reviewers**: Architecture Team
**Based on**: Detailed technical analysis and migration validation system