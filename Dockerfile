# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.82.0
ARG APP_NAME=cats-social-rust

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    gcc \
    pkgconfig

RUN cargo install sqlx-cli --version=0.7.3 --features postgres --root /usr/local

ENV RUSTFLAGS="-C target-feature=+crt-static"

COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/app/target/ \
    cargo build --target=x86_64-unknown-linux-musl --locked --release && \
    cp ./target/x86_64-unknown-linux-musl/release/$APP_NAME /bin/server

FROM alpine:latest AS migration

RUN apk add --no-cache libgcc postgresql-client

COPY --from=build /usr/local/bin/sqlx /usr/local/bin/sqlx
COPY --from=build /app/migrations /migrations

FROM alpine:latest AS final

RUN apk add --no-cache libgcc postgresql-client
COPY --from=build /bin/server /bin/server

EXPOSE 8080


CMD ["/bin/server"]