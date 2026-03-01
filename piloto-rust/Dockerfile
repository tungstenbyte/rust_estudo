# Build stage
FROM rust:1-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/piloto_rust /app/piloto_rust

ENV PORT=3000
EXPOSE 3000 2112

ENTRYPOINT ["/app/piloto_rust"]
