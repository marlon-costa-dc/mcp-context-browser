# =============================================================================
# GIT - Streamlined git operations
# =============================================================================
# Naming: short action verbs
# =============================================================================

.PHONY: status commit push tag sync

# -----------------------------------------------------------------------------
# Essential Git Commands
# -----------------------------------------------------------------------------

status: ## Show git status
	@echo "📊 Git Status:"
	@git status --short --branch

commit: ## Commit all changes (interactive message)
	@git add -A
	@git commit || echo "Nothing to commit"

push: ## Push to remote
	@git push origin $$(git branch --show-current)
	@echo "✅ Pushed to origin"

tag: ## Create and push version tag from Cargo.toml
	@VERSION=$$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\([^"]*\)".*/\1/'); \
	echo "🏷️  Tagging v$$VERSION..."; \
	git tag -a "v$$VERSION" -m "Release v$$VERSION" 2>/dev/null || echo "Tag v$$VERSION already exists"; \
	git push origin "v$$VERSION" 2>/dev/null || echo "Tag already pushed"

# -----------------------------------------------------------------------------
# Combined Operations
# -----------------------------------------------------------------------------

sync: ## Add, commit (auto-message), and push all changes
	@echo "🔄 Syncing changes..."
	@git add -A
	@git commit -m "chore: sync changes $$(date '+%Y-%m-%d %H:%M')" --allow-empty 2>/dev/null || true
	@git push origin $$(git branch --show-current) || git push --set-upstream origin $$(git branch --show-current)
	@echo "✅ Synced to remote"

