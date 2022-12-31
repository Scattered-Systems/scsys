/*
    Appellation: scsys-derive <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub(crate) mod impls;

extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Hashable)]
pub fn hashable(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let gen = impl_hashable(&ast);

    gen.into()
}

fn impl_hashable(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let res = quote::quote! {
        impl Hashable for #name {
            fn hash(&self) -> H256 {
                scsys::crypto::hasher(&self).into()
            }
        }
    };
    res
}

#[proc_macro_derive(Named, attributes(Alternative))]
pub fn named(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = impl_named(&ast);

    gen.into()
}

pub(crate) fn impl_named(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let res = quote::quote! {
        impl Named for #name {
            fn name() -> String {
                format!("{}", stringify!(#name))
            }
        }
    };
    res
}
