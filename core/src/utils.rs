/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::any::{Any, TypeId};
#[cfg(feature = "std")]
pub use self::std_utils::*;
#[cfg(all(feature = "alloc", no_std))]
pub use alloc::string::String;


pub(crate) mod std_utils;

/// Compare two types
pub fn type_of<U, V>() -> bool
where
    U: Any + ?Sized,
    V: Any + ?Sized,
{
    TypeId::of::<U>() == TypeId::of::<V>()
}

/// Remove the first and last charecters of a string
pub fn fnl_remove(data: impl ToString) -> String {
    let data = data.to_string();
    let mut chars = data.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}

pub fn snakecase(name: impl ToString) -> String {
    let data = name.to_string();

    let mut buffer = String::with_capacity(data.len() + data.len() / 2);

    let mut text = data.chars();

    if let Some(first) = text.next() {
        let mut n2: Option<(bool, char)> = None;
        let mut n1: (bool, char) = (first.is_lowercase(), first);

        for c in text {
            let prev_n1 = n1.clone();

            let n3 = n2;
            n2 = Some(n1);
            n1 = (c.is_lowercase(), c);

            // insert underscore if acronym at beginning
            // ABc -> a_bc
            if n1.0
                && matches!(n2, Some((false, _)))
                && matches!(n3, Some((false, _)))
                && n2.unwrap().1.is_uppercase()
                && n3.unwrap().1.is_uppercase()
            {
                buffer.push('_');
            }

            buffer.push_str(&prev_n1.1.to_lowercase().to_string());

            // insert underscore before next word
            // abC -> ab_c
            if matches!(n2, Some((true, _))) && n1.1.is_uppercase() {
                buffer.push('_');
            }
        }

        buffer.push_str(&n1.1.to_lowercase().to_string());
    }

    buffer
}
