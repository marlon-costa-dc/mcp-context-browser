//! Enterprise Code Intelligence Domain Model
//!
//! Defines the fundamental business entities that power the semantic code search
//! platform. These types represent the core business concepts of code intelligence,
//! from semantic embeddings that capture code meaning to search results that deliver
//! business value to development teams.
//!
//! ## Core Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`Embedding`] | Vector representation of text for similarity search |
//! | [`CodeChunk`] | Semantically meaningful code segment from AST parsing |
//! | [`Language`] | Supported programming languages (24 variants) |
//! | [`SearchResult`] | Ranked result from semantic search |
//!
//! ## Provider Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`EmbeddingProviderKind`] | Type-safe embedding provider selection |
//! | [`VectorStoreProviderKind`] | Type-safe vector store selection |
//! | [`EmbeddingConfig`] | Configuration for embedding providers |
//! | [`VectorStoreConfig`] | Configuration for vector stores |
//!
//! ## Snapshot Types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`FileSnapshot`] | File metadata for change tracking |
//! | [`CodebaseSnapshot`] | Complete codebase state capture |
//! | [`SnapshotChanges`] | Diff between two snapshots |
//!
//! ## Example: End-to-End Indexing Flow
//!
//! ```rust
//! use mcp_context_browser::domain::types::{
//!     CodeChunk, Language, Embedding, SearchResult,
//! };
//!
//! // 1. Create a code chunk from parsed AST
//! let chunk = CodeChunk {
//!     id: "chunk_001".to_string(),
//!     content: "fn authenticate(user: &str) -> bool { true }".to_string(),
//!     file_path: "src/auth.rs".to_string(),
//!     start_line: 10,
//!     end_line: 12,
//!     language: Language::Rust,
//!     metadata: serde_json::json!({"type": "function", "name": "authenticate"}),
//! };
//!
//! // 2. Generate embedding (normally done by provider)
//! let embedding = Embedding {
//!     vector: vec![0.1, 0.2, 0.3, 0.4],
//!     model: "text-embedding-3-small".to_string(),
//!     dimensions: 4,
//! };
//!
//! // 3. Search results returned after similarity search
//! let result = SearchResult {
//!     id: chunk.id.clone(),
//!     file_path: chunk.file_path.clone(),
//!     start_line: chunk.start_line,
//!     content: chunk.content.clone(),
//!     score: 0.95,
//!     metadata: chunk.metadata.clone(),
//! };
//!
//! assert!(result.score > 0.9);
//! ```

use serde::{Deserialize, Serialize};
use validator::Validate;

/// AI Semantic Understanding Representation
///
/// An embedding is a dense vector representation of text that captures semantic meaning.
/// Embeddings enable similarity search across code chunks.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::Embedding;
///
/// let embedding = Embedding {
///     vector: vec![0.1, 0.2, 0.3],
///     model: "text-embedding-3-small".to_string(),
///     dimensions: 3,
/// };
///
/// assert_eq!(embedding.dimensions, embedding.vector.len());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default, Validate)]
pub struct Embedding {
    /// The embedding vector values
    #[validate(length(min = 1, message = "Embedding vector cannot be empty"))]
    pub vector: Vec<f32>,
    /// Name of the model that generated this embedding
    #[validate(length(min = 1, message = "Model name cannot be empty"))]
    pub model: String,
    /// Dimensionality of the embedding vector
    #[validate(range(min = 1, message = "Dimensions must be positive"))]
    pub dimensions: usize,
}

/// Intelligent Code Segment with Business Context
///
/// A code chunk represents a semantically meaningful portion of source code,
/// extracted by the AST-based chunking engine. Each chunk is indexed for
/// semantic search.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::{CodeChunk, Language};
///
/// let chunk = CodeChunk {
///     id: "abc123".to_string(),
///     content: "fn hello() { println!(\"Hello!\"); }".to_string(),
///     file_path: "src/lib.rs".to_string(),
///     start_line: 1,
///     end_line: 3,
///     language: Language::Rust,
///     metadata: serde_json::json!({"type": "function"}),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Validate)]
pub struct CodeChunk {
    /// Unique identifier for this code chunk
    #[validate(length(min = 1, message = "ID cannot be empty"))]
    pub id: String,
    /// The actual code content
    #[validate(length(
        min = 1,
        max = 10000,
        message = "Content must be between 1 and 10000 characters"
    ))]
    pub content: String,
    /// Path to the source file
    #[validate(length(min = 1, message = "File path cannot be empty"))]
    pub file_path: String,
    /// Starting line number in the source file
    #[validate(range(min = 1, message = "Start line must be positive"))]
    pub start_line: u32,
    /// Ending line number in the source file
    #[validate(range(min = 1, message = "End line must be positive"))]
    pub end_line: u32,
    /// Programming language of the code
    pub language: Language,
    /// Additional metadata as JSON (context, AST info, etc.)
    pub metadata: serde_json::Value,
}

