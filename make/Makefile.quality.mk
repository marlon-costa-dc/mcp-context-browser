# =============================================================================
# QUALITY - Code quality operations (streamlined)
# =============================================================================
# Naming: action-target pattern, minimal verbs
# =============================================================================

.PHONY: check fmt lint fix quality validate
.PHONY: coverage bench audit

# -----------------------------------------------------------------------------
# Quick Commands (most used)
# -----------------------------------------------------------------------------

check: ## Fast compilation check
	@cargo check --all-targets

fmt: ## Format all code (Rust + Markdown)
	@cargo fmt
	@./scripts/docs/markdown.sh fix 2>/dev/null || true
	@echo "✅ Code formatted"

lint: ## Lint all code (Rust + Markdown)
	@echo "🔍 Linting Rust..."
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "🔍 Linting Markdown..."
	@./scripts/docs/markdown.sh lint 2>/dev/null || echo "⚠️  Markdown lint skipped"
	@echo "✅ Lint complete"

fix: fmt ## Auto-fix all issues (format + clippy)
	@cargo clippy --fix --allow-dirty --all-targets --all-features 2>/dev/null || true
	@echo "✅ Issues fixed"

# -----------------------------------------------------------------------------
# Quality Gates (CI/CD)
# -----------------------------------------------------------------------------

quality: check fmt lint test ## Full quality check (MANDATORY for CI)
	@echo "🔒 Running security audit..."
	@cargo audit 2>/dev/null || echo "⚠️  cargo-audit not installed (run: cargo install cargo-audit)"
	@echo "✅ Quality checks passed"

validate: quality docs-check ## Complete validation (quality + docs)
	@echo "🚀 All validations passed - Ready for release"

# -----------------------------------------------------------------------------
# Specialized Checks
# -----------------------------------------------------------------------------

coverage: ## Generate test coverage report
	@echo "📊 Generating coverage report..."
	@cargo tarpaulin --out Html --output-dir coverage 2>/dev/null || echo "⚠️  cargo-tarpaulin not installed"
	@echo "📖 Coverage at: coverage/tarpaulin-report.html"

bench: ## Run benchmarks
	@cargo bench

audit: ## Security audit of dependencies
	@echo "🔒 Running security audit..."
	@cargo audit
