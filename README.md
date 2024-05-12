# scsys

[![crates.io](https://img.shields.io/crates/v/scsys.svg)](https://crates.io/crates/scsys)
[![docs](https://docs.rs/scsys/badge.svg)](https://docs.rs/scsys)

[![clippy](https://github.com/Scattered-Systems/scsys/actions/workflows/clippy.yml/badge.svg)](https://github.com/Scattered-Systems/scsys/actions/workflows/clippy.yml)
[![rust](https://github.com/Scattered-Systems/scsys/actions/workflows/rust.yml/badge.svg)](https://github.com/Scattered-Systems/scsys/actions/workflows/rust.yml)

***

Welcome to scsys, this library provides a set of primitives and utilities used throughout the ecosystem.


# Getting Started

Use Rust's built-in package manager [crates](https://crates.io/crates/scsys) to install *scsys*.

## Building from the source

### _Clone the repository_

```bash
git clone https://github.com/scattered-systems/scsys
cd scsys
```

### _Build the workspace locally_

```bash
cargo build --all-features -v --workspace
```

#### _Testing_

Automatically format and analyze the codebase before building then testing.

```bash
cargo test --all-features -r -v --workspace
```

```bash
cargo test --all-features -r -v --workspace
```

# Usage

```rust
use scsys::prelude::*;

fn main() {
  println!("{:?}", Message::<String>::default());
}
```

# Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

# License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
