# API Surface Analysis

This document provides an overview of the public API surface of the MCP Context Browser.

## Public Modules

### Core Library Modules

\1-   adapters
\1-   admin
\1-   application
\1-   chunking
\1-   config_example
\1-   daemon
\1-   domain
\1-   infrastructure
\1-   server
\1-   snapshot
\1-   sync

### Public Re-exports

\1-   domain::error::{Error, Result}
\1-   domain::types::*
\1-   server::builder::McpServerBuilder
\1-   server::init::run_server
\1-   server::MCP_server::McpServer

## Public Functions

## Public Types

### Data Structures

\1-   NodeExtractionRule
\1-   LanguageConfig
\1-   NodeExtractionRuleBuilder
\1-   IntelligentChunker;
\1-   GenericFallbackChunker<'a>
\1-   RustProcessor
\1-   ".to_String(),
\1-   PythonProcessor
\1-   JavaScriptProcessor
\1-   JavaProcessor

### Enums

\1-   McpError
\1-   CompatibilityResult
\1-   SessionState
\1-   SessionError
\1-   TransportMode

## API Stability

### Current Status

\1-  **Version**: 0.1.0 (First Stable Release)
\1-  **Stability**: Experimental - APIs may change
\1-  **Compatibility**: Breaking changes expected until 1.0.0

### Public API Commitments

\1-   MCP protocol interface stability
\1-   Core semantic search functionality
\1-   Provider abstraction interfaces

*Generated automatically from source code analysis on: 2026-01-11 21:51:55 UTC*
