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
    let gen = impl_serde_display(&ast);

    gen.into()
}

fn impl_serde_display(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let res = quote::quote! {
        impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(f, "{}", serde_json::to_string(&self).unwrap())
                }
            }
    };
    res
}

#[proc_macro_derive(Display)]
pub fn any_display(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = impl_display(&ast);

    gen.into()
}

fn impl_display(ast: &DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let _data = &ast.data;
    let res = quote::quote! {
        impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    let mut s = String::new();
                    write!(f, "{}", s)
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
            &variant_name.to_string().to_lowercase(),
            proc_macro2::Span::call_site(),
        );

        let func = match &variant.fields {
            Fields::Named(fields) => {
                let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
                let field_types: Vec<_> = fields.named.iter().map(|f| &f.ty).collect();

                let constructor = quote! {
                    pub fn #constructor_name(#(#field_names: #field_types),*) -> Self {
                        Self::#variant_name { #(#field_names),* }
                    }
                };

                constructor
            }
            Fields::Unnamed(fields) => {
                let field_names: Vec<_> = (0..fields.unnamed.len())
                    .map(|i| syn::Ident::new(&format!("field_{}", i), variant.span()))
                    .collect();
                let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();

                let constructor = quote! {
                    pub fn #constructor_name(#(#field_names: #field_types),*) -> Self {
                        Self::#variant_name(#(#field_names),*)
                    }
                };

                constructor
            }
            Fields::Unit => {
                let constructor = quote! {
                    pub fn #constructor_name() -> Self {
                        Self::#variant_name
                    }
                };

                constructor
            }
        };
        constructors.extend(func);
    }
    quote! {
        impl #name {
            #constructors
        }
    }
}
