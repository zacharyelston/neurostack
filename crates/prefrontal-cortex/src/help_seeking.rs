//! Help-seeking integration for the Prefrontal Cortex.
//!
//! Implements the cross_brain_assistance policy by evaluating decision context
//! and determining when to seek external review.

use cortical_shared::{
    DecisionContext, EvidenceDiversity, HelpRequest, HelpSeekingConfig,
    HelpSeekingEvaluation, HelpTrigger, HelpType, ReviewCategory, ReviewerCriteria,
};
use tracing::info;

use crate::pathways::DeliberateLoopResult;

/// Build decision context from deliberate loop results
pub fn build_decision_context(
    deliberate: &DeliberateLoopResult,
    is_irreversible: bool,
    has_safety_impact: bool,
    revision_count: u32,
) -> DecisionContext {
    // Determine evidence diversity based on regions consulted
    let evidence_diversity = match deliberate.regions_consulted.len() {
        0..=1 => EvidenceDiversity::Low,
        2..=3 => EvidenceDiversity::Medium,
        _ => EvidenceDiversity::High,
    };

    // Novelty is inverse of confidence when we have results
    let novelty_score = if deliberate.results.is_empty() {
        1.0 // Completely novel
    } else {
        1.0 - deliberate.confidence
    };

    // Check for region conflict (BG blocked but other regions had high confidence)
    let has_region_conflict = deliberate.bg_gate.decision != cortical_shared::GateDecision::Approve
        && deliberate.confidence > 0.7;

    // Similarity to past success based on top result score
    let similarity_to_past_success = deliberate
        .results
        .first()
        .map(|r| r.score)
        .unwrap_or(0.0);

    DecisionContext {
        self_confidence: deliberate.confidence,
        evidence_diversity,
        novelty_score,
        is_irreversible,
        has_safety_impact,
        has_compliance_impact: false, // Would come from request metadata
        has_architecture_impact: false,
        revision_count,
        has_region_conflict,
        is_open_ended: false, // Would come from request metadata
        similarity_to_past_success,
    }
}

/// Evaluate whether help should be sought
pub fn evaluate_help_seeking(
    context: &DecisionContext,
    config: &HelpSeekingConfig,
) -> HelpSeekingEvaluation {
    let evaluation = context.evaluate(config);
    
    if evaluation.should_seek_help {
        info!(
            triggers = ?evaluation.triggers_fired,
            confidence = context.self_confidence,
            "Help-seeking triggered"
        );
    }
    
    evaluation
}

/// Build a help request based on triggers
pub fn build_help_request(
    evaluation: &HelpSeekingEvaluation,
    intent: impl Into<String>,
    current_approach: impl Into<String>,
) -> HelpRequest {
    let mut help_types = Vec::new();
    
    for trigger in &evaluation.triggers_fired {
        match trigger {
            HelpTrigger::ConfidenceRisk => {
                help_types.push(HelpType::Critique);
                help_types.push(HelpType::RiskAssessment);
            }
            HelpTrigger::NoveltyOrAmbiguity => {
                help_types.push(HelpType::Alternatives);
                help_types.push(HelpType::Inspiration);
            }
            HelpTrigger::HighStakes => {
                help_types.push(HelpType::RiskAssessment);
                help_types.push(HelpType::ProcessCheck);
            }
            HelpTrigger::StuckState => {
                help_types.push(HelpType::Alternatives);
                help_types.push(HelpType::StructureCheck);
            }
            HelpTrigger::InspirationMode => {
                help_types.push(HelpType::Inspiration);
                help_types.push(HelpType::Alternatives);
            }
            HelpTrigger::RegionConflict => {
                help_types.push(HelpType::Critique);
                help_types.push(HelpType::RiskAssessment);
            }
        }
    }
    
    // Deduplicate
    help_types.sort_by_key(|h| format!("{:?}", h));
    help_types.dedup();
    
    HelpRequest {
        intent: intent.into(),
        current_approach: current_approach.into(),
        stuck_description: if evaluation.triggers_fired.contains(&HelpTrigger::StuckState) {
            Some("Multiple revisions without improvement".to_string())
        } else {
            None
        },
        constraints: vec![],
        requested_help_types: help_types,
    }
}

