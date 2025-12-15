//! Internal decision pathways for the Prefrontal Cortex.
//!
//! Implements the three-pathway model from topology.yaml:
//! - `fast_reflex()` - Amygdala-negative threat response
//! - `deliberate_loop()` - Full region consultation (hippo + bg + cereb + amyg)
//! - `thalamus_feedback_update()` - Attention/gating tuning

use cortical_shared::{
    BGGateResult, CorticalResult, FastPathCheck, FastPathPolicy, GateDecision,
    PathwayType, PolicyConfig, QueryResponse, RegionClient, SimilarityResult,
    SlowPathPolicy,
};
use tracing::{info, warn};

/// Check if fast path should be triggered based on amygdala negative experiences
pub async fn fast_reflex(
    amygdala: &RegionClient,
    embedding: &[f32],
    policy: &FastPathPolicy,
) -> CorticalResult<FastPathCheck> {
    if !policy.enabled {
        return Ok(FastPathCheck::not_triggered());
    }

    // Query amygdala for negative experiences
    let filters = serde_json::json!({
        "polarity": "negative"
    });

    let result = amygdala
        .query(embedding.to_vec(), 5, Some(filters))
        .await;

    match result {
        Ok(response) => {
            // Check if any negative experience exceeds threshold
            if let Some(top_result) = response.results.first() {
                if top_result.score >= policy.negative_similarity_threshold {
                    let threat_confidence = calculate_threat_confidence(&response.results);
                    
                    if threat_confidence >= policy.threat_confidence_threshold {
                        info!(
                            similarity = top_result.score,
                            threat_confidence = threat_confidence,
                            "Fast path triggered: threat detected"
                        );
                        return Ok(FastPathCheck::threat_detected(
                            top_result.score,
                            threat_confidence,
                        ));
                    }
                }
            }
            Ok(FastPathCheck::not_triggered())
        }
        Err(e) => {
            warn!("Amygdala query failed in fast_reflex: {}", e);
            // On failure, don't trigger fast path - fall through to deliberate
            Ok(FastPathCheck::not_triggered())
        }
    }
}

/// Calculate threat confidence from negative experience results
fn calculate_threat_confidence(results: &[SimilarityResult]) -> f32 {
    if results.is_empty() {
        return 0.0;
    }

    // Weighted average of top results, with decay
    let mut total_weight = 0.0;
    let mut weighted_score = 0.0;

    for (i, result) in results.iter().take(3).enumerate() {
        let weight = 1.0 / (i as f32 + 1.0);
        weighted_score += result.score * weight;
        total_weight += weight;
    }

    if total_weight > 0.0 {
        weighted_score / total_weight
    } else {
        0.0
    }
}

/// Execute the deliberate loop through all regions
pub async fn deliberate_loop(
    hippocampus: &RegionClient,
    basal_ganglia: &RegionClient,
    cerebellum: &RegionClient,
    amygdala: &RegionClient,
    embedding: &[f32],
    limit: usize,
    policy: &SlowPathPolicy,
) -> CorticalResult<DeliberateLoopResult> {
    if !policy.enabled {
        return Ok(DeliberateLoopResult::disabled());
    }

    // Query all regions in parallel
    let embedding_vec = embedding.to_vec();
    let (hippo_result, bg_result, cereb_result, amyg_result) = tokio::join!(
        hippocampus.query(embedding_vec.clone(), limit, None),
        basal_ganglia.query(embedding_vec.clone(), limit, None),
        cerebellum.query(embedding_vec.clone(), limit, None),
        amygdala.query(embedding_vec.clone(), limit, None),
    );

    let mut all_results: Vec<SimilarityResult> = Vec::new();
    let mut regions_consulted: Vec<String> = Vec::new();

    // Collect hippocampus results (episodic memory)
    if let Ok(response) = hippo_result {
        regions_consulted.push("hippocampus".to_string());
        all_results.extend(response.results);
    } else {
        warn!("Hippocampus query failed in deliberate_loop");
    }

    // Collect cerebellum results (error correction patterns)
    if let Ok(response) = cereb_result {
        regions_consulted.push("cerebellum".to_string());
        all_results.extend(response.results);
    } else {
        warn!("Cerebellum query failed in deliberate_loop");
    }

    // Collect amygdala results (valence context)
    if let Ok(response) = amyg_result {
        regions_consulted.push("amygdala".to_string());
        all_results.extend(response.results);
    } else {
        warn!("Amygdala query failed in deliberate_loop");
    }

    // Basal ganglia gate check (if required)
    let bg_gate = if policy.require_bg_gate {
        match bg_result {
            Ok(response) => {
                regions_consulted.push("basal_ganglia".to_string());
                evaluate_bg_gate(&response, policy)
            }
            Err(e) => {
                warn!("Basal ganglia query failed: {}", e);
                // On BG failure with require_bg_gate, block action
                BGGateResult::block(0.0, "Basal ganglia unavailable")
            }
        }
    } else {
        // BG is advisory only
        if let Ok(response) = bg_result {
            regions_consulted.push("basal_ganglia".to_string());
            all_results.extend(response.results);
        }
        BGGateResult::approve(1.0)
    };

    // Sort and truncate results
    all_results.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    all_results.truncate(limit);

    // Compute confidence
    let confidence = compute_deliberate_confidence(&all_results, &regions_consulted);

    // Check novelty (low similarity = novel situation)
    let is_novel = all_results
        .first()
        .map(|r| r.score < policy.novelty_threshold)
        .unwrap_or(true);

    Ok(DeliberateLoopResult {
        results: all_results,
        regions_consulted,
        confidence,
        bg_gate,
        is_novel,
    })
}

