FROM rust:latest

RUN cargo new --bin perceptron
WORKDIR ./perceptron
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build --release

CMD ["./target/release/perceptron"]
