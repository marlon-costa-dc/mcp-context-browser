#!/bin/bash

# MCP Context Browser - ADR Creation Script
# Creates new Architecture Decision Records interactively

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Get next ADR number
get_next_adr_number() {
    local adr_dir="$PROJECT_ROOT/docs/architecture/adr"
    local existing_adrs=$(ls "$adr_dir" 2>/dev/null | grep -E '^[0-9]+\.md$' | sed 's/\.md$//' | sort -n | tail -1 || echo "0")

    echo $((existing_adrs + 1))
}

# Create ADR filename from title
create_filename() {
    local title="$1"
    # Convert to lowercase, replace spaces/special chars with hyphens
    local slug=$(echo "$title" | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]/-/g' | sed 's/--*/-/g' | sed 's/^-\|-$//g')
    echo "$slug"
}

# Interactive ADR creation
create_adr_interactive() {
    local adr_dir="$PROJECT_ROOT/docs/architecture/adr"
    local template_file="$PROJECT_ROOT/docs/templates/adr-template.md"

    # Check if template exists
    if [ ! -f "$template_file" ]; then
        log_error "ADR template not found: $template_file"
        exit 1
    fi

    # Get ADR details
    echo "Creating new Architecture Decision Record"
    echo "========================================"
    echo

    read -p "ADR Title: " adr_title
    if [ -z "$adr_title" ]; then
        log_error "ADR title cannot be empty"
        exit 1
    fi

    read -p "Status (Proposed/Accepted/Rejected/Deprecated/Superseded by ADR-xxx) [Proposed]: " adr_status
    adr_status=${adr_status:-Proposed}

    # Generate filename
    local adr_num=$(printf "%03d" $(get_next_adr_number))
    local filename_slug=$(create_filename "$adr_title")
    local adr_filename="${adr_num}-${filename_slug}.md"
    local adr_filepath="$adr_dir/$adr_filename"

    # Check if file already exists
    if [ -f "$adr_filepath" ]; then
        log_error "ADR file already exists: $adr_filepath"
        exit 1
    fi

    log_info "Creating ADR: $adr_filename"
    log_info "Title: $adr_title"
    log_info "Status: $adr_status"
    echo

    # Copy template and update
    cp "$template_file" "$adr_filepath"

    # Update ADR number and title
    sed -i "s/{number}/$adr_num/g" "$adr_filepath"
    sed -i "s/{title}/$adr_title/g" "$adr_filepath"
    sed -i "s/{Proposed | Accepted | Rejected | Deprecated | Superseded by ADR-xxx}/$adr_status/g" "$adr_filepath"

    # Add creation date
    local creation_date=$(date '+%Y-%m-%d')
    sed -i "s/{Proposed | Accepted | Rejected | Deprecated | Superseded by ADR-xxx}/$adr_status/" "$adr_filepath"

    log_success "ADR created: $adr_filepath"
    echo
    log_info "Next steps:"
    echo "1. Edit the ADR file to add context, decision, and consequences"
    echo "2. Run 'make docs-validate' to check the ADR format"
    echo "3. Add the ADR to the architecture documentation if applicable"
    echo
    log_info "Template sections to fill:"
    echo "- Context: What problem are we solving?"
    echo "- Decision: What decision was made?"
    echo "- Consequences: What are the positive/negative consequences?"
    echo "- Alternatives Considered: What other options were rejected?"
    echo "- Implementation Notes: Technical implementation details"
}

# Batch ADR creation (for automation)
create_adr_batch() {
    local title="$1"
    local status="${2:-Proposed}"

    if [ -z "$title" ]; then
        log_error "ADR title is required for batch creation"
        echo "Usage: $0 batch \"ADR Title\" [status]"
        exit 1
    fi

    # Set environment variables for non-interactive mode
    export ADR_TITLE="$title"
    export ADR_STATUS="$status"

    # Call interactive function (will use env vars)
    create_adr_batch_mode
}

create_adr_batch_mode() {
    local adr_dir="$PROJECT_ROOT/docs/architecture/adr"
    local template_file="$PROJECT_ROOT/docs/templates/adr-template.md"

    # Use environment variables or defaults
    local adr_title="${ADR_TITLE:-Unknown ADR}"
    local adr_status="${ADR_STATUS:-Proposed}"

    # Generate filename
    local adr_num=$(printf "%03d" $(get_next_adr_number))
    local filename_slug=$(create_filename "$adr_title")
    local adr_filename="${adr_num}-${filename_slug}.md"
    local adr_filepath="$adr_dir/$adr_filename"

    # Check if file already exists
    if [ -f "$adr_filepath" ]; then
        log_error "ADR file already exists: $adr_filepath"
        exit 1
    fi

    log_info "Creating ADR (batch mode): $adr_filename"

    # Copy template and update
    cp "$template_file" "$adr_filepath"

    # Update ADR number and title
    sed -i "s/{number}/$adr_num/g" "$adr_filepath"
    sed -i "s/{title}/$adr_title/g" "$adr_filepath"
    sed -i "s/{Proposed | Accepted | Rejected | Deprecated | Superseded by ADR-xxx}/$adr_status/g" "$adr_filepath"

    log_success "ADR created: $adr_filepath"
}

# Show usage
show_usage() {
    cat << EOF
MCP Context Browser - ADR Creation Tool

USAGE:
    $0                    # Interactive mode
    $0 batch "title" [status]  # Batch mode

INTERACTIVE MODE:
    Creates an ADR interactively, prompting for title and status.

BATCH MODE:
    Creates an ADR non-interactively using provided parameters.

ARGUMENTS:
    title     ADR title (required in batch mode)
    status    ADR status: Proposed, Accepted, Rejected, Deprecated, or "Superseded by ADR-xxx"
             (default: Proposed)

EXAMPLES:
    $0                                    # Interactive creation
    $0 batch "New Feature Decision"       # Create with default status
    $0 batch "Security Enhancement" Accepted  # Create with specific status

OUTPUT:
    Creates ADR file in: docs/architecture/adr/
    Format: NNN-title-slug.md

EOF
}

# Main execution
main() {
    local command="${1:-interactive}"

    log_info "MCP Context Browser - ADR Creation Tool"

    case "$command" in
        "batch")
            shift
            create_adr_batch "$@"
            ;;
        "interactive"|*)
            if [ $# -gt 0 ]; then
                show_usage
                exit 1
            fi
            create_adr_interactive
            ;;
    esac
}

# Run main function
main "$@"