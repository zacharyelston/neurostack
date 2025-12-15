# Cortical Compose Project Rules

**Version:** 0.1.0  
**Last Updated:** December 14, 2025  
**Status:** Active Development

---

## Executive Summary

Cortical Compose is a containerized cognitive system modeled after brain regions. This document establishes the development rules and standards for the project.

---

## Technology Stack

### Backend (Brain Region Services)

| Component | Technology | Rationale |
|-----------|------------|-----------|
| **Language** | Rust 2021 Edition | Performance, memory safety, concurrency |
| **Framework** | Axum 0.7 | Async-first, tower ecosystem |
| **Async Runtime** | Tokio | Industry standard for Rust async |
| **Vector Store** | Qdrant | Native Rust client, high performance |
| **Relational DB** | PostgreSQL 15 | ACID compliance, JSONB support |
| **DB Driver** | SQLx | Compile-time query verification |
| **Serialization** | Serde | Standard Rust serialization |
| **Embeddings** | Candle / ONNX Runtime | Rust-native ML inference |

### Infrastructure

| Component | Technology |
|-----------|------------|
| **Containers** | Docker + docker-compose |
| **Vector DB** | Qdrant (official image) |
| **Relational DB** | PostgreSQL 15 Alpine |
| **Networking** | Bridge network with service discovery |

### Optional Frontend (Admin Dashboard)

| Component | Technology |
|-----------|------------|
| **Framework** | Next.js 14 (App Router) |
| **Language** | TypeScript |
| **Styling** | Tailwind CSS |

---

## Code Standards

### File Size Limits

| Category | Max Lines | Rationale |
|----------|-----------|-----------|
| Service modules | 500 | Maintainability |
| API handlers | 300 | Single responsibility |
| Test files | 500 | Comprehensive coverage |
| Config files | 200 | Simplicity |

**If a file exceeds limits:**
1. Extract logical components into submodules
2. Create dedicated types/traits files
3. Split handlers by endpoint group

### Rust Code Style

```rust
// Use explicit error types, not anyhow in library code
pub type Result<T> = std::result::Result<T, CorticalError>;

// Prefer impl Trait over Box<dyn Trait> when possible
pub async fn query(&self, embedding: impl AsRef<[f32]>) -> Result<Vec<SimilarityResult>>

// Use Arc<T> for shared state, avoid Mutex when RwLock suffices
pub struct AppState {
    pub config: Arc<Config>,
    pub qdrant: Arc<QdrantClient>,
    pub postgres: Arc<PgPool>,
}

// Derive common traits consistently
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience { ... }
```

### Error Handling

```rust
// Define a unified error enum per service
#[derive(Debug, thiserror::Error)]
pub enum CorticalError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Vector store error: {0}")]
    VectorStore(#[from] qdrant_client::QdrantError),
    
    #[error("Embedding error: {0}")]
    Embedding(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

// Map to HTTP status codes
impl IntoResponse for CorticalError {
    fn into_response(self) -> Response { ... }
}
```

### Logging & Observability

```rust
// Use tracing for structured logging
use tracing::{info, warn, error, instrument};

#[instrument(skip(state), fields(experience_id = %request.experience_id))]
pub async fn store_experience(
    State(state): State<AppState>,
    Json(request): Json<StoreRequest>,
) -> Result<Json<StoreResponse>> {
    info!("Storing experience");
    // ...
}
```

### Testing Requirements

| Test Type | Location | Coverage Target |
|-----------|----------|-----------------|
| Unit tests | Same file as code | 80% |
| Integration tests | `tests/` directory | Critical paths |
| API tests | `tests/api/` | All endpoints |

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_store_experience() {
        // Arrange
        let state = create_test_state().await;
        
        // Act
        let result = store_experience(state, request).await;
        
        // Assert
        assert!(result.is_ok());
    }
}
```

---

## Architecture Rules

### Hub-and-Spoke Topology

```
                    ┌─────────────────────┐
                    │  Prefrontal Cortex  │
                    │   (Orchestrator)    │
                    └──────────┬──────────┘
                               │
        ┌──────────┬───────────┼───────────┬──────────┐
        │          │           │           │          │
        ▼          ▼           ▼           ▼          ▼
   ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐
   │Thalamus │ │Hippo-   │ │Basal    │ │Cere-    │ │Amygdala │
   │         │ │campus   │ │Ganglia  │ │bellum   │ │         │
   └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘
```

### Communication Rules

1. **All inter-region communication goes through Prefrontal Cortex**
   - Regions MUST NOT call each other directly
   - Prefrontal Cortex is the sole authorized caller

2. **Thalamus is the only embedding generator**
   - All text → embedding conversion happens in Thalamus
   - Other regions receive pre-computed embeddings

3. **Regions are independently deployable**
   - Each region is a separate Docker container
   - System must function under partial region failure

### API Contract

All region services MUST implement:

```rust
// Health check
GET /health -> HealthResponse