/// Determine required review categories based on triggers
pub fn required_review_categories(evaluation: &HelpSeekingEvaluation) -> Vec<ReviewCategory> {
    let mut categories = vec![
        ReviewCategory::DomainReview,
        ReviewCategory::StructureReview,
    ];
    
    for trigger in &evaluation.triggers_fired {
        match trigger {
            HelpTrigger::HighStakes | HelpTrigger::StuckState => {
                if !categories.contains(&ReviewCategory::ProcessReview) {
                    categories.push(ReviewCategory::ProcessReview);
                }
            }
            HelpTrigger::NoveltyOrAmbiguity | HelpTrigger::InspirationMode => {
                if !categories.contains(&ReviewCategory::OrganizationReview) {
                    categories.push(ReviewCategory::OrganizationReview);
                }
            }
            _ => {}
        }
    }
    
    categories
}

/// Check if the ask-two-others rule is satisfied
pub fn check_ask_two_others(
    reviewers_count: usize,
    has_domain_expert: bool,
    has_outside_reviewer: bool,
    criteria: &ReviewerCriteria,
) -> (bool, Vec<String>) {
    let mut missing = Vec::new();
    
    if reviewers_count < 2 {
        missing.push(format!(
            "Need at least 2 reviewers, have {}",
            reviewers_count
        ));
    }
    
    if !has_domain_expert {
        missing.push(format!(
            "Need domain expert with capability >= {}",
            criteria.domain_expert_min_level
        ));
    }
    
    if !has_outside_reviewer {
        missing.push(format!(
            "Need outside-domain reviewer with capability <= {}",
            criteria.outside_reviewer_max_level
        ));
    }
    
    (missing.is_empty(), missing)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cortical_shared::BGGateResult;

    fn make_deliberate_result(confidence: f32, regions: Vec<&str>) -> DeliberateLoopResult {
        DeliberateLoopResult {
            results: vec![],
            regions_consulted: regions.into_iter().map(String::from).collect(),
            confidence,
            bg_gate: BGGateResult::approve(confidence),
            is_novel: confidence < 0.6,
        }
    }

    #[test]
    fn test_low_confidence_triggers_help() {
        let deliberate = make_deliberate_result(0.4, vec!["hippocampus"]);
        let context = build_decision_context(&deliberate, false, false, 0);
        let config = HelpSeekingConfig::default();
        
        let eval = evaluate_help_seeking(&context, &config);
        
        assert!(eval.should_seek_help);
        assert!(eval.triggers_fired.contains(&HelpTrigger::ConfidenceRisk));
    }

    #[test]
    fn test_high_confidence_low_evidence_triggers_help() {
        let deliberate = make_deliberate_result(0.9, vec!["hippocampus"]);
        let context = build_decision_context(&deliberate, false, false, 0);
        let config = HelpSeekingConfig::default();
        
        let eval = evaluate_help_seeking(&context, &config);
        
        assert!(eval.should_seek_help);
        assert!(eval.triggers_fired.contains(&HelpTrigger::ConfidenceRisk));
    }

    #[test]
    fn test_safe_decision_can_proceed_alone() {
        let mut deliberate = make_deliberate_result(0.8, vec!["hippocampus", "amygdala", "cerebellum", "basal_ganglia"]);
        deliberate.results.push(cortical_shared::SimilarityResult {
            id: "test".to_string(),
            score: 0.85,
            payload: None,
            region: "hippocampus".to_string(),
        });
        
        let context = build_decision_context(&deliberate, false, false, 0);
        let config = HelpSeekingConfig::default();
        
        let eval = evaluate_help_seeking(&context, &config);
        
        assert!(eval.can_proceed_alone);
    }
}
