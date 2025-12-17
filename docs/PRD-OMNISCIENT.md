# Omniscient Project - Product Requirements Document

**Version**: 0.1.0  
**Status**: Draft  
**Created**: 2024-12-17  
**Authors**: Cascade + Human Architect

---

## Executive Summary

**Omniscient** is a neuroscience-inspired autonomous software system designed to remain continuously operational while simultaneously ingesting external data, responding to inputs, and observing the resulting interactions—all without interrupting its core processes. Built in Rust for performance and safety, it leverages existing neurostack architecture as its Central Processing Unit (CPU) and introduces **omni-core** as the interoperability layer for multi-system coordination.

---

## 1. Vision & Goals

### 1.1 Vision Statement
Create a self-sustaining cognitive software system that mirrors biological neural architecture—capable of continuous awareness, adaptive learning, and parallel observation without process interruption.

### 1.2 Primary Goals

| Goal | Description |
|------|-------------|
| **Continuous Operation** | Never shut down main processes to observe or learn |
| **External Data Ingestion** | Pull data from scientific tools, knowledge crawlers, APIs |
| **Reactive Processing** | Respond to inputs in real-time while maintaining state |
| **Self-Observation** | Monitor and analyze its own interactions without blocking |
| **Hardware Efficiency** | Fit within AMD + 100GB RAM + 2x Quadro 8K (48GB each) footprint |

### 1.3 Non-Goals

- Biological accuracy beyond functional analogy
- Claims of consciousness or sentience
- Autonomous self-modification of core architecture (v1)
- Real-time guarantees below 10ms latency
- Cloud-first deployment (local-first priority)

---

## 2. System Architecture

### 2.1 High-Level Topology

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                              OMNISCIENT                                      │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                         OMNI-CORE                                    │    │
│  │              (Interoperability & Message Bus)                        │    │
│  │                                                                      │    │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            │    │
│  │  │ gRPC Hub │  │ IPC Ring │  │ GPU Sched│  │ Mem Pool │            │    │
│  │  └──────────┘  └──────────┘  └──────────┘  └──────────┘            │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                    │                                         │
│         ┌──────────────────────────┼──────────────────────────┐             │
│         │                          │                          │             │
│         ▼                          ▼                          ▼             │
│  ┌─────────────────┐    ┌─────────────────────┐    ┌─────────────────┐     │
│  │   NEUROSTACK    │    │   OBSERVER RING     │    │  DATA INGESTORS │     │
│  │      (CPU)      │    │  (Non-blocking)     │    │   (External)    │     │
│  │                 │    │                     │    │                 │     │
│  │ ┌─────────────┐ │    │ ┌─────────────────┐ │    │ ┌─────────────┐ │     │
│  │ │ Prefrontal  │ │    │ │ Interaction Log │ │    │ │ Sci Papers  │ │     │
│  │ │   Cortex    │ │    │ └─────────────────┘ │    │ └─────────────┘ │     │
│  │ └─────────────┘ │    │ ┌─────────────────┐ │    │ ┌─────────────┐ │     │
│  │ ┌─────────────┐ │    │ │ Pattern Detect  │ │    │ │ Knowledge   │ │     │
│  │ │  Thalamus   │ │    │ └─────────────────┘ │    │ │  Crawlers   │ │     │
│  │ └─────────────┘ │    │ ┌─────────────────┐ │    │ └─────────────┘ │     │
│  │ ┌─────────────┐ │    │ │ Metric Stream   │ │    │ ┌─────────────┐ │     │
│  │ │ Hippocampus │ │    │ └─────────────────┘ │    │ │ MCP Servers │ │     │
│  │ └─────────────┘ │    └─────────────────────┘    │ └─────────────┘ │     │
│  │ ┌─────────────┐ │                               │ ┌─────────────┐ │     │
│  │ │Basal Ganglia│ │                               │ │ Pinecone    │ │     │
│  │ └─────────────┘ │                               │ └─────────────┘ │     │
│  │ ┌─────────────┐ │                               │ ┌─────────────┐ │     │
│  │ │ Cerebellum  │ │                               │ │ Atlassian   │ │     │
│  │ └─────────────┘ │                               │ └─────────────┘ │     │
│  │ ┌─────────────┐ │                               └─────────────────┘     │
│  │ │  Amygdala   │ │                                                        │
│  │ └─────────────┘ │                                                        │
│  └─────────────────┘                                                        │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────────────────┐    │
│  │                        GPU COMPUTE LAYER                             │    │
│  │  ┌─────────────────────────┐  ┌─────────────────────────┐           │    │
│  │  │     Quadro 8K #1        │  │     Quadro 8K #2        │           │    │
│  │  │       (48GB)            │  │       (48GB)            │           │    │
│  │  │  - Embedding Gen        │  │  - Inference Engine     │           │    │
│  │  │  - Vector Ops           │  │  - Pattern Analysis     │           │    │
│  │  │  - Fast Similarity      │  │  - Parallel Observation │           │    │
│  │  └─────────────────────────┘  └─────────────────────────┘           │    │
│  └─────────────────────────────────────────────────────────────────────┘    │
│                                                                              │
└─────────────────────────────────────────────────────────────────────────────┘
```

### 2.2 Core Components

#### 2.2.1 Neurostack (Central Processing Unit)
**Existing codebase**: `/Users/zacelston/CascadeProjects/neurostack`

The brain-inspired decision engine with 7 regions:
- **Prefrontal Cortex** - Decision orchestration, policy management
- **Thalamus** - Input gateway, embedding generation
- **Hippocampus** - Episodic memory, temporal context
- **Basal Ganglia** - Reinforcement learning, action scoring
- **Cerebellum** - Error correction, skill refinement
- **Amygdala** - Emotional valence, risk weighting

**Integration**: Neurostack becomes the "thinking" core—all decisions route through its Prefrontal Cortex.

#### 2.2.2 Omni-Core (Interoperability Layer)
**New component** - The nervous system connecting all parts.

| Subsystem | Responsibility |
|-----------|----------------|
| **gRPC Hub** | High-performance inter-process communication |
| **IPC Ring Buffer** | Lock-free message passing for observer pattern |
| **GPU Scheduler** | Workload distribution across dual Quadros |
| **Memory Pool** | Shared memory regions for zero-copy data transfer |
| **Health Monitor** | Watchdog for all subsystems |

#### 2.2.3 Observer Ring (Non-Blocking Observation)
**Key Innovation** - Observe without interrupting.

```rust
// Conceptual design
pub struct ObserverRing {
    /// Lock-free ring buffer for interaction events
    interaction_log: Arc<RingBuffer<InteractionEvent>>,
    
