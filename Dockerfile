# ============================================================================
#  RustForge Trading Terminal -- Multi-stage Production Dockerfile
#
#  Builds all binaries (daemon, tui, cli, benchmark_audit) in a cached
#  builder stage, then copies only the release artifacts into a minimal
#  Debian runtime image.
#
#  Supports multi-arch builds (linux/amd64, linux/arm64) via Docker Buildx.
#
#  Usage:
#    docker build -t rustforge:latest .
#    docker run --env-file .env rustforge:latest            # runs daemon
#    docker run --rm rustforge:latest tui                   # runs TUI
#    docker run --rm rustforge:latest benchmark_audit       # runs audit
#
#  Multi-arch:
#    docker buildx build --platform linux/amd64,linux/arm64 -t rustforge:latest .
# ============================================================================

# ── Build stage ────────────────────────────────────────────────────────────
FROM rust:1.78-slim-bookworm AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        pkg-config \
        libssl-dev \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build

# Layer 1: Cache dependency compilation
# Copy only manifests first so that source changes don't invalidate the
# expensive dependency-compilation layer.
COPY Cargo.toml Cargo.lock ./

# Copy workspace member Cargo.toml files (for dependency resolution)
# We use a find-and-copy pattern via shell to handle the dynamic crate list.
COPY crates/ crates/

# Build all workspace binaries in release mode
# fat LTO + strip is configured in [profile.release] in Cargo.toml
RUN cargo build --release \
    -p daemon \
    -p tui \
    -p cli \
    -p backtest --bin benchmark_audit

# ── Runtime stage ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.title="RustForge Trading Terminal"
LABEL org.opencontainers.image.description="Institutional-grade algorithmic trading platform"
LABEL org.opencontainers.image.source="https://github.com/Ashutosh0x/rust-finance"
LABEL org.opencontainers.image.licenses="MIT"
LABEL org.opencontainers.image.vendor="Ashutosh0x"

# Runtime dependencies only
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        tini \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user for security
RUN groupadd --gid 1000 rustforge && \
    useradd --uid 1000 --gid rustforge --shell /bin/sh --create-home rustforge

WORKDIR /app

# Copy release binaries from builder
COPY --from=builder /build/target/release/daemon           /usr/local/bin/daemon
COPY --from=builder /build/target/release/tui              /usr/local/bin/tui
COPY --from=builder /build/target/release/cli              /usr/local/bin/cli
COPY --from=builder /build/target/release/benchmark_audit  /usr/local/bin/benchmark_audit

# Copy configuration templates and default config
COPY .env.example /app/.env.example
COPY config/     /app/config/

# Ensure binaries are executable
RUN chmod +x /usr/local/bin/daemon \
             /usr/local/bin/tui \
             /usr/local/bin/cli \
             /usr/local/bin/benchmark_audit

# Switch to non-root user
USER rustforge

# Expose daemon ports: gRPC (50051), metrics (9090)
EXPOSE 50051 9090

# Health check -- daemon exposes metrics on :9090
HEALTHCHECK --interval=30s --timeout=5s --retries=3 \
    CMD curl -fsS http://localhost:9090/health || exit 1

# Use tini as init to properly handle signals
ENTRYPOINT ["tini", "--"]

# Default to daemon; override with: docker run rustforge tui
CMD ["daemon"]
