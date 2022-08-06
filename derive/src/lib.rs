/*
    Appellation: scsys-derive <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
extern crate proc_macro;

use proc_macro::TokenStream;
use quote;
use syn;

#[proc_macro_derive(SampleFunction)]
pub fn derive_sample_function(_item: TokenStream) -> TokenStream {
    "fn sample() -> u16 { 18 }".parse().unwrap()
}

#[proc_macro_derive(Describe)]
pub fn describe(input: TokenStream) -> TokenStream {
    let syn::DeriveInput { ident, data, .. } = syn::parse_macro_input!(input);

    let description = match data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
                let idents = named.iter().map(|f| &f.ident);
                format!(
                    "a struct with these named fields: {}",
                    quote::quote! {#(#idents), *}
                )
            }
            syn::Fields::Unnamed(syn::FieldsUnnamed { unnamed, .. }) => {
                let num_fields = unnamed.iter().count();
                format!("a struct with {} unnamed fields", num_fields)
            }
            syn::Fields::Unit => format!("a unit struct"),
        },
        syn::Data::Enum(syn::DataEnum { variants, .. }) => {
            let vs = variants.iter().map(|v| &v.ident);
            format!("an enum with these variants: {}", quote::quote! {#(#vs),*})
        }
        syn::Data::Union(syn::DataUnion {
                             fields: syn::FieldsNamed { named, .. },
                             ..
                         }) => {
            let idents = named.iter().map(|f| &f.ident);
            format!(
                "a union with these named fields: {}",
                quote::quote! {#(#idents),*}
            )
        }
    };

    let output = quote::quote! {
    impl #ident {
        fn describe() -> String { format!("{} is {}.", stringify!(#ident), #description) }
    }
    };

    output.into()
}
