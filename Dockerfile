FROM rust:latest as builder-base
RUN apt-get update -y && apt-get upgrade -y

RUN apt-get install -y \
    protobuf-compiler

RUN rustup update && \
    rustup install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly && \
    rustup component add clippy rustfmt

FROM builder-base as builder

ADD . /project
WORKDIR /project

COPY . .

RUN cargo fmt --all && \
    cargo build --release --workspace && \
    cargo clippy && \
    cargo test --all --release -v
