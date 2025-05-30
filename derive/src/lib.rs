/*
    Appellation: scsys-derive <library>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/favicon.ico"
)]
extern crate proc_macro;
extern crate quote;
extern crate syn;

pub(crate) use self::impls::*;

pub(crate) mod impls;
pub(crate) mod utils;

pub(crate) mod ast {
    #[allow(unused_imports)]
    pub use self::getter::GetterAst;

    mod getter;
}

pub(crate) mod attrs {
    #[doc(inline)]
    pub use self::prelude::*;

    pub mod display_attrs;
    pub mod nested;
    pub mod scsys;
    pub mod variants;

    pub(crate) mod prelude {
        pub use super::display_attrs::*;
        pub use super::nested::*;
        pub use super::scsys::*;
        pub use super::variants::*;
    }
}

use proc_macro::TokenStream;
use syn::{Data, DeriveInput, parse_macro_input};

#[proc_macro_derive(Display, attributes(scsys))]
pub fn display(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let res = impl_display(&ast);

    res.into()
}
/// This macro automatically generates functional constructors for all enclosed variants.
#[proc_macro_derive(VariantConstructors, attributes(scsys))]
pub fn variant_constructors(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(inner) => impl_functional_constructors(&ast.ident, &inner.variants),
        _ => panic!("This derive macro only works with enums"),
    }
    .into()
}

#[proc_macro_derive(Getter)]
pub fn getter_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_getter(&input).into()
}

#[proc_macro_derive(Set)]
pub fn set_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_set(&input).into()
}

#[proc_macro_derive(With)]
pub fn with_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    impl_with(&input).into()
}
