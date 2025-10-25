# Build stage
FROM rust:1.83 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build for release
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates curl && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN useradd -m -u 1001 appuser && \
    chown -R appuser:appuser /app

# Copy the binary from builder
COPY --from=builder /app/target/release/rust_readme_chess .

# Create engine directory and copy Stockfish binary
RUN mkdir -p engine
COPY engine/stockfish engine/stockfish
RUN chmod +x engine/stockfish && \
    chown -R appuser:appuser /app

# Set all environment variables with defaults
# These match the defaults in src/config.rs
ENV ENGINE_PATH=engine/stockfish \
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

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["./rust_readme_chess"]