/// Evaluate basal ganglia gate decision
fn evaluate_bg_gate(response: &QueryResponse, policy: &SlowPathPolicy) -> BGGateResult {
    if response.results.is_empty() {
        // No prior action patterns - approve with low confidence
        return BGGateResult::approve(0.3);
    }

    // Check if top results indicate successful action patterns
    let avg_score: f32 = response
        .results
        .iter()
        .take(3)
        .map(|r| r.score)
        .sum::<f32>()
        / response.results.len().min(3) as f32;

    if avg_score >= policy.min_confidence_to_answer {
        BGGateResult::approve(avg_score)
    } else {
        BGGateResult {
            decision: GateDecision::RequireAlternative,
            confidence: avg_score,
            reason: format!(
                "Low action confidence ({:.2}), consider alternative",
                avg_score
            ),
            alternative: None,
        }
    }
}

/// Compute confidence for deliberate loop
fn compute_deliberate_confidence(results: &[SimilarityResult], regions: &[String]) -> f32 {
    if results.is_empty() || regions.is_empty() {
        return 0.0;
    }

    // Base confidence from top result scores
    let avg_score: f32 = results.iter().take(3).map(|r| r.score).sum::<f32>()
        / results.len().min(3) as f32;

    // Boost for more regions responding (out of 4)
    let region_factor = regions.len() as f32 / 4.0;

    (avg_score * 0.7 + region_factor * 0.3).min(1.0)
}

/// Result of the deliberate loop
#[derive(Debug, Clone)]
pub struct DeliberateLoopResult {
    pub results: Vec<SimilarityResult>,
    pub regions_consulted: Vec<String>,
    pub confidence: f32,
    pub bg_gate: BGGateResult,
    pub is_novel: bool,
}

impl DeliberateLoopResult {
    fn disabled() -> Self {
        Self {
            results: vec![],
            regions_consulted: vec![],
            confidence: 0.0,
            bg_gate: BGGateResult::approve(1.0),
            is_novel: true,
        }
    }
}

/// Determine final pathway based on fast check and deliberate results
pub fn select_pathway(
    fast_check: &FastPathCheck,
    deliberate: &DeliberateLoopResult,
    policy: &PolicyConfig,
) -> (PathwayType, String) {
    // Fast path takes precedence if triggered
    if fast_check.triggered {
        return (
            PathwayType::FastReflex,
            fast_check.reason.clone(),
        );
    }

    // Check BG gate
    match deliberate.bg_gate.decision {
        GateDecision::Block => {
            return (
                PathwayType::Deferred,
                format!("Blocked by basal ganglia: {}", deliberate.bg_gate.reason),
            );
        }
        GateDecision::RequireAlternative => {
            if deliberate.confidence < policy.slow_path.min_confidence_to_answer {
                return (
                    PathwayType::Deferred,
                    "Low confidence and BG suggests alternative".to_string(),
                );
            }
        }
        GateDecision::Approve => {}
    }

    // Check confidence threshold
    if deliberate.confidence >= policy.slow_path.min_confidence_to_answer {
        (
            PathwayType::DeliberateLoop,
            format!(
                "Deliberate decision with confidence {:.2}",
                deliberate.confidence
            ),
        )
    } else if deliberate.is_novel {
        (
            PathwayType::Deferred,
            "Novel situation with insufficient confidence".to_string(),
        )
    } else {
        (
            PathwayType::Deferred,
            format!(
                "Confidence {:.2} below threshold {:.2}",
                deliberate.confidence, policy.slow_path.min_confidence_to_answer
            ),
        )
    }
}
