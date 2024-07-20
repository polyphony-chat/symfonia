# Build stage

FROM rust:1-bookworm AS build
RUN rustup target add x86_64-unknown-linux-gnu && \
update-ca-certificates
WORKDIR /usr/symfonia/
COPY . .
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid 10001 \
    "symfonia"
RUN cargo build --target x86_64-unknown-linux-gnu --release

# symfonia image

FROM debian:latest

# API
EXPOSE 3001
# CDN
EXPOSE 3002
# Gateway
EXPOSE 3003
EXPOSE 3003/udp

RUN apt update && apt install -y libssl-dev pkg-config

# Required to get "symfonia" user
COPY --from=build /etc/passwd /etc/passwd
# Required to get "symfonia" group
COPY --from=build /etc/group /etc/group 

COPY --from=build --chown=symfonia:symfonia /usr/symfonia/target/release/symfonia /app/symfonia/symfonia

USER symfonia:symfonia
WORKDIR /app/symfonia
ENTRYPOINT ["/app/symfonia/symfonia"]