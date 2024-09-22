/*
    Appellation: casing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Casing
//!
//! This module works to implement various naming conventions and name-related primitives.
#![cfg(feature = "alloc")]
#[doc(inline)]
pub use self::{kind::*, utils::*};

mod kind;

pub(crate) mod prelude {
    pub use super::kind::CaseType;
}

mod utils {
    #[cfg(feature = "alloc")]
    use alloc::string::String;

    /// Converts a string to snake_case.
    pub fn to_snakecase(s: &str) -> String {
        s.chars()
            .fold(String::new(), |mut acc, c| {
                if c.is_uppercase() {
                    if !acc.is_empty() {
                        acc.push('_');
                    }
                    acc.push(c.to_lowercase().next().unwrap());
                } else {
                    acc.push(c);
                }
                acc
            })
            .to_lowercase()
    }

    /// Converts a string to camelCase.
    pub fn to_camelcase(s: &str) -> String {
        let mut chars = s.chars();
        let first = chars.next().unwrap();
        let rest = chars.collect::<String>();
        format!("{}{}", first.to_lowercase(), rest)
    }

    /// Converts a string to PascalCase.
    pub fn to_pascalcase(s: &str) -> String {
        let mut chars = s.chars();
        let first = chars.next().unwrap();
        let rest = chars.collect::<String>();
        format!("{}{}", first.to_uppercase(), rest)
    }

    /// Converts a string to kebab-case.
    pub fn to_kebabcase(s: &str) -> String {
        s.chars()
            .fold(String::new(), |mut acc, c| {
                if c.is_uppercase() {
                    if !acc.is_empty() {
                        acc.push('-');
                    }
                    acc.push(c.to_lowercase().next().unwrap());
                } else {
                    acc.push(c);
                }
                acc
            })
            .to_lowercase()
    }

    /// Converts a string to SCREAMING_SNAKE_CASE.
    pub fn to_screaming_snakecase(s: &str) -> String {
        s.chars().fold(String::new(), |mut acc, c| {
            if c.is_uppercase() {
                if !acc.is_empty() {
                    acc.push('_');
                }
                acc.push(c);
            } else {
                acc.push(c.to_uppercase().next().unwrap());
            }
            acc
        })
    }
}
