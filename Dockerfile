# Build stage
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /usr/src/app

# Copy dependency specifications
COPY Cargo.toml ./

# Copy source code
COPY . .

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy built binary from builder
COPY --from=builder /usr/src/app/target/release/personal /app/personal

# Copy static assets
COPY --from=builder /usr/src/app/templates /app/templates
COPY --from=builder /usr/src/app/static /app/static

# Create content directories
RUN mkdir -p /app/content/blogs /app/content/notes /app/content/poems

# Runtime configuration
ENTRYPOINT ["/app/personal"]
EXPOSE 8080