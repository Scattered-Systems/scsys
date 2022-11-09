#!/usr/bin/env bash

sudo apt -y update 
sudo apt -y upgrade
sudo apt install -y protobuf-compiler
rustup install nightly
rustup component add clippy rustfmt --toolchain nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
