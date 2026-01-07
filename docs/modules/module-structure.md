# Module Structure

This document shows the hierarchical structure of modules in the MCP Context Browser.

## Module Tree

```

crate mcp_context_browser
├── mod chunking: pub
│   ├── mod config: pub
│   │   ├── struct LanguageConfig: pub
│   │   │   ├── fn get_language: pub
│   │   │   ├── fn new: pub
│   │   │   ├── fn with_chunk_size: pub
│   │   │   ├── fn with_fallback_patterns: pub
│   │   │   ├── fn with_rule: pub
│   │   │   └── fn with_rules: pub
│   │   ├── struct NodeExtractionRule: pub
│   │   └── struct NodeExtractionRuleBuilder: pub
│   │       ├── fn build: pub
│   │       ├── fn new: pub
│   │       ├── fn with_context: pub
│   │       ├── fn with_max_depth: pub
│   │       ├── fn with_min_length: pub
│   │       ├── fn with_min_lines: pub
│   │       ├── fn with_node_type: pub
│   │       ├── fn with_node_types: pub
│   │       └── fn with_priority: pub
│   ├── mod fallback: pub
│   │   ├── struct GenericFallbackChunker: pub
│   │   │   ├── fn chunk_with_patterns: pub
│   │   │   ├── fn create_chunk: pub(self)
│   │   │   ├── fn is_block_complete: pub(self)
│   │   │   └── fn new: pub
│   │   └── fn safe_json_value: pub(self)
│   ├── mod languages: pub
│   │   ├── mod javascript: pub
│   │   │   └── struct JavaScriptProcessor: pub
│   │   │       ├── fn config: pub(self)
│   │   │       ├── fn extract_chunks_fallback: pub(self)
│   │   │       ├── fn extract_chunks_with_tree_sitter: pub(self)
│   │   │       └── fn new: pub
│   │   ├── mod python: pub
│   │   │   └── struct PythonProcessor: pub
│   │   │       ├── fn config: pub(self)
│   │   │       ├── fn extract_chunks_fallback: pub(self)
│   │   │       ├── fn extract_chunks_with_tree_sitter: pub(self)
│   │   │       └── fn new: pub
│   │   └── mod rust: pub
│   │       └── struct RustProcessor: pub
│   │           ├── fn config: pub(self)
│   │           ├── fn extract_chunks_fallback: pub(self)
│   │           ├── fn extract_chunks_with_tree_sitter: pub(self)
│   │           └── fn new: pub
│   ├── mod processor: pub
│   │   ├── struct BaseProcessor: pub
│   │   │   ├── fn config: pub
│   │   │   ├── fn config: pub(self)
│   │   │   ├── fn extract_chunks_fallback: pub(self)
│   │   │   ├── fn extract_chunks_with_tree_sitter: pub(self)
│   │   │   └── fn new: pub
│   │   └── trait LanguageProcessor: pub
│   └── mod traverser: pub
│       ├── struct AstTraverser: pub
│       │   ├── fn create_chunk_from_node: pub(self)
│       │   ├── fn extract_node_content: pub(self)
│       │   ├── fn extract_node_with_context: pub(self)
│       │   ├── fn new: pub
│       │   ├── fn traverse_and_extract: pub
│       │   └── fn with_max_chunks: pub
│       ├── struct ChunkParams: pub(self)
│       └── fn safe_json_value: pub(self)
├── mod core: pub
│   ├── mod error: pub
│   │   ├── enum Error: pub
│   │   │   ├── fn config: pub
│   │   │   ├── fn embedding: pub
│   │   │   ├── fn generic: pub
│   │   │   ├── fn internal: pub
│   │   │   ├── fn invalid_argument: pub
│   │   │   ├── fn io: pub
│   │   │   ├── fn not_found: pub
│   │   │   └── fn vector_db: pub
│   │   └── type Result: pub
│   └── mod types: pub
│       ├── struct CodeChunk: pub
│       ├── struct Embedding: pub
│       ├── struct EmbeddingConfig: pub
│       ├── struct IndexingStats: pub
│       ├── enum Language: pub
│       │   └── fn from_extension: pub
│       ├── struct SearchResult: pub
│       └── struct VectorStoreConfig: pub
├── mod metrics: pub
├── mod providers: pub
│   ├── mod embedding: pub
│   └── mod vector_store: pub
├── mod server: pub
└── mod services: pub
```

## Structure Analysis

The module tree above shows the organization of code into logical units.

*Generated automatically on: 2026-01-07 18:58:38 UTC*
