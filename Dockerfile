FROM rust:slim-bullseye

WORKDIR /usr/src/todo-app-backend
COPY . .

RUN cargo build

CMD [ "cargo", "run" ]
