# =============================================================================
# DOCUMENTATION COMMANDS - Documentation generation and ADR management
# =============================================================================

.PHONY: docs adr-new adr-list diagrams

# Main documentation generation
docs: ## Generate all documentation
	@echo "🎨 Generating diagrams..."
	@bash scripts/docs/generate-diagrams.sh all
	@echo "🦀 Generating Rust docs..."
	@cargo doc --no-deps --document-private-items
	@echo "📖 Generating docs index..."
	@bash scripts/docs/generate-index.sh
	@echo "✅ Documentation generated"

# ADR (Architecture Decision Records) management
adr-new: ## Create new ADR
	@bash scripts/docs/create-adr.sh

adr-list: ## List ADRs
	@echo "📋 ADRs:"
	@ls -1 docs/architecture/adr/ | grep -E '\.md$$' | sed 's/\.md$$//' | sort

# Diagram generation
diagrams: ## Generate diagrams only
	@bash scripts/docs/generate-diagrams.sh all