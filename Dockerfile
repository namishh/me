FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && \
    apt-get install -y curl pkg-config build-essential && \
    curl -fsSL https://deb.nodesource.com/setup_20.x | bash - && \
    apt-get install -y nodejs

WORKDIR /usr/src/app

COPY package*.json ./

RUN npm install

COPY Cargo.toml ./
COPY . .

RUN npx tailwindcss -i ./static/input.css -o ./static/style.css --minify

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/personal /app/personal
COPY --from=builder /usr/src/app/templates /app/templates
COPY --from=builder /usr/src/app/content /app/content
COPY --from=builder /usr/src/app/static /app/static

RUN mkdir -p /app/content/blogs /app/content/notes /app/content/poems

ENTRYPOINT ["/app/personal"]
EXPOSE 8080