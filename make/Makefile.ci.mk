# =============================================================================
# CI - Continuous Integration Pipeline (Single Source of Truth)
# =============================================================================
# All CI logic lives here. GitHub Actions simply calls these targets.
# Usage: make ci-<target>
# =============================================================================

.PHONY: ci ci-quick ci-full
.PHONY: ci-test ci-lint ci-fmt ci-clippy ci-security ci-docs ci-build ci-coverage
.PHONY: ci-setup ci-setup-tools ci-setup-docs

# =============================================================================
# Main CI Targets
# =============================================================================

ci: ci-lint ci-test ## Standard CI pipeline (lint + test)
	@echo "✅ CI passed"

ci-quick: ci-fmt ci-clippy ## Quick CI check (format + clippy only)
	@echo "✅ Quick CI passed"

ci-full: ci-lint ci-test ci-security ci-docs ci-build ## Full CI pipeline
	@echo "✅ Full CI passed"

# =============================================================================
# Individual CI Jobs
# =============================================================================

ci-test: ## Run all tests with all features
	@echo "🧪 [CI] Running tests..."
	@cargo test --all-targets --all-features --verbose
	@echo "✅ Tests passed"

ci-fmt: ## Check code formatting (fails if not formatted)
	@echo "📝 [CI] Checking formatting..."
	@cargo fmt --all -- --check
	@echo "✅ Format check passed"

ci-clippy: ## Run clippy with strict warnings
	@echo "🔍 [CI] Running clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "✅ Clippy passed"

ci-lint: ci-fmt ci-clippy ## Combined lint check (format + clippy)
	@echo "✅ Lint passed"

ci-security: ## Security audit of dependencies
	@echo "🔒 [CI] Running security audit..."
	@cargo audit
	@echo "✅ Security audit passed"

ci-docs: ## Validate all documentation
	@echo "📚 [CI] Validating documentation..."
	@./scripts/docs/validate.sh all || echo "⚠️  Documentation has issues (non-blocking)"
	@echo "✅ Documentation validation complete"

ci-build: ## Build release binary
	@echo "🔨 [CI] Building release..."
	@cargo build --release
	@echo "✅ Build complete"

ci-build-target: ## Build for specific target (use TARGET=x86_64-unknown-linux-gnu)
	@echo "🔨 [CI] Building for $(TARGET)..."
	@cargo build --release --target $(TARGET)
	@echo "✅ Build complete for $(TARGET)"

ci-coverage: ## Generate test coverage report
	@echo "📊 [CI] Generating coverage report..."
	@cargo tarpaulin --out Lcov --output-dir coverage
	@echo "✅ Coverage report at coverage/lcov.info"

ci-rust-docs: ## Generate Rust API documentation
	@echo "📖 [CI] Generating Rust docs..."
	@cargo doc --no-deps --document-private-items
	@echo "✅ Rust docs generated"

# =============================================================================
# CI Setup (for fresh CI runners)
# =============================================================================

ci-setup: ci-setup-tools ## Setup CI environment
	@echo "✅ CI environment ready"

ci-setup-tools: ## Install CI tools (cargo-audit, cargo-tarpaulin)
	@echo "📦 [CI] Installing tools..."
	@cargo install cargo-audit --locked 2>/dev/null || true
	@cargo install cargo-tarpaulin --locked 2>/dev/null || true
	@echo "✅ Tools installed"

ci-setup-docs: ## Install documentation tools
	@echo "📦 [CI] Installing documentation tools..."
	@cargo install mdbook --locked 2>/dev/null || true
	@cargo install mdbook-linkcheck --locked 2>/dev/null || true
	@echo "✅ Documentation tools installed"
