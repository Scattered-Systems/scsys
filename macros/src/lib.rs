/*
    Appellation: scsys-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
//! # scsys-macros
//!
//!
extern crate proc_macro;

#[allow(dead_code)]
pub(crate) mod ast;
#[allow(dead_code)]
pub(crate) mod gets;

use proc_macro::TokenStream;

#[doc(hidden)]
/// A procedural macro for generativly creating getter methods; i.e. $field_name() -> &$field_type and $field_name_mut() -> &mut $field_type
#[proc_macro]
pub fn gets(input: TokenStream) -> TokenStream {
    println!("display: {:?}", input);
    input
}
