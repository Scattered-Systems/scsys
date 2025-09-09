/*
    Appellation: scsys-derive <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # scsys-derive
//!
//! This crate implements the derive macros for the `scsys` ecosystem.
//!
//! ## Macros
//!
//! - [`Display`]: automatically implements the `Display` trait for a struct or enum, using the
//!   `scsys` attributes to customize the output.
//! - [`VariantConstructors`]: generate functional constructors for all variants of an enum
#![allow(
    non_snake_case,
    clippy::module_inception,
    clippy::missing_safety_doc,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/favicon.ico"
)]
extern crate proc_macro;
extern crate quote;
extern crate syn;

pub(crate) mod attrs;
pub(crate) mod impls;
pub(crate) mod utils;

pub(crate) mod ast;
use proc_macro::TokenStream;
use syn::{Data, DeriveInput, parse_macro_input};

/// the [`Display`] macro automatically implements the [`Display`](core::fmt::Display) trait
/// for a struct or enum, using the `scsys` attributes to customize the output.
///
/// ## Examples
///
/// ### _Example #1: Using the `json` attribute_
///
/// ```rust
/// use scsys_derive::Display;
///
/// #[derive(Display, serde::Deserialize, serde::Serialize)]
/// #[scsys(json)]
/// pub struct MyStruct {
///     pub name: String,
///     pub age: u32,
/// }
///
/// fn main() {
///     let my_struct = MyStruct {
///         name: "Alice".to_string(),
///         age: 30,
///     };
///
///    // This will print the struct in JSON format
///     println!("{}", my_struct);
/// }
/// ```
///
/// #### Output:
///
/// ```json
/// {"name": "Alice", "age": 30}
/// ```
///
///
/// **note:** for now, the primary use case is to automatically implement the `Display` trait
/// for implementors of both `Deserialize` and `Serialize` from [`serde`](https://serde.rs),
///
#[proc_macro_derive(Display, attributes(scsys))]
pub fn display(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let res = impls::impl_display(&ast);

    res.into()
}

#[deprecated(since = "0.3.2", note = "use the variants crate instead")]
/// This macro automatically generates functional constructors for all enclosed variants.
#[proc_macro_derive(VariantConstructors, attributes(scsys))]
pub fn variant_constructors(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(inner) => impls::impl_functional_constructors(&ast.ident, &inner.variants),
        _ => panic!("This derive macro only works with enums"),
    }
    .into()
}
/// The [`Wrapper`] macro is designed for single-field structs, implementing additional methods
/// supporting interactions with the inner value
#[proc_macro_derive(Wrapper, attributes(scsys))]
pub fn wrapper(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let res = impls::impl_wrapper(&ast);

    res.into()
}

#[proc_macro_derive(Getter, attributes(scsys))]
pub fn derive_getter(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let get = impls::impl_getter(&input);
    let get_mut = impls::impl_getter_mut(&input);

    let merged = quote::quote! {
        #get
        #get_mut
    };

    merged.into()
}

#[proc_macro_derive(Get, attributes(scsys))]
pub fn derive_get(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impls::impl_getter(&input).into()
}

#[proc_macro_derive(GetMut, attributes(scsys))]
pub fn derive_get_mut(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impls::impl_getter_mut(&input).into()
}

#[proc_macro_derive(Set, attributes(scsys))]
pub fn derive_set(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impls::impl_set(&input).into()
}

#[proc_macro_derive(With, attributes(scsys))]
pub fn derive_with(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impls::impl_with(&input).into()
}
