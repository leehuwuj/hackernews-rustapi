FROM rust:latest AS builder

WORKDIR /tmp/rbuild

# Rebuild package from Cargo.toml
COPY Cargo.* .
RUN mkdir -p src && touch src/main.rs
RUN cargo update
RUN rm -rf src
# Copy source code and build them
COPY src src
RUN cargo build --release

FROM debian:buster-slim
WORKDIR /opt/crawler
RUN apt update && apt install -y openssl sqlite3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /tmp/rbuild/target/release/crawler /usr/local/bin/crawler