    /// Pattern detection runs on GPU #2
    pattern_detector: PatternDetector,
    
    /// Metrics stream for real-time monitoring
    metrics: MetricsStream,
}

impl ObserverRing {
    /// Non-blocking observation - never blocks main processing
    pub fn observe(&self, event: InteractionEvent) {
        // Fire-and-forget into ring buffer
        self.interaction_log.push(event);
    }
    
    /// Background analysis on separate thread/GPU
    pub async fn analyze_patterns(&self) -> PatternReport {
        // Runs independently of main decision loop
    }
}
```

#### 2.2.4 Data Ingestors (External Data Sources)
Modular connectors for external knowledge:

| Ingestor | Source | Purpose |
|----------|--------|---------|
| **Scientific Papers** | arXiv, PubMed, OpenAlex | Research knowledge |
| **Knowledge Crawlers** | Web, APIs | Real-time information |
| **MCP Servers** | Atlassian, Pinecone, Memory | Tool integration |
| **Vector Stores** | Qdrant, Pinecone | Semantic memory |

---

## 3. Hardware Constraints & Resource Allocation

### 3.1 Hardware Specification

| Component | Specification | Allocation |
|-----------|---------------|------------|
| **CPU** | AMD (multi-core) | Neurostack services, Omni-Core |
| **RAM** | 100 GB | See allocation below |
| **GPU #1** | Quadro 8K (48GB) | Embeddings, vector ops, similarity |
| **GPU #2** | Quadro 8K (48GB) | Inference, pattern analysis, observation |

### 3.2 Memory Allocation Strategy

```
┌─────────────────────────────────────────────────────────────────┐
│                    100 GB RAM ALLOCATION                         │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ NEUROSTACK SERVICES                           35 GB      │   │
│  │  - Prefrontal Cortex: 8 GB                               │   │
│  │  - Thalamus: 4 GB                                        │   │
│  │  - Hippocampus: 6 GB                                     │   │
│  │  - Basal Ganglia: 5 GB                                   │   │
│  │  - Cerebellum: 5 GB                                      │   │
│  │  - Amygdala: 5 GB                                        │   │
│  │  - Cortical-Shared: 2 GB                                 │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ OMNI-CORE                                     20 GB      │   │
│  │  - gRPC Hub: 4 GB                                        │   │
│  │  - IPC Ring Buffers: 8 GB                                │   │
│  │  - Memory Pool (shared): 6 GB                            │   │
│  │  - Health Monitor: 2 GB                                  │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ OBSERVER RING                                 15 GB      │   │
│  │  - Interaction Log Buffer: 8 GB                          │   │
│  │  - Pattern Detection Cache: 5 GB                         │   │
│  │  - Metrics Stream: 2 GB                                  │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ DATA INGESTORS                                15 GB      │   │
│  │  - Scientific Papers Cache: 5 GB                         │   │
│  │  - Knowledge Crawler Buffer: 4 GB                        │   │
│  │  - MCP Server Connections: 3 GB                          │   │
│  │  - Vector Store Client: 3 GB                             │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ VECTOR STORES (Qdrant/PostgreSQL)             10 GB      │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │ OS & SYSTEM OVERHEAD                           5 GB      │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 3.3 GPU Workload Distribution

