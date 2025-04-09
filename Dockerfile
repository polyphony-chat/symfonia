FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
RUN update-ca-certificates
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Separate builder stages for each component
FROM chef AS builder_api
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release -p symfonia-api --recipe-path recipe.json
COPY . .
RUN SQLX_OFFLINE=true cargo auditable build --release -p symfonia-api

FROM chef AS builder_gateway
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release -p symfonia-gateway --recipe-path recipe.json
COPY . .
RUN SQLX_OFFLINE=true cargo auditable build --release -p symfonia-gateway

# Create a common runtime base image
FROM debian:latest AS runtime_base
RUN apt-get update -qq -o Acquire::Languages=none && \
    env DEBIAN_FRONTEND=noninteractive apt-get install \
    -yqq \
        ca-certificates \
        libssl-dev \
        pkg-config \
        tzdata \
        curl && \
        rm -rf /var/lib/apt/lists/* && \
        ln -fs /usr/share/zoneinfo/Etc/UTC /etc/localtime

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    "symfonia"

# Client component image
FROM runtime_base AS runtime_client-component
COPY --from=builder_client-component --chown=symfonia:symfonia /app/target/release/symfonia-api /app/symfonia-api
USER symfonia:symfonia
WORKDIR /app/
ENTRYPOINT ["/app/symfonia-api"]

# Database component image
FROM runtime_base AS runtime_database-component
COPY --from=builder_database-component --chown=symfonia:symfonia /app/target/release/symfonia-gateway /app/symfonia-gateway
USER symfonia:symfonia
WORKDIR /app/
ENTRYPOINT ["/app/symfonia-gateway"]
