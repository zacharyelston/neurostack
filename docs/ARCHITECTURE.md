# Neurostack Architecture

## Overview

Neurostack is a brain-inspired cognitive architecture implemented as a distributed system of Rust microservices. Each service represents a brain region with specialized responsibilities.

---

## System Model

```
                                    ┌─────────────────────────────────────┐
                                    │         EXTERNAL BRAINS             │
                                    │    (via /interop endpoints)         │
                                    └──────────────┬──────────────────────┘
                                                   │
                                                   │ X25519 + ChaCha20
                                                   │ encrypted messages
                                                   ▼
┌──────────────────────────────────────────────────────────────────────────────┐
│                              NEUROSTACK BRAIN                                 │
│                                                                               │
│  ┌─────────────────────────────────────────────────────────────────────────┐ │
│  │                         INPUT LAYER                                      │ │
│  │                                                                          │ │
│  │    External Input ──▶ [THALAMUS] ──▶ Feature Packet (embedding + meta)  │ │
│  │                         :8001                                            │ │
│  │                                                                          │ │
│  └─────────────────────────────────────────────────────────────────────────┘ │
│                                    │                                          │
│                                    ▼                                          │
│  ┌─────────────────────────────────────────────────────────────────────────┐ │
│  │                      ORCHESTRATION LAYER                                 │ │
│  │                                                                          │ │
│  │                    [PREFRONTAL CORTEX]                                   │ │
│  │                          :8000                                           │ │
│  │                                                                          │ │
│  │    Pathways:                                                             │ │
│  │    ├── fast_reflex()      → Amygdala threat check (< 150ms)             │ │
│  │    ├── deliberate_loop()  → Full region consultation                    │ │
│  │    └── help_seeking()     → Cross-brain assistance triggers             │ │
│  │                                                                          │ │
│  └─────────────────────────────────────────────────────────────────────────┘ │
│                                    │                                          │
│                    ┌───────────────┼───────────────┐                         │
│                    │               │               │                         │
│                    ▼               ▼               ▼                         │
│  ┌─────────────────────────────────────────────────────────────────────────┐ │
│  │                        MEMORY LAYER                                      │ │
│  │                                                                          │ │
│  │  [HIPPOCAMPUS]    [BASAL GANGLIA]    [CEREBELLUM]    [AMYGDALA]         │ │
│  │     :8002             :8003             :8004          :8005             │ │
│  │                                                                          │ │
│  │  Episodic          Action/Reward       Error           Emotional         │ │
│  │  Memory            Learning            Correction      Valence           │ │
│  │                                                                          │ │
│  └─────────────────────────────────────────────────────────────────────────┘ │
│                                    │                                          │
│                                    ▼                                          │
│  ┌─────────────────────────────────────────────────────────────────────────┐ │
│  │                      PERSISTENCE LAYER                                   │ │
│  │                                                                          │ │
│  │         [QDRANT]                           [POSTGRESQL]                  │ │
│  │          :6333                                :5432                      │ │
│  │                                                                          │ │
│  │    Vector similarity                    Relational metadata              │ │
│  │    for embeddings                       and audit logs                   │ │
│  │                                                                          │ │
│  └─────────────────────────────────────────────────────────────────────────┘ │
│                                                                               │
└──────────────────────────────────────────────────────────────────────────────┘
```

---

## Crate Organization

### Current Structure

