# Implementation Plan v0.1.2 - Refatoração & Simplification

## Overview

This document outlines the implementation plan for MCP Context Browser v0.1.2, focusing on architectural improvements through targeted refactoring and simplification. The release introduces four key changes to reduce complexity and improve maintainability.

## Timeline

**Total Duration**: 6 weeks
**Target Release Date**: 6 weeks from planning completion
**Release Manager**: Development Team

## Architectural Changes

### 1. Inventory to Linkme Migration (ADR 023)
**Duration**: 1 week
**Impact**: Provider registration system
**Risk Level**: Medium

#### Goals
- Replace inventory crate with linkme for plugin registration
- Reduce compile-time overhead and platform limitations
- Maintain backward compatibility for provider APIs

#### Implementation Steps
1. **Week 1, Day 1-2**: Add linkme dependency, create migration utilities
2. **Week 1, Day 3-4**: Migrate embedding providers (OpenAI, Ollama, Gemini, FastEmbed)
3. **Week 1, Day 5-7**: Migrate vector store and cache providers

#### Validation
- All providers discoverable via new system
- Build succeeds on Linux, macOS, Windows
- WASM compatibility verified
- Performance benchmarks maintained

### 2. Simplified Dependency Injection (ADR 024)
**Duration**: 1 week
**Impact**: Core infrastructure
**Risk Level**: High

#### Goals
- Replace Shaku DI container with constructor injection
- Eliminate macro-heavy DI patterns
- Improve compile times and debugging experience

#### Implementation Steps
1. **Week 2, Day 1-3**: Create constructor injection patterns alongside Shaku
2. **Week 2, Day 4-5**: Migrate infrastructure services (cache, auth, metrics)
3. **Week 2, Day 6-7**: Remove Shaku dependencies and clean up code

#### Validation
- All services instantiable via constructor injection
- Compile time reduced by >20%
- Test mocking still functional
- No runtime performance regression

### 3. Figment Configuration Migration (ADR 025)
**Duration**: 1 week
**Impact**: Configuration system
**Risk Level**: Medium

#### Goals
- Replace config crate with Figment for unified configuration
- Add profile support (development/production)
- Improve error messages and validation

#### Implementation Steps
1. **Week 3, Day 1-3**: Migrate configuration loader to Figment
2. **Week 3, Day 4-5**: Add profile support and validation
3. **Week 3, Day 6-7**: Update configuration documentation

#### Validation
- All configuration sources load correctly
- Profile-based config works
- Error messages more helpful
- Backward compatibility maintained

### 4. API Routing Refactor - Rocket (ADR 026)
**Duration**: 1 week
**Impact**: HTTP server
**Risk Level**: High

#### Goals
- Migrate from Axum to Rocket for attribute-based routing
- Simplify route definitions and middleware
- Improve developer experience

#### Implementation Steps
1. **Week 4, Day 1-2**: Evaluate Rocket vs Poem, finalize Rocket selection
2. **Week 4, Day 3-5**: Migrate admin API routes to Rocket
3. **Week 4, Day 6-7**: Update middleware and error handling

#### Validation
- All API endpoints functional
- Route definitions simplified
- Performance maintained or improved
- Documentation updated

## Integration & Testing Phase (Weeks 5-6)

### Week 5: Integration Testing
- **Day 1-2**: End-to-end testing with all changes integrated
- **Day 3-4**: Cross-platform compatibility testing
- **Day 5**: Performance regression testing

### Week 6: Validation & Documentation
- **Day 1-3**: Architecture validation with updated mcb-validate rules
- **Day 4-5**: Documentation updates and migration guides
- **Day 6-7**: Final integration testing and release preparation

## Quality Gates

### Code Quality
- [ ] All mcb-validate rules pass (including new migration validators)
- [ ] Clippy warnings eliminated
- [ ] Code coverage maintained >90%
- [ ] No new unwrap/expect usage

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

### Rollback Strategy
- **Branch-based development**: Each change in separate feature branch
- **Incremental merging**: Master branch stability maintained
- **Quick rollback**: Ability to revert individual changes if needed

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

## Dependencies

### External Dependencies
- linkme = "0.3" (new)
- figment = "0.10" (new)
- rocket = "0.5" (new)

### Removed Dependencies
- inventory = "0.3" (removed)
- shaku = "0.6" (removed)
- config = "0.15" (removed)
- axum/tower ecosystem (replaced)

## Post-Release Activities

### Week 7-8: Monitoring & Support
- **Production monitoring**: Performance and error tracking
- **User feedback**: Collect feedback on new architecture
- **Bug fixes**: Address any issues discovered in production
- **Documentation updates**: User guides and API documentation

### Future Releases
- **v0.2.0**: Git-aware semantic indexing (original v0.2.0 features)
- **v0.3.0**: Multi-domain architecture expansion
- **v1.0.0**: Stable API and production hardening

---

**Document Version**: 1.0
**Last Updated**: January 18, 2026
**Authors**: Development Team
**Reviewers**: Architecture Team