// For memory-enabled regions (Hippocampus, Basal Ganglia, Cerebellum, Amygdala)
POST /store -> StoreResponse
POST /query -> QueryResponse
```

### Shared Types

All services use shared types from `cortical-shared` crate:

```
cortical-shared/
├── src/
│   ├── lib.rs
│   ├── models.rs      # Experience, Valence, Outcome, etc.
│   ├── errors.rs      # CorticalError enum
│   ├── config.rs      # Configuration structs
│   └── clients.rs     # HTTP clients for inter-service calls
```

---

## Project Structure

```
neurostack/
├── Cargo.toml                    # Workspace definition
├── docker-compose.yml            # Service orchestration
├── docs/
│   ├── PROJECT_RULES.md          # This document
│   ├── Architecture.md           # System design
│   ├── API-Reference.md          # Endpoint documentation
│   └── Deployment.md             # Deployment guide
├── crates/
│   ├── cortical-shared/          # Shared types and utilities
│   │   ├── Cargo.toml
│   │   └── src/
│   ├── thalamus/                 # Input gateway service
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   ├── prefrontal-cortex/        # Orchestrator service
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   ├── hippocampus/              # Episodic memory service
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   ├── basal-ganglia/            # Reinforcement learning service
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   ├── cerebellum/               # Error correction service
│   │   ├── Cargo.toml
│   │   ├── Dockerfile
│   │   └── src/
│   └── amygdala/                 # Emotional valence service
│       ├── Cargo.toml
│       ├── Dockerfile
│       └── src/
├── infrastructure/
│   └── postgres/
│       └── init.sql              # Database schema
└── scripts/
    ├── dev.sh                    # Local development
    └── test.sh                   # Run all tests
```

---

## Service Specifications

### Port Assignments

| Service | Internal Port | External Port |
|---------|---------------|---------------|
| Prefrontal Cortex | 8000 | 8000 |
| Thalamus | 8000 | 8001 |
| Hippocampus | 8000 | 8002 |
| Basal Ganglia | 8000 | 8003 |
| Cerebellum | 8000 | 8004 |
| Amygdala | 8000 | 8005 |
| Qdrant | 6333 | 6333 |
| PostgreSQL | 5432 | 5432 |

### Qdrant Collections

| Collection | Service | Purpose |
|------------|---------|---------|
| `hippocampus_episodes` | Hippocampus | Episodic memory vectors |
| `basal_ganglia_actions` | Basal Ganglia | Action-outcome vectors |
| `cerebellum_errors` | Cerebellum | Error delta vectors |
| `amygdala_positive_input` | Amygdala | Positive input experiences |
| `amygdala_negative_input` | Amygdala | Negative input experiences |
| `amygdala_positive_output` | Amygdala | Positive output experiences |
| `amygdala_negative_output` | Amygdala | Negative output experiences |

### PostgreSQL Schemas

| Schema | Service | Tables |
|--------|---------|--------|
| `hippocampus` | Hippocampus | experiences, sessions |
| `basal_ganglia` | Basal Ganglia | actions, outcomes, reward_stats |
| `cerebellum` | Cerebellum | error_deltas, micro_policies |
| `amygdala` | Amygdala | valence_records |
| `audit` | All | decision_log |

---

## Development Workflow

### Local Development

```bash
# Start infrastructure
docker-compose up -d qdrant postgres

# Run a single service
cd crates/thalamus
cargo run

# Run all tests
cargo test --workspace

# Check formatting and lints
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
```

### CI/CD Requirements

| Check | Command | Required |
|-------|---------|----------|
| Format | `cargo fmt --all -- --check` | ✅ |
| Clippy | `cargo clippy --workspace -- -D warnings` | ✅ |
| Tests | `cargo test --workspace` | ✅ |
| Build | `cargo build --release` | ✅ |

### Git Workflow

1. **Branch naming**: `feature/`, `fix/`, `refactor/`
2. **Commit messages**: Conventional commits (`feat:`, `fix:`, `docs:`)
3. **PR requirements**: All CI checks pass, code review

---

## Security Rules

1. **Network isolation**: Regions only accessible via Prefrontal Cortex
2. **Input validation**: All inputs validated with explicit schemas
3. **No hardcoded secrets**: Use environment variables
4. **Experience ID propagation**: Trace all operations

---

## Performance Guidelines

1. **Batch operations**: Prefer batch inserts/queries over individual calls
2. **Connection pooling**: Use SQLx pool, Qdrant connection reuse
3. **Async everywhere**: No blocking operations in async contexts
4. **Vector dimensions**: Use 384-dim embeddings (all-MiniLM-L6-v2)

---

## Checklist for New Services

- [ ] Implements `/health` endpoint
- [ ] Uses shared types from `cortical-shared`
- [ ] Has Dockerfile with multi-stage build
- [ ] Added to `docker-compose.yml`
- [ ] Has unit tests (80% coverage target)
- [ ] Has integration tests for critical paths
- [ ] Documented in API-Reference.md
- [ ] Under 500 lines per file

---

## Non-Goals (Explicit Exclusions)

- Biological accuracy beyond functional analogy
- Real-time guarantees or hard latency constraints
- Autonomous self-modification of architecture
- Claims of sentience, consciousness, or feelings
- Direct inter-region communication (bypass Prefrontal Cortex)

---

*This document is the source of truth for project standards. Update as the project evolves.*
