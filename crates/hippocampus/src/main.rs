//! Hippocampus Service - Episodic Memory
//!
//! Responsibilities:
//! - Store ordered sequences of experiences
//! - Support time-aware similarity queries
//! - Link related experiences into sessions

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use cortical_shared::{
    CorticalError, HealthResponse, QueryRequest, QueryResponse, RegionConfig,
    SimilarityResult, StoreRequest, StoreResponse,
};
use std::sync::Arc;
use std::time::Instant;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};

/// Application state
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<RegionConfig>,
    // TODO: Add Qdrant client
    // TODO: Add PostgreSQL pool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("hippocampus=info".parse().unwrap()),
        )
        .init();

    let mut config = RegionConfig::default();
    config.base.service_name = "hippocampus".to_string();

    info!(
        service = %config.base.service_name,
        version = %config.base.version,
        "Starting Hippocampus service"
    );

    let state = AppState {
        config: Arc::new(config.clone()),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/store", post(store))
        .route("/query", post(query))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("{}:{}", config.base.host, config.base.port);
    info!(address = %addr, "Server listening");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(
        HealthResponse::healthy(&state.config.base.service_name, &state.config.base.version)
            .with_dependency("qdrant", "healthy")
            .with_dependency("postgres", "healthy"),
    )
}

#[instrument(skip(state))]
async fn store(
    State(state): State<AppState>,
    Json(request): Json<StoreRequest>,
) -> Result<Json<StoreResponse>, AppError> {
    let experience_id = request.experience.experience_id;

    info!(experience_id = %experience_id, "Storing experience");

    // TODO: Store embedding in Qdrant
    // TODO: Store metadata in PostgreSQL

    Ok(Json(StoreResponse {
        experience_id,
        stored: true,
        region: state.config.base.service_name.clone(),
        message: Some("Experience stored in episodic memory".to_string()),
    }))
}

#[instrument(skip(state, request))]
async fn query(
    State(state): State<AppState>,
    Json(request): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, AppError> {
    let start = Instant::now();

    info!(limit = request.limit, "Querying episodic memory");

    // TODO: Query Qdrant for similar embeddings
    // TODO: Enrich with metadata from PostgreSQL

    let results: Vec<SimilarityResult> = vec![];

    Ok(Json(QueryResponse {
        results,
        region: state.config.base.service_name.clone(),
        query_time_ms: start.elapsed().as_secs_f64() * 1000.0,
    }))
}

struct AppError(CorticalError);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self.0.status_code() {
            400 => StatusCode::BAD_REQUEST,
            404 => StatusCode::NOT_FOUND,
            503 => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::json!({ "error": self.0.to_string() });
        (status, Json(body)).into_response()
    }
}
