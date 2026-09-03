# Chef stage - for dependency caching
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Planner stage - generate recipe for dependencies
FROM chef AS planner
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo chef prepare --recipe-path recipe.json

# Builder stage - build with cached dependencies
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release --bin rust_readme_chess

# Runtime stage - using Ubuntu 24.04 which has GLIBC 2.39
FROM ubuntu:24.04 AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1001 appuser

# Copy the binary from builder
COPY --from=builder /app/target/release/rust_readme_chess /usr/local/bin/rust_readme_chess

# Create engine directory and copy Stockfish binary
RUN mkdir -p /app/engine
COPY engine/stockfish /app/engine/stockfish
RUN chmod +x /app/engine/stockfish && \
    chown -R appuser:appuser /app

# Set all environment variables with defaults
# These match the defaults in src/config.rs
ENV ENGINE_PATH=/app/engine/stockfish \
    SERVER_ADDR=0.0.0.0:8080 \
    GITHUB_BRANCH=main \
    GITHUB_README_PATH=README.md \
    GITHUB_OWNER_REPO=grim-kalman \
    BASE_URL=https://rust-readme-chess.fly.dev \
    RUST_LOG=info

# Note: GITHUB_TOKEN must be provided at runtime via secrets
# Never hardcode secrets in Dockerfile!

# Switch to non-root user
USER appuser

# Expose the port
EXPOSE 8080

# Run the application using ENTRYPOINT for better signal handling
ENTRYPOINT ["/usr/local/bin/rust_readme_chess"]
