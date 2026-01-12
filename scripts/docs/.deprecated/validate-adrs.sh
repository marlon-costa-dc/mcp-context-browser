#!/bin/bash
# =============================================================================
# MCP Context Browser - ADR Validation
# =============================================================================
# Validates Architecture Decision Records format and consistency
# =============================================================================

set -e

# Source shared library
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/lib/common.sh"

# Validate ADR format
validate_adr_format() {
    local adr_file="$1"
    local filename
    filename=$(basename "$adr_file")

    log_info "Validating ADR: $filename"

    # Check filename format (should be NNN-title.md where NNN is 3 digits)
    if ! is_adr_file "$adr_file"; then
        log_error "ADR filename format incorrect: $filename (should be NNN-title.md)"
        inc_errors
        return
    fi

    # Extract ADR number (remove leading zeros for comparison)
    local adr_num
    adr_num=$(get_adr_number "$adr_file" | sed 's/^0*//')

    # Check ADR number in title (supports both "ADR 001:" and "ADR 1:" formats)
    local first_line
    first_line=$(head -1 "$adr_file" | tr -d '\r')
    local adr_num_padded
    adr_num_padded=$(printf "%03d" "$adr_num")
    if ! echo "$first_line" | grep -qE "^# ADR (0*)?$adr_num:"; then
        log_error "ADR number mismatch in $filename"
        log_error "Expected ADR number: $adr_num or $adr_num_padded"
        log_error "Found: $first_line"
        inc_errors
    fi

    # Check required sections
    local required_sections=("## Status" "## Context" "## Decision")
    local has_errors=false

    for section in "${required_sections[@]}"; do
        if ! grep -q "^$section" "$adr_file"; then
            log_error "ADR $filename missing required section: $section"
            has_errors=true
        fi
    done

    # Check status values using library function
    local status_value
    status_value=$(get_adr_status "$adr_file")
    if [[ -n "$status_value" ]]; then
        if ! validate_adr_status "$status_value"; then
            log_error "ADR $filename has invalid status: '$status_value'"
            has_errors=true
        fi
    fi

    # Check for consequences section if status is Accepted
    if [[ "$status_value" == "Accepted"* ]]; then
        if ! grep -q "^## Consequences" "$adr_file"; then
            log_warning "ADR $filename (Accepted) missing Consequences section"
            inc_warnings
        fi
        if ! grep -q "^## Alternatives Considered" "$adr_file"; then
            log_warning "ADR $filename (Accepted) missing Alternatives Considered section"
            inc_warnings
        fi
    fi

    if [[ "$has_errors" == "false" ]]; then
        log_success "ADR $filename format is valid"
    else
        inc_errors
    fi
}

# Check ADR numbering consistency
check_adr_numbering() {
    log_info "Checking ADR numbering consistency..."

    local adr_files
    adr_files=$(ls "$ADR_DIR" 2>/dev/null | grep -E '^[0-9]{3}-.*\.md$' | sort -V)
    local expected_num=1

    for adr_file in $adr_files; do
        local actual_num
        actual_num=$(echo "$adr_file" | grep -oE '^[0-9]+' | sed 's/^0*//')
        [[ -z "$actual_num" ]] && actual_num=0
        if [[ "$actual_num" -ne "$expected_num" ]]; then
            log_error "ADR numbering gap: expected $expected_num, found $actual_num"
            inc_errors
        fi
        ((expected_num++))
    done

    log_success "ADR numbering is consistent"
}

# Check ADR references in documentation
check_adr_references() {
    log_info "Checking ADR references in documentation..."

    local adr_files
    adr_files=$(ls "$ADR_DIR" 2>/dev/null | grep -E '^[0-9]{3}-.*\.md$')
    local arch_doc="$PROJECT_ROOT/docs/architecture/ARCHITECTURE.md"

    if [[ -f "$arch_doc" ]]; then
        for adr_file in $adr_files; do
            local adr_num
            adr_num=$(echo "$adr_file" | grep -oE '^[0-9]+' | sed 's/^0*//')
            if [[ -n "$adr_num" ]] && ! grep -q "ADR.* $adr_num\|ADR-0*$adr_num\|ADR 0*$adr_num" "$arch_doc"; then
                log_warning "ADR $adr_num not referenced in architecture documentation"
                inc_warnings
            fi
        done
    fi

    log_success "ADR reference check completed"
}

# Main execution
main() {
    log_info "MCP Context Browser - ADR Validation"
    log_info "===================================="

    if ! check_directory "$ADR_DIR" "ADR directory"; then
        exit 1
    fi

    # Validate each ADR file
    for adr_file in "$ADR_DIR"/*.md; do
        local basename_file
        basename_file=$(basename "$adr_file")
        # Skip non-ADR files
        if [[ -f "$adr_file" ]] && \
           [[ "$basename_file" != "README.md" ]] && \
           [[ "$basename_file" != "TEMPLATE.md" ]] && \
           [[ "$basename_file" != "adr-graph.md" ]] && \
           [[ "$basename_file" != "CLAUDE.md" ]]; then
            validate_adr_format "$adr_file"
        fi
    done

    check_adr_numbering
    check_adr_references

    exit_with_summary "ADR Validation Summary"
}

main "$@"
