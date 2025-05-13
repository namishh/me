FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    pkg-config \
    libssl-dev \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install sccache
ENV RUSTC_WRAPPER /usr/local/cargo/bin/sccache
ENV SCCACHE_CACHE_SIZE 90G
ENV SCCACHE_DIR /mnt/dispatcher-cache

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static    
RUN --mount=type=cache,target=/sccache \
    cargo build --release && \
    sccache --show-stats

COPY package.json package-lock.json ./
COPY templates ./templates
COPY content ./content

RUN npm ci && \
    npx tailwindcss -i ./static/input.css -o ./static/style.css && \
    rm -rf node_modules

FROM debian:bookworm-slim AS runtime

ARG GIT_COMMIT
ENV GIT_COMMIT=${GIT_COMMIT}

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