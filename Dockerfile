FROM rust:bullseye AS builder

WORKDIR /app
COPY . .
RUN cargo build

FROM ubuntu:latest

RUN apt -y update && apt -y install build-essential libssl-dev
WORKDIR /app
COPY --from=builder /app/target/debug/bot-cnvtr ./main
CMD ["./main"]
