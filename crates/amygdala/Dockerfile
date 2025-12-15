# Build stage
FROM rust:1.74-slim as builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY crates/cortical-shared ./crates/cortical-shared
COPY crates/amygdala ./crates/amygdala

RUN cargo build --release --package amygdala

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/amygdala /app/amygdala

ENV RUST_LOG=info

EXPOSE 8000

CMD ["/app/amygdala"]
