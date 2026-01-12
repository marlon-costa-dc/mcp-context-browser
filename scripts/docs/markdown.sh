#!/bin/bash
# =============================================================================
# MCP Context Browser - Unified Markdown Operations
# =============================================================================
# Combined lint and fix operations for markdown files
# Usage: ./markdown.sh [lint|fix|check] [--dry-run]
# =============================================================================

set -e

# Source shared library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/common.sh"

# Fix counter
FIXED=0

# =============================================================================
# Shared Markdown Checks
# =============================================================================

# Check for trailing whitespace
check_trailing_whitespace() {
    local file="$1"
    has_trailing_whitespace "$file"
}

# Check for multiple consecutive blank lines
check_multiple_blanks() {
    local file="$1"
    grep -qP '\n\n\n' "$file" 2>/dev/null
}

# Check for mixed list markers
check_mixed_lists() {
    local file="$1"
    grep -q '^[[:space:]]*\*[[:space:]]' "$file" && grep -q '^[[:space:]]*-[[:space:]]' "$file"
}

# Check for code blocks without language
check_unlabeled_codeblocks() {
    local file="$1"
    grep -q '^```$' "$file"
}

# =============================================================================
# Lint Mode
# =============================================================================

lint_mode() {
    log_info "MCP Context Browser - Markdown Linting"
    log_info "======================================"

    # Check for markdownlint-cli
    if check_executable markdownlint; then
        log_info "Using markdownlint-cli for comprehensive linting..."
        local config_file="$PROJECT_ROOT/.markdownlint.json"
        local args=("$DOCS_DIR/")
        [[ -f "$config_file" ]] && args+=(--config "$config_file")

        if markdownlint "${args[@]}"; then
            log_success "Markdown linting passed"
        else
            log_error "Markdown linting failed"
            exit 1
        fi
    else
        log_warning "markdownlint-cli not found, using fallback linting"
        lint_fallback
    fi
}

lint_fallback() {
    local files
    files=$(find_markdown_files "$DOCS_DIR")

    for file in $files; do
        local filename
        filename=$(basename "$file")

        check_trailing_whitespace "$file" && { log_error "Trailing whitespace in $filename"; inc_errors; }
        check_multiple_blanks "$file" && { log_warning "Multiple blank lines in $filename"; inc_warnings; }
        check_mixed_lists "$file" && { log_warning "Mixed list markers in $filename"; inc_warnings; }
        check_unlabeled_codeblocks "$file" && { log_warning "Code blocks without language in $filename"; inc_warnings; }
    done

    echo
    log_info "Linting Summary (Fallback Mode):"
    echo "  Errors: $(get_errors)"
    echo "  Warnings: $(get_warnings)"

    [[ $(get_errors) -gt 0 ]] && { log_error "Found issues. Run './markdown.sh fix' to auto-fix."; exit 1; }
    [[ $(get_warnings) -gt 0 ]] && log_warning "Found warnings. Consider running './markdown.sh fix'."
    log_success "No critical issues found."
}

# =============================================================================
# Fix Mode
# =============================================================================

fix_mode() {
    log_info "MCP Context Browser - Markdown Auto-Fix"
    log_info "======================================="

    is_dry_run && log_info "Running in dry-run mode (no changes will be made)"

    local files
    files=$(find_markdown_files "$DOCS_DIR")

    for file in $files; do
        local filename
        filename=$(basename "$file")

        # Fix trailing whitespace
        if check_trailing_whitespace "$file"; then
            log_info "Fixing trailing whitespace in: $filename"
            run_or_echo sed -i 's/[[:space:]]*$//' "$file"
            ((FIXED++)) || true
        fi

        # Fix multiple blank lines
        if check_multiple_blanks "$file"; then
            log_info "Fixing multiple blank lines in: $filename"
            if ! is_dry_run; then
                awk 'BEGIN{RS=""} {gsub(/\n\n+/,"\n\n"); print $0 "\n"}' "$file" > "${file}.tmp" && mv "${file}.tmp" "$file"
            else
                echo "[DRY-RUN] Would fix multiple blank lines in: $filename"
            fi
            ((FIXED++)) || true
        fi

        # Fix list markers (asterisks to dashes)
        if grep -q '^[[:space:]]*\*[[:space:]]' "$file"; then
            log_info "Converting asterisks to dashes in: $filename"
            run_or_echo sed -i 's/^[[:space:]]*\*[[:space:]]/  - /g' "$file"
            ((FIXED++)) || true
        fi

        # Warn about code blocks (can't auto-fix)
        if check_unlabeled_codeblocks "$file"; then
            log_warning "Found code blocks without language tags in: $filename (manual fix needed)"
        fi
    done

    echo
    log_info "Auto-fix Summary:"
    echo "  Issues fixed: $FIXED"

    [[ $FIXED -gt 0 ]] && log_success "Auto-fix completed. Run './markdown.sh lint' to verify."
    [[ $FIXED -eq 0 ]] && log_success "No auto-fixable issues found."
}

# =============================================================================
# Main
# =============================================================================

show_usage() {
    cat << EOF
MCP Context Browser - Unified Markdown Tool

USAGE:
    $0 lint              # Check markdown files for issues
    $0 fix               # Auto-fix markdown issues
    $0 fix --dry-run     # Show what would be fixed
    $0 check             # Alias for lint

EXAMPLES:
    $0 lint              # Run linting
    $0 fix --dry-run     # Preview fixes
    $0 fix               # Apply fixes

EOF
}

main() {
    local command="${1:-lint}"

    # Handle --dry-run flag
    if [[ "$command" == "--dry-run" ]]; then
        DRY_RUN=true
        command="${2:-lint}"
    elif [[ "${2:-}" == "--dry-run" ]]; then
        DRY_RUN=true
    fi

    case "$command" in
        lint|check)
            lint_mode
            ;;
        fix)
            fix_mode
            ;;
        help|--help|-h)
            show_usage
            ;;
        *)
            log_error "Unknown command: $command"
            show_usage
            exit 1
            ;;
    esac
}

main "$@"
