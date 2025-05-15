# scsys

[![crates.io](https://img.shields.io/crates/v/scsys?style=for-the-badge&logo=rust)](https://crates.io/crates/scsys)
![docs.rs](https://img.shields.io/docsrs/scsys?style=for-the-badge&logo=rust)

[![license](https://img.shields.io/crates/l/scsys?style=for-the-badge)](https://crates.io/crates/scsys)

***

_**Warning: the library is still in the early stages of development so make sure to use with caution!**_

Welcome to `scsys`, a collection of useful utilities, types, and other primitives that are used in various projects developed by [Scattered Systems](https://github.com/scattered-systems). The library is designed to be a general-purpose utility library that can be used in any Rust project, aiming to provide a standardized set of tools that can be used to build robust and reliable software.

## Getting Started

### Prerequisites

#### _Rust Toolchain_

Make sure you have the latest version of the Rust toolchain installed on your system.

```bash
rustup update
```

If you don't have it installed, you can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Building from the source

#### _Clone the repository_

Start by cloning the repository locally to your machine.

```bash
git clone https://github.com/scattered-systems/scsys.git
cd scsys
```

#### _Building Locally_

```bash
cargo build --all-features -v --workspace
```

#### _Testing_

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

##### _Example: Using the `Getter` derive macro_

The `Getter` derive macro can be used to automatically generate functional accessors for named fields within a given structure. For example, given the following structure:

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, scsys::Getter)]
pub struct Node<T> {
    pub weight: T,
}
```

we can automatically generate a getter method for the `weight` field:

```rust
let node = Node { weight: 42 };
assert_eq!(node.weight(), 42);
```

## License

Licensed under the Apache License, Version 2.0, ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))

## Contribution

Contributions are welcome, however, ensure that you have read the [CONTRIBUTING.md](CONTRIBUTING.md) file before submitting a pull request.
