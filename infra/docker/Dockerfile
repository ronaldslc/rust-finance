# Build stage — pinned by digest for supply-chain security
# To update: docker pull rust:1.78-slim-bookworm && docker inspect --format='{{index .RepoDigests 0}}' rust:1.78-slim-bookworm
FROM rust:1.78-slim-bookworm@sha256:d3867a40c89665affebc0bbcebeab1e10ac9cce9e9e148e50032d9dd4b23025c AS builder

WORKDIR /app
COPY . .

# Build in release mode (no OpenSSL — using rustls)
RUN cargo build --release -p daemon -p tui -p cli

# Run stage — pinned by digest
# To update: docker pull debian:bookworm-slim && docker inspect --format='{{index .RepoDigests 0}}' debian:bookworm-slim
FROM debian:bookworm-slim@sha256:ad86386827b083b3d71571f8e544a9cca6d505325037f67572652ad20de77a02

WORKDIR /app
COPY --from=builder /app/target/release/daemon /usr/local/bin/daemon
COPY --from=builder /app/target/release/tui /usr/local/bin/tui
COPY --from=builder /app/target/release/cli /usr/local/bin/cli

# Install runtime dependencies (CA certs only — no OpenSSL needed with rustls)
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["/usr/local/bin/daemon"]
