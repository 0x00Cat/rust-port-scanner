# Multi-stage build for optimized Docker image
FROM rust:1.75 as builder

WORKDIR /usr/src/port-scanner

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build release binary
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 scanner

# Copy binary from builder
COPY --from=builder /usr/src/port-scanner/target/release/port-scanner /usr/local/bin/port-scanner

# Set ownership
RUN chown scanner:scanner /usr/local/bin/port-scanner

# Switch to non-root user
USER scanner

# Set working directory
WORKDIR /home/scanner

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD /usr/local/bin/port-scanner --help || exit 1

# Entry point
ENTRYPOINT ["/usr/local/bin/port-scanner"]

# Default command (show help)
CMD ["--help"]

# Labels
LABEL org.opencontainers.image.title="Port Scanner"
LABEL org.opencontainers.image.description="A modular and extensible port scanner written in Rust"
LABEL org.opencontainers.image.version="2.0.0"
LABEL org.opencontainers.image.authors="Your Name <your.email@example.com>"
LABEL org.opencontainers.image.source="https://github.com/yourusername/port-scanner"
