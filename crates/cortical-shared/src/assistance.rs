//! Cross-brain assistance policy types.
//!
//! Implements the cross_brain_assistance policy for structured help-seeking,
//! review requirements, and anti-bias safeguards.

use serde::{Deserialize, Serialize};

/// Help-seeking trigger evaluation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpSeekingEvaluation {
    pub should_seek_help: bool,
    pub triggers_fired: Vec<HelpTrigger>,
    pub can_proceed_alone: bool,
    pub reason: String,
}

/// Types of help-seeking triggers
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpTrigger {
    /// Confidence too low (<0.65) or suspiciously high (>0.85 with low evidence)
    ConfidenceRisk,
    /// Novel situation or ambiguous problem framing
    NoveltyOrAmbiguity,
    /// Irreversible, safety, compliance, or architecture impact
    HighStakes,
    /// Repeated loops or revisions without improvement
    StuckState,
    /// Open-ended design or early ideation
    InspirationMode,
    /// Internal region conflict detected
    RegionConflict,
}

/// Configuration for help-seeking triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpSeekingConfig {
    /// Confidence below this triggers help-seeking
    pub low_confidence_threshold: f32,
    /// Confidence above this with low evidence triggers help-seeking
    pub high_confidence_threshold: f32,
    /// Novelty score above this triggers help-seeking
    pub novelty_threshold: f32,
    /// Number of revisions without improvement before triggering
    pub max_revisions_without_improvement: u32,
    /// Whether to require help for irreversible decisions
    pub require_help_for_irreversible: bool,
    /// Whether to require help for safety-impacting decisions
    pub require_help_for_safety: bool,
}

impl Default for HelpSeekingConfig {
    fn default() -> Self {
        Self {
            low_confidence_threshold: 0.65,
            high_confidence_threshold: 0.85,
            novelty_threshold: 0.60,
            max_revisions_without_improvement: 2,
            require_help_for_irreversible: true,
            require_help_for_safety: true,
        }
    }
}

/// Context for evaluating help-seeking triggers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionContext {
    pub self_confidence: f32,
    pub evidence_diversity: EvidenceDiversity,
    pub novelty_score: f32,
    pub is_irreversible: bool,
    pub has_safety_impact: bool,
    pub has_compliance_impact: bool,
    pub has_architecture_impact: bool,
    pub revision_count: u32,
    pub has_region_conflict: bool,
    pub is_open_ended: bool,
    pub similarity_to_past_success: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceDiversity {
    Low,
    Medium,
    High,
}

impl DecisionContext {
    /// Evaluate help-seeking triggers
    pub fn evaluate(&self, config: &HelpSeekingConfig) -> HelpSeekingEvaluation {
        let mut triggers = Vec::new();

        // Confidence risk checks
        if self.self_confidence < config.low_confidence_threshold {
            triggers.push(HelpTrigger::ConfidenceRisk);
        }
        if self.self_confidence > config.high_confidence_threshold
            && self.evidence_diversity == EvidenceDiversity::Low
        {
            triggers.push(HelpTrigger::ConfidenceRisk);
        }

        // Region conflict
        if self.has_region_conflict {
            triggers.push(HelpTrigger::RegionConflict);
        }

        // Novelty/ambiguity
        if self.novelty_score > config.novelty_threshold {
            triggers.push(HelpTrigger::NoveltyOrAmbiguity);
        }

        // High stakes
        if (self.is_irreversible && config.require_help_for_irreversible)
            || (self.has_safety_impact && config.require_help_for_safety)
            || self.has_compliance_impact
            || self.has_architecture_impact
        {
            triggers.push(HelpTrigger::HighStakes);
        }

        // Stuck state
        if self.revision_count >= config.max_revisions_without_improvement {
            triggers.push(HelpTrigger::StuckState);
        }

        // Inspiration mode
        if self.is_open_ended {
            triggers.push(HelpTrigger::InspirationMode);
        }

        let should_seek_help = !triggers.is_empty();

        // Check if can proceed alone (exception conditions)
        let can_proceed_alone = self.self_confidence >= 0.75
            && !self.is_irreversible
            && !self.has_safety_impact
            && self.similarity_to_past_success > 0.8
            && !self.has_region_conflict;

        let reason = if should_seek_help {
            format!(
                "Help recommended: {} trigger(s) fired",
                triggers.len()
            )
        } else if can_proceed_alone {
            "May proceed alone (low risk, high similarity to past success)".to_string()
        } else {
            "No triggers fired, but post-hoc review recommended".to_string()
        };

        HelpSeekingEvaluation {
            should_seek_help,
            triggers_fired: triggers,
            can_proceed_alone,
            reason,
        }
    }
}

/// Help request to send to another brain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HelpRequest {
    pub intent: String,
    pub current_approach: String,
    pub stuck_description: Option<String>,
    pub constraints: Vec<String>,
    pub requested_help_types: Vec<HelpType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpType {
    Critique,
    Alternatives,
    RiskAssessment,
    Inspiration,
    StructureCheck,
    ProcessCheck,
}

/// Review response from another brain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewResponse {
    pub perspective_type: String,
    pub key_observations: Vec<String>,
    pub risks_identified: Vec<String>,
    pub suggested_changes: Vec<String>,
    pub confidence: f32,
    pub assumptions: Vec<String>,
}

/// Review categories that must be covered
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewCategory {
    /// Correctness, assumption validity, edge cases
    DomainReview,
    /// Logical flow, dependency ordering, modularity
    StructureReview,
    /// Clarity, naming, separation of concerns
    OrganizationReview,
    /// Repeatability, failure modes, monitoring
    ProcessReview,
}

/// Reviewer selection criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewerCriteria {
    /// Minimum capability level for domain expert
    pub domain_expert_min_level: u8,
    /// Maximum capability level for outside reviewer (capability inversion)
    pub outside_reviewer_max_level: u8,
    /// Required review categories
    pub required_categories: Vec<ReviewCategory>,
}

impl Default for ReviewerCriteria {
    fn default() -> Self {
        Self {
            domain_expert_min_level: 7,
            outside_reviewer_max_level: 4,
            required_categories: vec![
                ReviewCategory::DomainReview,
                ReviewCategory::StructureReview,
                ReviewCategory::OrganizationReview,
                ReviewCategory::ProcessReview,
            ],
        }
    }
}

/// Ask-two-others rule enforcement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AskTwoOthersResult {
    pub rule_satisfied: bool,
    pub reviewers_selected: Vec<SelectedReviewer>,
    pub missing_requirements: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedReviewer {
    pub brain_id: String,
    pub role: ReviewerRole,
    pub capability_level: u8,
    pub primary_domain: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewerRole {
    DomainExpert,
    OutsideDomainReviewer,
}

/// Disagreement handling result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisagreementResolution {
    pub resolution_path: ResolutionPath,
    pub divergent_assumptions: Vec<String>,
    pub missing_evidence: Vec<String>,
    pub final_recommendation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionPath {
    RequestMoreData,
    ChooseSafestReversible,
    EscalateToAdditionalReviewer,
    Reconciled,
}
