/*
    Appellation: scsys-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
//! procedural macros for the `scsys` ecosystem
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/favicon.ico"
)]

extern crate proc_macro;

use proc_macro::TokenStream;

/// A procedural macro for generativly creating getter methods; i.e. $field_name() -> &$field_type and $field_name_mut() -> &mut $field_type
#[proc_macro]
pub fn getter(input: TokenStream) -> TokenStream {
    println!("display: {:?}", input);
    input
}
