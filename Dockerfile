FROM rust:slim-bullseye

WORKDIR /app
COPY . .

RUN cargo build
