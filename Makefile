# MCP Context Browser - Complete Development & Documentation Makefile

.PHONY: help
.DEFAULT_GOAL := help

# =============================================================================
# DEVELOPMENT TARGETS
# =============================================================================

# Development setup
setup: ## Setup development environment
	cargo install cargo-audit
	cargo install cargo-tarpaulin
	cargo install cargo-watch
	@echo "✅ Development environment ready"

# Development commands
dev: ## Run development server with auto-reload
	cargo watch -x run

build: ## Build the project
	cargo build

build-release: ## Build the project in release mode
	cargo build --release

test: ## Run all tests
	cargo test

test-unit: ## Run unit tests only
	cargo test --lib

test-integration: ## Run integration tests only
	cargo test --test integration

test-mcp: ## Run MCP protocol tests only
	cargo test --test mcp_protocol

test-watch: ## Run tests with auto-reload
	cargo watch -x test

# Quality checks
fmt: ## Format code
	cargo fmt

check: ## Check code without building
	cargo check

lint: ## Run clippy linter
	cargo clippy -- -D warnings

audit: ## Run security audit
	cargo audit

# Documentation - Legacy (use docs-* targets below)
doc: ## Generate Rust documentation
	cargo doc --open

doc-private: ## Generate documentation including private items
	cargo doc --document-private-items --open

# Coverage and benchmarking
coverage: ## Generate test coverage report
	cargo tarpaulin --out Html --output-dir coverage
	@echo "Coverage report generated in coverage/tarpaulin-report.html"

coverage-lcov: ## Generate LCOV coverage report
	cargo tarpaulin --out Lcov --output-dir coverage

bench: ## Run benchmarks
	cargo bench

# =============================================================================
# DOCUMENTATION TARGETS (FULLY AUTOMATED)
# =============================================================================

# Main documentation targets
docs: docs-validate docs-generate docs-consistency ## Generate and validate all documentation
	@echo "✅ All documentation generated and validated"

docs-generate: diagrams rust-docs docs-index ## Generate documentation artifacts
	@echo "📚 Documentation generated successfully"

docs-validate: validate-diagrams validate-doc-structure validate-doc-links ## Validate documentation structure
	@echo "✅ Documentation validation completed"

docs-consistency: check-code-doc-sync validate-adr-consistency ## Check code-doc synchronization
	@echo "✅ Documentation consistency verified"

docs-ci: clean-docs docs-generate docs-validate docs-consistency ## Full documentation CI pipeline
	@echo "🚀 Documentation CI pipeline completed"

# Diagram generation
diagrams: ## Generate architecture diagrams
	@echo "🎨 Generating architecture diagrams..."
	@bash scripts/docs/generate-diagrams.sh all

validate-diagrams: ## Validate PlantUML syntax
	@echo "🔍 Validating PlantUML diagrams..."
	@bash scripts/docs/generate-diagrams.sh validate

# Documentation structure validation
validate-doc-structure: ## Validate documentation structure
	@echo "📋 Validating documentation structure..."
	@bash scripts/docs/validate-structure.sh

validate-doc-links: ## Validate documentation links
	@echo "🔗 Validating documentation links..."
	@bash scripts/docs/validate-links.sh

# Code-documentation consistency
check-code-doc-sync: ## Check code-documentation synchronization
	@echo "🔄 Checking code-documentation synchronization..."
	@bash scripts/docs/check-sync.sh

validate-adr-consistency: ## Validate ADR consistency
	@echo "📋 Validating ADR consistency..."
	@bash scripts/docs/validate-adrs.sh

# Documentation generation
rust-docs: ## Generate Rust API documentation
	@echo "🦀 Generating Rust API documentation..."
	@cargo doc --no-deps --document-private-items

docs-index: ## Generate documentation index
	@echo "📖 Generating documentation index..."
	@bash scripts/docs/generate-index.sh

# ADR management
adr-list: ## List all ADRs
	@echo "📋 Architecture Decision Records:"
	@ls -1 docs/architecture/adr/ | grep -E '\.md$$' | sed 's/\.md$$//' | sort

adr-new: ## Create new ADR interactively
	@echo "📝 Creating new ADR..."
	@bash scripts/docs/create-adr.sh

# =============================================================================
# QUALITY ASSURANCE TARGETS
# =============================================================================

# Comprehensive quality checks
quality: fmt check lint test audit docs-validate ## Run all quality checks
	@echo "✅ Quality assurance completed"

# Validation (all quality checks)
validate: quality ## Run all validation checks
	@echo "✅ All validation checks passed!"

# CI/CD simulation
ci: quality coverage docs-ci ## Simulate complete CI pipeline
	@echo "🚀 CI pipeline completed successfully!"

# =============================================================================
# RELEASE TARGETS
# =============================================================================

# Release
release: test build-release ## Build release version
	@echo "Release build completed"

release-prep: clean quality build-release docs-generate package-release ## Prepare for release
	@echo "📦 Release preparation completed"

