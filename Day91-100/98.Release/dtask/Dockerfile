# Builder stage
FROM rust:1.80-bookworm as builder

WORKDIR /usr/src/app

# Copy manifests
COPY Cargo.toml Cargo.lock build.rs ./

# Copy source code and other necessary files
COPY src ./src
COPY proto ./proto
COPY migrations ./migrations

# Build for release
# protoc-bin-vendored will handle protoc installation
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    openssl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/dtask /app/dtask

# Expose ports
EXPOSE 3000 50051

# Environment variables
ENV RUST_LOG=info

# Run the binary
CMD ["./dtask"]