/// Supported programming languages for AST parsing
///
/// Each variant corresponds to a tree-sitter grammar for AST-based code chunking.
/// The chunking engine uses language-specific rules to extract meaningful code segments.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::Language;
///
/// let lang = Language::Rust;
/// assert!(matches!(lang, Language::Rust));
///
/// // Unknown is used for unrecognized file extensions
/// let unknown = Language::Unknown;
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    Java,
    C,
    Cpp,
    CSharp,
    Php,
    Ruby,
    Swift,
    Kotlin,
    Scala,
    Haskell,
    Shell,
    SQL,
    HTML,
    XML,
    JSON,
    YAML,
    TOML,
    Markdown,
    PlainText,
    Unknown,
}

/// System operation types for metrics and rate limiting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OperationType {
    Indexing,
    Search,
    Embedding,
    Maintenance,
    Other(String),
}

// =============================================================================
// Provider Kind Enums (Type-Safe Provider Selection)
// =============================================================================

/// Type-safe embedding provider selection
///
/// Replaces string-based provider selection with compile-time type safety.
/// Invalid provider names are caught at config deserialization time.
///
/// # Providers
///
/// | Variant | Use Case |
/// |---------|----------|
/// | `OpenAI` | Cloud-hosted, high quality |
/// | `Ollama` | Local, privacy-focused |
/// | `VoyageAI` | Cloud, code-optimized |
/// | `Gemini` | Google Cloud |
/// | `FastEmbed` | Local, fast, default |
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::EmbeddingProviderKind;
///
/// // Default is FastEmbed for local development
/// let provider = EmbeddingProviderKind::default();
/// assert_eq!(provider, EmbeddingProviderKind::FastEmbed);
///
/// // Parse from config string
/// let parsed = EmbeddingProviderKind::from_string("openai");
/// assert_eq!(parsed, Some(EmbeddingProviderKind::OpenAI));
///
/// // Display for logging
/// assert_eq!(format!("{}", EmbeddingProviderKind::Ollama), "ollama");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum EmbeddingProviderKind {
    /// OpenAI embedding API
    OpenAI,
    /// Ollama local embeddings
    Ollama,
    /// VoyageAI embedding API
    VoyageAI,
    /// Google Gemini embeddings
    Gemini,
    /// FastEmbed local embeddings (default)
    #[default]
    FastEmbed,
}

impl std::fmt::Display for EmbeddingProviderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OpenAI => write!(f, "openai"),
            Self::Ollama => write!(f, "ollama"),
            Self::VoyageAI => write!(f, "voyageai"),
            Self::Gemini => write!(f, "gemini"),
            Self::FastEmbed => write!(f, "fastembed"),
        }
    }
}

impl EmbeddingProviderKind {
    /// Parse a provider string into the enum variant.
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "openai" => Some(Self::OpenAI),
            "ollama" => Some(Self::Ollama),
            "voyageai" => Some(Self::VoyageAI),
            "gemini" => Some(Self::Gemini),
            "fastembed" => Some(Self::FastEmbed),
            _ => None,
        }
    }

    /// Get all supported provider names
    pub fn supported_providers() -> &'static [&'static str] {
        &["openai", "ollama", "voyageai", "gemini", "fastembed"]
    }
}

