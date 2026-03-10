# syntax=docker/dockerfile:1.7
ARG RUST_VERSION=nightly-bookworm-slim
ARG RUST_TOOLCHAIN=nightly-2026-03-07

FROM rustlang/rust:${RUST_VERSION}

ARG RUST_TOOLCHAIN=nightly-2026-03-07
ARG WASM_BINDGEN_CLI_VERSION=0.2.108
ARG CARGO_LEPTOS_VERSION=0.3.4
ARG CARGO_CHEF_VERSION=0.1.73

RUN set -ex; \
    apt-get update; \
    apt-get install -y \
        bash \
        build-essential \
        ca-certificates \
        curl \
        iputils-ping \
        libssl-dev \
        nano \
        openssl \
        pkg-config; \
    apt-get autoremove -y; \
    apt-get clean -y; \
    rm -rf /var/lib/apt/lists/*

RUN set -ex; \
    rustup toolchain install ${RUST_TOOLCHAIN} --profile minimal; \
    rustup default ${RUST_TOOLCHAIN}; \
    rustup target add wasm32-unknown-unknown --toolchain ${RUST_TOOLCHAIN}

RUN set -ex; \
    curl -L --proto '=https' --tlsv1.2 -sSf \
        https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
        | bash; \
    cargo binstall -y wasm-bindgen-cli@${WASM_BINDGEN_CLI_VERSION}; \
    cargo binstall -y cargo-leptos@${CARGO_LEPTOS_VERSION}; \
    cargo binstall -y cargo-chef@${CARGO_CHEF_VERSION}

WORKDIR /app
