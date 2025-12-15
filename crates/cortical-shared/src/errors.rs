//! Unified error types for Cortical Compose services.

use thiserror::Error;

/// Unified error type for all Cortical Compose services
#[derive(Debug, Error)]
pub enum CorticalError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Vector store error: {0}")]
    VectorStore(String),

    #[error("Embedding error: {0}")]
    Embedding(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("HTTP client error: {0}")]
    HttpClient(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

impl CorticalError {
    /// Returns the appropriate HTTP status code for this error
    pub fn status_code(&self) -> u16 {
        match self {
            CorticalError::Validation(_) => 400,
            CorticalError::NotFound(_) => 404,
            CorticalError::ServiceUnavailable(_) => 503,
            CorticalError::Database(_)
            | CorticalError::VectorStore(_)
            | CorticalError::Embedding(_)
            | CorticalError::Internal(_)
            | CorticalError::HttpClient(_)
            | CorticalError::Json(_) => 500,
        }
    }

    /// Returns true if this error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            CorticalError::ServiceUnavailable(_)
                | CorticalError::Database(_)
                | CorticalError::VectorStore(_)
        )
    }
}

/// Result type alias for Cortical operations
pub type CorticalResult<T> = Result<T, CorticalError>;
