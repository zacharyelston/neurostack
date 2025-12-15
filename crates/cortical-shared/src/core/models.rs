//! Shared data models for Cortical Compose.
//! All services use these common schemas for inter-service communication.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Source of an experience
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExperienceSource {
    Input,
    Output,
    Feedback,
    System,
}

/// Emotional valence polarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ValencePolarity {
    Positive,
    Negative,
}

/// Contextual information for an experience
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Context {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Emotional valence assessment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Valence {
    /// Score from -1.0 (negative) to 1.0 (positive)
    pub score: f32,
    /// Confidence in the valence assessment (0.0 to 1.0)
    pub confidence: f32,
    /// Descriptive labels for the valence
    #[serde(default)]
    pub labels: Vec<String>,
}

/// Outcome of an action or decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Outcome {
    pub success: bool,
    #[serde(default)]
    pub reward: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Core experience data model used across all regions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub experience_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub source: ExperienceSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding: Option<Vec<f32>>,
    #[serde(default)]
    pub context: Context,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valence: Option<Valence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<Outcome>,
}

impl Experience {
    pub fn new(source: ExperienceSource, text: impl Into<String>) -> Self {
        Self {
            experience_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            source,
            actor: None,
            text: text.into(),
            embedding: None,
            context: Context::default(),
            valence: None,
            outcome: None,
        }
    }

    pub fn with_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.embedding = Some(embedding);
        self
    }

    pub fn with_context(mut self, context: Context) -> Self {
        self.context = context;
        self
    }
}

/// Normalized input from Thalamus to Prefrontal Cortex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturePacket {
    pub experience_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub source: ExperienceSource,
    pub text: String,
    pub embedding: Vec<f32>,
    pub context: Context,
}

/// Standard request for storing experiences in a region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreRequest {
    pub experience: Experience,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Standard response after storing an experience
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreResponse {
    pub experience_id: Uuid,
    pub stored: bool,
    pub region: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Standard request for querying a region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    pub embedding: Vec<f32>,
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub filters: serde_json::Value,
    #[serde(default = "default_true")]
    pub include_metadata: bool,
}

fn default_limit() -> usize {
    10
}

fn default_true() -> bool {
    true
}

/// Single result from a similarity search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimilarityResult {
    pub experience_id: Uuid,
    pub score: f32,
    pub text: String,
    #[serde(default)]
    pub metadata: serde_json::Value,
}

/// Standard response from a region query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    pub results: Vec<SimilarityResult>,
    pub region: String,
    pub query_time_ms: f64,
}

/// Request to the Prefrontal Cortex for a decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionRequest {
    pub input_text: String,
    #[serde(default)]
    pub context: Context,
    #[serde(default = "default_confidence")]
    pub require_confidence: f32,
}

fn default_confidence() -> f32 {
    0.5
}

/// Action type for a decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionAction {
    Act,
    Defer,
    RequestMoreData,
}

/// Decision output from the Prefrontal Cortex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {
    pub experience_id: Uuid,
    pub action: DecisionAction,
    pub confidence: f32,
    pub reasoning: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_response: Option<String>,
    #[serde(default)]
    pub similar_experiences: Vec<SimilarityResult>,
    #[serde(default)]
    pub regions_consulted: Vec<String>,
}

/// Feedback to record outcome of a decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackRequest {
    pub experience_id: Uuid,
    pub outcome: Outcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valence: Option<Valence>,
}

/// Standard health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub service: String,
    #[serde(default = "default_healthy")]
    pub status: String,
    pub version: String,
    #[serde(default)]
    pub dependencies: std::collections::HashMap<String, String>,
}

fn default_healthy() -> String {
    "healthy".to_string()
}

impl HealthResponse {
    pub fn healthy(service: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            status: "healthy".to_string(),
            version: version.into(),
            dependencies: std::collections::HashMap::new(),
        }
    }

    pub fn with_dependency(mut self, name: impl Into<String>, status: impl Into<String>) -> Self {
        self.dependencies.insert(name.into(), status.into());
        self
    }
}

/// Request to ingest raw input via Thalamus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestRequest {
    pub text: String,
    #[serde(default = "default_input_source")]
    pub source: ExperienceSource,
    #[serde(default)]
    pub context: Context,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
}

fn default_input_source() -> ExperienceSource {
    ExperienceSource::Input
}
