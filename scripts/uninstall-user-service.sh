#!/bin/bash
# MCP Context Browser - User Service Uninstallation Script
#
# Removes the MCP Context Browser systemd user service.
# Optionally removes data and configuration.
#
# Usage: ./scripts/uninstall-user-service.sh [--all]
#   --all: Also remove configuration and data

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

REMOVE_ALL=false
if [ "$1" = "--all" ]; then
    REMOVE_ALL=true
fi

echo -e "${YELLOW}Removing MCP Context Browser user service...${NC}"
echo ""

# Stop and disable service
echo "Stopping service..."
systemctl --user stop mcp-context-browser 2>/dev/null || true
systemctl --user disable mcp-context-browser 2>/dev/null || true

# Remove service file
echo "Removing service file..."
rm -f ~/.config/systemd/user/mcp-context-browser.service

# Remove binary
echo "Removing binary..."
rm -f ~/.local/bin/mcp-context-browser

# Reload systemd
systemctl --user daemon-reload

echo ""
echo -e "${GREEN}Service removed.${NC}"
echo ""

if [ "$REMOVE_ALL" = true ]; then
    echo -e "${YELLOW}Removing configuration and data...${NC}"
    rm -rf ~/.config/mcp-context-browser
    rm -rf ~/.local/share/mcp-context-browser
    echo -e "${GREEN}All data removed.${NC}"
else
    echo "Data preserved at:"
    echo "  Config: ~/.config/mcp-context-browser/"
    echo "  Data:   ~/.local/share/mcp-context-browser/"
    echo ""
    echo -e "${YELLOW}To remove all data:${NC} $0 --all"
fi

echo ""
echo -e "${GREEN}Uninstallation complete.${NC}"
