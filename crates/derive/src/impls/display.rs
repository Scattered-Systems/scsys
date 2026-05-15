/*
    Appellation: impl_display <module>
    Contrib: @FL03
*/
use crate::attrs::ScsysAttr;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Generics, Ident};

pub fn impl_display(ast: &DeriveInput) -> TokenStream {
    let DeriveInput {
        mut generics,
        attrs: attributes,
        ident: name,
        ..
    } = ast.clone();

    if let Ok(attr) = ScsysAttr::extract(&attributes) {
        // check if the kind attribute is present
        if let Some(disp) = attr.display {
            // check if the kind is json
            if let Some(fmt) = disp.format {
                let fmt_str = fmt.to_string();
                return match fmt_str.as_str() {
                    "json" => handle_serde_display(&name, &mut generics),
                    _ => handle_display(&name, &mut generics),
                };
            }
        }
    }
    // handle the standard implementation
    handle_display(&name, &mut generics)
}

fn handle_display(name: &Ident, generics: &mut Generics) -> TokenStream {
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

fn handle_serde_display(name: &Ident, generics: &mut Generics) -> TokenStream {
    add_serialize_bounds(generics);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics core::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", serde_json::to_string(self).unwrap().as_str())
            }
        }
    }
}

fn add_serialize_bounds(generics: &mut Generics) {
    let (_impl_generics, _ty_generics, where_clause) = generics.split_for_impl();

    // Clone or create a new where clause
    let mut where_clause = where_clause
        .cloned()
        .unwrap_or_else(|| syn::parse_quote!(where));

    // Add Serialize bound for each type parameter
    for param in generics.type_params() {
        let param_ident = &param.ident;
        where_clause
            .predicates
            .push(syn::parse_quote!(#param_ident: serde::Serialize));
    }

    // Update the generics with the modified where clause
    generics.where_clause = Some(where_clause);
}
