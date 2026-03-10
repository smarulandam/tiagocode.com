ARG BASE_IMAGE_NAME
ARG BASE_IMAGE_VERSION

FROM ${BASE_IMAGE_NAME}:${BASE_IMAGE_VERSION} AS planner

WORKDIR /app

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM ${BASE_IMAGE_NAME}:${BASE_IMAGE_VERSION} AS builder

WORKDIR /app

COPY --from=planner /app/recipe.json ./recipe.json

ENV LEPTOS_TAILWIND_VERSION="v4.1.18"

RUN --mount=type=cache,target=/root/.cache/cargo-leptos \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/git/checkouts \
    cargo chef cook --recipe-path recipe.json --release --bin website --no-default-features --features ssr --target-dir target

RUN --mount=type=cache,target=/root/.cache/cargo-leptos \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/git/checkouts \
    cargo chef cook --recipe-path recipe.json --profile wasm-release --package website --target wasm32-unknown-unknown --no-default-features --features hydrate --target-dir target/front

COPY . .

RUN --mount=type=cache,target=/root/.cache/cargo-leptos \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/git/checkouts \
    cargo leptos build --release -vv

FROM ${BASE_IMAGE_NAME}:${BASE_IMAGE_VERSION} AS runner

WORKDIR /app

# Copy project builded files
COPY --from=builder /app/target/release/website /app/
COPY --from=builder /app/target/release/hash.txt /app/hash.txt
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT=./site
ENV LEPTOS_TAILWIND_VERSION="v4.1.18"
ENV LEPTOS_ENV="prod"
ENV LEPTOS_HASH_FILES=true
ENV LEPTOS_HASH_FILE_NAME=hash.txt

EXPOSE 3000

# Start the server
CMD ["/app/website"]
