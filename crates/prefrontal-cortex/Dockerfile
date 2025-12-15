# Build stage
FROM rust:1.74-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY crates/cortical-shared ./crates/cortical-shared
COPY crates/prefrontal-cortex ./crates/prefrontal-cortex

RUN cargo build --release --package prefrontal-cortex

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/prefrontal-cortex /app/prefrontal-cortex

ENV RUST_LOG=info

EXPOSE 8000

CMD ["/app/prefrontal-cortex"]
