FROM rust:latest

WORKDIR /ll-server

RUN cargo install diesel_cli

COPY ./ ./

RUN cargo build --release

CMD diesel migration run && cargo run --release
