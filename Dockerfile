FROM golang:1.21.4-bookworm AS teahook-builder

WORKDIR /app

# version is teahook's commit hash
RUN go install github.com/H1rono/teahook-rs@cc4258d
RUN mv "$(which teahook-rs)" ./teahook-rs

FROM rust:bookworm AS builder

WORKDIR /app

COPY . .
COPY --from=teahook-builder /app/teahook-rs /usr/local/bin/teahook-rs
RUN env GITEA_TRANSPILER_PATH=/usr/local/bin/teahook-rs cargo build --release

FROM debian:bookworm-slim

RUN apt-get -y update \
    && apt-get install -y --no-install-recommends \
    coreutils libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && update-ca-certificates --fresh
WORKDIR /app

COPY --from=builder /app/target/release/bot-cnvtr /app/bin/bot-cnvtr

CMD [ "/app/bin/bot-cnvtr" ]
