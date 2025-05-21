/*
    Appellation: impl_display <module>
    Contrib: @FL03
*/
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Generics, Ident};

pub fn impl_display(ast: &DeriveInput) -> TokenStream {
    let DeriveInput {
        attrs,
        ident: name,
        generics,
        ..
    } = ast;
    if attrs.iter().any(|attr| attr.path().is_ident("display")) {
        // handle serde
        return handle_serde_display(name, generics);
    }

    _handle_display(name, generics)
}

pub fn _handle_display(name: &Ident, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, _where) = generics.split_for_impl();
    let mut where_clause = _where.cloned();
    // ensure that every generic implements serde::Serialize
    if let Some(where_clause) = where_clause.as_mut() {
        generics.type_params().for_each(|param| {
            where_clause
                .predicates
                .push(syn::parse_quote!(#param: core::fmt::Display))
        });
    }

    quote! {
        impl #impl_generics core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                let mut s = String::new();
                write!(f, "{}", s)
            }
        }
    }
}

pub fn handle_serde_display(name: &Ident, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let mut where_clause = where_clause.cloned();
    // ensure that every generic implements serde::Serialize
    if let Some(where_clause) = where_clause.as_mut() {
        generics.type_params().for_each(|param| {
            where_clause
                .predicates
                .push(syn::parse_quote!(#param: serde::Serialize))
        });
    }

    quote! {
        impl #impl_generics core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str(serde_json::to_string(self).unwrap().as_str())
            }
        }
    }
}

// fn handle_attrs(attrs: &Vec<syn::Attribute>) -> TokenStream {
//     if let Some(attr) = attrs
//         .iter()
//         .find(|attr| attr.path().is_ident("display")) {
//         if let Ok(meta) = attr.parse_nested_meta() {
//             if let syn::Meta::List(list) = meta {
//                 for nested in list.nested {
//                     if let syn::NestedMeta::Meta(meta) = nested {
//                         if let syn::Meta::Path(path) = meta {
//                             if path.is_ident("serde") {
//                                 // handle serde
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }
