//! Embedding service for Thalamus.
//! 
//! Generates vector embeddings for text input.
//! Currently uses a mock implementation - replace with actual model inference.

use thiserror::Error;
use tracing::info;

/// Service for generating text embeddings
pub struct EmbeddingService {
    model_name: String,
    vector_size: usize,
    ready: bool,
}

impl EmbeddingService {
    /// Create a new embedding service
    pub fn new(model_name: &str) -> Self {
        info!(model = %model_name, "Initializing embedding service");
        
        // Determine vector size based on model
        let vector_size = match model_name {
            "all-MiniLM-L6-v2" => 384,
            "all-mpnet-base-v2" => 768,
            _ => 384, // default
        };

        Self {
            model_name: model_name.to_string(),
            vector_size,
            ready: true, // Mock is always ready
        }
    }

    /// Check if the service is ready
    pub fn is_ready(&self) -> bool {
        self.ready
    }

    /// Generate embedding for text
    /// 
    /// TODO: Replace with actual model inference using candle or onnxruntime
    pub fn embed(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        if !self.ready {
            return Err(EmbeddingError::NotReady);
        }

        // Mock embedding: deterministic based on text hash
        // This allows consistent results for testing
        let hash = simple_hash(text);
        let embedding = generate_mock_embedding(hash, self.vector_size);

        Ok(embedding)
    }

    /// Generate embeddings for multiple texts
    pub fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        texts.iter().map(|t| self.embed(t)).collect()
    }

    /// Get the model name
    pub fn model_name(&self) -> &str {
        &self.model_name
    }

    /// Get the vector size
    pub fn vector_size(&self) -> usize {
        self.vector_size
    }
}

/// Errors that can occur during embedding generation
#[derive(Debug, Error)]
pub enum EmbeddingError {
    #[error("Embedding service not ready")]
    NotReady,
    
    #[error("Model inference failed: {0}")]
    InferenceFailed(String),
}

/// Simple hash function for deterministic mock embeddings
fn simple_hash(text: &str) -> u64 {
    let mut hash: u64 = 5381;
    for byte in text.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }
    hash
}

/// Generate a mock embedding vector from a hash seed
fn generate_mock_embedding(seed: u64, size: usize) -> Vec<f32> {
    let mut embedding = Vec::with_capacity(size);
    let mut state = seed;
    
    for _ in 0..size {
        // Simple LCG for reproducible pseudo-random numbers
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        // Convert to float in range [-1, 1]
        let value = ((state >> 33) as f32 / (u32::MAX as f32)) * 2.0 - 1.0;
        embedding.push(value);
    }
    
    // Normalize to unit vector
    let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
    if magnitude > 0.0 {
        for v in &mut embedding {
            *v /= magnitude;
        }
    }
    
    embedding
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_service_creation() {
        let service = EmbeddingService::new("all-MiniLM-L6-v2");
        assert!(service.is_ready());
        assert_eq!(service.vector_size(), 384);
    }

    #[test]
    fn test_embedding_generation() {
        let service = EmbeddingService::new("all-MiniLM-L6-v2");
        let embedding = service.embed("hello world").unwrap();
        assert_eq!(embedding.len(), 384);
    }

    #[test]
    fn test_embedding_deterministic() {
        let service = EmbeddingService::new("all-MiniLM-L6-v2");
        let e1 = service.embed("hello world").unwrap();
        let e2 = service.embed("hello world").unwrap();
        assert_eq!(e1, e2);
    }

    #[test]
    fn test_embedding_normalized() {
        let service = EmbeddingService::new("all-MiniLM-L6-v2");
        let embedding = service.embed("test text").unwrap();
        let magnitude: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 0.001);
    }
}
