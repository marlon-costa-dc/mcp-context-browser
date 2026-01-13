# Testing Strategy and Documentation

## Overview

This test suite provides comprehensive coverage for the MCP Context Browser, implementing multiple testing strategies to ensure code quality, performance, and reliability. Tests are organized by module following the source code structure.

## Test Organization

Tests are organized in a structure that mirrors the `src/` directory (Clean Architecture layers):

```text
tests/
├── server/             # Tests for src/server/
│   ├── admin/          # Admin web interface tests
│   ├── transport/      # HTTP transport tests
│   └── handlers_test.rs
├── domain/             # Tests for src/domain/
│   ├── chunking/       # Code chunking tests
│   ├── core_types.rs   # Domain types
│   ├── error_handling.rs
│   └── validation_tests.rs
├── infrastructure/     # Tests for src/infrastructure/
│   ├── config/         # Configuration tests
│   ├── di/             # DI/Shaku tests
│   ├── metrics/        # Metrics tests
│   ├── snapshot/       # Snapshot tests
│   └── sync/           # Sync manager tests
├── adapters/           # Tests for src/adapters/
│   ├── providers/      # Embedding, vector store, routing
│   ├── repository/     # Repository tests
│   └── hybrid_search/  # Hybrid search tests
├── application/        # Tests for src/application/
│   └── services_logic.rs
├── e2e/                # End-to-end integration tests
│   └── docker/         # Docker-based tests
├── perf/               # Performance benchmarks
├── fixtures/           # Test data and helpers
│   ├── artifacts/      # Test data files
│   └── test_helpers.rs # Shared utilities
├── validation/         # Validation system tests
└── README.md           # This documentation
```

**Naming Convention**: `{source_file}_test.rs` for test files that correspond to source files.

## Test Categories

### 1. Layer-Specific Tests

Tests mirror the Clean Architecture layers:

