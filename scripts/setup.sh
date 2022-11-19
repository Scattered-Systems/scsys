#!/bin/bash
rustup default nightly
rustup target add wasm32-wasi wasm32-unknown-unknown --toolchain nightly
rustup component add clippy rustfmt --toolchain nightly
sudo apt update -y && sudo apt upgrade -y
sudo apt install -y clang llvm protobuf-compiler