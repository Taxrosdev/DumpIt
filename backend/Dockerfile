FROM rust:1.95-slim-bookworm AS builder
 
WORKDIR /app
 
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libsqlite3-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*
 
# Cache Dependencies
COPY Cargo.toml Cargo.lock ./
 
RUN mkdir src && touch src/lib.rs
RUN cargo build --release
RUN rm -rf src
 
# Build
COPY migrations ./migrations
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
 
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libsqlite3-0 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*
 
COPY --from=builder /app/target/release/backend /backend
 
EXPOSE 3000
 
ENTRYPOINT ["/backend"]
