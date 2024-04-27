/*
    Appellation: display <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn impl_serde_display(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let res = quote! {
        impl core::fmt::Display for #name {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(f, "{}", serde_json::to_string(&self).unwrap())
                }
            }
    };
    res
}

pub fn impl_display(ast: &DeriveInput) -> TokenStream {
    if ast
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("serde"))
        .is_some()
    {
        return impl_serde_display(ast);
    }
    let name = &ast.ident;
    let _data = &ast.data;
    let res = quote! {
        impl core::fmt::Display for #name {
                fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                    let mut s = String::new();
                    write!(f, "{}", s)
                }
            }
    };
    res
}
