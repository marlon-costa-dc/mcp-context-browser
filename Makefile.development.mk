# =============================================================================
# DEVELOPMENT WORKFLOWS - Development server, setup, and workflows
# =============================================================================

.PHONY: dev dev-metrics dev-sync setup check ci dev-cycle dev-ready dev-deploy

# Development server
dev: ## Run development server
	cargo watch -x run

dev-metrics: ## Development with metrics
	@echo "🚀 Starting development server with metrics..."
	cargo watch -x "run -- --metrics"

dev-sync: ## Development with sync testing
	@echo "🔄 Starting development with sync features..."
	cargo watch -x "run -- --sync-test"

# Development setup
setup: ## Setup development tools (MANDATORY)
	cargo install cargo-watch
	cargo install cargo-tarpaulin
	cargo install cargo-audit
	@echo "📦 Installing markdownlint-cli (required for markdown linting)..."
	@if ! command -v npm >/dev/null 2>&1; then \
		echo "❌ ERROR: npm required for markdownlint-cli installation"; \
		echo "Install Node.js and npm first: https://nodejs.org/"; \
		exit 1; \
	fi
	@if ! npm install -g markdownlint-cli; then \
		echo "❌ ERROR: Failed to install markdownlint-cli"; \
		echo "Check npm permissions or install manually: npm install -g markdownlint-cli"; \
		exit 1; \
	fi
	@if ! command -v markdownlint >/dev/null 2>&1; then \
		echo "❌ ERROR: markdownlint-cli not found after installation"; \
		exit 1; \
	fi
	@echo "✅ Development environment ready with full markdown linting"

# Development workflows
check: build test ## Build and test (basic validation)

ci: clean validate test build docs ## Run full CI pipeline

# Development iteration cycles
dev-cycle: fix test-quiet ## Development iteration: fix + test

dev-ready: dev-cycle quality ## Development ready: iteration + quality

dev-deploy: dev-ready version-all github-release ## Development deploy: ready + version + release