```
neurostack/
├── Cargo.toml                    # Workspace root
├── docker-compose.yml
├── docs/
│   ├── ARCHITECTURE.md           # This file
│   ├── PROJECT_RULES.md          # Development standards
│   ├── topology.yaml             # Brain topology design
│   ├── interop.yaml              # Inter-brain protocol
│   ├── omni_core_comms.yaml      # Crypto layer design
│   ├── cross_brain_assistance.yaml
│   ├── neurostack_domain_catalog.yaml
│   ├── expose-knobs.yaml         # Policy configuration
│   └── prd.yaml                  # Deviations from biology
│
├── crates/
│   ├── cortical-shared/          # Shared types and utilities
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── models.rs         # Core data types
│   │       ├── errors.rs         # Unified error types
│   │       ├── config.rs         # Service configuration
│   │       ├── clients.rs        # HTTP clients
│   │       ├── policies.rs       # Fast/slow path policies
│   │       ├── interop.rs        # Inter-brain protocol types
│   │       ├── crypto.rs         # X25519 + ChaCha20
│   │       ├── assistance.rs     # Help-seeking types
│   │       └── domains.rs        # Domain catalog
│   │
│   ├── thalamus/                 # Input gateway
│   │   └── src/
│   │       ├── main.rs           # Axum server + /ingest
│   │       └── embeddings.rs     # Mock embedding service
│   │
│   ├── prefrontal-cortex/        # Orchestrator
│   │   └── src/
│   │       ├── main.rs           # Axum server + /decide
│   │       ├── pathways.rs       # fast_reflex, deliberate_loop
│   │       └── help_seeking.rs   # Cross-brain triggers
│   │
│   ├── hippocampus/              # Episodic memory
│   │   └── src/main.rs           # Stub: /store, /query
│   │
│   ├── basal-ganglia/            # Reinforcement learning
│   │   └── src/main.rs           # Stub: /store, /query
│   │
│   ├── cerebellum/               # Error correction
│   │   └── src/main.rs           # Stub: /store, /query
│   │
│   └── amygdala/                 # Emotional valence
│       └── src/main.rs           # Stub: /store, /query
│
└── infrastructure/
    └── postgres/
        └── init.sql              # Schema definitions
```

### Proposed Reorganization

The `cortical-shared` crate has grown to 9 modules. Group by concern:

```
crates/cortical-shared/src/
├── lib.rs
│
├── core/                         # Core types (always needed)
│   ├── mod.rs
│   ├── models.rs                 # Experience, FeaturePacket, Decision
│   ├── errors.rs                 # CorticalError
│   └── config.rs                 # Service configs
│
├── comms/                        # Communication (inter-service + inter-brain)
│   ├── mod.rs
│   ├── clients.rs                # HTTP clients for regions
│   ├── interop.rs                # Inter-brain protocol
│   └── crypto.rs                 # X25519 + ChaCha20
│
└── policy/                       # Decision policies
    ├── mod.rs
    ├── pathways.rs               # Fast/slow path policies
    ├── assistance.rs             # Help-seeking triggers
    └── domains.rs                # Domain catalog
```

---

## Data Flow

### 1. Input Processing (Thalamus)

```
POST /ingest
  │
  ├── Validate input text (max 8192 chars)
  ├── Normalize text (lowercase, trim, collapse whitespace)
  ├── Generate embedding (384-dim vector)
  └── Return FeaturePacket {
        experience_id: UUID,
        embedding: Vec<f32>,
        source: Input | Output,
        context: Option<String>,
        timestamp: DateTime
      }
```

### 2. Decision Flow (Prefrontal Cortex)

```
POST /decide
  │
  ├── Step 1: Get embedding from Thalamus
  │
  ├── Step 2: Fast path check (< 150ms budget)
  │   └── Query Amygdala for negative experiences
  │       └── If similarity > 0.82 AND threat_confidence > 0.70
  │           └── RETURN: Defer (threat detected)
  │
  ├── Step 3: Deliberate loop (parallel queries)
  │   ├── Hippocampus.query(embedding)  → episodic matches
  │   ├── Basal Ganglia.query(embedding) → action patterns
  │   ├── Cerebellum.query(embedding)   → error corrections
  │   └── Amygdala.query(embedding)     → valence context
  │
  ├── Step 4: Basal Ganglia gate check
  │   └── If require_bg_gate AND BG blocks
  │       └── RETURN: Defer or RequestMoreData
  │
  ├── Step 5: Help-seeking evaluation
  │   └── Check triggers: ConfidenceRisk, Novelty, HighStakes, etc.
  │       └── If triggered AND !can_proceed_alone
  │           └── RETURN: RequestMoreData (seek external review)
  │
  └── Step 6: Return Decision {
        experience_id,
        action: Act | Defer | RequestMoreData,
        confidence: f32,
        reasoning: String,
        similar_experiences: Vec<SimilarityResult>,
        regions_consulted: Vec<String>
      }
```

### 3. Memory Storage (Region Services)

```
POST /store
  │
  ├── Receive StoreRequest {
  │     experience_id,
  │     embedding,
  │     payload (region-specific)
  │   }
  │
  ├── Store embedding in Qdrant collection
  ├── Store metadata in PostgreSQL
  └── Return StoreResponse { success, id }

POST /query
  │
  ├── Receive QueryRequest {
  │     embedding,
  │     limit,
  │     filters (optional)
  │   }
  │
  ├── Search Qdrant for similar vectors
  ├── Enrich with PostgreSQL metadata
  └── Return QueryResponse {
        results: Vec<SimilarityResult>,
        region: String
      }
```

