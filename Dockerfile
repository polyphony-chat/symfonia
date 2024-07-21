FROM rust:1-bookworm AS chef
RUN cargo install cargo-chef
RUN rustup target add x86_64-unknown-linux-gnu && \
    update-ca-certificates
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-gnu --recipe-path recipe.json
COPY . .
RUN cargo build --target x86_64-unknown-linux-gnu --release

FROM debian:latest AS runtime

# API
EXPOSE 3001
# CDN
EXPOSE 3002
# Gateway
EXPOSE 3003
EXPOSE 3003/udp

RUN apt update && apt install -y libssl-dev pkg-config

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    "symfonia"

COPY --from=builder --chown=symfonia:symfonia /app/target/x86_64-unknown-linux-gnu/release/symfonia /app/symfonia

USER symfonia:symfonia
WORKDIR /app/
ENTRYPOINT ["/app/symfonia"]