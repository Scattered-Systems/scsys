/*
    Appellation: temporal <impls>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub fn impl_temporal(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let res = quote::quote! {
        impl Temporal for #name {
            fn timestamp(&self) -> i64 {
                self.timestamp.clone().into()
            }
        }
    };
    res
}
