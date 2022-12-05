# scsys

[![Clippy](https://github.com/Scattered-Systems/scsys/actions/workflows/clippy.yml/badge.svg)](https://github.com/Scattered-Systems/scsys/actions/workflows/clippy.yml)
[![Rust](https://github.com/Scattered-Systems/scsys/actions/workflows/rust.yml/badge.svg)](https://github.com/Scattered-Systems/scsys/actions/workflows/rust.yml)

Welcome to scsys, this crate was created in support of the Scattered-Systems ecosystem. The crate is reserved primarily for implementing a variety of critical primitives and utilities.

## Installation

Use Rust's built-in package manager [crates](https://crates.io/crates/package) to install *package*.

```bash
cargo install package
```

## Usage

- [crates.io](https://crates.io/crates/scsys)
- [docs.rs](https://docs.rs/scsys)

```rust
use scsys::prelude::*;

fn main() {
  println!("{:?}", Message::<String>::default());
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

[MIT](https://choosealicense.com/licenses/mit/)