# Build stage
FROM rust:1.75-alpine AS builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache musl-dev

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Copy the binary from builder
COPY --from=builder /app/target/release/mini-redis-cicd /usr/local/bin/mini-redis-cicd

# Expose port (if needed)
EXPOSE 6379

# Run the binary
CMD ["mini-redis-cicd"]
