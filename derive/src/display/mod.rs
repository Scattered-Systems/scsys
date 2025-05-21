#[allow(unused_imports)]
#[doc(inline)]
pub use self::{ast::*, attr::*};

pub mod ast;
pub mod attr;

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
        generics,
        ident: name,
        ..
    } = &ast;
    // check if the display attribute is present
    let display_attr = attrs
        .iter()
        .find(|attr| attr.path().is_ident("scsys"))
        .and_then(|attr| attr.parse_args::<crate::attrs::ScsysAttr>().ok())
        .and_then(|attr| attr.display);

    if let Some(display_attr) = display_attr {
        // check if the serde attribute is present
        if let Some(_) = display_attr.serde {
            // handle the serde implementation
            return handle_serde_display(name, generics);
        }
        // check if the kind attribute is present
        if let Some(_) = display_attr.kind {
            // handle the kind implementation
            return handle_serde_display(name, generics);
        }
    }

    // handle the standard implementation
    _handle_display(&name, &generics)
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
