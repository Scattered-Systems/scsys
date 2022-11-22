/*
    Appellation: impls <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{naming::*, temporal::*};

pub(crate) mod temporal;

pub(crate) mod naming {

    pub fn impl_named(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
        let name = &ast.ident;
        let res = quote::quote! {
            impl Named for #name {
                fn name() -> String {
                    format!("{}", stringify!(#name))
                }
            }
        };
        res
    }
}
