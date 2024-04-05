/*
    Appellation: enum <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Fields, Ident, Variant};

fn impl_functional_constructors(
    name: &Ident,
    variants: &Punctuated<Variant, Comma>,
) -> TokenStream {
    let mut constructors = TokenStream::new();

    for variant in variants {
        let variant_name = &variant.ident;
        let constructor_name =
            Ident::new(&variant_name.to_string().to_lowercase(), Span::call_site());

        let func = match &variant.fields {
            Fields::Named(fields) => {
                let mut field_names = Vec::new();
                let mut field_types = Vec::new();
                for i in fields.named.iter() {
                    if let Some(ident) = &i.ident {
                        field_names.push(ident);
                    }
                    field_types.push(&i.ty);
                }

                let constructor = quote! {
                    pub fn #constructor_name(#(#field_names: #field_types),*) -> Self {
                        Self::#variant_name { #(#field_names),* }
                    }
                };

                constructor
            }
            Fields::Unnamed(fields) => {
                let mut field_names = Vec::new();
                let mut field_types = Vec::new();
                for (i, field) in fields.unnamed.iter().enumerate() {
                    field_names.push(Ident::new(&format!("field_{}", i), Span::call_site()));
                    field_types.push(&field.ty);
                }

                let constructor = quote! {
                    pub fn #constructor_name(#(#field_names: #field_types),*) -> Self {
                        Self::#variant_name { #(#field_names),* }
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