**GPU #1 (48GB) - "Sensory Processing"**
- Embedding generation (Thalamus)
- Vector similarity search
- Real-time input processing
- Allocation: 40GB active, 8GB buffer

**GPU #2 (48GB) - "Cognitive Processing"**
- LLM inference (if local models)
- Pattern analysis (Observer Ring)
- Parallel observation processing
- Background learning tasks
- Allocation: 40GB active, 8GB buffer

---

## 4. Technical Requirements

### 4.1 Language & Runtime

| Aspect | Choice | Rationale |
|--------|--------|-----------|
| **Primary Language** | Rust | Memory safety, performance, concurrency |
| **Fallback** | C | Post-prototype optimization if needed |
| **Async Runtime** | Tokio | Proven, high-performance async |
| **Serialization** | Serde + Protobuf | JSON for debug, Protobuf for perf |

### 4.2 Core Dependencies

```toml
[dependencies]
# Async runtime
tokio = { version = "1.35", features = ["full"] }

# Web framework (existing neurostack)
axum = "0.7"

# gRPC for omni-core
tonic = "0.10"
prost = "0.12"

# Lock-free data structures
crossbeam = "0.8"
parking_lot = "0.12"

# GPU compute
cudarc = "0.10"  # CUDA bindings

# Vector operations
ndarray = "0.15"
faiss = "0.12"  # GPU-accelerated similarity

# Observability
tracing = "0.1"
tracing-subscriber = "0.3"
metrics = "0.22"

# Database clients
qdrant-client = "1.7"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio"] }
```

### 4.3 Non-Blocking Requirements

| Requirement | Implementation |
|-------------|----------------|
| **Lock-free observation** | Crossbeam MPMC channels, ring buffers |
| **Zero-copy messaging** | Shared memory regions via `memmap2` |
| **Async-first design** | All I/O through Tokio, no blocking calls |
| **Graceful degradation** | Circuit breakers, fallback paths |
| **Hot reload** | Dynamic config without restart |

---

## 5. Functional Requirements

### 5.1 Continuous Operation

**FR-1.1**: System MUST remain operational 24/7 without scheduled downtime for observation or learning.

**FR-1.2**: Main decision loop (Prefrontal Cortex) MUST NOT block on observation tasks.

**FR-1.3**: System MUST support hot configuration updates without restart.

**FR-1.4**: Individual components MUST be restartable without full system shutdown.

### 5.2 External Data Ingestion

**FR-2.1**: System MUST support concurrent ingestion from multiple data sources.

**FR-2.2**: Data ingestors MUST operate independently of the main decision loop.

**FR-2.3**: System MUST support the following data sources (v1):
- Scientific paper APIs (arXiv, OpenAlex, PubMed)
- MCP server integrations (Atlassian, Pinecone, Memory)
- Custom knowledge crawlers

**FR-2.4**: Ingested data MUST be normalized through Thalamus before storage.

### 5.3 Reactive Processing

**FR-3.1**: System MUST respond to inputs within 100ms p95 latency.

**FR-3.2**: Decision flow MUST follow neurostack pathway model:
- Fast reflex (amygdala-negative threat detection)
- Deliberate loop (full region consultation)
- Deferred (request more data)

**FR-3.3**: System MUST maintain context across interactions via Hippocampus.

### 5.4 Self-Observation

**FR-4.1**: All interactions MUST be logged to the Observer Ring without blocking.

**FR-4.2**: Pattern detection MUST run asynchronously on GPU #2.

**FR-4.3**: System MUST expose real-time metrics stream.

**FR-4.4**: Observation data MUST be queryable for post-hoc analysis.

---

## 6. Integration Points

### 6.1 Neurostack Integration

