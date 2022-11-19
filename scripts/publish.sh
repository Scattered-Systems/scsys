#!/bin/bash

cargo publish --all-features --jobs 1 --verbose -p scsys-agents
cargo publish --all-features --jobs 1 --verbose -p scsys-core
cargo publish --all-features --jobs 1 --verbose -p scsys-crypto
cargo publish --all-features --jobs 1 --verbose -p scsys-derive
cargo publish --all-features --jobs 1 --verbose -p scsys-gen
cargo publish --all-features --jobs 1 --verbose -p scsys-macros