# FastEmbed Provider

The FastEmbed Provider offers high-quality local embeddings without external API dependencies.

## Overview

\1-  **Type**: Local Embedding Provider
\1-  **Model**: AllMiniLML6V2 (384 dimensions)
\1-  **Dependencies**: Zero external APIs
\1-  **Performance**: Optimized ONNX inference
\1-  **Download**: Model downloaded automatically on first execution

## Configuration

### Basic Configuration

```yaml
embedding:
  provider: "fastembed"
  model: "AllMiniLML6V2"  # optional, default is this
  dimensions: 384         # optional, default is 384
  max_tokens: 512         # optional, default is 512
```

### Full Configuration

```yaml
embedding:
  provider: "fastembed"
  model: "AllMiniLML6V2"
  dimensions: 384
  max_tokens: 512
```

## Features

### Advantages

✅**Completely Local**: No external APIs or API keys required
✅**High Performance**: Uses optimized ONNX inference
✅**Quality Models**: Based on sentence-transformers
✅**Intelligent Cache**: Model downloaded once and cached
✅**Compatibility**: Same output format as OpenAI/Gemini

### Limitations

⚠️**Initial Download**: Requires internet to download the model on first execution (~23MB)
⚠️**Memory**: Model loaded in RAM
⚠️**CPU**: Inference on CPU (not GPU)

## Supported Models

| Model | Dimensions | Description |
|--------|-----------|-----------|
| AllMiniLML6V2 | 384 | Default model, good balance of quality/performance |
| AllMiniLML12V2 | 384 | Larger version with better quality |
| AllMpnetBaseV2 | 768 | High-quality model |
| BGE Models | Varied | Family of optimized models |

## Programmatic Usage

```rust
use mcp_context_browser::providers::embedding::FastEmbedProvider;

// Create provider with default model
let provider = FastEmbedProvider::new()?;

// Or specify model
let provider = FastEmbedProvider::with_model(fastembed::EmbeddingModel::AllMiniLML12V2)?;

// Generate embedding
let embedding = provider.embed("Your text here").await?;
println!("Dimensions: {}", embedding.dimensions);
println!("Model: {}", embedding.model);

// Generate embeddings in batch
let texts = vec!["Text 1".to_string(), "Text 2".to_string()];
let embeddings = provider.embed_batch(&texts).await?;
```

## Performance

### Expected Benchmarks

\1-  **Initialization**: ~2-5 seconds (model download)
\1-  **Single embedding**: ~10-50ms
\1-  **Batch of 100**: ~100-500ms
\1-  **Memory**: ~100-500MB (depending on model)

### Optimization

For better performance:

\1-   Use batch embedding when possible
\1-   Cache embeddings when appropriate
\1-   Consider smaller models for applications with memory constraints

## Troubleshooting

### Problem: "Failed to initialize FastEmbed model"

**Solution**: Check internet connection for model download on first execution.

### Problem: "Out of memory"

**Solution**: Use a smaller model or increase available memory.

### Problem: "Model download failed"

**Solution**:

1.  Check internet connection
2.  Check write permissions in cache directory
3.  Try again (downloads are resumable)

## Comparison with Other Providers

| Provider | Local | API Key | Performance | Quality |
|----------|-------|---------|-------------|-----------|
| FastEmbed | ✅ | ❌ | High | High |
| Ollama | ✅ | ❌ | Medium | High |
| OpenAI | ❌ | ✅ | Very High | Very High |
| Mock | ✅ | ❌ | Very High | Low |

## Technical Architecture

The FastEmbed Provider:

1.  Uses the `fastembed` library for ONNX inference
2.  Loads optimized sentence-transformers models
3.  Implements the `EmbeddingProvider` trait of MCP Context Browser
4.  Provides a consistent interface with other providers
5.  Manages automatic model cache

## Next Steps

\1-   Support for more FastEmbed models
\1-   Configuration of execution providers (CPU/GPU)
\1-   Automatic quantization to reduce memory usage
\1-   Embedding cache for frequent texts
