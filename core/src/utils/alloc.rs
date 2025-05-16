/*
    Appellation: alloc <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

/// Remove the first and last charecters of a string
#[cfg(feature = "alloc")]
pub fn fnl_remove(data: impl ToString) -> alloc::string::String {
    let data = data.to_string();
    let mut chars = data.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}