-   **domain/**: Domain types, validation, and error handling tests
-   **application/**: Business service layer tests (indexing, search, context)
-   **adapters/**: Provider and repository implementation tests
-   **infrastructure/**: Auth, cache, events, and daemon tests
-   **server/**: MCP handlers, admin, and protocol tests
-   **core/**: Legacy core type tests
-   **validation/**: Input validation and business rules tests

### 2. Integration Tests (`tests/e2e/`)

Component interaction and end-to-end testing:

-   MCP protocol implementation tests
-   Docker container integration tests
-   Cross-component interaction validation
-   End-to-end request processing
-   Concurrent access patterns
-   System boundary testing

### 3. Benchmark Tests (`tests/perf/`)

Performance measurement with Criterion:

-   Core type operations benchmarking
-   Provider performance characteristics
-   Repository operation benchmarks
-   Memory usage analysis
-   Concurrent operation performance
-   System throughput measurements

### 4. Unit Tests (`tests/unit/`)

General unit tests that don't fit specific modules:

-   Property-based tests with proptest
-   Security and safety tests
-   Rate limiting functionality tests
-   General utility function tests

## Testing Strategy

### TDD (Test-Driven Development)

All new features follow TDD principles:

1.  Write failing test first
2.  Implement minimal code to pass
3.  Refactor while maintaining test coverage

### Coverage Goals

-   **Unit Tests**: 80%+ coverage of individual functions
-   **Integration Tests**: All component interactions tested
-   **Property Tests**: Edge cases and invariants verified
-   **Performance Tests**: Benchmarks for critical paths

### Quality Gates

-   All tests must pass before commits
-   Coverage reports generated and reviewed
-   Performance benchmarks tracked over time
-   Property tests catch edge cases missed by example tests

## Running Tests

### Basic Test Execution

```bash
# Run all tests (organized by module)
cargo test

# Run tests for specific module
cargo test chunking
cargo test config
cargo test core

# Run integration tests
cargo test integration

# Run benchmark tests
cargo test benchmark

# Run unit tests
cargo test unit

# Run with coverage
cargo tarpaulin --out Html

# Run performance benchmarks
cargo bench
```

### Module-Specific Testing

```bash
# Test individual modules
cargo test providers::embedding_providers
cargo test core::core_types
cargo test validation

# Test specific functionality
cargo test chunking::chunking::tests::test_rust_chunking_with_tree_sitter
```

### Integration Testing

```bash
# Run all integration tests
cargo test integration

# Run specific integration tests
cargo test integration::mcp_protocol
cargo test integration::docker
```

### Property-Based Testing

```bash
# Run property tests
cargo test unit::property_based

# Run with more test cases
PROPTEST_CASES=1000 cargo test unit::property_based
```

## Test Organization

### Directory Structure

Tests follow Clean Architecture layers matching the source structure:

```text
tests/
├── domain/                # Domain layer tests
│   └── mod.rs            # Domain type and validation tests
├── application/          # Application service tests
│   └── mod.rs            # Service layer tests
├── adapters/             # Adapter tests
│   ├── mod.rs            # Adapter aggregator
│   └── database.rs       # Database adapter tests
├── infrastructure/       # Infrastructure tests
│   ├── mod.rs            # Infrastructure aggregator
│   ├── auth.rs           # Authentication tests
│   ├── events.rs         # Event bus tests
│   ├── nats_event_bus_integration.rs  # NATS integration
│   └── daemon/           # Daemon tests
├── server/               # Server tests
│   ├── mod.rs            # Server aggregator
│   ├── handlers/         # Handler tests
│   └── admin/            # Admin tests
├── integration/          # Integration tests
│   ├── mod.rs            # Integration aggregator
│   ├── docker/           # Docker integration tests
│   └── mcp*.rs           # MCP protocol tests
├── benchmark/            # Performance benchmarks
│   └── mod.rs            # Benchmark tests
├── core/                 # Legacy core tests
│   ├── mod.rs            # Core aggregator
│   └── core_types.rs     # Type tests
├── unit/                 # General unit tests
│   ├── mod.rs            # Unit test aggregator
│   └── property_based.rs # Property-based tests
├── validation/           # Validation tests
│   ├── mod.rs            # Validation aggregator
│   └── comprehensive.rs  # Comprehensive validation
└── README.md             # This documentation
```

### Naming Conventions

-   `mod.rs`: Module declaration file in each directory
-   `*_tests.rs`: Test files containing multiple test modules
-   `*_unit.rs`: Unit tests for specific functionality
-   `*_integration.rs`: Tests for component interactions
-   `*_property.rs`: Property-based tests
-   `*_benchmark.rs`: Performance benchmarks

## Coverage Analysis

### Current Coverage Status

-   **Unit Tests**: Comprehensive coverage of core functionality
-   **Integration**: Component interaction validation
-   **Property Tests**: Edge case and invariant verification
-   **Performance**: Benchmark tracking for optimization

### Coverage Goals by Module

-   Core Types: 95%+ coverage
-   Validation: 90%+ coverage
-   Repository: 85%+ coverage
-   Services: 80%+ coverage
-   Configuration: 85%+ coverage

## Continuous Integration

### Automated Testing

-   All tests run on every commit
-   Coverage reports generated automatically
-   Performance regression detection
-   Property test failure alerts

### Quality Gates

-   Test pass rate: 100%
-   Minimum coverage thresholds
-   Performance benchmark baselines
-   No memory leaks or crashes

## Contributing

### Adding New Tests

1.  Identify the appropriate test category
2.  Follow naming conventions
3.  Include comprehensive documentation
4.  Ensure tests are deterministic
5.  Add performance benchmarks for critical paths

### Test Best Practices

-   Tests should be fast and reliable
-   Use descriptive names that explain the behavior being tested
-   Include edge cases and error conditions
-   Mock external dependencies appropriately
-   Clean up test resources properly

## Troubleshooting

### Common Issues

-   **Flaky Tests**: Ensure tests don't depend on external state
-   **Slow Tests**: Profile and optimize or move to benchmarks
-   **Coverage Gaps**: Add missing test cases
-   **Integration Failures**: Check dependency setup and mocking

### Debug Tools

-   `cargo test -- --nocapture`: See test output
-   `cargo tarpaulin`: Generate coverage reports
-   `cargo bench`: Run performance benchmarks
-   `PROPTEST_CASES=10000 cargo test`: Increase property test iterations

---

## Cross-References

-   **Architecture**: [ARCHITECTURE.md](../docs/architecture/ARCHITECTURE.md)
-   **Contributing**: [CONTRIBUTING.md](../docs/developer/CONTRIBUTING.md)
-   **Examples**: [examples/](../examples/)
-   **Module Documentation**: [docs/modules/](../docs/modules/)
