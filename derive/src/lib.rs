/*
    Appellation: scsys-derive <library>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;


use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};


#[proc_macro_derive(HelloWorld, attributes(HelloWorldName))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Parse the inputs into the proper struct
    let ast = parse_macro_input!(input as DeriveInput);

    // Build the impl
    let gen = impl_hello_world(&ast);
    
    gen.into()
}

fn impl_hello_world(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let res = quote! {
        impl HelloWorld for #name {
            fn hello_world() -> String {
                format!("Hello, World! My name is {}", stringify!(#name))
            }
        }
    };
    res
}
