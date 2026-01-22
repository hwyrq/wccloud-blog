# Build stage
FROM rust:1.75-slim-bookworm as builder

# Set working directory
WORKDIR /app

# Copy manifest files first for better caching
COPY wccloud-gateway-rust/Cargo.toml ./
COPY wccloud-gateway-rust/Cargo.lock ./

# Create dummy src for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (cached unless Cargo.toml changes)
RUN cargo build --release

# Remove dummy and copy actual source
RUN rm -rf src
COPY wccloud-gateway-rust/src ./src

# Build the actual application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies if needed
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary
COPY --from=builder /app/target/release/wccloud-gateway-rust ./

# Copy configuration
COPY wccloud-gateway-rust/application-pro.yml ./application.yml

# Expose port
EXPOSE 8081

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8081/actuator/health || exit 1

# Run the application
CMD ["./wccloud-gateway-rust"]
