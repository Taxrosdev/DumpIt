FROM rust:1.94-slim-bookworm AS builder
 
WORKDIR /app
 
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*
 
# Cache Dependencies
COPY Cargo.toml Cargo.lock ./
 
RUN mkdir src && touch src/lib.rs
RUN cargo build --release
RUN rm -rf src
 
# Build
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
 
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*
 
COPY --from=builder /app/target/release/backend /backend
 
EXPOSE 3000
 
ENTRYPOINT ["/backend"]
