# syntax=docker/dockerfile:1
FROM rust:1.77-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /usr/src/parkamera

COPY Cargo.toml Cargo.lock ./

COPY src src
COPY migration migration

RUN cargo build --release

FROM alpine:latest

RUN apk add --no-cache libgcc

COPY --from=builder /usr/src/parkamera/target/release/backend /usr/local/bin

CMD ["backend"]
