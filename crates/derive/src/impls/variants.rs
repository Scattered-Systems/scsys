/*
    appellation: variants <module>
    authors: @FL03
*/
use crate::utils::snakecase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Fields, Ident, Variant};

pub fn impl_functional_constructors(
    name: &Ident,
    variants: &Punctuated<Variant, Comma>,
) -> TokenStream {
    use syn::{FieldsNamed, FieldsUnnamed};
    // initialize a new token stream to hold the constructors
    let mut constructors = TokenStream::new();
    // iterate through each variant
    variants.iter().for_each(
        |Variant {
             ident: variant_name,
             fields: variant_fields,
             ..
         }| {
            let constructor_name = Ident::new(&snakecase(variant_name), Span::call_site());

            let func = match variant_fields {
                Fields::Named(FieldsNamed { named, .. }) => {
                    let mut field_names = Vec::new();
                    let mut field_types = Vec::new();
                    // iterate through named fields to create field names and types
                    named.iter().for_each(|i| {
                        if let Some(ident) = &i.ident {
                            field_names.push(ident);
                        }
                        field_types.push(&i.ty);
                    });
                    // create the constructor function for variants with named fields
                    let constructor = quote! {
                        pub fn #constructor_name(#(#field_names: #field_types),*) -> Self {
                            Self::#variant_name { #(#field_names),* }
                        }
                    };
                    // return the constructor
                    constructor
                }
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    // initialize vectors to hold field names and types
                    let mut field_names = Vec::new();
                    let mut field_types = Vec::new();
                    // enumerate through unnamed fields to create field names
                    unnamed.iter().enumerate().for_each(|(i, field)| {
                        field_names.push(Ident::new(&format!("field_{}", i), Span::call_site()));
                        field_types.push(&field.ty);
                    });
                    // create the constructor function for variants with unnamed fields
                    let constructor = quote! {
                        pub fn #constructor_name(#(#field_names: #field_types),*) -> Self {
                            Self::#variant_name(#(#field_names),*)
                        }
                    };
                    // return the constructor
                    constructor
                }
                Fields::Unit => {
                    // create a constructor function for unit variants
                    let constructor = quote! {
                        pub fn #constructor_name() -> Self {
                            Self::#variant_name
                        }
                    };
                    // return the constructor
                    constructor
                }
            };
            constructors.extend(func);
        },
    );
    // wrap the constructors in an impl block for the given name
    quote! {
        impl #name {
            #constructors
        }
    }
}
