# Build stage
FROM rust:latest AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy everything from the outer folder
COPY . .

# Move into the actual Rust project folder
WORKDIR /app/chat-server

ENV SQLX_OFFLINE=true
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev git && rm -rf /var/lib/apt/lists/*

# Binary is built inside /app/chat-server/target/release/
COPY --from=builder /app/chat-server/target/release/chat-server .

EXPOSE 9000
CMD ["./chat-server"]