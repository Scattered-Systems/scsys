/*
    Appellation: scsys-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
*/
//! procedural macros for the `scsys` ecosystem
#![allow(
    non_snake_case,
    clippy::module_inception,
    clippy::missing_safety_doc,
    clippy::needless_doctest_main,
    clippy::upper_case_acronyms
)]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(all(feature = "alloc", feature = "nightly"), feature(allocator_api))]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/logo.png",
    html_favicon_url = "https://raw.githubusercontent.com/scattered-systems/.github/main/assets/favicon.ico"
)]

extern crate proc_macro;

use proc_macro::TokenStream;

pub(crate) mod gsw;

/// A procedural macro for generativly creating getter methods; i.e. $field_name() -> &$field_type and $field_name_mut() -> &mut $field_type
#[proc_macro]
pub fn getter(input: TokenStream) -> TokenStream {
    println!("display: {:?}", input);
    input
}
