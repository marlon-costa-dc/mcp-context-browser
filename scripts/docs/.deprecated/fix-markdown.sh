#!/bin/bash
# Wrapper for backward compatibility - use markdown.sh fix instead
exec "$(dirname "${BASH_SOURCE[0]}")/markdown.sh" fix "$@"
