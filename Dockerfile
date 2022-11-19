FROM rust:latest as builder-base

RUN apt-get update -y && apt-get upgrade -y && rustup update

RUN apt-get install -y curl libssl-dev protobuf-compiler

FROM builder-base as base

RUN apt-get install -y \
    clang \
    llvm

FROM base as runner-base

ADD . /space
WORKDIR /space

COPY . .

FROM runner-base as wasm-base

RUN rustup install nightly && \
    rustup target add wasm32-wasi wasm32-unknown-unknown --toolchain nightly && \
    rustup default nightly

FROM wasm-base as wasm-setup

RUN curl -sSf https://raw.githubusercontent.com/WasmEdge/WasmEdge/master/utils/install.sh | bash

FROM runner-base as std

RUN cargo test --all --all-features --color always -v

FROM wasm-base as wasm

RUN cargo build --color always --verbose --workspace --target wasm32-unknown-unknown