```rust
// Omni-Core wraps neurostack's Prefrontal Cortex
pub struct OmniscientCore {
    /// The thinking brain - neurostack
    neurostack: PrefrontalCortexClient,
    
    /// Interoperability layer
    omni_core: OmniCore,
    
    /// Non-blocking observation
    observer: ObserverRing,
    
    /// External data sources
    ingestors: Vec<Box<dyn DataIngestor>>,
}

impl OmniscientCore {
    pub async fn process(&self, input: Input) -> Decision {
        // 1. Log to observer (non-blocking)
        self.observer.observe(InteractionEvent::Input(input.clone()));
        
        // 2. Route through neurostack
        let decision = self.neurostack.decide(input.into()).await?;
        
        // 3. Log decision (non-blocking)
        self.observer.observe(InteractionEvent::Decision(decision.clone()));
        
        decision
    }
}
```

### 6.2 MCP Server Integration

Leverage existing MCP servers:
- **Atlassian** - Jira/Confluence for project knowledge
- **Pinecone** - Vector search at scale
- **Memory** - Knowledge graph persistence
- **Scientific Papers** - Research ingestion

### 6.3 Omni-Core Protocol

```protobuf
syntax = "proto3";

package omnicore;

service OmniCore {
    // Route message to appropriate subsystem
    rpc Route(Message) returns (Response);
    
    // Subscribe to event stream
    rpc Subscribe(SubscribeRequest) returns (stream Event);
    
    // Health check all subsystems
    rpc HealthCheck(Empty) returns (HealthReport);
}

message Message {
    string source = 1;
    string destination = 2;
    bytes payload = 3;
    map<string, string> metadata = 4;
}
```

---

## 7. Development Phases

### Phase 1: Foundation (Weeks 1-3)
- [ ] Set up Omniscient project structure
- [ ] Create omni-core skeleton with gRPC
- [ ] Integrate existing neurostack as dependency
- [ ] Implement basic IPC ring buffer
- [ ] Establish GPU workload distribution

### Phase 2: Observer Ring (Weeks 4-5)
- [ ] Implement lock-free interaction logging
- [ ] Create pattern detection pipeline
- [ ] Set up metrics streaming
- [ ] GPU #2 integration for observation

### Phase 3: Data Ingestors (Weeks 6-7)
- [ ] Scientific papers ingestor (arXiv, OpenAlex)
- [ ] MCP server connectors
- [ ] Knowledge crawler framework
- [ ] Thalamus integration for normalization

### Phase 4: Integration & Testing (Weeks 8-10)
- [ ] End-to-end integration testing
- [ ] Performance benchmarking
- [ ] Memory/GPU optimization
- [ ] Documentation

### Phase 5: Hardening (Weeks 11-12)
- [ ] Circuit breakers and fallbacks
- [ ] Hot reload implementation
- [ ] Production deployment config
- [ ] Monitoring dashboards

---

## 8. Success Criteria

| Metric | Target |
|--------|--------|
| **Uptime** | 99.9% (no planned downtime) |
| **Decision Latency** | < 100ms p95 |
| **Observation Overhead** | < 1% of main loop time |
| **Memory Usage** | < 95GB peak |
| **GPU Utilization** | > 60% average |
| **Data Ingestion Rate** | > 100 documents/minute |

---

## 9. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| GPU memory exhaustion | High | Strict allocation limits, spill to RAM |
| Lock contention in observer | Medium | Lock-free data structures only |
| Neurostack integration complexity | Medium | Thin wrapper, minimal changes |
| Data ingestor failures | Low | Circuit breakers, retry logic |
| Hot reload instability | Medium | Staged rollout, rollback capability |

---

## 10. Open Questions

1. **Local LLM vs API**: Should inference run locally on GPU #2 or use external APIs?
2. **Persistence Strategy**: Extend Qdrant/Postgres or add new stores for observation data?
3. **Multi-node**: Future support for distributed deployment?
4. **C Migration**: Criteria for deciding when to port from Rust to C?

---

## Appendix A: Neurostack Crate Structure

```
neurostack/
├── crates/
│   ├── cortical-shared/     # Shared types, clients, policies
│   ├── prefrontal-cortex/   # Decision orchestration
│   ├── thalamus/            # Input gateway, embeddings
│   ├── hippocampus/         # Episodic memory
│   ├── basal-ganglia/       # Reinforcement learning
│   ├── cerebellum/          # Error correction
│   └── amygdala/            # Emotional valence
```

## Appendix B: Glossary

| Term | Definition |
|------|------------|
| **Omni-Core** | Interoperability layer connecting all Omniscient subsystems |
| **Observer Ring** | Non-blocking observation system using ring buffers |
| **Neurostack** | Brain-inspired decision engine (existing codebase) |
| **Data Ingestor** | Module for pulling external data into the system |
| **Fast Reflex** | Amygdala-driven threat detection pathway |
| **Deliberate Loop** | Full region consultation for complex decisions |

---

*This PRD is a living document. Updates will be tracked via version control.*
