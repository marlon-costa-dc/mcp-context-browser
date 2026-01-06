# =============================================================================
# CORE DEVELOPMENT - Basic build, test and clean operations
# =============================================================================

.PHONY: build test clean clean-target clean-docs clean-deep

# Core build operations
build: ## Build project in debug mode
	cargo build

# Core testing
test: ## Run all tests
	cargo test

test-quiet: ## Run tests quietly
	cargo test --quiet

# Core cleaning operations
clean: ## Clean everything
	cargo clean
	rm -rf docs/architecture/diagrams/generated/
	rm -rf target/doc/
	rm -rf docs/build/
	rm -rf coverage/
	rm -rf dist/

clean-target: ## Clean target directory
	@echo "🧹 Cleaning target directory..."
	rm -rf target/

clean-docs: ## Clean documentation artifacts
	@echo "🧹 Cleaning documentation..."
	rm -rf docs/architecture/diagrams/generated/
	rm -rf docs/*/index.html docs/index.html

clean-deep: clean clean-docs clean-target ## Deep clean all artifacts