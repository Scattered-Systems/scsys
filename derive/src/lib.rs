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

#[proc_macro_derive(Display, attributes(scsys))]
pub fn display(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let res = impls::impl_display(&ast);

    res.into()
}
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
pub fn getter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impls::impl_getter(&input).into()
}

#[proc_macro_derive(Set, attributes(scsys))]
pub fn set_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impls::impl_set(&input).into()
}

#[proc_macro_derive(With, attributes(scsys))]
pub fn with_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impls::impl_with(&input).into()
}
