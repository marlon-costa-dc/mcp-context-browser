# MCP Context Browser - Development Makefile

.PHONY: help build test clean fmt check lint doc release install dev setup audit coverage bench git-status git-add-all git-commit-force git-push-force git-force-all force-commit

# Default target
help: ## Show this help message
	@echo "MCP Context Browser - Development Commands"
	@echo ""
	@echo "Available commands:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

# Development setup
setup: ## Setup development environment
	cargo install cargo-audit
	cargo install cargo-tarpaulin
	cargo install cargo-watch

# Development commands
dev: ## Run development server with auto-reload
	cargo watch -x run

build: ## Build the project
	cargo build

build-release: ## Build the project in release mode
	cargo build --release

test: ## Run all tests
	cargo test

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

# Documentation
doc: ## Generate documentation
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

# Validation (all quality checks)
validate: fmt check lint test audit ## Run all validation checks
	@echo "✅ All validation checks passed!"

# Release
release: test build-release ## Build release version
	@echo "Release build completed"

# Installation
install: ## Install the binary locally
	cargo install --path .

# Cleanup
clean: ## Clean build artifacts
	cargo clean
	rm -rf coverage/

# Docker (if needed in future)
docker-build: ## Build Docker image
	docker build -t mcp-context-browser .

docker-run: ## Run Docker container
	docker run -it --rm mcp-context-browser

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

# CI simulation
ci: validate coverage ## Simulate CI pipeline locally
	@echo "CI simulation completed successfully!"