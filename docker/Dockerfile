FROM golang:1.22.5-bookworm AS teahook-builder

WORKDIR /app

# version is teahook's commit hash
RUN --mount=type=cache,target=/go/pkg/mod/ \
    go install github.com/H1rono/teahook-rs@cc4258d
RUN mv "$(which teahook-rs)" ./teahook-rs

FROM rust:bookworm AS builder

WORKDIR /app

COPY --from=teahook-builder /app/teahook-rs /usr/local/bin/teahook-rs

ENV GITEA_TRANSPILER_PATH=/usr/local/bin/teahook-rs \
    CARGO_TARGET_DIR=/artifact \
    RUSTUP_HOME=/var/cache/rustup \
    CARGO_HOME=/var/cache/cargo
RUN --mount=type=cache,target=/var/cache/rustup \
    --mount=type=cache,target=/var/cache/cargo \
    --mount=type=bind,source=.,target=. \
    cargo build --release --locked

FROM gcr.io/distroless/cc-debian12:nonroot AS server-distroless

WORKDIR /app
COPY --from=builder /artifact/release/bot-cnvtr /app/bin/bot-cnvtr

CMD [ "/app/bin/bot-cnvtr" ]

FROM debian:bookworm-slim AS server-debian-slim

WORKDIR /app
COPY --from=builder /artifact/release/bot-cnvtr /app/bin/bot-cnvtr

CMD [ "/app/bin/bot-cnvtr" ]
