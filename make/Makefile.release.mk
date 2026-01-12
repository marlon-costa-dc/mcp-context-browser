# =============================================================================
# RELEASE - Packaging, publishing and installation
# =============================================================================
# Version is read dynamically from Cargo.toml
# =============================================================================

.PHONY: release package github-release install install-debug uninstall

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

# -----------------------------------------------------------------------------
# Installation (systemd user service)
# -----------------------------------------------------------------------------

INSTALL_DIR := $(HOME)/.claude/servers/claude-context-mcp
SERVICE_NAME := claude-context-daemon.service

install: build-release ## Install release binary and restart service
	@echo "📦 Installing MCP Context Browser v$(VERSION)..."
	@mkdir -p $(INSTALL_DIR)
	@echo "🛑 Stopping service..."
	@systemctl --user stop $(SERVICE_NAME) 2>/dev/null || true
	@echo "📋 Copying binary..."
	@cp target/release/mcp-context-browser $(INSTALL_DIR)/mcp-context-browser
	@chmod +x $(INSTALL_DIR)/mcp-context-browser
	@echo "🚀 Starting service..."
	@systemctl --user start $(SERVICE_NAME) 2>/dev/null || echo "⚠️  Service not enabled. Run: systemctl --user enable --now $(SERVICE_NAME)"
	@echo "✅ Installed v$(VERSION) to $(INSTALL_DIR)"

install-debug: build ## Install debug binary (faster build, for testing)
	@echo "📦 Installing MCP Context Browser v$(VERSION) (debug)..."
	@mkdir -p $(INSTALL_DIR)
	@echo "🛑 Stopping service..."
	@systemctl --user stop $(SERVICE_NAME) 2>/dev/null || true
	@echo "📋 Copying binary..."
	@cp target/debug/mcp-context-browser $(INSTALL_DIR)/mcp-context-browser
	@chmod +x $(INSTALL_DIR)/mcp-context-browser
	@echo "🚀 Starting service..."
	@systemctl --user start $(SERVICE_NAME) 2>/dev/null || echo "⚠️  Service not enabled. Run: systemctl --user enable --now $(SERVICE_NAME)"
	@echo "✅ Installed v$(VERSION) (debug) to $(INSTALL_DIR)"

uninstall: ## Stop service and remove installed binary
	@echo "🗑️  Uninstalling MCP Context Browser..."
	@systemctl --user stop $(SERVICE_NAME) 2>/dev/null || true
	@systemctl --user disable $(SERVICE_NAME) 2>/dev/null || true
	@rm -f $(INSTALL_DIR)/mcp-context-browser
	@echo "✅ Uninstalled"