---

## Inter-Brain Communication

### Protocol Stack

```
┌─────────────────────────────────────────┐
│           Application Layer              │
│  InteropMessage, TaskContract, Feedback  │
├─────────────────────────────────────────┤
│            Policy Layer                  │
│  Help-seeking, Reviewer selection        │
├─────────────────────────────────────────┤
│            Crypto Layer                  │
│  X25519 key exchange, ChaCha20 AEAD      │
├─────────────────────────────────────────┤
│           Transport Layer                │
│  HTTPS, EncryptedEnvelope                │
└─────────────────────────────────────────┘
```

### Handshake Flow

```
Brain A                                    Brain B
   │                                          │
   │  POST /interop/handshake/init            │
   │  { brain_id, public_key, capabilities }  │
   │ ────────────────────────────────────────▶│
   │                                          │
   │  { challenge, responder_public_key }     │
   │ ◀────────────────────────────────────────│
   │                                          │
   │  [Derive shared secret via DH]           │
   │                                          │
   │  POST /interop/handshake/complete        │
   │  { challenge_response: HMAC(challenge) } │
   │ ────────────────────────────────────────▶│
   │                                          │
   │  { success, session_id, capabilities }   │
   │ ◀────────────────────────────────────────│
   │                                          │
   │  [Encrypted channel established]         │
   │                                          │
```

---

## Policy Configuration

### expose-knobs.yaml

```yaml
policies:
  fast_path:
    enabled: true
    negative_similarity_threshold: 0.82
    threat_confidence_threshold: 0.70
    max_latency_ms: 150
    
  slow_path:
    enabled: true
    min_confidence_to_answer: 0.65
    novelty_threshold: 0.60
    require_bg_gate: true
    
  thalamus_gating:
    enabled: true
    allow_pfc_feedback: true
```

### Help-Seeking Triggers

| Trigger | Condition | Action |
|---------|-----------|--------|
| ConfidenceRisk | confidence < 0.65 OR (confidence > 0.85 AND low evidence) | Seek critique |
| NoveltyOrAmbiguity | novelty_score > 0.60 | Seek alternatives |
| HighStakes | irreversible OR safety_impact | Seek risk assessment |
| StuckState | revisions >= 2 without improvement | Seek alternatives |
| RegionConflict | BG blocks but confidence > 0.70 | Seek critique |

---

## File Size Compliance

All files under 500-line limit (PROJECT_RULES.md):

| File | Lines | Status |
|------|-------|--------|
| interop.rs | 426 | ✅ |
| pathways.rs | 310 | ✅ |
| assistance.rs | 273 | ✅ |
| models.rs | 263 | ✅ |
| domains.rs | 259 | ✅ |
| crypto.rs | 245 | ✅ |
| clients.rs | 253 | ✅ |
| config.rs | 232 | ✅ |
| policies.rs | 221 | ✅ |

---

## Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| **cortical-shared** | ✅ Complete | Core types, clients, policies, crypto |
| **thalamus** | ⚠️ Stub | Mock embeddings, needs real model |
| **prefrontal-cortex** | ✅ Complete | Pathways + help-seeking integrated |
| **hippocampus** | ⚠️ Stub | Needs Qdrant + PostgreSQL integration |
| **basal-ganglia** | ⚠️ Stub | Needs Qdrant + PostgreSQL integration |
| **cerebellum** | ⚠️ Stub | Needs Qdrant + PostgreSQL integration |
| **amygdala** | ⚠️ Stub | Needs Qdrant + PostgreSQL integration |
| **Inter-brain API** | 📋 Designed | Needs endpoint implementation |
| **Brain Registry** | 📋 Designed | Needs implementation |
| **Sync Service** | 📋 Designed | Needs implementation |

---

## Next Steps

1. **Reorganize cortical-shared** into `core/`, `comms/`, `policy/` subdirectories
2. **Implement Qdrant integration** for memory regions
3. **Implement PostgreSQL integration** for metadata storage
4. **Add real embedding model** to Thalamus (Candle/ONNX)
5. **Implement inter-brain API endpoints** in Prefrontal Cortex
6. **Add Brain Registry** and Sync Service
7. **Write integration tests** for decision flow
