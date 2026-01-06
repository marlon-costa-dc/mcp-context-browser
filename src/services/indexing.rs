//! Indexing service for processing codebases

use crate::error::{Error, Result};
use crate::services::context::ContextService;
use crate::snapshot::{SnapshotManager, SnapshotChanges};
use crate::sync::SyncManager;
use crate::types::CodeChunk;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;

/// Advanced indexing service with snapshot-based incremental processing
pub struct IndexingService {
    context_service: Arc<ContextService>,
    snapshot_manager: SnapshotManager,
    sync_manager: Option<Arc<SyncManager>>,
}

impl IndexingService {
    /// Create a new indexing service
    pub fn new(context_service: Arc<ContextService>) -> Result<Self> {
        Ok(Self {
            context_service,
            snapshot_manager: SnapshotManager::new()?,
            sync_manager: None,
        })
    }

    /// Create indexing service with sync coordination
    pub fn with_sync_manager(
        context_service: Arc<ContextService>,
        sync_manager: Arc<SyncManager>
    ) -> Result<Self> {
        Ok(Self {
            context_service,
            snapshot_manager: SnapshotManager::new()?,
            sync_manager: Some(sync_manager),
        })
    }

    /// Index a directory with incremental processing and sync coordination
    pub async fn index_directory(&self, path: &Path, collection: &str) -> Result<usize> {
        if !path.exists() || !path.is_dir() {
            return Err(Error::not_found("Directory not found"));
        }

        // Canonicalize path for consistent snapshots
        let canonical_path = path.canonicalize()
            .map_err(|e| Error::io(format!("Failed to canonicalize path: {}", e)))?;

        // Check if sync is needed (if sync manager is available)
        if let Some(sync_mgr) = &self.sync_manager {
            if sync_mgr.should_debounce(&canonical_path).await? {
                println!("[INDEX] Skipping {} - debounced", canonical_path.display());
                return Ok(0);
            }
        }

        // Get changed files using snapshots
        let changed_files = self.snapshot_manager.get_changed_files(&canonical_path).await?;
        println!("[INDEX] Found {} changed files in {}", changed_files.len(), canonical_path.display());

        if changed_files.is_empty() {
            return Ok(0);
        }

        let mut total_chunks = 0;

        // Process changed files
        for file_path in &changed_files {
            let full_path = canonical_path.join(file_path);

            // Only process supported file types
            if let Some(ext) = full_path.extension().and_then(|e| e.to_str()) {
                if self.is_supported_file_type(ext) {
                    match self.process_file(&full_path).await {
                        Ok(file_chunks) => {
                            if !file_chunks.is_empty() {
                                // Store chunks with better error handling
                                match self.context_service.store_chunks(collection, &file_chunks).await {
                                    Ok(()) => {
                                        total_chunks += file_chunks.len();
                                        println!("[INDEX] Processed {} chunks from {}", file_chunks.len(), file_path);
                                    }
                                    Err(e) => {
                                        eprintln!("[INDEX] Failed to store chunks for {}: {}", file_path, e);
                                        // Continue with other files
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("[INDEX] Failed to process {}: {}", file_path, e);
                            // Continue with other files
                        }
                    }
                }
            }
        }

        // Update sync timestamp if sync manager is available
        if let Some(sync_mgr) = &self.sync_manager {
            sync_mgr.update_last_sync(&canonical_path).await;
        }

        println!("[INDEX] Completed indexing {} files with {} total chunks", changed_files.len(), total_chunks);
        Ok(total_chunks)
    }

    /// Process a single file into intelligent chunks
    async fn process_file(&self, path: &Path) -> Result<Vec<CodeChunk>> {
        let content = fs::read_to_string(path)
            .map_err(|e| Error::io(format!("Failed to read file {}: {}", path.display(), e)))?;

        if content.trim().is_empty() {
            return Ok(Vec::new());
        }

        let file_name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let file_path = path.display().to_string();
        let language = self.detect_language(path)?;

        // Use intelligent chunking based on language
        let chunks = match language {
            crate::core::types::Language::Rust => self.chunk_rust_code(&content, &file_name, &file_path),
            crate::core::types::Language::Python => self.chunk_python_code(&content, &file_name, &file_path),
            crate::core::types::Language::JavaScript | crate::core::types::Language::TypeScript =>
                self.chunk_js_code(&content, &file_name, &file_path, language),
            _ => self.chunk_generic_code(&content, &file_name, &file_path, language),
        };

        Ok(chunks)
    }

    /// Detect programming language from file extension
    fn detect_language(&self, path: &Path) -> Result<crate::core::types::Language> {
        let ext = path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "rs" => Ok(crate::core::types::Language::Rust),
            "py" => Ok(crate::core::types::Language::Python),
            "js" => Ok(crate::core::types::Language::JavaScript),
            "ts" => Ok(crate::core::types::Language::TypeScript),
            "java" => Ok(crate::core::types::Language::Java),
            "cpp" | "cc" | "cxx" => Ok(crate::core::types::Language::Cpp),
            "c" => Ok(crate::core::types::Language::C),
            "go" => Ok(crate::core::types::Language::Go),
            "php" => Ok(crate::core::types::Language::Php),
            "rb" => Ok(crate::core::types::Language::Ruby),
            _ => Ok(crate::core::types::Language::Unknown),
        }
    }

    /// Check if file type is supported for indexing
    fn is_supported_file_type(&self, ext: &str) -> bool {
        matches!(ext.to_lowercase().as_str(),
            "rs" | "py" | "js" | "ts" | "java" | "cpp" | "cc" | "cxx" | "c" |
            "go" | "php" | "rb" | "scala" | "kt" | "swift" | "cs" | "fs" | "vb" |
            "pl" | "pm" | "sh" | "bash" | "zsh" | "fish" | "ps1" | "sql" |
            "html" | "xml" | "json" | "yaml" | "yml" | "toml" | "ini" | "cfg" |
            "md" | "txt" | "rst")
    }

    /// Intelligent chunking for Rust code
    fn chunk_rust_code(&self, content: &str, file_name: &str, file_path: &str) -> Vec<CodeChunk> {
        let mut chunks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        // Find function/struct/impl blocks
        let mut current_block = Vec::new();
        let mut block_start = 0;
        let mut brace_count = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Skip empty lines and comments at block level
            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            // Count braces
            brace_count += line.chars().filter(|&c| c == '{').count();
            brace_count -= line.chars().filter(|&c| c == '}').count();

            current_block.push(line.clone());

            // Check if this line starts a significant block
            if trimmed.starts_with("fn ") ||
               trimmed.starts_with("struct ") ||
               trimmed.starts_with("impl ") ||
               trimmed.starts_with("trait ") ||
               trimmed.starts_with("mod ") ||
               (trimmed.starts_with("pub ") &&
                (trimmed.contains("fn ") || trimmed.contains("struct ") || trimmed.contains("impl "))) {

                if current_block.len() > 1 {
                    // Create chunk for previous content
                    self.create_chunk(&current_block[..current_block.len()-1], block_start, i-1, file_name, file_path, crate::core::types::Language::Rust, &mut chunks);
                    current_block = vec![line.clone()];
                    block_start = i;
                }
            }

            // End of block (brace count returns to 0)
            if brace_count == 0 && !current_block.is_empty() && current_block.len() > 2 {
                self.create_chunk(&current_block, block_start, i, file_name, file_path, crate::core::types::Language::Rust, &mut chunks);
                current_block.clear();
                block_start = i + 1;
            }
        }

        // Add remaining content
        if !current_block.is_empty() {
            self.create_chunk(&current_block, block_start, lines.len() - 1, file_name, file_path, crate::core::types::Language::Rust, &mut chunks);
        }

        // Fallback to line-based chunking if no blocks found
        if chunks.is_empty() {
            self.chunk_by_lines(content, file_name, file_path, crate::core::types::Language::Rust)
        } else {
            chunks
        }
    }

    /// Chunking for Python code
    fn chunk_python_code(&self, content: &str, file_name: &str, file_path: &str) -> Vec<CodeChunk> {
        let mut chunks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut current_function = Vec::new();
        let mut function_start = 0;
        let mut indent_level = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            // Detect function/class definition
            if trimmed.starts_with("def ") || trimmed.starts_with("class ") {
                if !current_function.is_empty() {
                    self.create_chunk(&current_function, function_start, i-1, file_name, file_path, crate::core::types::Language::Python, &mut chunks);
                    current_function.clear();
                }
                current_function.push(line.clone());
                function_start = i;
                indent_level = line.chars().take_while(|c| c.is_whitespace()).count();
            } else if !current_function.is_empty() {
                let current_indent = line.chars().take_while(|c| c.is_whitespace()).count();

                // If we go back to the same or less indentation, end the function
                if current_indent <= indent_level && !line.chars().all(|c| c.is_whitespace()) {
                    self.create_chunk(&current_function, function_start, i-1, file_name, file_path, crate::core::types::Language::Python, &mut chunks);
                    current_function.clear();

                    // Start new chunk with this line if it's significant
                    if trimmed.starts_with("def ") || trimmed.starts_with("class ") {
                        current_function.push(line.clone());
                        function_start = i;
                        indent_level = current_indent;
                    }
                } else {
                    current_function.push(line.clone());
                }
            } else if trimmed.starts_with("def ") || trimmed.starts_with("class ") {
                current_function.push(line.clone());
                function_start = i;
                indent_level = line.chars().take_while(|c| c.is_whitespace()).count();
            }
        }

        // Add remaining function
        if !current_function.is_empty() {
            self.create_chunk(&current_function, function_start, lines.len() - 1, file_name, file_path, crate::core::types::Language::Python, &mut chunks);
        }

        if chunks.is_empty() {
            self.chunk_by_lines(content, file_name, file_path, crate::core::types::Language::Python)
        } else {
            chunks
        }
    }

