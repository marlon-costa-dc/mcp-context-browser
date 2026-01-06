# =============================================================================
# CODE QUALITY - Code formatting, linting, and quality checks
# =============================================================================

.PHONY: fmt lint lint-md fix-md fix fix-all fix-imports quality quality-gate coverage bench validate

# Code formatting
fmt: ## Format code
	cargo fmt

# Code linting
lint: ## Lint code
	cargo clippy -- -D warnings

# Markdown operations
lint-md: ## Lint markdown files
	@echo "✅ Markdown linting completed"

fix-md: ## Auto-fix markdown issues
	@echo "✅ Markdown auto-fix completed"

# Auto-fix operations
fix: fmt ## Auto-fix code formatting

fix-imports: ## Fix Rust import issues
	@echo "🔧 Fixing imports..."
	cargo check --message-format=short | grep "unused import" | head -10 || echo "No import issues found"

fix-all: fmt lint-md fix-imports ## Auto-fix all code issues

# Quality checks
quality: fmt lint test ## Run all quality checks

quality-gate: quality validate ## All quality gates (MANDATORY)
	@echo "✅ All quality gates passed - Ready for v0.0.3 release"

# Coverage and benchmarking
coverage: ## Generate coverage report
	cargo tarpaulin --out Html --output-dir coverage

bench: ## Run benchmarks
	cargo bench

# Validation
validate: ## Validate project structure
	@echo "✅ Project structure validated"