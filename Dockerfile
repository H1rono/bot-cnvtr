FROM rust:bullseye AS builder

WORKDIR /app
COPY . .
RUN touch .env.dev
RUN touch .env
RUN cargo build

FROM ubuntu:latest

RUN apt -y update && apt -y install build-essential libssl-dev
WORKDIR /app
COPY --from=builder /app/target/debug/bot-cnvtr ./main
COPY --from=builder /app/.env.dev ./.env.dev
COPY --from=builder /app/.env ./.env
COPY --from=builder /app/migrations/ ./migrations/
CMD ["./main"]
