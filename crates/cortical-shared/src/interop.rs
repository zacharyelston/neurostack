//! Inter-brain communication protocol types.
//!
//! Implements the neurostack-interop protocol for brain-to-brain communication:
//! - Discovery and capability advertisement
//! - Message envelope with epistemics
//! - Task contracts for delegation
//! - Loop prevention controls

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Protocol version for interop messages
pub const PROTOCOL_VERSION: &str = "1.0";

/// Capability level scale (0-9)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum CapabilityLevel {
    NoSupport = 0,
    Experimental = 1,
    Basic = 3,
    Competent = 5,
    Strong = 7,
    Expert = 9,
}

impl From<u8> for CapabilityLevel {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::NoSupport,
            1..=2 => Self::Experimental,
            3..=4 => Self::Basic,
            5..=6 => Self::Competent,
            7..=8 => Self::Strong,
            9 => Self::Expert,
            _ => Self::NoSupport,
        }
    }
}

/// Brain identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrainIdentity {
    /// Stable unique identifier for this brain
    pub brain_id: String,
    /// Ephemeral runtime identifier
    pub instance_id: Uuid,
    /// Protocol version supported
    pub protocol_version: String,
}

impl BrainIdentity {
    pub fn new(brain_id: impl Into<String>) -> Self {
        Self {
            brain_id: brain_id.into(),
            instance_id: Uuid::new_v4(),
            protocol_version: PROTOCOL_VERSION.to_string(),
        }
    }
}

/// Capability advertisement for a brain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityAdvertisement {
    pub brain_id: String,
    pub protocol_version: String,
    /// Domain -> capability level (0-9)
    pub capabilities: HashMap<String, u8>,
    /// Optional free-text tags for routing
    pub specialties: Vec<String>,
    pub limits: InteropLimits,
    pub share_policy: SharePolicy,
    pub llm_profile: Option<LlmProfile>,
}

/// Operational limits for interop
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteropLimits {
    pub max_payload_kb: u32,
    pub max_turns_per_conversation: u32,
    pub max_hops: u8,
    pub rate_limit_rpm: u32,
}

impl Default for InteropLimits {
    fn default() -> Self {
        Self {
            max_payload_kb: 256,
            max_turns_per_conversation: 12,
            max_hops: 3,
            rate_limit_rpm: 60,
        }
    }
}

/// Data sharing policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharePolicy {
    pub allowed_exports: Vec<ExportLevel>,
    pub forbidden_exports: Vec<ExportLevel>,
}

impl Default for SharePolicy {
    fn default() -> Self {
        Self {
            allowed_exports: vec![ExportLevel::ShareableSummary, ExportLevel::ShareableArtifact],
            forbidden_exports: vec![ExportLevel::PrivateMemory, ExportLevel::RawUserData],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportLevel {
    ShareableSummary,
    ShareableArtifact,
    PrivateMemory,
    RawUserData,
}

/// LLM profile for capability matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmProfile {
    pub model_family: String,
    pub context_window_tokens: u32,
    pub tool_access: ToolAccess,
    pub deterministic_mode_supported: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolAccess {
    None,
    Restricted,
    Full,
}

/// Message types for inter-brain communication
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Discover,
    Ask,
    Tell,
    ProposeTask,
    AcceptTask,
    RejectTask,
    RequestReview,
    ReviewResponse,
    ShareSummary,
    ShareArtifact,
    Warn,
    Reconcile,
    Feedback,
}

/// Risk level for epistemic metadata
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

/// Evidence reference for claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRef {
    pub source_brain: String,
    pub source_kind: EvidenceKind,
    pub source_id: String,
    pub redaction_level: RedactionLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    Experience,
    Artifact,
    Trace,
    ExternalRef,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionLevel {
    Private,
    Summary,
    Artifact,
}

/// Epistemic metadata for messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Epistemics {
    /// Confidence level (0.0 - 1.0)
    pub confidence: f32,
    pub risk_level: RiskLevel,
    pub assumptions: Vec<String>,
    pub evidence_refs: Vec<EvidenceRef>,
}

impl Default for Epistemics {
    fn default() -> Self {
        Self {
            confidence: 0.5,
            risk_level: RiskLevel::Medium,
            assumptions: vec![],
            evidence_refs: vec![],
        }
    }
}

/// Message envelope for inter-brain communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteropMessage {
    pub protocol_version: String,
    pub message_id: Uuid,
    pub trace_id: Uuid,
    pub span_id: Option<Uuid>,
    pub from_brain: String,
    pub to_brain: String,
    pub timestamp: DateTime<Utc>,
    pub ttl_ms: u64,
    pub hop_limit: u8,
    pub conversation_id: Option<Uuid>,
    pub message_type: MessageType,
    pub epistemics: Epistemics,
    pub payload: serde_json::Value,
    pub latex: Option<LatexPayload>,
}

