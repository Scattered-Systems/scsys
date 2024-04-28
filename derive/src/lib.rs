/*
    Appellation: scsys-derive <library>
    Contrib: FL03 <jo3mccain@icloud.com>

*/
//! # scsys-derive
//!
//! Useful derive macros for the scsys ecosystem

extern crate proc_macro;
extern crate quote;
extern crate syn;

pub(crate) mod ast;
pub(crate) mod attrs;
pub(crate) mod display;
pub(crate) mod enums;
pub(crate) mod params;
pub(crate) mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Display, attributes(display))]
pub fn any_display(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = display::impl_display(&ast);

    gen.into()
}

#[proc_macro_derive(VariantConstructors)]
pub fn derive_functional_constructors(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(inner) => enums::impl_functional_constructors(&ast.ident, &inner.variants),
        _ => panic!("This derive macro only works with enums"),
    }
    .into()
}

/// This macro generates a parameter struct and an enum of parameter keys.
#[proc_macro_derive(Keyed, attributes(key))]
pub fn keyed(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let gen = params::impl_keyed(&input);

    // Return the generated code as a TokenStream
    gen.into()
}
