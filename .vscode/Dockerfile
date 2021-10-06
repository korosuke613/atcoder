# syntax = docker/dockerfile:1.3-labs

# AtCoder運営者向けの情報 - AtCoderコンテストにRustで参加するためのガイドブック を参照
# https://doc.rust-jp.rs/atcoder-rust-resources/atcoder-env/index.html

FROM ubuntu

SHELL ["/bin/bash", "-c"]

ENV RUST_TOOLCHAIN=stable
ENV RUST_HOME=/usr/local/lib/rust
ENV RUSTUP_HOME=${RUST_HOME}/rustup
ENV CARGO_HOME=${RUST_HOME}/cargo
ENV RUST_LIB=$RUST_HOME/lib

RUN <<EOT
    apt-get update 
    apt-get install -y \
        curl \
        build-essential \
        openssl
    apt-get clean
    rm -rf /var/lib/apt/lists/*
EOT

RUN mkdir -p $RUST_HOME && chmod 0755 $RUST_HOME
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain "${RUST_TOOLCHAIN}" --no-modify-path
RUN echo 'source ${CARGO_HOME}/env' >> ~/.bashrc
ENV PATH="/usr/local/lib/rust/cargo/bin:$PATH"

RUN apt-get update
RUN apt-get install -y libssl-dev
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get install -y pkg-config

RUN cargo install --git https://github.com/rust-lang-ja/atcoder-rustc-dep-option-generator.git

RUN apt-get install -y git
RUN git clone https://github.com/rust-lang-ja/atcoder-rust-base.git \
    --branch ja-all-enabled --single-branch \
    ${RUST_HOME}/lib
RUN echo "${RUST_TOOLCHAIN}" > ${RUST_HOME}/lib/rust-toolchain
RUN cd ${RUST_HOME}/lib && cargo build --release

# rustc --edition=2018 -C opt-level=3 \
#     $(rustc-dep-option-generator --manifest-path $RUST_LIB/Cargo.toml) \
#     main.rs
