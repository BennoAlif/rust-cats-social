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

ENV RUSTFLAGS="-C target-feature=+crt-static"

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN --mount=type=cache,target=/usr/local/cargo/registry/ \
    --mount=type=cache,target=/app/target/ \
    cargo build --target=aarch64-unknown-linux-musl --locked --release && \
    cp ./target/aarch64-unknown-linux-musl/release/$APP_NAME /bin/server

FROM scratch AS final
COPY --from=build /bin/server /bin/server

EXPOSE 8080

USER 10001
CMD ["/bin/server"]