/// Type-safe vector store provider selection
///
/// Provides compile-time type safety for vector database configuration.
/// Each variant corresponds to a different storage backend with its own
/// tradeoffs for performance, persistence, and scalability.
///
/// # Providers
///
/// | Variant | Persistence | Use Case |
/// |---------|-------------|----------|
/// | `InMemory` | None | Testing, ephemeral |
/// | `Filesystem` | File-based | Development, single-node |
/// | `Milvus` | Database | Production, distributed |
/// | `EdgeVec` | Memory | Edge computing |
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::VectorStoreProviderKind;
///
/// // Default is Filesystem for local persistence
/// let provider = VectorStoreProviderKind::default();
/// assert_eq!(provider, VectorStoreProviderKind::Filesystem);
///
/// // Parse from config string
/// let parsed = VectorStoreProviderKind::from_string("in-memory");
/// assert_eq!(parsed, Some(VectorStoreProviderKind::InMemory));
///
/// // Check available providers
/// let providers = VectorStoreProviderKind::supported_providers();
/// assert!(providers.contains(&"filesystem"));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum VectorStoreProviderKind {
    /// In-memory vector store (for testing/development)
    #[serde(rename = "in-memory")]
    InMemory,
    /// Filesystem-based vector store
    #[default]
    Filesystem,
    /// Milvus vector database
    #[cfg(feature = "milvus")]
    Milvus,
    /// EdgeVec in-memory store
    EdgeVec,
}

impl std::fmt::Display for VectorStoreProviderKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InMemory => write!(f, "in-memory"),
            Self::Filesystem => write!(f, "filesystem"),
            #[cfg(feature = "milvus")]
            Self::Milvus => write!(f, "milvus"),
            Self::EdgeVec => write!(f, "edgevec"),
        }
    }
}

impl VectorStoreProviderKind {
    /// Parse a provider string into the enum variant.
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "in-memory" | "inmemory" => Some(Self::InMemory),
            "filesystem" => Some(Self::Filesystem),
            #[cfg(feature = "milvus")]
            "milvus" => Some(Self::Milvus),
            "edgevec" => Some(Self::EdgeVec),
            _ => None,
        }
    }

    /// Get all supported provider names
    pub fn supported_providers() -> Vec<&'static str> {
        let mut providers = vec!["in-memory", "filesystem", "edgevec"];
        #[cfg(feature = "milvus")]
        providers.push("milvus");
        providers
    }
}

/// Query performance metrics tracking
///
/// Aggregated statistics for monitoring search query performance.
/// Used by the admin dashboard and Prometheus metrics endpoint.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::QueryPerformanceMetrics;
///
/// let metrics = QueryPerformanceMetrics {
///     total_queries: 1000,
///     average_latency: 45.5,  // milliseconds
///     p99_latency: 150.0,     // milliseconds
///     success_rate: 0.998,    // 99.8%
/// };
///
/// assert!(metrics.success_rate > 0.99);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryPerformanceMetrics {
    /// Total number of queries processed
    pub total_queries: u64,
    /// Average query latency in milliseconds
    pub average_latency: f64,
    /// 99th percentile latency in milliseconds
    pub p99_latency: f64,
    /// Ratio of successful queries (0.0 to 1.0)
    pub success_rate: f64,
}

/// Cache performance metrics tracking
///
/// Statistics for monitoring embedding and search result caches.
/// High hit rates indicate effective caching; low rates may indicate
/// cache sizing issues or highly diverse query patterns.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::CacheMetrics;
///
/// let metrics = CacheMetrics {
///     hits: 850,
///     misses: 150,
///     hit_rate: 0.85,      // 85% hit rate
///     size: 1024 * 1024,   // 1 MB
/// };
///
/// assert!(metrics.hit_rate > 0.8);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CacheMetrics {
    /// Number of cache hits
    pub hits: u64,
    /// Number of cache misses
    pub misses: u64,
    /// Hit rate ratio (0.0 to 1.0)
    pub hit_rate: f64,
    /// Current cache size in bytes
    pub size: u64,
}

impl std::fmt::Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperationType::Indexing => write!(f, "indexing"),
            OperationType::Search => write!(f, "search"),
            OperationType::Embedding => write!(f, "embedding"),
            OperationType::Maintenance => write!(f, "maintenance"),
            OperationType::Other(s) => write!(f, "{}", s),
        }
    }
}

impl From<&str> for OperationType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "indexing" => OperationType::Indexing,
            "search" => OperationType::Search,
            "embedding" => OperationType::Embedding,
            "maintenance" => OperationType::Maintenance,
            _ => OperationType::Other(s.to_string()),
        }
    }
}

