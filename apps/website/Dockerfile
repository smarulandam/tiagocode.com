ARG BASE_IMAGE_NAME
ARG BASE_IMAGE_VERSION

FROM ${BASE_IMAGE_NAME}:${BASE_IMAGE_VERSION} AS builder

WORKDIR /app

COPY . .

ENV LEPTOS_TAILWIND_VERSION="v4.1.18"
ENV LEPTOS_HASH_FILENAME=true

# Build the project
RUN --mount=type=cache,target=/root/.cache/cargo-leptos \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/git/checkouts \
    cargo leptos build --release -vv

FROM ${BASE_IMAGE_NAME}:${BASE_IMAGE_VERSION} AS runner

WORKDIR /app

# Copy project builded files
COPY --from=builder /app/target/release/website /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT=./site
ENV LEPTOS_TAILWIND_VERSION="v4.1.18"
ENV LEPTOS_ENV="prod"

EXPOSE 3000

# Start the server
CMD ["/app/website"]
