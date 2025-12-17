# Omniscient Project - Task Breakdown

**Reference**: [PRD-OMNISCIENT.md](./PRD-OMNISCIENT.md)  
**Created**: 2024-12-17

---

## Task Organization

Tasks are organized into **parallel workstreams** that can be executed by different processes/agents simultaneously. Dependencies are explicitly marked.

---

## Workstream 1: Omni-Core Foundation

**Owner**: Core Infrastructure Team  
**Priority**: P0 (Critical Path)  
**Dependencies**: None (can start immediately)

### Task 1.1: Project Scaffolding
```
ID: OMNI-001
Type: Setup
Effort: 2 hours
Parallel: Yes

Description:
  Create the omniscient project structure alongside neurostack.

Deliverables:
  - /Users/zacelston/CascadeProjects/omniscient/
  - Cargo.toml (workspace)
  - README.md
  - .gitignore
  - docker-compose.yml (skeleton)

Acceptance Criteria:
  - [ ] `cargo build` succeeds
  - [ ] Project structure mirrors neurostack patterns
```

### Task 1.2: Omni-Core Crate
```
ID: OMNI-002
Type: Implementation
Effort: 8 hours
Parallel: Yes (after OMNI-001)

Description:
  Create the omni-core crate with gRPC hub and basic message routing.

Deliverables:
  - crates/omni-core/
  - proto/omnicore.proto
  - src/grpc_hub.rs
  - src/router.rs
  - src/lib.rs

Acceptance Criteria:
  - [ ] gRPC server starts on port 9000
  - [ ] Basic message routing works
  - [ ] Health check endpoint responds
```

### Task 1.3: IPC Ring Buffer
```
ID: OMNI-003
Type: Implementation
Effort: 6 hours
Parallel: Yes (after OMNI-001)

Description:
  Implement lock-free ring buffer for inter-process communication.

Deliverables:
  - crates/omni-core/src/ipc/
  - ring_buffer.rs (lock-free SPMC)
  - shared_memory.rs (memmap2 wrapper)
  - tests/ipc_tests.rs

Acceptance Criteria:
  - [ ] Lock-free push/pop operations
  - [ ] Zero-copy message passing
  - [ ] Benchmark: >1M messages/sec
```

### Task 1.4: Memory Pool Manager
```
ID: OMNI-004
Type: Implementation
Effort: 4 hours
Parallel: Yes (after OMNI-001)

Description:
  Shared memory pool for zero-copy data transfer between components.

Deliverables:
  - crates/omni-core/src/memory/
  - pool.rs
  - allocator.rs
  - guard.rs (RAII cleanup)

Acceptance Criteria:
  - [ ] Pre-allocated memory regions
  - [ ] Thread-safe allocation/deallocation
  - [ ] Memory usage tracking
```

### Task 1.5: GPU Scheduler
```
ID: OMNI-005
Type: Implementation
Effort: 8 hours
Parallel: Yes (after OMNI-001)
Dependencies: GPU hardware available

Description:
  Workload distribution across dual Quadro 8K GPUs.

Deliverables:
  - crates/omni-core/src/gpu/
  - scheduler.rs
  - device_manager.rs
  - memory_tracker.rs

Acceptance Criteria:
  - [ ] Detect both GPUs
  - [ ] Assign workloads by type (embedding vs inference)
  - [ ] Memory limit enforcement (40GB active per GPU)
```

---

## Workstream 2: Neurostack Integration

**Owner**: Integration Team  
**Priority**: P0 (Critical Path)  
**Dependencies**: OMNI-001

### Task 2.1: Neurostack as Dependency
```
ID: OMNI-010
Type: Integration
Effort: 4 hours
Parallel: Yes (after OMNI-001)

Description:
  Add neurostack crates as dependencies to omniscient.

Deliverables:
  - Update Cargo.toml with path dependencies
  - Create integration wrapper module
  - Verify all neurostack APIs accessible

Acceptance Criteria:
  - [ ] Can import cortical_shared types
  - [ ] Can create PrefrontalCortexClient
  - [ ] Can create RegionClient instances
```

### Task 2.2: OmniscientCore Wrapper
```
ID: OMNI-011
Type: Implementation
Effort: 6 hours
Parallel: No
Dependencies: OMNI-010, OMNI-002

Description:
  Create the main OmniscientCore struct that wraps neurostack.

Deliverables:
  - crates/omniscient-core/src/lib.rs
  - OmniscientCore struct
  - process() method
  - Integration with omni-core routing

Acceptance Criteria:
  - [ ] Decisions route through PrefrontalCortex
  - [ ] Non-blocking observation hooks in place
  - [ ] Graceful degradation if neurostack unavailable
```

