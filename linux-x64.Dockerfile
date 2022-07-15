FROM ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian

RUN apt-get update && \
  apt-get install libc++-14-dev ruby -y --fix-missing --no-install-recommends && \
  ln -sf /usr/lib/llvm-14/lib/libc++abi.so.1.0 /usr/lib/llvm-14/lib/libc++abi.so && \
  rm -rf /var/lib/apt/lists/*