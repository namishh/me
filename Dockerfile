FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl3 \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

ENV CARGO_HOME=/usr/local/cargo

# Create a caching layer for dependencies
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    mkdir -p src && \
    echo "fn main() {println!(\"DEPENDENCY_CACHE_ONLY\")}" > src/main.rs && \
    cargo build --release

# Now copy the actual source and build (using cached dependencies)
COPY src ./src
COPY static ./static    
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    rm -rf target/release && \
    cargo build --release && \
    ls -la target/release/

COPY package.json package-lock.json ./
COPY templates ./templates
COPY content ./content

RUN --mount=type=cache,target=/root/.npm \
    npm ci --no-audit --no-fund && \
    npx tailwindcss -i ./static/input.css -o ./static/style.css && \
    rm -rf node_modules

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/personal /app/
COPY --from=builder /app/static /app/static
COPY --from=builder /app/content /app/content
COPY --from=builder /app/templates /app/templates

RUN useradd -m appuser
USER appuser
ENV ENVIRONMENT=PRODUCTION

ARG GIT_COMMIT
ENV GIT_COMMIT=${GIT_COMMIT}

EXPOSE 8080
CMD ["/app/personal"]