### Task 2.3: Unified Health Check
```
ID: OMNI-012
Type: Implementation
Effort: 2 hours
Parallel: Yes (after OMNI-011)

Description:
  Aggregate health from all neurostack regions + omni-core.

Deliverables:
  - Health aggregation endpoint
  - Per-component status
  - Degraded mode detection

Acceptance Criteria:
  - [ ] Single /health endpoint for entire system
  - [ ] Reports status of all 7 neurostack regions
  - [ ] Reports omni-core subsystem status
```

---

## Workstream 3: Observer Ring

**Owner**: Observation Team  
**Priority**: P1 (High)  
**Dependencies**: OMNI-003

### Task 3.1: Observer Ring Core
```
ID: OMNI-020
Type: Implementation
Effort: 6 hours
Parallel: Yes (after OMNI-003)

Description:
  Non-blocking observation system using ring buffers.

Deliverables:
  - crates/observer-ring/
  - src/lib.rs
  - src/interaction_log.rs
  - src/event_types.rs

Acceptance Criteria:
  - [ ] observe() never blocks main thread
  - [ ] Events persisted to ring buffer
  - [ ] Configurable buffer size
```

### Task 3.2: Pattern Detection Pipeline
```
ID: OMNI-021
Type: Implementation
Effort: 10 hours
Parallel: Yes (after OMNI-020, OMNI-005)

Description:
  Background pattern analysis running on GPU #2.

Deliverables:
  - src/pattern_detector.rs
  - src/gpu_analysis.rs
  - Pattern types and thresholds

Acceptance Criteria:
  - [ ] Runs on GPU #2 exclusively
  - [ ] Detects repetitive patterns
  - [ ] Detects anomalies
  - [ ] Does not impact main decision latency
```

### Task 3.3: Metrics Stream
```
ID: OMNI-022
Type: Implementation
Effort: 4 hours
Parallel: Yes (after OMNI-020)

Description:
  Real-time metrics streaming for monitoring.

Deliverables:
  - src/metrics_stream.rs
  - Prometheus/OpenMetrics export
  - Key metrics: latency, throughput, memory

Acceptance Criteria:
  - [ ] Metrics endpoint at /metrics
  - [ ] Histogram for decision latency
  - [ ] Gauge for memory usage
  - [ ] Counter for events processed
```

### Task 3.4: Observation Query API
```
ID: OMNI-023
Type: Implementation
Effort: 4 hours
Parallel: Yes (after OMNI-020)

Description:
  Query interface for post-hoc analysis of observations.

Deliverables:
  - src/query.rs
  - Time-range queries
  - Pattern-based filtering

Acceptance Criteria:
  - [ ] Query by time range
  - [ ] Query by event type
  - [ ] Export to JSON/CSV
```

---

## Workstream 4: Data Ingestors

**Owner**: Data Team  
**Priority**: P1 (High)  
**Dependencies**: OMNI-002, OMNI-010

### Task 4.1: Ingestor Trait Definition
```
ID: OMNI-030
Type: Design
Effort: 2 hours
Parallel: Yes (after OMNI-001)

Description:
  Define the DataIngestor trait for all external data sources.

Deliverables:
  - crates/ingestors/src/lib.rs
  - DataIngestor trait
  - IngestResult type
  - Error types

Acceptance Criteria:
  - [ ] Trait is async-compatible
  - [ ] Supports backpressure
  - [ ] Supports graceful shutdown
```

### Task 4.2: Scientific Papers Ingestor
```
ID: OMNI-031
Type: Implementation
Effort: 8 hours
Parallel: Yes (after OMNI-030)

Description:
  Ingestor for arXiv, OpenAlex, PubMed via MCP scientific-papers server.

Deliverables:
  - crates/ingestors/src/scientific_papers.rs
  - Integration with MCP scientific-papers
  - Rate limiting
  - Caching layer

Acceptance Criteria:
  - [ ] Fetch papers by category
  - [ ] Fetch papers by search query
  - [ ] Extract full text when available
  - [ ] Route through Thalamus for embedding
```

