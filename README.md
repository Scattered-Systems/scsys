# scsys

[![crates.io](https://img.shields.io/crates/v/scsys.svg)](https://crates.io/crates/scsys)
[![docs](https://docs.rs/scsys/badge.svg)](https://docs.rs/scsys)
[![license](https://img.shields.io/crates/l/scsys.svg)](https://crates.io/crates/scsys)

[![clippy](https://github.com/Scattered-Systems/scsys/actions/workflows/clippy.yml/badge.svg)](https://github.com/Scattered-Systems/scsys/actions/workflows/clippy.yml)
[![rust](https://github.com/Scattered-Systems/scsys/actions/workflows/rust.yml/badge.svg)](https://github.com/Scattered-Systems/scsys/actions/workflows/rust.yml)

***

_**Warning: the library is currently in development so be prepared for major modifications to the API!**_

Welcome to `scsys`, a collection of useful utilities, types, and other primitives that are used in various projects developed by [Scattered Systems](https://github.com/scattered-systems). The library is designed to be a general-purpose utility library that can be used in any Rust project, aiming to provide a standardized set of tools that can be used to build robust and reliable software.

## Getting Started

## Building from the source

Make sure you have the latest version of the Rust toolchain installed on your system.

```bash
rustup update
```

### _Clone the repository_

```bash
git clone https://github.com/scattered-systems/scsys.git
```

then, navigate to the project directory

```bash
cd scsys
```

### _Building Locally_

```bash
cargo build --all-features -v --workspace
```

### _Testing_

Automatically format and analyze the codebase before building then testing.

```bash
cargo test --all-features -r -v --workspace
```

### Usage

#### _Add the dependency to your project_

```toml
[dependencies.scsys]
features = ["full"]
version = "0.2.*"
```

#### Examples

##### _Example: Using the `Message` type_

```rust
use scsys::prelude::*;

fn main() {
  println!("{:?}", Message::<String>::default());
}
```

## License

Licensed under the Apache License, Version 2.0, ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))

## Contribution

Contributions are welcome, however, ensure that you have read the [CONTRIBUTING.md](CONTRIBUTING.md) file before submitting a pull request.
