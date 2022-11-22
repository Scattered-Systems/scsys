/*
    Appellation: temporal <impls>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub fn impl_hashable(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let res = quote::quote! {
        impl scsys::crypto::Hashable for #name {
            fn hash(&self) -> scsys::crypto::hash::H256 {
                scsys::crypto::hash::hasher(&self).into()
            }
        }
    };
    res
}
