#!/usr/bin/env zsh
cargo publish --color always --jobs 1 --package scsys-core --token $CARGO_REGISTRY_TOKEN
cargo publish --color always --jobs 1 --package scsys-crypto --token $CARGO_REGISTRY_TOKEN
cargo publish --color always --jobs 1 --package scsys-derive --token $CARGO_REGISTRY_TOKEN
cargo publish --color always --jobs 1 --package scsys-macros --token $CARGO_REGISTRY_TOKEN
cargo publish --color always --jobs 1 --package scsys --token $CARGO_REGISTRY_TOKEN
