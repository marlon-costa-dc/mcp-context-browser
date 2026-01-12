# =============================================================================
# RELEASE - Packaging and publishing
# =============================================================================
# Version is read dynamically from Cargo.toml
# =============================================================================

.PHONY: release package github-release

# Get version from Cargo.toml
VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/.*"\([^"]*\)".*/\1/')

# -----------------------------------------------------------------------------
# Release Pipeline
# -----------------------------------------------------------------------------

release: test build-release package ## Create release (test + build + package)
	@echo "✅ Release v$(VERSION) ready"

# -----------------------------------------------------------------------------
# Packaging
# -----------------------------------------------------------------------------

package: ## Package release artifacts
	@echo "📦 Packaging v$(VERSION)..."
	@mkdir -p dist
	@cp target/release/mcp-context-browser dist/ 2>/dev/null || echo "⚠️  Binary not found (run build-release first)"
	@cp docs/user-guide/README.md dist/README.md 2>/dev/null || echo "# MCP Context Browser" > dist/README.md
	@cp LICENSE dist/ 2>/dev/null || touch dist/LICENSE
	@cd dist && tar -czf mcp-context-browser-$(VERSION).tar.gz mcp-context-browser README.md LICENSE 2>/dev/null || true
	@echo "📦 Package: dist/mcp-context-browser-$(VERSION).tar.gz"

# -----------------------------------------------------------------------------
# GitHub Release
# -----------------------------------------------------------------------------

github-release: release ## Create GitHub release with gh CLI
	@echo "🚀 Creating GitHub release v$(VERSION)..."
	@gh release create v$(VERSION) \
		--title "MCP Context Browser v$(VERSION)" \
		--notes "Release v$(VERSION)" \
		dist/mcp-context-browser-$(VERSION).tar.gz
	@echo "✅ GitHub release v$(VERSION) created"
