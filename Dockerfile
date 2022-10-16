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

RUN cargo build --release --workspace && \
    cargo test --all --release -v

# FROM builder as publisher

# ENV CARGO_REGISTRY_TOKEN="" \
#     PACKAGE=""

# CMD [ "cargo", "publish", "-p",  ${PACKAGE}, "--token", ${CARGO_REGISTRY_TOKEN}]