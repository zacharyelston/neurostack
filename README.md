# Cortical Compose

A containerized cognitive system modeled after seven major brain regions. Each region is implemented as an independent Rust service with vector and SQL persistence. All access is mediated by a Prefrontal Cortex orchestrator.

## Architecture

```
                    ┌─────────────────────┐
                    │  Prefrontal Cortex  │
                    │   (Orchestrator)    │
                    │     Port 8000       │
                    └──────────┬──────────┘
                               │
        ┌──────────┬───────────┼───────────┬──────────┐
        │          │           │           │          │
        ▼          ▼           ▼           ▼          ▼
   ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐
   │Thalamus │ │Hippo-   │ │Basal    │ │Cere-    │ │Amygdala │
   │  8001   │ │campus   │ │Ganglia  │ │bellum   │ │  8005   │
   │         │ │  8002   │ │  8003   │ │  8004   │ │         │
   └─────────┘ └─────────┘ └─────────┘ └─────────┘ └─────────┘
```

## Brain Regions

| Region | Role | Persistence |
|--------|------|-------------|
| **Prefrontal Cortex** | Decision orchestration, policy management | None |
| **Thalamus** | Input gateway, embedding generation | None |
| **Hippocampus** | Episodic memory, temporal context | Qdrant + PostgreSQL |
| **Basal Ganglia** | Reinforcement learning, action scoring | Qdrant + PostgreSQL |
| **Cerebellum** | Error correction, skill refinement | Qdrant + PostgreSQL |
| **Amygdala** | Emotional valence, risk weighting | Qdrant + PostgreSQL |

## Quick Start

### Prerequisites

- Rust 1.74+
- Docker & Docker Compose
- PostgreSQL 15 (via Docker)
- Qdrant (via Docker)

### Development

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

### Production

```bash
# Build and start all services
docker-compose up --build

# API is available at http://localhost:8000
```

## API Endpoints

### Prefrontal Cortex (Port 8000)

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/decide` | POST | Make a decision based on input |
| `/feedback` | POST | Record outcome feedback |

### Thalamus (Port 8001)

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/ingest` | POST | Normalize input and generate embedding |

### Memory Regions (Ports 8002-8005)

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check |
| `/store` | POST | Store an experience |
| `/query` | POST | Query for similar experiences |

## Decision Flow

1. **Thalamus** ingests raw input and generates embeddings
2. **Prefrontal Cortex** evaluates risk, similarity, and confidence
3. Relevant regions are queried for contextual memory
4. **Prefrontal Cortex** synthesizes a decision or requests more data
5. Output is generated
6. Outcome feedback is written back to all relevant regions

## Project Structure

```
neurostack/
├── Cargo.toml                    # Workspace definition
├── docker-compose.yml            # Service orchestration
├── docs/
│   └── PROJECT_RULES.md          # Development standards
├── crates/
│   ├── cortical-shared/          # Shared types and utilities
│   ├── thalamus/                 # Input gateway
│   ├── prefrontal-cortex/        # Orchestrator
│   ├── hippocampus/              # Episodic memory
│   ├── basal-ganglia/            # Reinforcement learning
│   ├── cerebellum/               # Error correction
│   └── amygdala/                 # Emotional valence
└── infrastructure/
    └── postgres/
        └── init.sql              # Database schema
```

## Technology Stack

- **Language**: Rust 2021 Edition
- **Framework**: Axum 0.7
- **Async Runtime**: Tokio
- **Vector Store**: Qdrant
- **Relational DB**: PostgreSQL 15
- **Serialization**: Serde

## Documentation

- [Project Rules](docs/PROJECT_RULES.md) - Development standards and guidelines
- [Design Specification](design.yaml) - Original system design

## License

MIT
