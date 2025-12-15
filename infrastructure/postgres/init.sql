-- Cortical Compose Database Initialization
-- Creates schemas for each brain region that uses persistence

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Hippocampus Schema (Episodic Memory)
CREATE SCHEMA IF NOT EXISTS hippocampus;

CREATE TABLE hippocampus.experiences (
    experience_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    source VARCHAR(50) NOT NULL CHECK (source IN ('input', 'output', 'feedback', 'system')),
    actor VARCHAR(255),
    text TEXT NOT NULL,
    session_id UUID,
    intent VARCHAR(255),
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE hippocampus.sessions (
    session_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    metadata JSONB DEFAULT '{}'
);

CREATE INDEX idx_hippocampus_experiences_session ON hippocampus.experiences(session_id);
CREATE INDEX idx_hippocampus_experiences_timestamp ON hippocampus.experiences(timestamp);
CREATE INDEX idx_hippocampus_experiences_source ON hippocampus.experiences(source);

-- Basal Ganglia Schema (Reinforcement Learning)
CREATE SCHEMA IF NOT EXISTS basal_ganglia;

CREATE TABLE basal_ganglia.actions (
    action_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    experience_id UUID NOT NULL,
    action_type VARCHAR(255) NOT NULL,
    context JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE basal_ganglia.outcomes (
    outcome_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    action_id UUID NOT NULL REFERENCES basal_ganglia.actions(action_id),
    success BOOLEAN NOT NULL,
    reward FLOAT NOT NULL DEFAULT 0.0,
    error TEXT,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE basal_ganglia.reward_stats (
    action_type VARCHAR(255) PRIMARY KEY,
    total_count INTEGER NOT NULL DEFAULT 0,
    success_count INTEGER NOT NULL DEFAULT 0,
    total_reward FLOAT NOT NULL DEFAULT 0.0,
    avg_reward FLOAT NOT NULL DEFAULT 0.0,
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_basal_ganglia_actions_experience ON basal_ganglia.actions(experience_id);
CREATE INDEX idx_basal_ganglia_outcomes_action ON basal_ganglia.outcomes(action_id);

-- Cerebellum Schema (Error Correction)
CREATE SCHEMA IF NOT EXISTS cerebellum;

CREATE TABLE cerebellum.error_deltas (
    delta_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    experience_id UUID NOT NULL,
    expected_outcome JSONB NOT NULL,
    actual_outcome JSONB NOT NULL,
    delta_magnitude FLOAT NOT NULL,
    correction_applied JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE cerebellum.micro_policies (
    policy_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    pattern_signature VARCHAR(255) NOT NULL UNIQUE,
    adjustment JSONB NOT NULL,
    confidence FLOAT NOT NULL DEFAULT 0.5,
    hit_count INTEGER NOT NULL DEFAULT 0,
    last_used TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cerebellum_error_deltas_experience ON cerebellum.error_deltas(experience_id);
CREATE INDEX idx_cerebellum_micro_policies_pattern ON cerebellum.micro_policies(pattern_signature);

-- Amygdala Schema (Emotional Valence)
CREATE SCHEMA IF NOT EXISTS amygdala;

CREATE TABLE amygdala.valence_records (
    valence_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    experience_id UUID NOT NULL,
    source VARCHAR(50) NOT NULL CHECK (source IN ('input', 'output')),
    polarity VARCHAR(20) NOT NULL CHECK (polarity IN ('positive', 'negative')),
    score FLOAT NOT NULL CHECK (score >= -1.0 AND score <= 1.0),
    confidence FLOAT NOT NULL CHECK (confidence >= 0.0 AND confidence <= 1.0),
    labels TEXT[] DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_amygdala_valence_experience ON amygdala.valence_records(experience_id);
CREATE INDEX idx_amygdala_valence_polarity ON amygdala.valence_records(polarity);
CREATE INDEX idx_amygdala_valence_source ON amygdala.valence_records(source);

-- Audit log for observability
CREATE SCHEMA IF NOT EXISTS audit;

CREATE TABLE audit.decision_log (
    log_id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    experience_id UUID NOT NULL,
    decision_type VARCHAR(100) NOT NULL,
    confidence FLOAT,
    regions_consulted TEXT[] DEFAULT '{}',
    decision_payload JSONB,
    latency_ms INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_decision_experience ON audit.decision_log(experience_id);
CREATE INDEX idx_audit_decision_timestamp ON audit.decision_log(created_at);
