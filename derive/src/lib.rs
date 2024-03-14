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

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, DeriveInput};
use syn::{Data, Fields, Variant};

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
    let gen = impl_serde_display(&ast);

    gen.into()
}

fn impl_serde_display(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let res = quote::quote! {
        impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", scsys::prelude::fnl_remove(serde_json::to_string(&self).unwrap()))
                }
            }
    };
    res
}

#[proc_macro_derive(FunctionalConstructors)]
pub fn derive_functional_constructors(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    match ast.data {
        Data::Enum(inner) => impl_functional_constructors(&ast.ident, &inner.variants),
        _ => panic!("This derive macro only works with enums"),
    }
    .into()
}

fn impl_functional_constructors(
    name: &syn::Ident,
    variants: &Punctuated<Variant, Comma>,
) -> proc_macro2::TokenStream {
    let mut constructors = proc_macro2::TokenStream::new();

    for variant in variants {
        let variant_name = &variant.ident;
        let constructor_name = syn::Ident::new(
            &format!(
                "{}_{}",
                name.to_string().to_lowercase(),
                variant_name.to_string().to_lowercase()
            ),
            proc_macro2::Span::call_site(),
        );

        let fields = match &variant.fields {
            Fields::Named(fields) => fields.named.clone(),
            Fields::Unnamed(fields) => fields.unnamed.clone(),
            Fields::Unit => Punctuated::new(),
        };

        let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
        let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

        let constructor = quote! {
            pub fn #variant(#(#field_names: #field_types),*) -> Self {
                    Self::#variant_name { #(#field_names),* }
                }
        };

        constructors.extend(constructor);
    }
    quote! {
        impl #name {
            #constructors
        }
    }
}

pub(crate) fn handle_functional_unit() -> proc_macro2::TokenStream {
    quote! {}
}
