//! Decision pathway policies for Cortical Compose.
//!
//! Implements the three-pathway model:
//! - **Fast Path**: Amygdala-driven threat/reflex responses (low latency)
//! - **Slow Path**: Deliberate loop through all regions (high confidence)
//! - **Thalamus Gating**: Attention routing and PFC feedback

use serde::{Deserialize, Serialize};

/// Fast path policy for threat/reflex responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FastPathPolicy {
    /// Whether fast path is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Similarity threshold for triggering on negative experiences
    /// If amygdala finds a highly similar negative experience, bypass deliberation
    #[serde(default = "default_negative_similarity")]
    pub negative_similarity_threshold: f32,
    
    /// Confidence threshold for threat detection
    #[serde(default = "default_threat_confidence")]
    pub threat_confidence_threshold: f32,
    
    /// Maximum latency allowed for fast path decisions (ms)
    #[serde(default = "default_max_latency")]
    pub max_latency_ms: u64,
}

fn default_negative_similarity() -> f32 {
    0.82
}

fn default_threat_confidence() -> f32 {
    0.70
}

fn default_max_latency() -> u64 {
    150
}

impl Default for FastPathPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            negative_similarity_threshold: default_negative_similarity(),
            threat_confidence_threshold: default_threat_confidence(),
            max_latency_ms: default_max_latency(),
        }
    }
}

/// Slow path policy for deliberate decision-making
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlowPathPolicy {
    /// Whether slow path is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Minimum confidence required to provide an answer
    #[serde(default = "default_min_confidence")]
    pub min_confidence_to_answer: f32,
    
    /// Threshold for detecting novel situations requiring extra deliberation
    #[serde(default = "default_novelty")]
    pub novelty_threshold: f32,
    
    /// Whether basal ganglia must approve action (hard gate vs advisory)
    #[serde(default = "default_true")]
    pub require_bg_gate: bool,
}

fn default_min_confidence() -> f32 {
    0.65
}

fn default_novelty() -> f32 {
    0.60
}

impl Default for SlowPathPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            min_confidence_to_answer: default_min_confidence(),
            novelty_threshold: default_novelty(),
            require_bg_gate: true,
        }
    }
}

/// Thalamus gating policy for attention routing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThalamusGatingPolicy {
    /// Whether thalamus gating is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
    
    /// Whether PFC can send feedback to adjust thalamus gating
    #[serde(default = "default_true")]
    pub allow_pfc_feedback: bool,
}

impl Default for ThalamusGatingPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            allow_pfc_feedback: true,
        }
    }
}

fn default_true() -> bool {
    true
}

/// Combined policy configuration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PolicyConfig {
    #[serde(default)]
    pub fast_path: FastPathPolicy,
    #[serde(default)]
    pub slow_path: SlowPathPolicy,
    #[serde(default)]
    pub thalamus_gating: ThalamusGatingPolicy,
}

/// Result of pathway selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathwayType {
    /// Fast reflex path (amygdala-driven)
    FastReflex,
    /// Deliberate loop (full region consultation)
    DeliberateLoop,
    /// Deferred (needs more data or human review)
    Deferred,
}

/// Amygdala fast-path check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FastPathCheck {
    /// Whether fast path should be triggered
    pub triggered: bool,
    /// The pathway type if triggered
    pub pathway: PathwayType,
    /// Similarity score of the most relevant negative experience
    pub negative_similarity: Option<f32>,
    /// Threat confidence level
    pub threat_confidence: Option<f32>,
    /// Reason for the decision
    pub reason: String,
}

impl FastPathCheck {
    pub fn not_triggered() -> Self {
        Self {
            triggered: false,
            pathway: PathwayType::DeliberateLoop,
            negative_similarity: None,
            threat_confidence: None,
            reason: "No fast-path trigger detected".to_string(),
        }
    }

    pub fn threat_detected(similarity: f32, confidence: f32) -> Self {
        Self {
            triggered: true,
            pathway: PathwayType::FastReflex,
            negative_similarity: Some(similarity),
            threat_confidence: Some(confidence),
            reason: format!(
                "Threat detected: similarity={:.2}, confidence={:.2}",
                similarity, confidence
            ),
        }
    }
}

/// Basal ganglia gate decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateDecision {
    /// Action approved
    Approve,
    /// Action blocked
    Block,
    /// Requires alternative action
    RequireAlternative,
}

/// Basal ganglia gating result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BGGateResult {
    pub decision: GateDecision,
    pub confidence: f32,
    pub reason: String,
    /// Suggested alternative if RequireAlternative
    pub alternative: Option<String>,
}

impl BGGateResult {
    pub fn approve(confidence: f32) -> Self {
        Self {
            decision: GateDecision::Approve,
            confidence,
            reason: "Action approved by basal ganglia gate".to_string(),
            alternative: None,
        }
    }

    pub fn block(confidence: f32, reason: impl Into<String>) -> Self {
        Self {
            decision: GateDecision::Block,
            confidence,
            reason: reason.into(),
            alternative: None,
        }
    }
}