    /// Chunking for JavaScript/TypeScript
    fn chunk_js_code(&self, content: &str, file_name: &str, file_path: &str, language: crate::core::types::Language) -> Vec<CodeChunk> {
        let mut chunks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut current_function = Vec::new();
        let mut function_start = 0;
        let mut brace_count = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            // Count braces
            brace_count += line.chars().filter(|&c| c == '{').count();
            brace_count -= line.chars().filter(|&c| c == '}').count();

            // Detect function/class definition
            if trimmed.starts_with("function ") ||
               trimmed.starts_with("const ") && trimmed.contains("=>") ||
               trimmed.starts_with("class ") ||
               trimmed.contains("= function") ||
               (trimmed.starts_with("export ") && (
                   trimmed.contains("function ") ||
                   trimmed.contains("class ") ||
                   trimmed.contains("const ") && trimmed.contains("=>")
               )) {

                if !current_function.is_empty() {
                    self.create_chunk(&current_function, function_start, i-1, file_name, file_path, language.clone(), &mut chunks);
                    current_function.clear();
                }
                current_function.push(line.clone());
                function_start = i;
            } else if !current_function.is_empty() {
                current_function.push(line.clone());

                // End function when braces balance
                if brace_count == 0 && current_function.len() > 1 {
                    self.create_chunk(&current_function, function_start, i, file_name, file_path, language.clone(), &mut chunks);
                    current_function.clear();
                }
            }
        }