impl InteropMessage {
    pub fn new(
        from_brain: impl Into<String>,
        to_brain: impl Into<String>,
        message_type: MessageType,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            protocol_version: PROTOCOL_VERSION.to_string(),
            message_id: Uuid::new_v4(),
            trace_id: Uuid::new_v4(),
            span_id: None,
            from_brain: from_brain.into(),
            to_brain: to_brain.into(),
            timestamp: Utc::now(),
            ttl_ms: 60_000, // 1 minute default
            hop_limit: 3,
            conversation_id: None,
            message_type,
            epistemics: Epistemics::default(),
            payload,
            latex: None,
        }
    }

    /// Check if message has expired
    pub fn is_expired(&self) -> bool {
        let elapsed = Utc::now()
            .signed_duration_since(self.timestamp)
            .num_milliseconds();
        elapsed > self.ttl_ms as i64
    }

    /// Check if hop limit is exhausted
    pub fn is_hop_exhausted(&self) -> bool {
        self.hop_limit == 0
    }

    /// Decrement hop limit for relay
    pub fn decrement_hop(&mut self) {
        self.hop_limit = self.hop_limit.saturating_sub(1);
    }
}

/// LaTeX payload for structured communication documents
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatexPayload {
    pub enabled: bool,
    pub format: LatexFormat,
    pub content: String,
    pub attachments: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatexFormat {
    Latex,
    LatexPlusJson,
}

/// Task contract for delegation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskContract {
    pub task_id: Uuid,
    pub title: String,
    pub objective: String,
    pub inputs: TaskInputs,
    pub deliverables: Vec<Deliverable>,
    pub evaluation: TaskEvaluation,
    pub budgets: TaskBudgets,
    pub permissions: TaskPermissions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskInputs {
    pub context: serde_json::Value,
    pub constraints: serde_json::Value,
    pub latex_brief: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deliverable {
    pub name: String,
    pub deliverable_type: DeliverableType,
    pub acceptance_criteria: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliverableType {
    Json,
    Latex,
    Markdown,
    Code,
    ArtifactRef,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEvaluation {
    pub rubric: Vec<EvaluationMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationMetric {
    pub metric: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskBudgets {
    pub ttl_ms: u64,
    pub max_turns: u32,
    pub max_tokens: u32,
}

impl Default for TaskBudgets {
    fn default() -> Self {
        Self {
            ttl_ms: 600_000, // 10 minutes
            max_turns: 6,
            max_tokens: 4000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskPermissions {
    pub can_request_more_data: bool,
    pub can_use_external_tools: bool,
    pub export_level: ExportLevel,
}

impl Default for TaskPermissions {
    fn default() -> Self {
        Self {
            can_request_more_data: true,
            can_use_external_tools: false,
            export_level: ExportLevel::ShareableSummary,
        }
    }
}

/// Handshake request for brain discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeRequest {
    pub identity: BrainIdentity,
    pub capabilities: CapabilityAdvertisement,
}

/// Handshake response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeResponse {
    pub identity: BrainIdentity,
    pub capabilities: CapabilityAdvertisement,
    pub accepted: bool,
    pub reason: Option<String>,
}

/// Loop prevention state
#[derive(Debug, Clone, Default)]
pub struct LoopPrevention {
    /// Seen message IDs for deduplication
    seen_messages: std::collections::HashSet<(String, Uuid)>,
}

impl LoopPrevention {
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if message should be dropped (duplicate or expired)
    pub fn should_drop(&mut self, msg: &InteropMessage) -> Option<DropReason> {
        if msg.is_expired() {
            return Some(DropReason::Expired);
        }
        if msg.is_hop_exhausted() {
            return Some(DropReason::HopExhausted);
        }
        let key = (msg.from_brain.clone(), msg.message_id);
        if self.seen_messages.contains(&key) {
            return Some(DropReason::Duplicate);
        }
        self.seen_messages.insert(key);
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DropReason {
    Expired,
    HopExhausted,
    Duplicate,
}
