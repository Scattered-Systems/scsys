/*
    Appellation: scsys-derive <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
extern crate proc_macro;
extern crate quote;
extern crate syn;

pub(crate) mod ast;
pub(crate) mod attrs;
pub(crate) mod utils;

pub(crate) mod impls {
    #[doc(inline)]
    pub(crate) use self::prelude::*;

    pub mod impl_display;
    pub mod impl_gsw;
    pub mod impl_variants;

    mod prelude {
        pub use super::impl_display::*;
        pub use super::impl_gsw::*;
        pub use super::impl_variants::*;
    }
}

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

#[proc_macro_derive(Getter)]
pub fn getter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impls::impl_getter(&input).into()
}

#[proc_macro_derive(Set)]
pub fn set_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impls::impl_set(&input).into()
}

#[proc_macro_derive(With)]
pub fn with_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impls::impl_with(&input).into()
}