### Task 4.3: Knowledge Crawler Framework
```
ID: OMNI-032
Type: Implementation
Effort: 10 hours
Parallel: Yes (after OMNI-030)

Description:
  Extensible web crawler for knowledge acquisition.

Deliverables:
  - crates/ingestors/src/crawler/
  - Crawler trait
  - URL frontier
  - Content extraction
  - robots.txt compliance

Acceptance Criteria:
  - [ ] Respects robots.txt
  - [ ] Configurable crawl rate
  - [ ] Content type detection
  - [ ] Deduplication
```

### Task 4.4: MCP Server Connectors
```
ID: OMNI-033
Type: Implementation
Effort: 6 hours
Parallel: Yes (after OMNI-030)

Description:
  Connectors for existing MCP servers (Atlassian, Pinecone, Memory).

Deliverables:
  - crates/ingestors/src/mcp/
  - atlassian.rs
  - pinecone.rs
  - memory.rs

Acceptance Criteria:
  - [ ] Connect to configured MCP servers
  - [ ] Bidirectional data flow
  - [ ] Error handling and retry
```

### Task 4.5: Thalamus Integration for Ingestors
```
ID: OMNI-034
Type: Integration
Effort: 4 hours
Parallel: No
Dependencies: OMNI-031, OMNI-032, OMNI-033, OMNI-010

Description:
  Route all ingested data through Thalamus for normalization.

Deliverables:
  - Ingestion pipeline
  - Batch embedding support
  - Queue management

Acceptance Criteria:
  - [ ] All external data normalized via Thalamus
  - [ ] Batch processing for efficiency
  - [ ] Backpressure when Thalamus overloaded
```

---

## Workstream 5: Testing & Benchmarking

**Owner**: QA Team  
**Priority**: P1 (High)  
**Dependencies**: All implementation tasks

### Task 5.1: Unit Test Suite
```
ID: OMNI-040
Type: Testing
Effort: 8 hours
Parallel: Yes (ongoing)

Description:
  Comprehensive unit tests for all crates.

Deliverables:
  - tests/ in each crate
  - Mocking infrastructure
  - CI integration

Acceptance Criteria:
  - [ ] >80% code coverage
  - [ ] All public APIs tested
  - [ ] CI runs on every PR
```

### Task 5.2: Integration Tests
```
ID: OMNI-041
Type: Testing
Effort: 8 hours
Parallel: No
Dependencies: OMNI-011, OMNI-020, OMNI-034

Description:
  End-to-end integration tests.

Deliverables:
  - tests/integration/
  - Docker-based test environment
  - Scenario-based tests

Acceptance Criteria:
  - [ ] Full decision flow tested
  - [ ] Observer ring verified non-blocking
  - [ ] Data ingestion pipeline tested
```

### Task 5.3: Performance Benchmarks
```
ID: OMNI-042
Type: Testing
Effort: 6 hours
Parallel: Yes (after OMNI-041)

Description:
  Benchmark suite for performance validation.

Deliverables:
  - benches/
  - Criterion benchmarks
  - Memory profiling
  - GPU utilization tracking

Acceptance Criteria:
  - [ ] Decision latency <100ms p95
  - [ ] Observation overhead <1%
  - [ ] Memory within 95GB limit
  - [ ] GPU utilization >60%
```

### Task 5.4: Stress Testing
```
ID: OMNI-043
Type: Testing
Effort: 4 hours
Parallel: Yes (after OMNI-042)

Description:
  Stress tests for continuous operation.

Deliverables:
  - 24-hour soak test
  - Memory leak detection
  - Resource exhaustion handling

Acceptance Criteria:
  - [ ] No memory leaks over 24 hours
  - [ ] Graceful degradation under load
  - [ ] Recovery from component failures
```

---

## Workstream 6: Documentation & DevOps

**Owner**: Platform Team  
**Priority**: P2 (Medium)  
**Dependencies**: None (can start immediately)

### Task 6.1: Architecture Documentation
```
ID: OMNI-050
Type: Documentation
Effort: 4 hours
Parallel: Yes

Description:
  Detailed architecture documentation with diagrams.

Deliverables:
  - docs/architecture.md
  - Component diagrams
  - Data flow diagrams
  - Sequence diagrams

Acceptance Criteria:
  - [ ] All components documented
  - [ ] Integration points clear
  - [ ] Diagrams in Mermaid format
```

