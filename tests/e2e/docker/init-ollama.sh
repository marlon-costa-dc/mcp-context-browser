#!/bin/bash

# Wait for Ollama to be ready
echo "Waiting for Ollama to be ready..."
while ! curl -s http://localhost:11434/api/version > /dev/null; do
  sleep 1
done

echo "Ollama is ready. Pulling embedding model..."
ollama pull nomic-embed-text

echo "Model ready for testing."