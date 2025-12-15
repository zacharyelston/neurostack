//! Domain catalog types for brain capability profiles.
//!
//! Defines review lenses, domain profiles, and cross-reviewer recommendations
//! based on neurostack_domain_catalog.yaml.

use serde::{Deserialize, Serialize};

/// Review lenses - domain-agnostic ways of judging output
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewLens {
    Correctness,
    EvidenceQuality,
    Assumptions,
    Structure,
    Organization,
    Process,
    Risk,
    Safety,
    Operability,
    Maintainability,
    Clarity,
    Performance,
    Security,
    Cost,
    Alternatives,
    SecondOrderEffects,
    FailureModes,
}

/// Domain category for grouping related brain profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainCategory {
    CoreReasoningMeta,
    CognitiveArchitecture,
    EngineeringTechnical,
    QualityRoles,
    CreativeExploratory,
    HumanOrg,
    Wildcards,
}

/// A domain profile defining a brain's specialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainProfile {
    pub id: String,
    pub title: String,
    pub category: DomainCategory,
    pub primary_lenses: Vec<ReviewLens>,
    pub common_blindspots: Vec<String>,
    pub recommended_cross_reviewers: Vec<String>,
}

/// Built-in domain profiles from the catalog
pub fn get_domain_catalog() -> Vec<DomainProfile> {
    vec![
        // Core Reasoning Meta
        DomainProfile {
            id: "general_reasoner".into(),
            title: "General Reasoner".into(),
            category: DomainCategory::CoreReasoningMeta,
            primary_lenses: vec![ReviewLens::Correctness, ReviewLens::Assumptions, ReviewLens::EvidenceQuality],
            common_blindspots: vec!["overgeneralization".into(), "vague_acceptance_criteria".into()],
            recommended_cross_reviewers: vec!["critical_reviewer".into(), "assumptions_auditor".into()],
        },
        DomainProfile {
            id: "critical_reviewer".into(),
            title: "Critical Reviewer".into(),
            category: DomainCategory::CoreReasoningMeta,
            primary_lenses: vec![ReviewLens::Risk, ReviewLens::Assumptions, ReviewLens::EvidenceQuality],
            common_blindspots: vec!["over_index_on_risks".into(), "slow_to_converge".into()],
            recommended_cross_reviewers: vec!["general_reasoner".into(), "clarity_communication_reviewer".into()],
        },
        DomainProfile {
            id: "structure_analyst".into(),
            title: "Structure Analyst".into(),
            category: DomainCategory::CoreReasoningMeta,
            primary_lenses: vec![ReviewLens::Structure, ReviewLens::Maintainability],
            common_blindspots: vec!["underweight_domain_correctness".into()],
            recommended_cross_reviewers: vec!["process_engineer".into()],
        },
        DomainProfile {
            id: "process_engineer".into(),
            title: "Process Engineer".into(),
            category: DomainCategory::CoreReasoningMeta,
            primary_lenses: vec![ReviewLens::Process, ReviewLens::Operability],
            common_blindspots: vec!["bureaucratic_overhead".into()],
            recommended_cross_reviewers: vec!["simplicity_extremist".into(), "operational_readiness_reviewer".into()],
        },
        DomainProfile {
            id: "risk_assessor".into(),
            title: "Risk Assessor".into(),
            category: DomainCategory::CoreReasoningMeta,
            primary_lenses: vec![ReviewLens::Risk, ReviewLens::Safety],
            common_blindspots: vec!["overconservatism".into()],
            recommended_cross_reviewers: vec!["alternative_solution_generator".into()],
        },
        DomainProfile {
            id: "failure_mode_analyst".into(),
            title: "Failure Mode Analyst".into(),
            category: DomainCategory::CoreReasoningMeta,
            primary_lenses: vec![ReviewLens::Risk, ReviewLens::Operability, ReviewLens::Process],
            common_blindspots: vec!["doom_spirals".into()],
            recommended_cross_reviewers: vec!["performance_latency_optimizer".into(), "maintainability_advocate".into()],
        },
        // Cognitive Architecture
        DomainProfile {
            id: "cognitive_architecture_designer".into(),
            title: "Cognitive Architecture Designer".into(),
            category: DomainCategory::CognitiveArchitecture,
            primary_lenses: vec![ReviewLens::Structure, ReviewLens::Process, ReviewLens::Assumptions],
            common_blindspots: vec!["metaphor_overreach".into()],
            recommended_cross_reviewers: vec!["distributed_systems_architect".into(), "ai_safety_alignment_reviewer".into()],
        },
        DomainProfile {
            id: "distributed_systems_architect".into(),
            title: "Distributed Systems Architect".into(),
            category: DomainCategory::CognitiveArchitecture,
            primary_lenses: vec![ReviewLens::Operability, ReviewLens::FailureModes, ReviewLens::Structure],
            common_blindspots: vec!["complexity_creep".into()],
            recommended_cross_reviewers: vec!["simplicity_extremist".into(), "security_threat_modeler".into()],
        },
        DomainProfile {
            id: "ai_safety_alignment_reviewer".into(),
            title: "AI Safety & Alignment Reviewer".into(),
            category: DomainCategory::CognitiveArchitecture,
            primary_lenses: vec![ReviewLens::Safety, ReviewLens::Risk, ReviewLens::Assumptions],
            common_blindspots: vec!["blocking_bias".into()],
            recommended_cross_reviewers: vec!["process_governance_designer".into(), "user_perspective_advocate".into()],
        },
        // Engineering Technical
        DomainProfile {
            id: "software_architecture_engineer".into(),
            title: "Software Architecture Engineer".into(),
            category: DomainCategory::EngineeringTechnical,
            primary_lenses: vec![ReviewLens::Structure, ReviewLens::Maintainability, ReviewLens::Correctness],
            common_blindspots: vec!["underweight_operability".into()],
            recommended_cross_reviewers: vec!["devops_infrastructure_engineer".into()],
        },
        DomainProfile {
            id: "security_threat_modeler".into(),
            title: "Security & Threat Modeling Specialist".into(),
            category: DomainCategory::EngineeringTechnical,
            primary_lenses: vec![ReviewLens::Security, ReviewLens::Risk],
            common_blindspots: vec!["overconservatism".into()],
            recommended_cross_reviewers: vec!["api_protocol_designer".into(), "operational_readiness_reviewer".into()],
        },
        DomainProfile {
            id: "api_protocol_designer".into(),
            title: "API & Protocol Designer".into(),
            category: DomainCategory::EngineeringTechnical,
            primary_lenses: vec![ReviewLens::Organization, ReviewLens::Structure, ReviewLens::Correctness],
            common_blindspots: vec!["underweight_human_clarity".into()],
            recommended_cross_reviewers: vec!["clarity_communication_reviewer".into(), "distributed_systems_architect".into()],
        },
        // Quality Roles
        DomainProfile {
            id: "clarity_communication_reviewer".into(),
            title: "Clarity & Communication Reviewer".into(),
            category: DomainCategory::QualityRoles,
            primary_lenses: vec![ReviewLens::Clarity, ReviewLens::Organization],
            common_blindspots: vec!["underweight_edge_cases".into()],
            recommended_cross_reviewers: vec!["critical_reviewer".into(), "structure_analyst".into()],
        },
        DomainProfile {
            id: "maintainability_advocate".into(),
            title: "Maintainability Advocate".into(),
            category: DomainCategory::QualityRoles,
            primary_lenses: vec![ReviewLens::Maintainability, ReviewLens::Structure, ReviewLens::Process],
            common_blindspots: vec!["underweight_speed".into()],
            recommended_cross_reviewers: vec!["cost_resource_minimalist".into(), "performance_latency_optimizer".into()],
        },
        DomainProfile {
            id: "operational_readiness_reviewer".into(),
            title: "Operational Readiness Reviewer".into(),
            category: DomainCategory::QualityRoles,
            primary_lenses: vec![ReviewLens::Operability, ReviewLens::Process, ReviewLens::Risk],
            common_blindspots: vec!["gatekeeping".into()],
            recommended_cross_reviewers: vec!["clarity_communication_reviewer".into()],
        },
        // Creative/Exploratory
        DomainProfile {
            id: "alternative_solution_generator".into(),
            title: "Alternative Solution Generator".into(),
            category: DomainCategory::CreativeExploratory,
            primary_lenses: vec![ReviewLens::Alternatives, ReviewLens::Structure],
            common_blindspots: vec!["ignores_constraints".into()],
            recommended_cross_reviewers: vec!["risk_assessor".into(), "api_protocol_designer".into()],
        },
        // Wildcards
        DomainProfile {
            id: "simplicity_extremist".into(),
            title: "Simplicity Extremist".into(),
            category: DomainCategory::Wildcards,
            primary_lenses: vec![ReviewLens::Structure, ReviewLens::Cost, ReviewLens::Maintainability],
            common_blindspots: vec!["oversimplification".into()],
            recommended_cross_reviewers: vec!["distributed_systems_architect".into(), "failure_mode_analyst".into()],
        },
        DomainProfile {
            id: "cost_resource_minimalist".into(),
            title: "Cost & Resource Minimalist".into(),
            category: DomainCategory::Wildcards,
            primary_lenses: vec![ReviewLens::Cost, ReviewLens::Process, ReviewLens::Maintainability],
            common_blindspots: vec!["underbuild_risk".into()],
            recommended_cross_reviewers: vec!["risk_assessor".into()],
        },
    ]
}