impl Language {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "rs" => Language::Rust,
            "py" => Language::Python,
            "js" => Language::JavaScript,
            "ts" => Language::TypeScript,
            "go" => Language::Go,
            "java" => Language::Java,
            "c" => Language::C,
            "cpp" | "cc" | "cxx" => Language::Cpp,
            "cs" => Language::CSharp,
            "php" => Language::Php,
            "rb" => Language::Ruby,
            "swift" => Language::Swift,
            "kt" => Language::Kotlin,
            "scala" => Language::Scala,
            "hs" => Language::Haskell,
            "sh" | "bash" | "zsh" | "fish" => Language::Shell,
            "sql" => Language::SQL,
            "html" => Language::HTML,
            "xml" => Language::XML,
            "json" => Language::JSON,
            "yaml" | "yml" => Language::YAML,
            "toml" => Language::TOML,
            "md" | "markdown" => Language::Markdown,
            "txt" | "text" => Language::PlainText,
            _ => Language::Unknown,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::Python => "Python",
            Language::JavaScript => "JavaScript",
            Language::TypeScript => "TypeScript",
            Language::Go => "Go",
            Language::Java => "Java",
            Language::C => "C",
            Language::Cpp => "Cpp",
            Language::CSharp => "CSharp",
            Language::Php => "Php",
            Language::Ruby => "Ruby",
            Language::Swift => "Swift",
            Language::Kotlin => "Kotlin",
            Language::Scala => "Scala",
            Language::Haskell => "Haskell",
            Language::Shell => "Shell",
            Language::SQL => "SQL",
            Language::HTML => "HTML",
            Language::XML => "XML",
            Language::JSON => "JSON",
            Language::YAML => "YAML",
            Language::TOML => "TOML",
            Language::Markdown => "Markdown",
            Language::PlainText => "PlainText",
            Language::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Rust" => Ok(Language::Rust),
            "Python" => Ok(Language::Python),
            "JavaScript" => Ok(Language::JavaScript),
            "TypeScript" => Ok(Language::TypeScript),
            "Go" => Ok(Language::Go),
            "Java" => Ok(Language::Java),
            "C" => Ok(Language::C),
            "Cpp" => Ok(Language::Cpp),
            "CSharp" => Ok(Language::CSharp),
            "Php" => Ok(Language::Php),
            "Ruby" => Ok(Language::Ruby),
            "Swift" => Ok(Language::Swift),
            "Kotlin" => Ok(Language::Kotlin),
            "Scala" => Ok(Language::Scala),
            "Haskell" => Ok(Language::Haskell),
            "Shell" => Ok(Language::Shell),
            "SQL" => Ok(Language::SQL),
            "HTML" => Ok(Language::HTML),
            "XML" => Ok(Language::XML),
            "JSON" => Ok(Language::JSON),
            "YAML" => Ok(Language::YAML),
            "TOML" => Ok(Language::TOML),
            "Markdown" => Ok(Language::Markdown),
            "PlainText" => Ok(Language::PlainText),
            _ => Ok(Language::Unknown),
        }
    }
}

/// Semantic search result
///
/// A ranked result from vector similarity search. Results are ordered
/// by score (highest first) and include the matched code content with
/// file location information.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::SearchResult;
///
/// let result = SearchResult {
///     id: "chunk_abc123".to_string(),
///     file_path: "src/auth/login.rs".to_string(),
///     start_line: 42,
///     content: "pub fn authenticate(token: &str) -> Result<User>".to_string(),
///     score: 0.92,
///     metadata: serde_json::json!({
///         "type": "function",
///         "language": "rust",
///         "end_line": 55
///     }),
/// };
///
/// // High scores indicate strong semantic match
/// assert!(result.score > 0.9);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResult {
    /// Unique chunk identifier
    pub id: String,
    /// Source file path
    pub file_path: String,
    /// Starting line number in source
    pub start_line: u32,
    /// Matched code content
    pub content: String,
    /// Similarity score (0.0 to 1.0, higher is better)
    pub score: f32,
    /// Additional metadata (language, end_line, AST info)
    pub metadata: serde_json::Value,
}

