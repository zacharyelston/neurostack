//! Prefrontal Cortex Service - Decision Orchestration
//!
//! Implements three internal pathways (from topology.yaml TL;DR):
//! - `fast_reflex()` - Amygdala-negative threat response
//! - `deliberate_loop()` - Full region consultation (hippo + bg + cereb + amyg)
//! - `thalamus_feedback_update()` - Attention/gating tuning
//!
//! This gets 90% of the realism for 10% of the complexity.

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use cortical_shared::{
    CorticalError, Decision, DecisionAction, DecisionRequest, ExperienceSource,
    FeedbackRequest, GateDecision, HealthResponse, PathwayType, PolicyConfig,
    PrefrontalCortexConfig, RegionClient, ThalamusClient,
};
use std::sync::Arc;
use std::time::Instant;
use tower_http::trace::TraceLayer;
use tracing::{info, instrument, warn};

mod pathways;
use pathways::{deliberate_loop, fast_reflex, select_pathway};

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub config: Arc<PrefrontalCortexConfig>,
    pub policies: Arc<PolicyConfig>,
    pub thalamus: ThalamusClient,
    pub hippocampus: RegionClient,
    pub basal_ganglia: RegionClient,
    pub cerebellum: RegionClient,
    pub amygdala: RegionClient,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("prefrontal_cortex=info".parse().unwrap()),
        )
        .init();

    let config = PrefrontalCortexConfig::default();
    info!(
        service = %config.base.service_name,
        version = %config.base.version,
        "Starting Prefrontal Cortex service"
    );

    // Load policy configuration (defaults from expose-knobs.yaml)
    let policies = PolicyConfig::default();
    
    let state = AppState {
        thalamus: ThalamusClient::new(&config.thalamus_url),
        hippocampus: RegionClient::new(&config.hippocampus_url, "hippocampus"),
        basal_ganglia: RegionClient::new(&config.basal_ganglia_url, "basal_ganglia"),
        cerebellum: RegionClient::new(&config.cerebellum_url, "cerebellum"),
        amygdala: RegionClient::new(&config.amygdala_url, "amygdala"),
        config: Arc::new(config.clone()),
        policies: Arc::new(policies),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/decide", post(decide))
        .route("/feedback", post(feedback))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("{}:{}", config.base.host, config.base.port);
    info!(address = %addr, "Server listening");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse::healthy(
        &state.config.base.service_name,
        &state.config.base.version,
    ))
}

#[instrument(skip(state), fields(text_length = request.input_text.len()))]
async fn decide(
    State(state): State<AppState>,
    Json(request): Json<DecisionRequest>,
) -> Result<Json<Decision>, AppError> {
    let start = Instant::now();

    info!("Processing decision request via pathway model");

    // Step 1: Get embedding from Thalamus
    let feature_packet = state
        .thalamus
        .ingest(&request.input_text, ExperienceSource::Input, Some(request.context.clone()))
        .await
        .map_err(AppError)?;

    let experience_id = feature_packet.experience_id;
    let embedding = feature_packet.embedding;

    info!(experience_id = %experience_id, "Feature packet received");

    // Step 2: Fast path check (amygdala-negative threat detection)
    let fast_check = fast_reflex(&state.amygdala, &embedding, &state.policies.fast_path)
        .await
        .map_err(AppError)?;

    if fast_check.triggered {
        let elapsed_ms = start.elapsed().as_millis() as f64;
        info!(
            experience_id = %experience_id,
            pathway = "fast_reflex",
            elapsed_ms = elapsed_ms,
            "Fast path triggered"
        );

        return Ok(Json(Decision {
            experience_id,
            action: DecisionAction::Defer, // Fast path = defer to safety
            confidence: fast_check.threat_confidence.unwrap_or(0.0),
            reasoning: fast_check.reason,
            suggested_response: Some("Threat pattern detected - exercise caution".to_string()),
            similar_experiences: vec![],
            regions_consulted: vec!["amygdala".to_string()],
        }));
    }

    // Step 3: Deliberate loop (full region consultation)
    let limit = state.config.max_similar_results;
    let deliberate_result = deliberate_loop(
        &state.hippocampus,
        &state.basal_ganglia,
        &state.cerebellum,
        &state.amygdala,
        &embedding,
        limit,
        &state.policies.slow_path,
    )
    .await
    .map_err(AppError)?;

    // Step 4: Select pathway and determine action
    let (pathway, reasoning) = select_pathway(&fast_check, &deliberate_result, &state.policies);

    let action = match pathway {
        PathwayType::FastReflex => DecisionAction::Defer,
        PathwayType::DeliberateLoop => {
            // Check BG gate for final approval
            match deliberate_result.bg_gate.decision {
                GateDecision::Approve => DecisionAction::Act,
                GateDecision::Block => DecisionAction::Defer,
                GateDecision::RequireAlternative => DecisionAction::RequestMoreData,
            }
        }
        PathwayType::Deferred => {
            if deliberate_result.is_novel {
                DecisionAction::RequestMoreData
            } else {
                DecisionAction::Defer
            }
        }
    };

    let elapsed_ms = start.elapsed().as_millis() as f64;
    info!(
        experience_id = %experience_id,
        pathway = ?pathway,
        action = ?action,
        confidence = deliberate_result.confidence,
        elapsed_ms = elapsed_ms,
        "Decision made via deliberate loop"
    );

    Ok(Json(Decision {
        experience_id,
        action,
        confidence: deliberate_result.confidence,
        reasoning,
        suggested_response: None,
        similar_experiences: deliberate_result.results,
        regions_consulted: deliberate_result.regions_consulted,
    }))
}

#[instrument(skip(_state))]
async fn feedback(
    State(_state): State<AppState>,
    Json(request): Json<FeedbackRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    info!(experience_id = %request.experience_id, "Recording feedback");

    // TODO: Distribute feedback to relevant regions
    // This would store the outcome in each region that was consulted

    Ok(Json(serde_json::json!({
        "experience_id": request.experience_id,
        "recorded": true
    })))
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
