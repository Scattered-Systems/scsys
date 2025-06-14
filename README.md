# scsys

[![crates.io](https://img.shields.io/crates/v/scsys?style=for-the-badge&logo=rust)](https://crates.io/crates/scsys)
[![docs.rs](https://img.shields.io/docsrs/scsys?style=for-the-badge&logo=docs.rs)](https://docs.rs/scsys)
[![GitHub License](https://img.shields.io/github/license/scattered-systems/scsys?style=for-the-badge&logo=github)](https://github.com/scattered-systems/scsys/blob/main/LICENSE)

***

Welcome to `scsys`, a collection of useful utilities, types, and other primitives that are used in various projects developed by [Scattered Systems](https://github.com/scattered-systems). The library is designed to be a general-purpose utility library that can be used in any Rust project, aiming to provide a standardized set of tools that can be used to build robust and reliable software.

## Usage

Before you start using `scsys`, make sure to add it as a dependency in your `Cargo.toml` file. You can do this by adding the following lines:

```toml
[dependencies.scsys]
default-features = true
features = [
    "derive",
]
version = "0.2.x"
```

### Examples

#### Example 1: _Using the `VariantConstructors` derive macro_

The `VariantConstructors` derive macro can be used to automatically generate functional accessors for named fields within a given structure. For example, given the following structure:

```rust
#[derive(
    Clone, 
    Copy, 
    Debug, 
    Default, 
    Eq, 
    Hash, 
    Ord, 
    PartialEq, 
    PartialOrd, 
    scsys::VariantConstructors
)
pub enum Sample {
    A,
    B(usize),
    C { x: f32, y: f32 },
}
```

we can automatically generate a functional constructors for each of the `Sample` variants:

```rust
let a = Sample::a();
let b = Sample::b(42);
let c = Sample::c(1.0, 2.0);
assert_eq!(a, Sample::A);
assert_eq!(b, Sample::B(42));
assert_eq!(c, Sample::C { x: 1.0, y: 2.0 });
```

## Getting Started

To get started with `scsys`, you can check out the [QUICKSTART.md](QUICKSTART.md) file, which provides a step-by-step guide on how to set up your development environment and start using the library.

## License

Licensed under the Apache License, Version 2.0, ([LICENSE-APACHE](http://www.apache.org/licenses/LICENSE-2.0))

## Contribution

Contributions are welcome, however, ensure that you have read the [CONTRIBUTING.md](CONTRIBUTING.md) file before submitting a pull request.
