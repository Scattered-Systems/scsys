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

pub(crate) use self::utils::*;

pub(crate) mod display;
pub(crate) mod enums;
pub(crate) mod params;
pub(crate) mod utils;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::Comma;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Variant};

#[proc_macro_derive(Name, attributes(Alternative))]
pub fn name(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = impl_name(&ast);

    gen.into()
}

fn impl_name(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let res = quote::quote! {
        impl scsys::prelude::Name for #name {
            fn name(&self) -> String {
                format!("{}", stringify!(#name))
            }
        }
    };
    res
}

#[proc_macro_derive(SerdeDisplay)]
pub fn serde_display(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = display::impl_serde_display(&ast);

    gen.into()
}

#[proc_macro_derive(Display, attributes(json))]
pub fn any_display(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = display::impl_display(&ast);

    gen.into()
}

#[proc_macro_derive(EnumConstructors)]
pub fn derive_functional_constructors(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(inner) => enums::impl_functional_constructors(&ast.ident, &inner.variants),
        _ => panic!("This derive macro only works with enums"),
    }
    .into()
}

/// This macro generates a parameter struct and an enum of parameter keys.
#[proc_macro_derive(Params, attributes(param))]
pub fn params(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Get the name of the struct
    let struct_name = &input.ident;
    let store_name = format_ident!("{}Key", struct_name);

    // Generate the parameter struct definition
    let _param_struct = match &input.data {
        Data::Struct(s) => match &s.fields {
            _ => {}
        },
        _ => panic!("Only structs are supported"),
    };

    // Generate the parameter keys enum
    let param_keys_enum = match &input.data {
        Data::Struct(s) => {
            let DataStruct { fields, .. } = s;

            params::generate_keys(fields, &store_name)
        }
        _ => panic!("Only structs are supported"),
    };

    // Combine the generated code
    let generated_code = quote! {

        #param_keys_enum
    };

    // Return the generated code as a TokenStream
    generated_code.into()
}
