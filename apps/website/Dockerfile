FROM rustlang/rust:nightly-bookworm-slim AS builder

ARG WASM_BINDGEN_CLI_VERSION=0.2.108
ARG CARGO_LEPTOS_VERSION=0.3.4

# Install Rust OS dependencies
RUN set -ex; \
    apt-get update; \
    apt-get install -y \
        build-essential \
        libssl-dev \
        pkg-config \
        openssl \
        ca-certificates \
        curl \
        bash \
        binaryen; \
    apt-get autoremove -y; \
    apt-get clean -y; \
    rm -rf /var/lib/apt/lists/*

# Install Rust target
RUN set -ex; \
    rustup target add wasm32-unknown-unknown;

RUN set -ex; \
    curl -L --proto '=https' --tlsv1.2 -sSf \
         https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh \
         | bash; \
    cargo binstall -y wasm-bindgen-cli@${WASM_BINDGEN_CLI_VERSION}; \
    cargo binstall -y cargo-leptos@${CARGO_LEPTOS_VERSION}

WORKDIR /app

COPY . .

ENV LEPTOS_TAILWIND_VERSION="v3.4.16"
ENV LEPTOS_HASH_FILENAME=true

# Build the project
RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-bookworm-slim AS runner

WORKDIR /app

# Install Rust OS dependencies
RUN set -ex; \
    apt-get update; \
    apt-get install -y \
        iputils-ping \
        nano \
        curl; \
    apt-get autoremove -y; \
    apt-get clean -y; \
    rm -rf /var/lib/apt/lists/*

# Copy project builded files
COPY --from=builder /app/target/release/website /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT=./site
ENV LEPTOS_TAILWIND_VERSION="v3.4.16"
ENV LEPTOS_ENV="prod"

EXPOSE 3000

# Start the server
CMD ["/app/website"]
