# syntax=docker/dockerfile:1

FROM rust:1.95-bookworm AS api-builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --locked --release --package acebau-api

FROM debian:bookworm-slim AS api

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 acebau

COPY --from=api-builder /app/target/release/acebau-api /usr/local/bin/acebau-api

USER acebau

EXPOSE 8080

ENTRYPOINT ["acebau-api"]

FROM node:24-bookworm-slim AS web-builder

RUN corepack enable && corepack prepare pnpm@10.14.0 --activate

WORKDIR /app/web

COPY web/package.json web/pnpm-lock.yaml ./
RUN pnpm install --frozen-lockfile

COPY web ./
RUN pnpm build

FROM node:24-bookworm-slim AS web

ENV NODE_ENV=production

WORKDIR /app

COPY --from=web-builder --chown=node:node /app/web/build ./build

USER node

EXPOSE 3000

CMD ["node", "build"]
