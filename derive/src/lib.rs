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
        impl scsys::crypto::Hashable for #name {
            fn hash(&self) -> scsys::crypto::hash::H256 {
                scsys::crypto::hash::hasher(&self).into()
            }
        }
    };
    res
}

#[proc_macro_derive(Temporal)]
pub fn temporal(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let gen = impls::temporal::impl_temporal(&ast);
    gen.into()
}


#[proc_macro_derive(Named, attributes(Alternative))]
pub fn named(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = impls::naming::impl_named(&ast);

    gen.into()
}
