FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y curl

RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - 

RUN apt-get install -y \
    build-essential \
    cmake \
    libssl-dev \
    openssl \
    pkg-config \
    nodejs

RUN npm install -g @tailwindcss/cli

WORKDIR /app

COPY . .

RUN npm install

RUN npx @tailwindcss/cli -i ./static/input.css -o ./static/style.css

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev libssl3 pkg-config \
    openssl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/personal /app/personal
COPY --from=builder /app/static /app/static
COPY --from=builder /app/content /app/content
COPY --from=builder /app/templates /app/templates

EXPOSE 8080

ENV ENVIRONMENT=PRODUCTION

CMD ["/app/personal"]