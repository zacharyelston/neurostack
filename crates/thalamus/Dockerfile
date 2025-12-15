# Build stage
FROM rust:1.74-slim as builder

WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml Cargo.lock ./
COPY crates/cortical-shared ./crates/cortical-shared
COPY crates/thalamus ./crates/thalamus

# Build release binary
RUN cargo build --release --package thalamus

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/thalamus /app/thalamus

ENV RUST_LOG=info

EXPOSE 8000

CMD ["/app/thalamus"]