        // Add remaining function
        if !current_function.is_empty() {
            self.create_chunk(&current_function, function_start, lines.len() - 1, file_name, file_path, language, &mut chunks);
        }

        if chunks.is_empty() {
            self.chunk_by_lines(content, file_name, file_path, language)
        } else {
            chunks
        }
    }

    /// Generic chunking for unsupported languages
    fn chunk_generic_code(&self, content: &str, file_name: &str, file_path: &str, language: crate::core::types::Language) -> Vec<CodeChunk> {
        self.chunk_by_lines(content, file_name, file_path, language)
    }

    /// Fallback: chunk by lines with reasonable sizes
    fn chunk_by_lines(&self, content: &str, file_name: &str, file_path: &str, language: crate::core::types::Language) -> Vec<CodeChunk> {
        let lines: Vec<&str> = content.lines().collect();
        let mut chunks = Vec::new();

        // Group lines into chunks of reasonable size
        let chunk_size = 10; // lines per chunk

        for (chunk_idx, chunk_lines) in lines.chunks(chunk_size).enumerate() {
            let start_line = chunk_idx * chunk_size;
            let end_line = start_line + chunk_lines.len() - 1;

            let content = chunk_lines.join("\n");
            if content.trim().is_empty() {
                continue;
            }

            let chunk = CodeChunk {
                id: format!("{}_{}", file_name, chunk_idx),
                content,
                file_path: file_path.clone(),
                start_line: start_line as u32,
                end_line: end_line as u32,
                language: language.clone(),
                embedding: None,
                metadata: {
                    let mut meta = HashMap::new();
                    meta.insert("file".to_string(), serde_json::json!(file_name));
                    meta.insert("chunk_index".to_string(), serde_json::json!(chunk_idx));
                    meta
                },
            };
            chunks.push(chunk);
        }

        chunks
    }

    /// Create a chunk from lines
    fn create_chunk(
        &self,
        lines: &[String],
        start_line: usize,
        end_line: usize,
        file_name: &str,
        file_path: &str,
        language: crate::core::types::Language,
        chunks: &mut Vec<CodeChunk>
    ) {
        let content = lines.join("\n").trim().to_string();
        if content.is_empty() {
            return;
        }

        // Skip very small chunks
        if content.len() < 10 {
            return;
        }

        let chunk = CodeChunk {
            id: format!("{}_{}_{}", file_name, start_line, end_line),
            content,
            file_path: file_path.to_string(),
            start_line: start_line as u32,
            end_line: end_line as u32,
            language,
            embedding: None,
            metadata: {
                let mut meta = HashMap::new();
                meta.insert("file".to_string(), serde_json::json!(file_name));
                meta.insert("type".to_string(), serde_json::json!("code_block"));
                meta
            },
        };
        chunks.push(chunk);
    }
}