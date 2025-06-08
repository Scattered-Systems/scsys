# scsys

[![crates.io](https://img.shields.io/crates/v/scsys?style=for-the-badge&logo=rust)](https://crates.io/crates/scsys)
[![docs.rs](https://img.shields.io/docsrs/scsys?style=for-the-badge&logo=docs.rs)](https://docs.rs/scsys)
[![GitHub License](https://img.shields.io/github/license/scattered-systems/scsys?style=for-the-badge&logo=github)](https://github.com/scattered-systems/scsys/blob/main/LICENSE)

***

_**Warning: the library is still in the early stages of development so make sure to use with caution!**_

Welcome to `scsys`, a collection of useful utilities, types, and other primitives that are used in various projects developed by [Scattered Systems](https://github.com/scattered-systems). The library is designed to be a general-purpose utility library that can be used in any Rust project, aiming to provide a standardized set of tools that can be used to build robust and reliable software.

## Usage

Before you start using `scsys`, make sure to add it as a dependency in your `Cargo.toml` file. You can do this by adding the following lines:

```toml
[dependencies.scsys]
default-features = true
features = []
version = "0.2.x"
```

### Examples

#### Example 1: _Using the `Getter` derive macro_

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

#### Example 2: _Using the `VariantConstructors` derive macro_

The `VariantConstructors` derive macro can be used to automatically generate constructors for the variants of an enum. For example, given the following enum:

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, scsys::VariantConstructors)]
pub enum Sample {
    A,
    B(usize),
    C { x: usize, y: usize },
}
```

we can automatically generate a functional constructor for each variant:

```rust
let a = Sample::a();
let b = Sample::b(42);
let c = Sample::c(1, 2);
assert_eq!(a, Sample::A);
assert_eq!(b, Sample::B(42));
assert_eq!(c, Sample::C { x: 1, y: 2 });
```

## Getting Started

### Prerequisites

#### Rust

If you don't have rustup installed, you can install it by following the instructions on the [official website](https://www.rust-lang.org/tools/install).

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

##### _rustup_

Once installed, you can use rustup to manage your Rust toolchain. This includes installing the latest stable version of Rust, as well as any other versions you may need for your projects. For now, we simply recommend using the latest stable version of Rust and making sure that any other toolchains you may have installed are up to date.

```bash
rustup update
```

### Building from the source

Start by cloning the repository locally to your machine:

```bash
git clone https://github.com/scattered-systems/scsys.git --branch main
```

Then, navigate to the cloned directory:

```bash
cd scsys
```

#### cargo: use the built-in tool to manage the project

Build the project using the `cargo build` command:

```bash
cargo build --all-features --locked --release --workspace
```

Test the project using the `cargo test` command:

```bash
cargo test --all-features --locked --release --workspace
```

Or, benchmark the project using the `cargo bench` command:

```bash
cargo bench --all-features --verbose --workspace 
```

## License

Licensed under the Apache License, Version 2.0, ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))

## Contribution

Contributions are welcome, however, ensure that you have read the [CONTRIBUTING.md](CONTRIBUTING.md) file before submitting a pull request.
