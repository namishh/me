FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Create a caching layer for dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && \
    echo "fn main() {println!(\"DEPENDENCY_CACHE_ONLY\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf target/release/*/

# Now copy the actual source and build (using cached dependencies)
COPY src ./src
COPY static ./static    

COPY package.json package-lock.json ./
COPY templates ./templates
COPY content ./content

RUN npm ci && \
    npx @tailwindcss/cli -i ./static/input.css -o ./static/style.css && \
    rm -rf node_modules

RUN rm -rf target/release && cargo build --release && \
    ls -la target/release/

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

EXPOSE 8080

ENV ENVIRONMENT=PRODUCTION

RUN useradd -m appuser
USER appuser

CMD ["/app/personal"]