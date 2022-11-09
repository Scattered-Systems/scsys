#!/usr/bin/env bash
cargo publish --all-features --color always --jobs 1 --verbose -p scsys-core
cargo publish --all-features --color always --jobs 1 --verbose -p scsys-crypto
cargo publish --all-features --color always --jobs 1 --verbose -p scsys-derive
cargo publish --all-features --color always --jobs 1 --verbose -p scsys-macros
cargo publish --all-features --color always --jobs 1 --verbose -p scsys