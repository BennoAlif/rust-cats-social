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

# Install SQLx CLI using cargo
RUN cargo install sqlx-cli --version=0.7.3 --features postgres --root /usr/local

ENV RUSTFLAGS="-C target-feature=+crt-static"

COPY Cargo.toml Cargo.lock ./
COPY migrations ./migrations
COPY src ./src

# Build the application
RUN --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/app/target/ \
    cargo build --target=aarch64-unknown-linux-musl --locked --release && \
    cp ./target/aarch64-unknown-linux-musl/release/$APP_NAME /bin/server

FROM alpine:latest AS final
RUN apk add --no-cache libgcc postgresql-client

# Copy SQLx CLI from build stage
COPY --from=build /usr/local/bin/sqlx /usr/local/bin/sqlx

# Copy migration files and binary
COPY --from=build /app/migrations /migrations
COPY --from=build /bin/server /bin/server

EXPOSE 8080

# Create run script
RUN echo '#!/bin/sh' > /run.sh && \
    echo 'sqlx migrate run' >> /run.sh && \
    echo '/bin/server' >> /run.sh && \
    chmod +x /run.sh

USER 10001
CMD ["/run.sh"]