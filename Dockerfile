# ref: https://marcopolo.io/code/nix-and-small-containers/
FROM nixpkgs/nix-flakes:latest AS builder

WORKDIR /app

ENV NIX_CONFIG='filter-syscalls = false'

COPY flake.nix flake.lock ./
RUN nix build .#otherDeps

COPY rust-toolchain.toml Cargo.toml Cargo.lock ./

COPY domain/Cargo.toml            ./domain/
COPY usecases/Cargo.toml          ./usecases/
COPY infra/repository/Cargo.toml  ./infra/repository/
COPY infra/traq-client/Cargo.toml ./infra/traq-client/
COPY app/wh-handler/Cargo.toml    ./app/wh-handler/
COPY app/bot/Cargo.toml           ./app/bot/
COPY router/Cargo.toml            ./router/
COPY cnvtr/Cargo.toml             ./cnvtr/
RUN nix build .#cargoDeps

COPY . .
RUN nix build .

RUN mkdir /tmp/nix-store-closure
RUN cp -R $(nix-store -qR result/) /tmp/nix-store-closure

FROM debian:bookworm-slim

RUN apt-get -y update \
    && apt-get -y install build-essential libssl-dev ca-certificates \
    && update-ca-certificates --fresh
WORKDIR /app

COPY --from=builder /tmp/nix-store-closure /nix/store
COPY --from=builder /app/result /app

CMD [ "/app/bin/cnvtr" ]