### Task 6.2: API Documentation
```
ID: OMNI-051
Type: Documentation
Effort: 4 hours
Parallel: Yes (after implementation)

Description:
  API documentation for all endpoints.

Deliverables:
  - OpenAPI spec
  - gRPC documentation
  - Usage examples

Acceptance Criteria:
  - [ ] All endpoints documented
  - [ ] Request/response examples
  - [ ] Error codes explained
```

### Task 6.3: Docker Compose Production
```
ID: OMNI-052
Type: DevOps
Effort: 4 hours
Parallel: Yes

Description:
  Production-ready Docker Compose configuration.

Deliverables:
  - docker-compose.yml
  - docker-compose.prod.yml
  - Dockerfile for each crate
  - GPU passthrough config

Acceptance Criteria:
  - [ ] All services containerized
  - [ ] GPU access from containers
  - [ ] Resource limits enforced
  - [ ] Health checks configured
```

### Task 6.4: Monitoring Dashboard
```
ID: OMNI-053
Type: DevOps
Effort: 6 hours
Parallel: Yes (after OMNI-022)

Description:
  Grafana dashboard for system monitoring.

Deliverables:
  - Grafana dashboard JSON
  - Prometheus config
  - Alert rules

Acceptance Criteria:
  - [ ] Real-time metrics visualization
  - [ ] GPU utilization graphs
  - [ ] Memory usage tracking
  - [ ] Latency histograms
```

---

## Task Dependency Graph

```
                    OMNI-001 (Scaffolding)
                           │
        ┌──────────────────┼──────────────────┐
        │                  │                  │
        ▼                  ▼                  ▼
    OMNI-002           OMNI-003           OMNI-004
    (gRPC Hub)      (Ring Buffer)      (Memory Pool)
        │                  │                  │
        │                  ▼                  │
        │              OMNI-020               │
        │           (Observer Ring)           │
        │                  │                  │
        │         ┌───────┴───────┐          │
        │         ▼               ▼          │
        │     OMNI-021        OMNI-022       │
        │    (Patterns)       (Metrics)      │
        │                                    │
        └──────────────┬─────────────────────┘
                       │
                       ▼
                   OMNI-010
              (Neurostack Dep)
                       │
                       ▼
                   OMNI-011
              (OmniscientCore)
                       │
        ┌──────────────┼──────────────┐
        │              │              │
        ▼              ▼              ▼
    OMNI-012       OMNI-030       OMNI-005
    (Health)      (Ingestor)     (GPU Sched)
                   Trait              │
                       │              │
        ┌──────────────┼──────────────┤
        │              │              │
        ▼              ▼              ▼
    OMNI-031       OMNI-032       OMNI-033
    (Papers)      (Crawler)       (MCP)
        │              │              │
        └──────────────┴──────────────┘
                       │
                       ▼
                   OMNI-034
              (Thalamus Integration)
                       │
                       ▼
                   OMNI-041
              (Integration Tests)
                       │
                       ▼
                   OMNI-042
                (Benchmarks)
```

---

## Parallel Execution Matrix

| Week | Process A | Process B | Process C | Process D |
|------|-----------|-----------|-----------|-----------|
| 1 | OMNI-001 | OMNI-050 | - | - |
| 1-2 | OMNI-002 | OMNI-003 | OMNI-004 | OMNI-030 |
| 2-3 | OMNI-005 | OMNI-010 | OMNI-020 | OMNI-052 |
| 3-4 | OMNI-011 | OMNI-021 | OMNI-022 | OMNI-031 |
| 4-5 | OMNI-012 | OMNI-023 | OMNI-032 | OMNI-033 |
| 5-6 | OMNI-034 | OMNI-040 | OMNI-051 | OMNI-053 |
| 6-7 | OMNI-041 | - | - | - |
| 7-8 | OMNI-042 | OMNI-043 | - | - |

---

## Quick Start Commands

```bash
# Clone and setup
cd /Users/zacelston/CascadeProjects
mkdir omniscient && cd omniscient
cargo init --name omniscient

# Add neurostack as path dependency
echo '[dependencies]
cortical-shared = { path = "../neurostack/crates/cortical-shared" }
' >> Cargo.toml

# Start development
cargo build
```

---

## Notes for Parallel Processes

1. **Communication**: Use this document as source of truth. Update task status here.
2. **Conflicts**: If two tasks modify same files, coordinate via PR reviews.
3. **Blockers**: Mark blocked tasks with `BLOCKED: <reason>` in status.
4. **Completion**: Update status to `DONE` with date when complete.

---

*Last Updated: 2024-12-17*