/// Find a domain profile by ID
pub fn find_domain(id: &str) -> Option<DomainProfile> {
    get_domain_catalog().into_iter().find(|d| d.id == id)
}

/// Get recommended cross-reviewers for a domain
pub fn get_cross_reviewers(domain_id: &str) -> Vec<DomainProfile> {
    let catalog = get_domain_catalog();
    if let Some(domain) = catalog.iter().find(|d| d.id == domain_id) {
        domain
            .recommended_cross_reviewers
            .iter()
            .filter_map(|r| catalog.iter().find(|d| &d.id == r).cloned())
            .collect()
    } else {
        vec![]
    }
}

/// Select reviewers satisfying the ask-two-others rule
pub fn select_reviewers_for_domain(
    primary_domain: &str,
    available_brains: &[(String, String, u8)], // (brain_id, domain_id, capability_level)
) -> Vec<(String, String, u8, bool)> {
    // Returns: (brain_id, domain_id, level, is_domain_expert)
    let mut selected = Vec::new();
    
    // Find domain expert (capability >= 7)
    for (brain_id, domain_id, level) in available_brains {
        if domain_id == primary_domain && *level >= 7 {
            selected.push((brain_id.clone(), domain_id.clone(), *level, true));
            break;
        }
    }
    
    // Find outside-domain reviewer (capability <= 4 in primary domain, but strong in critique)
    let cross_reviewers = get_cross_reviewers(primary_domain);
    for (brain_id, domain_id, level) in available_brains {
        if domain_id != primary_domain && *level <= 4 {
            // Check if this brain is good at a recommended cross-review domain
            if cross_reviewers.iter().any(|cr| &cr.id == domain_id) {
                selected.push((brain_id.clone(), domain_id.clone(), *level, false));
                break;
            }
        }
    }
    
    selected
}
