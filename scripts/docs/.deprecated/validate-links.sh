#!/bin/bash
# =============================================================================
# MCP Context Browser - Documentation Link Validation
# =============================================================================
# Validates that all internal documentation links are working
# =============================================================================

set -e

# Source shared library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/common.sh"

# Extract links from markdown file
extract_links() {
    local file="$1"
    # Extract markdown links: [text](link) that point to docs/
    grep -o '\[.*\](\([^)]*\))' "$file" 2>/dev/null | sed 's/.*(\([^)]*\))/\1/' | grep '^docs/' || true
}

# Validate internal link
validate_link() {
    local link="$1"
    local source_file="$2"

    # Remove fragment identifiers (#anchor)
    local clean_link="${link%%#*}"

    # Check if link exists
    if [[ -f "$PROJECT_ROOT/$clean_link" ]]; then
        return 0
    elif [[ -d "$PROJECT_ROOT/$clean_link" ]]; then
        # Check for index file in directory
        if [[ -f "$PROJECT_ROOT/$clean_link/README.md" ]] || [[ -f "$PROJECT_ROOT/$clean_link/index.md" ]]; then
            return 0
        fi
    fi

    return 1
}

# Validate all documentation links
validate_doc_links() {
    log_info "Validating documentation links..."

    local doc_files
    doc_files=$(find_markdown_files "$DOCS_DIR")

    for doc_file in $doc_files; do
        local filename
        filename=$(basename "$doc_file")
        log_info "Checking links in: $filename"

        # Extract internal documentation links
        local links
        links=$(extract_links "$doc_file")

        for link in $links; do
            if ! validate_link "$link" "$doc_file"; then
                log_error "Broken link in $filename: $link"
                inc_errors
            fi
        done
    done
}

# Validate cross-references between documentation
validate_cross_references() {
    log_info "Validating cross-references..."

    local key_files=(
        "docs/README.md"
        "docs/user-guide/README.md"
        "docs/architecture/ARCHITECTURE.md"
        "docs/operations/DEPLOYMENT.md"
    )

    for file in "${key_files[@]}"; do
        if [[ -f "$PROJECT_ROOT/$file" ]]; then
            local filename
            filename=$(basename "$file")

            # Check that main docs index references this file
            if ! grep -q "$filename" "$PROJECT_ROOT/docs/README.md" 2>/dev/null; then
                log_warning "Main docs index may not reference: $filename"
                inc_warnings
            fi
        fi
    done
}

# Check for external link validity (basic check)
validate_external_links() {
    log_info "Checking external links (basic validation)..."

    local doc_files
    doc_files=$(find_markdown_files "$DOCS_DIR")

    for doc_file in $doc_files; do
        # Look for http/https links
        local external_links
        external_links=$(grep -o 'https*://[^)]*' "$doc_file" 2>/dev/null || true)

        for link in $external_links; do
            # Skip localhost/development URLs
            if [[ "$link" =~ localhost ]] || [[ "$link" =~ 127\.0\.0\.1 ]] || [[ "$link" =~ example\.com ]]; then
                continue
            fi

            # Basic HTTP status check (requires curl) - WARNING ONLY for external links
            if check_executable curl; then
                local status
                status=$(curl -s -o /dev/null -w "%{http_code}" --max-time 5 "$link" 2>/dev/null || echo "000")
                if [[ "$status" =~ ^[45][0-9][0-9]$ ]]; then
                    log_warning "External link may be broken: $link (status: $status)"
                fi
            fi
        done
    done
}

# Main execution
main() {
    log_info "MCP Context Browser - Documentation Link Validation"
    log_info "==================================================="

    validate_doc_links
    validate_cross_references

    # Only run external link check if curl is available
    if check_executable curl; then
        validate_external_links
    else
        log_warning "curl not available - skipping external link validation"
    fi

    exit_with_summary "Link Validation Summary"
}

main "$@"
