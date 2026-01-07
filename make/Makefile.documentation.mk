# =============================================================================
# DOCUMENTATION - Geração de documentação e gerenciamento ADR
# =============================================================================

.PHONY: docs docs-auto docs-manual module-docs api-docs status-docs sync-docs sync-docs-update rust-docs index-docs adr-new adr-list diagrams

# Main documentation generation
docs: docs-auto docs-manual ## Generate all documentation (auto + manual)
	@echo "🤖 Generating auto documentation..."
	@make docs-auto
	@echo "📝 Generating manual documentation..."
	@make docs-manual
	@echo "✅ All documentation generated"

# Auto-generated documentation from source code
docs-auto: module-docs api-docs status-docs ## Generate automated documentation from source code
	@echo "📊 Auto-generated docs updated"

# Manual documentation generation
docs-manual: diagrams rust-docs index-docs ## Generate manually maintained documentation
	@echo "📖 Manual docs updated"

# Module documentation
module-docs: ## Generate module documentation from source code
	@bash scripts/docs/generate-module-docs.sh

# API reference
api-docs: ## Generate API reference documentation
	@bash scripts/docs/generate-module-docs.sh
	@echo "📋 API reference generated"

# Implementation status
status-docs: ## Generate implementation status documentation
	@bash scripts/docs/generate-module-docs.sh
	@echo "📊 Implementation status generated"

# Documentation synchronization
sync-docs: ## Check if documentation is synchronized with code
	@bash scripts/docs/sync-docs.sh

sync-docs-update: ## Check documentation sync and update auto-generated docs
	@bash scripts/docs/sync-docs.sh --update

# Rust documentation
rust-docs: ## Generate Rust API documentation
	@echo "🦀 Generating Rust docs..."
	@cargo doc --no-deps --document-private-items

# Index generation
index-docs: ## Generate documentation index
	@echo "📖 Generating docs index..."
	@bash scripts/docs/generate-index.sh

# ADR management (Architecture Decision Records)
adr-new: ## Create new ADR using professional adrs tool
	@echo "📝 Creating new ADR..."
	@~/.cargo/bin/adrs new

adr-list: ## List ADRs using professional adrs tool
	@echo "📋 ADRs:"
	@~/.cargo/bin/adrs list

adr-generate: ## Generate ADR summary documentation
	@echo "📊 Generating ADR summary..."
	@~/.cargo/bin/adrs generate toc > docs/adr/README.md
	@~/.cargo/bin/adrs generate graph > docs/adr/adr-graph.md

adr-status: ## Show ADR status and lifecycle
	@echo "📈 ADR Status:"
	@~/.cargo/bin/adrs list --status

# Diagram generation
diagrams: ## Generate diagrams only
	@bash scripts/docs/generate-diagrams.sh all