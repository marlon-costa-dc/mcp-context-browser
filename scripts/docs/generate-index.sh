#!/bin/bash

# MCP Context Browser - Documentation Index Generation
# Generates navigation index for documentation

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

# Generate main documentation index
generate_main_index() {
    log_info "Generating main documentation index..."

    local index_file="$PROJECT_ROOT/docs/index.html"
    local last_updated=$(date -u '+%Y-%m-%d %H:%M:%S UTC')

    cat > "$index_file" << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>MCP Context Browser - Documentation</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            line-height: 1.6;
            color: #333;
            background: #f5f5f5;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 20px;
        }
        header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 40px 0;
            text-align: center;
            border-radius: 8px;
            margin-bottom: 30px;
        }
        h1 { font-size: 2.5em; margin-bottom: 10px; }
        .subtitle { font-size: 1.2em; opacity: 0.9; }
        .grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
            gap: 20px;
            margin-bottom: 40px;
        }
        .card {
            background: white;
            border-radius: 8px;
            padding: 25px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            transition: transform 0.2s, box-shadow 0.2s;
        }
        .card:hover {
            transform: translateY(-2px);
            box-shadow: 0 4px 20px rgba(0,0,0,0.15);
        }
        .card-icon {
            font-size: 2em;
            margin-bottom: 15px;
            display: block;
        }
        .card-title {
            font-size: 1.3em;
            font-weight: bold;
            margin-bottom: 10px;
            color: #2c3e50;
        }
        .card-description {
            color: #666;
            margin-bottom: 15px;
        }
        .card-link {
            display: inline-block;
            background: #3498db;
            color: white;
            padding: 8px 16px;
            text-decoration: none;
            border-radius: 4px;
            font-weight: 500;
            transition: background 0.2s;
        }
        .card-link:hover {
            background: #2980b9;
        }
        .status-badges {
            display: flex;
            gap: 10px;
            margin-bottom: 20px;
            flex-wrap: wrap;
        }
        .badge {
            padding: 4px 8px;
            border-radius: 12px;
            font-size: 0.8em;
            font-weight: 500;
        }
        .badge.automated { background: #27ae60; color: white; }
        .badge.manual { background: #f39c12; color: white; }
        .badge.generated { background: #9b59b6; color: white; }
        footer {
            text-align: center;
            padding: 20px;
            color: #666;
            border-top: 1px solid #ddd;
            background: white;
            border-radius: 8px;
        }
    </style>
</head>
<body>
    <header>
        <div class="container">
            <h1>📚 MCP Context Browser</h1>
            <div class="subtitle">Comprehensive Documentation Hub</div>
        </div>
    </header>

    <div class="container">
        <div class="status-badges">
            <span class="badge automated">🤖 Automated</span>
            <span class="badge generated">🎨 Generated</span>
            <span class="badge automated">✅ Validated</span>
        </div>

        <div class="grid">
            <div class="card">
                <span class="card-icon">📖</span>
                <div class="card-title">User Guide</div>
                <div class="card-description">
                    Installation, quick start, and basic usage instructions for end users.
                </div>
                <a href="user-guide/README.html" class="card-link">View Guide →</a>
            </div>

            <div class="card">
                <span class="card-icon">🛠️</span>
                <div class="card-title">Developer Guide</div>
                <div class="card-description">
                    Development setup, contribution guidelines, and coding standards.
                </div>
                <a href="developer/CONTRIBUTING.html" class="card-link">Contribute →</a>
            </div>

            <div class="card">
                <span class="card-icon">🏗️</span>
                <div class="card-title">Architecture</div>
                <div class="card-description">
                    System architecture, design decisions, and technical documentation.
                </div>
                <a href="architecture/ARCHITECTURE.html" class="card-link">Explore Architecture →</a>
            </div>

            <div class="card">
                <span class="card-icon">🚀</span>
                <div class="card-title">Operations</div>
                <div class="card-description">
                    Deployment guides, monitoring, and operational procedures.
                </div>
                <a href="operations/DEPLOYMENT.html" class="card-link">Deploy →</a>
            </div>

            <div class="card">
                <span class="card-icon">📊</span>
                <div class="card-title">Architecture Diagrams</div>
                <div class="card-description">
                    Visual representations of system architecture and data flows.
                </div>
                <a href="architecture/diagrams/index.html" class="card-link">View Diagrams →</a>
            </div>

            <div class="card">
                <span class="card-icon">📋</span>
                <div class="card-title">ADRs</div>
                <div class="card-description">
                    Architecture Decision Records documenting design choices and rationale.
                </div>
                <a href="architecture/adr/README.html" class="card-link">Review Decisions →</a>
            </div>
        </div>
    </div>

    <footer>
        <div class="container">
            <p><strong>Last updated:</strong> $last_updated</p>
            <p><strong>Version:</strong> $(grep '^version' "$PROJECT_ROOT/Cargo.toml" | head -1 | sed 's/.*= *"\([^"]*\)".*/\1/')</p>
            <p><em>Documentation automatically generated and validated</em></p>
        </div>
    </footer>
</body>
</html>
EOF

    log_success "Generated main documentation index: $index_file"
}

# Generate section-specific navigation
generate_section_nav() {
    log_info "Generating section navigation files..."

    # Generate user-guide navigation
    generate_section_index "user-guide" "📖 User Guide" "Installation and usage documentation"

    # Generate developer navigation
    generate_section_index "developer" "🛠️ Developer Guide" "Development and contribution guides"

    # Generate architecture navigation
    generate_section_index "architecture" "🏗️ Architecture" "Technical architecture and design"

    # Generate operations navigation
    generate_section_index "operations" "🚀 Operations" "Deployment and operational guides"
}

# Generate section index
generate_section_index() {
    local section="$1"
    local title="$2"
    local description="$3"
    local section_dir="$PROJECT_ROOT/docs/$section"
    local index_file="$section_dir/index.html"

    if [ ! -d "$section_dir" ]; then
        log_warning "Section directory not found: $section_dir"
        return
    fi

    local files=$(find "$section_dir" -name "*.md" -type f | sort)

    cat > "$index_file" << EOF
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>$title - MCP Context Browser</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif; margin: 40px; line-height: 1.6; }
        .header { background: #f8f9fa; padding: 20px; border-radius: 8px; margin-bottom: 30px; }
        .nav-item { margin: 10px 0; padding: 15px; border: 1px solid #ddd; border-radius: 6px; }
        .nav-link { text-decoration: none; color: #007bff; font-weight: 500; }
        .nav-link:hover { color: #0056b3; }
        .nav-description { color: #666; font-size: 0.9em; margin-top: 5px; }
        .back-link { display: inline-block; margin-bottom: 20px; text-decoration: none; color: #6c757d; }
        .back-link:hover { color: #495057; }
    </style>
</head>
<body>
    <a href="../index.html" class="back-link">← Back to Documentation Index</a>

    <div class="header">
        <h1>$title</h1>
        <p>$description</p>
    </div>

    <div class="nav-list">
EOF

    for file in $files; do
        local filename=$(basename "$file" .md)
        local relative_path="${file#$PROJECT_ROOT/docs/}"
        local html_path="${relative_path%.md}.html"

        # Extract first line as description
        local first_line=$(head -1 "$file" | sed 's/^# *//')
        local description="Documentation file"

        if [ "$first_line" != "$filename" ]; then
            description="$first_line"
        fi

        cat >> "$index_file" << EOF
        <div class="nav-item">
            <a href="$html_path" class="nav-link">$filename</a>
            <div class="nav-description">$description</div>
        </div>
EOF
    done

    cat >> "$index_file" << EOF
    </div>
</body>
</html>
EOF

    log_success "Generated section index: $index_file"
}

# Main execution
main() {
    log_info "MCP Context Browser - Documentation Index Generation"
    log_info "==================================================="

    generate_main_index
    generate_section_nav

    log_success "Documentation index generation completed"
}

# Run main function
main "$@"