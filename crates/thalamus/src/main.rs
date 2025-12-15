//! Thalamus Service - Input Gateway and Embedding Generation
//!
//! Responsibilities:
//! - Accept raw input
//! - Normalize structure and metadata
//! - Generate embeddings using a single shared model version
//! - Emit feature packets to the Prefrontal Cortex

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use cortical_shared::{
    CorticalError, FeaturePacket, HealthResponse, IngestRequest, ThalamusConfig,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};
use uuid::Uuid;

mod embeddings;

use embeddings::EmbeddingService;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<ThalamusConfig>,
    pub embedding_service: Arc<RwLock<EmbeddingService>>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("thalamus=info".parse().unwrap()),
        )
        .init();

    // Load configuration
    let config = ThalamusConfig::default();
    info!(
        service = %config.base.service_name,
        version = %config.base.version,
        "Starting Thalamus service"
    );

    // Initialize embedding service
    let embedding_service = EmbeddingService::new(&config.embedding_model);
    
    let state = AppState {
        config: Arc::new(config.clone()),
        embedding_service: Arc::new(RwLock::new(embedding_service)),
    };

    // Build router
    let app = Router::new()
        .route("/health", get(health))
        .route("/ingest", post(ingest))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = format!("{}:{}", config.base.host, config.base.port);
    info!(address = %addr, "Server listening");
    
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Health check endpoint
async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let embedding_status = {
        let service = state.embedding_service.read().await;
        if service.is_ready() { "healthy" } else { "not_ready" }
    };

    Json(
        HealthResponse::healthy(&state.config.base.service_name, &state.config.base.version)
            .with_dependency("embedding_model", embedding_status),
    )
}

/// Ingest raw input, normalize it, and generate embeddings
#[instrument(skip(state), fields(text_length = request.text.len()))]
async fn ingest(
    State(state): State<AppState>,
    Json(request): Json<IngestRequest>,
) -> Result<Json<FeaturePacket>, AppError> {
    let experience_id = Uuid::new_v4();
    let timestamp = chrono::Utc::now();

    info!(
        experience_id = %experience_id,
        source = ?request.source,
        "Ingesting input"
    );

    // Validate text length
    if request.text.len() > state.config.max_text_length {
        return Err(AppError(CorticalError::Validation(format!(
            "Text exceeds maximum length of {} characters",
            state.config.max_text_length
        ))));
    }

    // Normalize text
    let normalized_text = normalize_text(&request.text);

    // Generate embedding
    let embedding = {
        let service = state.embedding_service.read().await;
        service
            .embed(&normalized_text)
            .map_err(|e| AppError(CorticalError::Embedding(e.to_string())))?
    };

    info!(
        experience_id = %experience_id,
        embedding_dim = embedding.len(),
        "Feature packet created"
    );

    Ok(Json(FeaturePacket {
        experience_id,
        timestamp,
        source: request.source,
        text: normalized_text,
        embedding,
        context: request.context,
    }))
}

/// Normalize input text for consistent processing
fn normalize_text(text: &str) -> String {
    // Strip leading/trailing whitespace
    let text = text.trim();
    
    // Collapse multiple whitespace to single space
    let mut result = String::with_capacity(text.len());
    let mut prev_whitespace = false;
    
    for c in text.chars() {
        if c.is_whitespace() {
            if !prev_whitespace {
                result.push(' ');
                prev_whitespace = true;
            }
        } else {
            result.push(c);
            prev_whitespace = false;
        }
    }
    
    result
}

/// Application error wrapper for proper HTTP responses
struct AppError(CorticalError);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self.0.status_code() {
            400 => StatusCode::BAD_REQUEST,
            404 => StatusCode::NOT_FOUND,
            503 => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({
            "error": self.0.to_string()
        });

        (status, Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_text() {
        assert_eq!(normalize_text("  hello  world  "), "hello world");
        assert_eq!(normalize_text("hello\n\nworld"), "hello world");
        assert_eq!(normalize_text("hello\t\tworld"), "hello world");
    }
}
