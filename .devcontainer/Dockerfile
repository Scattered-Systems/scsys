# Rust Version: 1.86.0
ARG RUST_VERSION=latest 
# setup the base image
FROM rust:${RUST_VERSION} AS builder-base
# update and upgrade the image packages
RUN apt-get -y update && \
    apt-get upgrade -y
# update any installed toolchains
RUN rustup update
# install cargo-binstall to future-proof the build
RUN cargo install cargo-binstall
# install any extra required packages
RUN apt-get install -y \
    build-essential \
    ca-certificates \
    cmake \
    clang \
    curl \
    file \
    gcc \
    g++ \
    libopenblas-dev \
    libssl-dev \
    libz-dev \
    pkg-config \
    python3 \
    python3-pip
# create a new group before adding a new user
RUN groupadd -g 10001 agroup && \
    useradd -m -u 10001 -g agroup auser
# switch to the new user
USER auser

