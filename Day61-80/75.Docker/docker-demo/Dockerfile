# Builder stage
FROM rust:1.80-slim-bookworm as builder

WORKDIR /app

# Create a dummy project to cache dependencies
RUN USER=root cargo new --bin docker-demo
WORKDIR /app/docker-demo
COPY Cargo.toml Cargo.lock ./
# If Cargo.lock doesn't exist yet, it's fine, but good practice to copy if it does.
# We will skip Cargo.lock in copy if it is not there in source, but here we assume it might not be.
# Just copying Cargo.toml is enough to trigger fetch.

# Build dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy actual source code
COPY src ./src

# Build the actual application
# touch main.rs to ensure rebuild
RUN touch src/main.rs
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies (e.g. OpenSSL if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/docker-demo/target/release/docker-demo /usr/local/bin/docker-demo

ENV RUST_LOG=info
EXPOSE 3000

CMD ["docker-demo"]
