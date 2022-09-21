#!/usr/bin/env bash
cargo publish --package scsys-derive --token $CARGO_REGISTRY_TOKEN
cargo publish --package scsys-macros --token $CARGO_REGISTRY_TOKEN
cargo publish --package scsys --token $CARGO_REGISTRY_TOKEN