/// Indexing statistics
///
/// Summary of a codebase indexing operation. Returned by the indexing
/// service after processing files to show progress and timing.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::IndexingStats;
///
/// let stats = IndexingStats {
///     total_files: 150,
///     indexed_files: 148,
///     total_chunks: 1200,
///     duration_ms: 5432,
/// };
///
/// // Calculate indexing rate
/// let files_per_sec = stats.indexed_files as f64 / (stats.duration_ms as f64 / 1000.0);
/// assert!(files_per_sec > 20.0, "Should index at least 20 files/sec");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IndexingStats {
    /// Total files discovered
    pub total_files: u32,
    /// Files successfully indexed
    pub indexed_files: u32,
    /// Total code chunks extracted
    pub total_chunks: u32,
    /// Total indexing time in milliseconds
    pub duration_ms: u64,
}

/// Configuration for embedding providers
///
/// Settings for configuring how code embeddings are generated. Different
/// providers have different requirements for API keys and models.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::EmbeddingConfig;
///
/// // Local FastEmbed (default, no API key needed)
/// let local = EmbeddingConfig::default();
/// assert_eq!(local.provider, "fastembed");
///
/// // OpenAI configuration
/// let openai = EmbeddingConfig {
///     provider: "openai".to_string(),
///     model: "text-embedding-3-small".to_string(),
///     api_key: Some("sk-...".to_string()),
///     base_url: None,
///     dimensions: Some(1536),
///     max_tokens: Some(8191),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct EmbeddingConfig {
    /// Provider name (openai, ollama, fastembed, etc.)
    #[validate(length(min = 1))]
    pub provider: String,
    /// Model identifier specific to the provider
    #[validate(length(min = 1))]
    pub model: String,
    /// API key for cloud providers
    pub api_key: Option<String>,
    /// Custom API endpoint URL
    pub base_url: Option<String>,
    /// Output embedding dimensions
    pub dimensions: Option<usize>,
    /// Maximum input token limit
    pub max_tokens: Option<usize>,
}

impl Default for EmbeddingConfig {
    fn default() -> Self {
        Self {
            provider: "fastembed".to_string(),
            model: "BAAI/bge-small-en-v1.5".to_string(),
            api_key: None,
            base_url: None,
            dimensions: Some(384),
            max_tokens: None,
        }
    }
}

/// Configuration for vector store providers
///
/// Settings for configuring where and how code embeddings are stored.
/// Different providers have different capabilities for persistence
/// and scalability.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::VectorStoreConfig;
///
/// // Local filesystem (default)
/// let local = VectorStoreConfig::default();
/// assert_eq!(local.provider, "filesystem");
///
/// // In-memory for testing
/// let test_config = VectorStoreConfig {
///     provider: "in-memory".to_string(),
///     address: None,
///     token: None,
///     collection: Some("test".to_string()),
///     dimensions: Some(384),
///     timeout_secs: Some(5),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct VectorStoreConfig {
    /// Provider name (filesystem, in-memory, milvus, edgevec)
    #[validate(length(min = 1))]
    pub provider: String,
    /// Server address for remote providers (e.g., Milvus)
    pub address: Option<String>,
    /// Authentication token for remote providers
    pub token: Option<String>,
    /// Collection/namespace for organizing embeddings
    pub collection: Option<String>,
    /// Expected embedding dimensions (must match provider)
    pub dimensions: Option<usize>,
    /// Operation timeout in seconds
    pub timeout_secs: Option<u64>,
}

impl Default for VectorStoreConfig {
    fn default() -> Self {
        Self {
            provider: "filesystem".to_string(),
            address: None,
            token: None,
            collection: None,
            dimensions: Some(384),
            timeout_secs: Some(30),
        }
    }
}

/// Sync batch for queue processing
///
/// Represents a batch of files queued for synchronization/re-indexing.
/// Used by the file watcher daemon to batch file changes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SyncBatch {
    /// Unique batch identifier
    pub id: String,
    /// Root path for the batch
    pub path: String,
    /// Unix timestamp when batch was created
    pub created_at: u64,
}

