FROM --platform=$BUILDPLATFORM golang:1.23.3-bookworm AS teahook-builder

WORKDIR /app

# version is teahook's commit hash
RUN --mount=type=cache,target=/go/pkg/mod/ \
    go install github.com/H1rono/teahook-rs@cc4258d
RUN mv "$(which teahook-rs)" ./teahook-rs

FROM --platform=$BUILDPLATFORM python:3.13-slim-bookworm AS target-triple

ARG TARGETOS
ARG TARGETARCH
ARG TARGETVARIANT

WORKDIR /app

RUN --mount=type=bind,source=docker,target=src \
    python3 src/target-triple.py rust-platform \
    --os "$TARGETOS" --arch "$TARGETARCH" --variant "$TARGETVARIANT" \
    > rust-platform
RUN --mount=type=bind,source=docker,target=src \
    python3 src/target-triple.py gcc-prefix \
    --os "$TARGETOS" --arch "$TARGETARCH" --variant "$TARGETVARIANT" \
    > gcc-prefix
RUN --mount=type=bind,source=docker,target=src \
    python3 src/target-triple.py crossbuild-essential \
    --os "$TARGETOS" --arch "$TARGETARCH" --variant "$TARGETVARIANT" \
    > crossbuild-essential

CMD [ "cat", "/app/rust-platform", "/app/crossbuild-essential" ]

FROM --platform=$BUILDPLATFORM rust:bookworm AS builder

WORKDIR /app

COPY --from=teahook-builder /app/teahook-rs /usr/local/bin/teahook-rs
COPY --from=target-triple /app/rust-platform /target
COPY --from=target-triple /app/gcc-prefix /gcc-prefix
COPY --from=target-triple /app/crossbuild-essential /crossbuild-essential

RUN --mount=type=cache,target=/var/lib/apt/lists,sharing=locked \
    apt-get -y update \
    && apt-get install -y --no-install-recommends \
    "$(cat /crossbuild-essential)"

ENV GITEA_TRANSPILER_PATH=/usr/local/bin/teahook-rs \
    CARGO_TARGET_DIR=/artifact \
    RUSTUP_HOME=/var/cache/rustup \
    CARGO_HOME=/var/cache/cargo
RUN --mount=type=cache,target=/var/cache/rustup,sharing=locked \
    --mount=type=bind,source=.,target=. \
    rustup target add "$(cat /target)"
RUN --mount=type=cache,target=/var/cache/rustup,sharing=locked \
    --mount=type=cache,target=/var/cache/cargo,sharing=locked \
    --mount=type=bind,source=.,target=. \
    cargo build --target "$(cat /target)" --release --locked \
    --config "env.CC_$(cat /target) = '$(cat /gcc-prefix)-gcc'" \
    --config "env.CXX_$(cat /target) = '$(cat /gcc-prefix)-g++'" \
    --config "target.$(cat /target).linker = '$(cat /gcc-prefix)-gcc'"

RUN cp "/artifact/$(cat /target)/release/bot-cnvtr" /bot-cnvtr

FROM --platform=$TARGETPLATFORM gcr.io/distroless/cc-debian12:nonroot AS server-distroless

WORKDIR /app
COPY --from=builder /bot-cnvtr /app/bin/bot-cnvtr

CMD [ "/app/bin/bot-cnvtr" ]

FROM --platform=$TARGETPLATFORM debian:bookworm-slim AS server-debian-slim

WORKDIR /app
COPY --from=builder /bot-cnvtr /app/bin/bot-cnvtr

CMD [ "/app/bin/bot-cnvtr" ]
