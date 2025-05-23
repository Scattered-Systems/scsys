/*
    Appellation: string <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[cfg(feature = "alloc")]
use alloc::string::{String, ToString};

/// This trait defines a method for removing the first and last entries within an entity.
///
/// Typically, this is used to remove the first and last characters of a string, such as a
/// quote or a parenthesis.
pub trait RemoveFnl {
    type Output;

    fn remove_fnl(&self) -> Self::Output;
}
/// [`StringFmt`] is a trait that provides methods for formatting strings.
/// **Note** This crate requires the `alloc` feature
#[cfg(feature = "alloc")]
pub trait StringFmt {
    /// Converts the string to a `kebab-case` format.
    fn kebab_case(&self) -> String;
    /// Converts the string to a `dot.case` format.
    fn dot_case(&self) -> String;
    /// converts into a `snake_case` format.
    fn snake_case(&self) -> String;
    /// Converts the string to a `Title case` format.
    fn title_case(&self) -> String;
}

/*
 ************* Implementations *************
*/

impl<'a> RemoveFnl for &'a str {
    type Output = &'a str;

    fn remove_fnl(&self) -> Self::Output {
        &self[1..self.len() - 1]
    }
}

#[cfg(feature = "alloc")]
impl RemoveFnl for String {
    type Output = String;

    fn remove_fnl(&self) -> Self::Output {
        self[1..self.len() - 1].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_fnl() {
        let s = "\"Hello, World!\"";
        assert_eq!(s.chars().nth(0), Some('"'));
        assert_eq!(s.remove_fnl(), "Hello, World!");
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_remove_fnl_alloc() {
        let s = String::from("\"Hello, World!\"");
        assert_eq!(s.remove_fnl(), String::from("Hello, World!"));
    }
}
