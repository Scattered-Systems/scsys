/*
    Appellation: scsys-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
//! # scsys-macros
//!
//!
extern crate proc_macro;

use proc_macro::TokenStream;

#[doc(hidden)]
#[proc_macro]
pub fn display(input: TokenStream) -> TokenStream {
    println!("display: {:?}", input);
    input
}