package-release: ## Package release
	@echo "📦 Packaging release..."
	@mkdir -p dist
	@cp target/release/mcp-context-browser dist/
	@cp docs/user-guide/README.md dist/README.md
	@cp LICENSE dist/
	@tar -czf dist/mcp-context-browser-$(shell grep '^version' Cargo.toml | cut -d'"' -f2).tar.gz -C dist .
	@echo "📦 Release package created: dist/mcp-context-browser-$(shell grep '^version' Cargo.toml | cut -d'"' -f2).tar.gz"

# Installation
install: ## Install the binary locally
	cargo install --path .

# =============================================================================
# DOCKER TARGETS
# =============================================================================

# Docker (if needed in future)
docker-build: ## Build Docker image
	docker build -t mcp-context-browser .

docker-run: ## Run Docker container
	docker run -it --rm mcp-context-browser

# =============================================================================
# MAINTENANCE TARGETS
# =============================================================================

# Cleanup
clean: ## Clean build artifacts
	cargo clean
	rm -rf coverage/
	rm -rf dist/

clean-docs: ## Clean documentation artifacts
	rm -rf docs/architecture/diagrams/generated/
	rm -rf target/doc/
	rm -rf docs/build/

clean-all: clean clean-docs ## Clean all artifacts
	rm -rf node_modules/
	rm -rf .cargo/

# Health checks
health-check: ## Run health checks
	@echo "🏥 Running health checks..."
	@cargo check
	@cargo test --no-run
	@bash scripts/docs/generate-diagrams.sh validate > /dev/null 2>&1 && echo "✅ Diagrams: OK" || echo "❌ Diagrams: FAILED"

# =============================================================================
# UTILITY TARGETS
# =============================================================================

# Update dependencies
update-deps: ## Update dependencies
	cargo update

# Run specific tests with output
test-verbose: ## Run tests with verbose output
	cargo test -- --nocapture

# Development server with logging
dev-logged: ## Run development server with debug logging
	RUST_LOG=debug cargo watch -x run

# Generate all artifacts
all-artifacts: build-release docs docker-build package-release ## Generate all artifacts
	@echo "🎯 All artifacts generated"

# Git operations
git-status: ## Show git repository status
	@echo "Git repository status:"
	@git status --short

git-add-all: ## Add all changes to git
	@echo "Adding all changes to git..."
	@git add -A
	@echo "All changes added"

git-commit-force: ## Force commit all changes
	@echo "Committing all changes with force..."
	@git commit --allow-empty -m "Force commit: $(shell date '+%Y-%m-%d %H:%M:%S') - Automated update" || echo "No changes to commit"

git-push-force: ## Force push to remote repository
	@echo "Pushing changes with force..."
	@git push --force-with-lease origin main || git push --force origin main
	@echo "Changes pushed successfully"

git-force-all: git-add-all git-commit-force git-push-force ## Add, commit and push all changes with force
	@echo "Force commit and push completed!"

# Alternative script-based force commit
force-commit: ## Run force commit script (alternative method)
	@echo "Running force commit script..."
	@bash scripts/force-commit.sh

# =============================================================================
# HELP SYSTEM
# =============================================================================

help: ## Show this help message
	@echo "MCP Context Browser - Complete Development & Documentation Makefile"
	@echo "=================================================================="
	@echo ""
	@echo "BUILD & DEVELOPMENT:"
	@echo "  build              - Build debug version"
	@echo "  build-release      - Build release version"
	@echo "  test               - Run all tests"
	@echo "  test-unit          - Run unit tests only"
	@echo "  test-integration   - Run integration tests only"
	@echo "  lint               - Format and lint code"
	@echo "  dev                - Run with auto-reload"
	@echo "  setup              - Setup development environment"
	@echo ""
	@echo "DOCUMENTATION (FULLY AUTOMATED):"
	@echo "  docs               - Generate and validate all documentation"
	@echo "  docs-generate      - Generate documentation artifacts"
	@echo "  docs-validate      - Validate documentation structure"
	@echo "  docs-consistency   - Check code-doc synchronization"
	@echo "  docs-ci            - Full documentation CI pipeline"
	@echo "  diagrams           - Generate architecture diagrams"
	@echo "  adr-new            - Create new ADR interactively"
	@echo ""
	@echo "QUALITY ASSURANCE:"
	@echo "  quality            - Run all quality checks"
	@echo "  validate           - Run all validation checks"
	@echo "  coverage           - Generate test coverage report"
	@echo "  audit              - Run security audit"
	@echo "  bench              - Run performance benchmarks"
	@echo ""
	@echo "CI/CD & RELEASE:"
	@echo "  ci                 - Simulate complete CI pipeline"
	@echo "  release-prep       - Prepare for release"
	@echo "  docker-build       - Build Docker image"
	@echo "  docker-run         - Run Docker container"
	@echo ""
	@echo "MAINTENANCE:"
	@echo "  clean              - Clean build artifacts"
	@echo "  clean-docs         - Clean documentation artifacts"
	@echo "  health-check       - Run health checks"
	@echo "  update-deps        - Update dependencies"
	@echo ""
	@echo "EXAMPLES:"
	@echo "  make build && make test          # Build and test"
	@echo "  make docs                        # Generate all docs"
	@echo "  make ci                          # Full CI pipeline"
	@echo "  make adr-new                     # Create new ADR"
	@echo "  make release-prep                # Prepare release"
	@echo ""
	@echo "For more information, see docs/README.md"