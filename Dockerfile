FROM rust:bullseye AS builder

WORKDIR /app
COPY . .
RUN cargo build

FROM ubuntu:latest

WORKDIR /app
COPY --from=builder /app/target/debug/bot-cnvtr ./main
CMD ["./main"]
