# ── Build Stage ──────────────────────────────────────────────────────────────
FROM rust:1.81-slim AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy workspace manifests first for layer caching
COPY Cargo.toml Cargo.toml
COPY crates/api/Cargo.toml crates/api/Cargo.toml
COPY crates/application/Cargo.toml crates/application/Cargo.toml
COPY crates/domain/Cargo.toml crates/domain/Cargo.toml
COPY crates/infrastructure/Cargo.toml crates/infrastructure/Cargo.toml

# Create dummy source files so cargo can build dependencies
RUN mkdir -p crates/api/src crates/application/src crates/domain/src crates/infrastructure/src \
    && echo "fn main() {}" > crates/api/src/main.rs \
    && touch crates/application/src/lib.rs \
    && touch crates/domain/src/lib.rs \
    && touch crates/infrastructure/src/lib.rs

RUN cargo build --release

# Copy actual source
COPY . .

# Touch to force rebuild
RUN touch crates/api/src/main.rs && cargo build --release

# ── Runtime Stage ─────────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update && apt-get install -y ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/edukit-api .
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

CMD ["./edukit-api"]
