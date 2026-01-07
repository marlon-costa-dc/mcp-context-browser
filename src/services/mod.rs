//! Business logic services

use crate::core::types::{CodeChunk, SearchResult};
use crate::factory::ServiceProvider;
use crate::providers::{EmbeddingProvider, VectorStoreProvider};
use std::path::Path;
use std::sync::Arc;

/// Context service for managing embeddings and vector storage
pub struct ContextService {
    embedding_provider: Arc<dyn EmbeddingProvider>,
    vector_store_provider: Arc<dyn VectorStoreProvider>,
}

impl ContextService {
    pub async fn new_with_config(
        embedding_config: &crate::core::types::EmbeddingConfig,
        vector_store_config: &crate::core::types::VectorStoreConfig,
        service_provider: &ServiceProvider,
    ) -> crate::core::error::Result<Self> {
        let embedding_provider = service_provider.get_embedding_provider(embedding_config).await?;
        let vector_store_provider = service_provider.get_vector_store_provider(vector_store_config).await?;

        Ok(Self {
            embedding_provider,
            vector_store_provider,
        })
    }

    pub fn new(_service_provider: &ServiceProvider) -> crate::core::error::Result<Self> {
        // For backward compatibility, use default providers
        let _embedding_config = crate::core::types::EmbeddingConfig {
            provider: "mock".to_string(),
            model: "mock".to_string(),
            api_key: None,
            base_url: None,
            dimensions: Some(128),
            max_tokens: Some(512),
        };

        let _vector_store_config = crate::core::types::VectorStoreConfig {
            provider: "in-memory".to_string(),
            address: None,
            token: None,
            collection: None,
            dimensions: Some(128),
        };

        // Create synchronously for backward compatibility
        // In production, this should be async
        let embedding_provider = Arc::new(crate::providers::MockEmbeddingProvider::new());
        let vector_store_provider = Arc::new(crate::providers::InMemoryVectorStoreProvider::new());

        Ok(Self {
            embedding_provider,
            vector_store_provider,
        })
    }

    pub async fn embed_text(
        &self,
        text: &str,
    ) -> crate::core::error::Result<crate::core::types::Embedding> {
        self.embedding_provider.embed(text).await
    }

    pub async fn store_chunks(
        &self,
        collection: &str,
        chunks: &[CodeChunk],
    ) -> crate::core::error::Result<()> {
        let texts: Vec<String> = chunks.iter().map(|c| c.content.clone()).collect();
        let embeddings = self.embedding_provider.embed_batch(&texts).await?;

        // Create metadata for each chunk
        let metadata: Vec<std::collections::HashMap<String, serde_json::Value>> = chunks
            .iter()
            .map(|chunk| {
                let mut meta = std::collections::HashMap::new();
                meta.insert(
                    "content".to_string(),
                    serde_json::Value::String(chunk.content.clone()),
                );
                meta.insert(
                    "file_path".to_string(),
                    serde_json::Value::String(chunk.file_path.clone()),
                );
                meta.insert(
                    "start_line".to_string(),
                    serde_json::Value::Number(chunk.start_line.into()),
                );
                meta.insert(
                    "end_line".to_string(),
                    serde_json::Value::Number(chunk.end_line.into()),
                );
                meta.insert(
                    "language".to_string(),
                    serde_json::Value::String(format!("{:?}", chunk.language)),
                );
                meta
            })
            .collect();

        self.vector_store_provider
            .insert_vectors(collection, &embeddings, metadata)
            .await?;
        Ok(())
    }

    pub async fn search_similar(
        &self,
        collection: &str,
        query: &str,
        limit: usize,
    ) -> crate::core::error::Result<Vec<SearchResult>> {
        let query_embedding = self.embed_text(query).await?;
        let results = self
            .vector_store_provider
            .search_similar(collection, &query_embedding.vector, limit, None)
            .await?;

        let search_results = results;

        Ok(search_results)
    }
}

/// Indexing service for processing codebases
pub struct IndexingService {
    #[allow(dead_code)]
    context_service: Arc<ContextService>,
}

impl IndexingService {
    pub fn new(context_service: Arc<ContextService>) -> Self {
        Self { context_service }
    }

    pub async fn index_directory(
        &self,
        path: &Path,
        collection: &str,
    ) -> crate::core::error::Result<usize> {
        use crate::chunking::IntelligentChunker;
        use walkdir::WalkDir;

        let chunker = IntelligentChunker::new();
        let mut total_chunks = 0;

        // First, create the collection if it doesn't exist
        let dimensions = 768; // This should come from the embedding provider
        if !self.context_service.vector_store_provider.collection_exists(collection).await? {
            self.context_service.vector_store_provider.create_collection(collection, dimensions).await?;
        }

        // Walk through all files in the directory
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            // Skip directories and non-text files
            if path.is_dir() {
                continue;
            }

            // Check if it's a code file we can process
            if let Some(extension) = path.extension() {
                if !Self::is_supported_file(extension) {
                    continue;
                }
            } else {
                continue;
            }

            // Read and process the file
            match tokio::fs::read_to_string(path).await {
                Ok(content) => {
                    // Detect language from file extension
                    let language = path.extension()
                        .and_then(|ext| ext.to_str())
                        .map(crate::core::types::Language::from_extension)
                        .unwrap_or(crate::core::types::Language::Unknown);

                    // Chunk the content
                    let chunks = chunker.chunk_code(&content, path.to_string_lossy().as_ref(), language);

                    if !chunks.is_empty() {
                        // Store the chunks
                        self.context_service.store_chunks(collection, &chunks).await?;
                        total_chunks += chunks.len();

                        println!("Indexed {} chunks from {}", chunks.len(), path.display());
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read file {}: {}", path.display(), e);
                    continue;
                }
            }
        }

        // Flush the collection to ensure all data is persisted
        self.context_service.vector_store_provider.flush(collection).await?;

        Ok(total_chunks)
    }

    /// Check if a file extension is supported for indexing
    fn is_supported_file(extension: &std::ffi::OsStr) -> bool {
        let supported_extensions = [
            "rs", "py", "js", "ts", "java", "cpp", "c", "cc", "cxx", "h", "hpp",
            "go", "rb", "php", "swift", "kt", "scala", "hs", "ml", "fs", "cs",
            "vb", "sql", "html", "css", "scss", "less", "json", "xml", "yaml",
            "yml", "toml", "ini", "cfg", "conf", "sh", "bash", "zsh", "fish",
            "ps1", "bat", "cmd", "dockerfile", "makefile", "cmake"
        ];

        extension
            .to_str()
            .map(|ext| supported_extensions.contains(&ext.to_lowercase().as_str()))
            .unwrap_or(false)
    }
}

/// Search service for querying indexed code
pub struct SearchService {
    context_service: Arc<ContextService>,
}

impl SearchService {
    pub fn new(context_service: Arc<ContextService>) -> Self {
        Self { context_service }
    }

    pub async fn search(
        &self,
        collection: &str,
        query: &str,
        limit: usize,
    ) -> crate::core::error::Result<Vec<SearchResult>> {
        self.context_service
            .search_similar(collection, query, limit)
            .await
    }
}
