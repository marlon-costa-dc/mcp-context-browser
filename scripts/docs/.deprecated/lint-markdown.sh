#!/bin/bash
# Wrapper for backward compatibility - use markdown.sh lint instead
exec "$(dirname "${BASH_SOURCE[0]}")/markdown.sh" lint "$@"
