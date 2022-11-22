/*
    Appellation: temporal <impls>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub fn impl_hashable(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let res = quote::quote! {
        impl Hashable for #name {
            fn hash(&self) -> H256 {
                scsys::crypto::hash::hasher(&self).into()
            }
        }
    };
    res
}
