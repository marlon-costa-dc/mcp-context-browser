# =============================================================================
# CORE - Build, test and clean operations
# =============================================================================

.PHONY: build build-release test test-unit test-integration clean run

# -----------------------------------------------------------------------------
# Build
# -----------------------------------------------------------------------------

build: ## Build project (debug mode)
	@cargo build

build-release: ## Build project (release mode)
	@cargo build --release

# -----------------------------------------------------------------------------
# Test
# -----------------------------------------------------------------------------

test: ## Run all tests
	@cargo test --all-targets --all-features

test-unit: ## Run unit tests only
	@cargo test --lib --all-features

test-integration: ## Run integration tests only
	@cargo test --test '*'

# -----------------------------------------------------------------------------
# Run
# -----------------------------------------------------------------------------

run: ## Build and run the server
	@cargo run

# -----------------------------------------------------------------------------
# Clean
# -----------------------------------------------------------------------------

clean: ## Clean all build artifacts
	@echo "🧹 Cleaning..."
	@cargo clean
	@rm -rf docs/generated/ docs/build/ coverage/ dist/
	@echo "✅ Clean complete"
