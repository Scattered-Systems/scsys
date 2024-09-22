/*
    Appellation: alloc <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

use alloc::string::String;
/// Remove the first and last charecters of a string
pub fn fnl_remove(data: impl ToString) -> String {
    let data = data.to_string();
    let mut chars = data.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}
