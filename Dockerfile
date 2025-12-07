FROM rust:1-alpine AS alpine-builder

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /build
COPY . .

FROM rust:latest AS builder

# 运行服务镜像为scratch或alpine时候使用
# RUN rustup target add x86_64-unknown-linux-musl && apt update && apt install -y musl-tools musl-dev

RUN update-ca-certificates

WORKDIR /build
COPY . .

RUN cargo build \
    # 运行服务镜像为scratch或alpine时候使用
    # --target x86_64-unknown-linux-musl \
    --release \
    && strip -s target/release/coding-usage

FROM debian:trixie-slim AS app

COPY --from=builder /build/target/release/coding-usage .

CMD ["/build/coding-usage"]
