FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian

RUN apt-get update && \
  apt-get install libc++-14-dev libc++abi-14-dev ruby -y --fix-missing --no-install-recommends && \
  rm -rf /var/lib/apt/lists/*