#!/usr/bin/env bash
cargo publish --all-features --color always --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN --verbose -p scsys-core
cargo publish --all-features --color always --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN --verbose -p scsys-crypto
cargo publish --all-features --color always --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN --verbose -p scsys-derive
cargo publish --all-features --color always --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN --verbose -p scsys-macros
cargo publish --all-features --color always --jobs 1 --token ${'$'}CARGO_REGISTRY_TOKEN --verbose -p scsys