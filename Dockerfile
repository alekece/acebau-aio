# syntax=docker/dockerfile:1

FROM rust:1.95-bookworm AS server-builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

RUN cargo build --locked --release --package acebau-server

FROM debian:bookworm-slim AS server

RUN apt-get update \
    && apt-get install --yes --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --create-home --uid 10001 acebau

COPY --from=server-builder /app/target/release/acebau-server /usr/local/bin/acebau-server

USER acebau

EXPOSE 8080

ENTRYPOINT ["acebau-server"]

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