/// Statistics for repository operations
///
/// Aggregated metrics about the chunk repository state.
/// Used for monitoring storage usage and data distribution.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::RepositoryStats;
///
/// let stats = RepositoryStats {
///     total_chunks: 10_000,
///     total_collections: 5,
///     storage_size_bytes: 50 * 1024 * 1024,  // 50 MB
///     avg_chunk_size_bytes: 5120.0,          // ~5 KB avg
/// };
///
/// assert_eq!(stats.total_collections, 5);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RepositoryStats {
    /// Total chunks stored across all collections
    pub total_chunks: u64,
    /// Number of collections/namespaces
    pub total_collections: u64,
    /// Total storage used in bytes
    pub storage_size_bytes: u64,
    /// Average chunk size in bytes
    pub avg_chunk_size_bytes: f64,
}

/// Statistics for search operations
///
/// Aggregated metrics about search query performance.
/// Used by the admin dashboard for monitoring.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::SearchStats;
///
/// let stats = SearchStats {
///     total_queries: 5000,
///     avg_response_time_ms: 45.0,
///     cache_hit_rate: 0.75,
///     indexed_documents: 10_000,
/// };
///
/// assert!(stats.cache_hit_rate > 0.5);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchStats {
    /// Total queries executed
    pub total_queries: u64,
    /// Average response time in milliseconds
    pub avg_response_time_ms: f64,
    /// Cache hit rate (0.0 to 1.0)
    pub cache_hit_rate: f64,
    /// Number of indexed documents
    pub indexed_documents: u64,
}

/// File snapshot with metadata
///
/// Captures the state of a single file at a point in time.
/// Used for change detection in incremental indexing.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::FileSnapshot;
///
/// let snapshot = FileSnapshot {
///     path: "src/main.rs".to_string(),
///     size: 1024,
///     modified: 1705000000,
///     hash: "abc123def456".to_string(),
///     extension: "rs".to_string(),
/// };
///
/// assert_eq!(snapshot.extension, "rs");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FileSnapshot {
    /// Relative file path
    pub path: String,
    /// File size in bytes
    pub size: u64,
    /// Last modified timestamp (Unix epoch)
    pub modified: u64,
    /// Content hash for change detection
    pub hash: String,
    /// File extension (without dot)
    pub extension: String,
}

/// Codebase snapshot with all files
///
/// Complete state capture of a codebase at a point in time.
/// Used for incremental indexing by comparing snapshots.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::{CodebaseSnapshot, FileSnapshot};
/// use std::collections::HashMap;
///
/// let mut files = HashMap::new();
/// files.insert("src/main.rs".to_string(), FileSnapshot {
///     path: "src/main.rs".to_string(),
///     size: 1024,
///     modified: 1705000000,
///     hash: "abc123".to_string(),
///     extension: "rs".to_string(),
/// });
///
/// let snapshot = CodebaseSnapshot {
///     root_path: "/project".to_string(),
///     created_at: 1705000000,
///     files,
///     file_count: 1,
///     total_size: 1024,
/// };
///
/// assert_eq!(snapshot.file_count, 1);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CodebaseSnapshot {
    /// Absolute path to project root
    pub root_path: String,
    /// Snapshot creation timestamp (Unix epoch)
    pub created_at: u64,
    /// Map of relative paths to file snapshots
    pub files: std::collections::HashMap<String, FileSnapshot>,
    /// Total number of files in snapshot
    pub file_count: usize,
    /// Total size of all files in bytes
    pub total_size: u64,
}

/// Changes between snapshots
///
/// Diff result from comparing two codebase snapshots.
/// Used to determine which files need re-indexing.
///
/// # Example
///
/// ```rust
/// use mcp_context_browser::domain::types::SnapshotChanges;
///
/// let changes = SnapshotChanges {
///     added: vec!["src/new.rs".to_string()],
///     modified: vec!["src/main.rs".to_string()],
///     removed: vec!["src/old.rs".to_string()],
///     unchanged: vec!["src/lib.rs".to_string()],
/// };
///
/// assert!(changes.has_changes());
/// assert_eq!(changes.total_changes(), 3);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SnapshotChanges {
    /// Newly added file paths
    pub added: Vec<String>,
    /// Modified file paths
    pub modified: Vec<String>,
    /// Removed file paths
    pub removed: Vec<String>,
    /// Unchanged file paths
    pub unchanged: Vec<String>,
}

impl SnapshotChanges {
    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.modified.is_empty() || !self.removed.is_empty()
    }

    pub fn total_changes(&self) -> usize {
        self.added.len() + self.modified.len() + self.removed.len()
    }
}
