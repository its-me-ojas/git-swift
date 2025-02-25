# Build stage
FROM rust:1.78-slim as builder

WORKDIR /usr/src/app
COPY . .

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/server /usr/local/bin/server
# Copy the .env file
COPY .env /usr/src/app/.env

# Set environment variables
ENV RUST_LOG=info

# Expose the port your application uses
EXPOSE 5000

# Run the binary
CMD ["server"]
