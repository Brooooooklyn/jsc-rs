FROM  ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-alpine

ENV RUSTFLAGS=""

RUN apk add --update --no-cache lld ruby perl openssl-dev
