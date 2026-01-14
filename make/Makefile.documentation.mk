# =============================================================================
# DOCUMENTATION - Streamlined documentation automation
# =============================================================================
# Core scripts use shared library: scripts/docs/lib/common.sh
# Single source of truth: scripts/docs/extract-metrics.sh
# =============================================================================

.PHONY: docs docs-build docs-serve docs-check docs-fix docs-setup docs-sync docs-metrics
.PHONY: adr-new adr-list adr-check
.PHONY: lint-md fix-md validate-all pre-commit
.PHONY: rust-docs doc module-docs diagrams info

# Path to mdbook (uses cargo-installed binary)
MDBOOK := $(HOME)/.cargo/bin/mdbook

# -----------------------------------------------------------------------------
# Main Documentation Commands
# -----------------------------------------------------------------------------

docs: docs-metrics docs-sync docs-build rust-docs docs-check ## Build and validate all documentation
	@echo "✅ Documentation complete"

docs-metrics: ## Update all docs with metrics from codebase (single source of truth)
	@echo "📊 Injecting metrics into documentation..."
	@./scripts/docs/inject-metrics.sh 2>/dev/null || echo "⚠️  inject-metrics.sh not available"
	@echo "✅ Metrics updated"

docs-sync: ## Sync docs/ to book/src/ for mdbook
	@echo "🔄 Syncing documentation..."
	@./scripts/docs/mdbook-sync.sh 2>/dev/null || echo "⚠️  mdbook-sync.sh not found"
	@echo "✅ Documentation synced"

docs-build: ## Build mdbook documentation
	@echo "📚 Building documentation..."
	@if [ -x "$(MDBOOK)" ]; then \
		$(MDBOOK) build book/; \
		echo "📖 Documentation at: book/book/index.html"; \
	else \
		echo "⚠️  mdbook not installed. Run: make docs-setup"; \
	fi

docs-serve: docs-sync ## Serve interactive docs at http://localhost:3000
	@echo "🌐 Starting documentation server..."
	@if [ -x "$(MDBOOK)" ]; then \
		$(MDBOOK) serve book/ --open; \
	else \
		echo "⚠️  mdbook not installed. Run: make docs-setup"; \
	fi

docs-check: ## Validate documentation (unified validation)
	@echo "🔍 Validating documentation..."
	@./scripts/docs/validate.sh all
	@echo "✅ Documentation validation complete"

docs-fix: ## Auto-fix markdown formatting issues
	@echo "🔧 Fixing markdown issues..."
	@./scripts/docs/markdown.sh fix
	@echo "✅ Markdown fixed"

docs-setup: ## Install documentation tools (mdbook + plugins)
	@echo "🔧 Setting up documentation tools..."
	@cargo install mdbook 2>/dev/null || echo "mdbook already installed"
	@cargo install mdbook-linkcheck 2>/dev/null || echo "mdbook-linkcheck already installed"
	@cargo install mdbook-toc 2>/dev/null || echo "mdbook-toc already installed"
	@npm install -g markdownlint-cli 2>/dev/null || echo "⚠️  markdownlint-cli requires npm"
	@echo "✅ Documentation tools ready"

# -----------------------------------------------------------------------------
# ADR (Architecture Decision Records)
# -----------------------------------------------------------------------------

adr-new: ## Create new ADR interactively
	@./scripts/docs/create-adr.sh

adr-list: ## List all ADRs with numbers and titles
	@echo "📋 Architecture Decision Records:"
	@ls -1 docs/adr/[0-9]*.md 2>/dev/null | while read f; do \
		num=$$(basename "$$f" .md | cut -d- -f1); \
		title=$$(head -1 "$$f" | sed 's/^# ADR [0-9]*: //'); \
		printf "  %s: %s\n" "$$num" "$$title"; \
	done

adr-check: ## Validate ADR compliance (format, numbering, references)
	@echo "📋 Checking ADR compliance..."
	@./scripts/docs/validate.sh adrs

# -----------------------------------------------------------------------------
# Markdown Operations (unified script)
# -----------------------------------------------------------------------------

lint-md: ## Lint markdown files
	@./scripts/docs/markdown.sh lint

fix-md: ## Fix markdown issues (auto-fix)
	@./scripts/docs/markdown.sh fix

# -----------------------------------------------------------------------------
# Unified Validation
# -----------------------------------------------------------------------------

validate-all: ## Run all documentation validations
	@./scripts/docs/validate.sh all

pre-commit: lint-md validate-all ## Run all pre-commit documentation checks
	@echo "✅ Pre-commit documentation checks passed"

# -----------------------------------------------------------------------------
# Specialized Documentation
# -----------------------------------------------------------------------------

rust-docs: ## Generate Rust API documentation (docs.rs compatible)
	@echo "🦀 Generating Rust docs..."
	@cargo doc \
		--no-deps \
		--all-features \
		--document-private-items
	@echo "📖 Docs at: target/doc/mcp_context_browser/index.html"

doc: rust-docs ## Alias: Generate documentation (standard verb)

module-docs: ## Generate module documentation from source analysis
	@echo "📄 Generating module documentation..."
	@./scripts/docs/generate-module-docs.sh
	@echo "✅ Module docs generated in docs/modules/"

diagrams: ## Generate architecture diagrams (PlantUML)
	@echo "📊 Generating diagrams..."
	@./scripts/docs/generate-diagrams.sh all 2>/dev/null || echo "⚠️  Diagram generation requires PlantUML"

# -----------------------------------------------------------------------------
# Project Info (quick reference)
# -----------------------------------------------------------------------------

info: ## Display current project metrics and stats
	@./scripts/docs/extract-metrics.sh --markdown
