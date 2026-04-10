# SENTIENT AI OS - Production Dockerfile
# Multi-stage build for minimal image size

# ============================================
# Stage 1: Build dependencies
# ============================================
FROM rust:1.75-bookworm AS chef

# Install cargo-chef for dependency caching
RUN cargo install cargo-chef

WORKDIR /app

# ============================================
# Stage 2: Analyze dependencies
# ============================================
FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

# ============================================
# Stage 3: Build dependencies (cached)
# ============================================
FROM chef AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

COPY --from=planner /app/recipe.json recipe.json

# Build dependencies (this layer will be cached)
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source code
COPY . .

# Build the application
RUN cargo build --release -p sentient_cli -p sentient_gateway

# ============================================
# Stage 4: Runtime image
# ============================================
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 sentient

WORKDIR /app

# Copy binaries
COPY --from=builder /app/target/release/sentient /usr/local/bin/
COPY --from=builder /app/target/release/asena-web /usr/local/bin/sentient-web

# Copy configuration
COPY config/ /app/config/

# Set ownership
RUN chown -R sentient:sentient /app

# Switch to non-root user
USER sentient

# Expose ports
EXPOSE 8080 8443

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Default command
CMD ["sentient", "gateway"]

# ============================================
# Labels
# ============================================
LABEL org.opencontainers.image.title="SENTIENT AI OS"
LABEL org.opencontainers.image.description="Production-ready AI Agent Operating System"
LABEL org.opencontainers.image.version="4.0.0"
LABEL org.opencontainers.image.vendor="SENTIENT"
LABEL org.opencontainers.image.source="https://github.com/nexsusagent-coder/SENTIENT_CORE"
