/*
    Appellation: string <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[cfg(feature = "alloc")]
pub use impl_alloc::*;

/// This trait defines a method for removing the first and last entries within an entity.
///
/// Typically, this is used to remove the first and last characters of a string, such as a
/// quote or a parenthesis.
pub trait RemoveFnl {
    type Output;

    fn remove_fnl(&self) -> Self::Output;
}

impl<'a> RemoveFnl for &'a str {
    type Output = &'a str;

    fn remove_fnl(&self) -> Self::Output {
        &self[1..self.len() - 1]
    }
}

#[cfg(feature = "alloc")]
impl RemoveFnl for alloc::string::String {
    type Output = alloc::string::String;

    fn remove_fnl(&self) -> Self::Output {
        self[1..self.len() - 1].to_string()
    }
}

#[cfg(feature = "alloc")]
mod impl_alloc {
    use alloc::string::String;

    pub trait StringFmt {
        fn snake_case(&self) -> String;

        fn title_case(&self) -> String;
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
