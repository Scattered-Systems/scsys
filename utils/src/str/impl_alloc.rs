/*
    Appellation: impl_alloc <str>
    Contrib: @FL03
*/
use alloc::string::{String, ToString};
use core::str::Chars;

/// Remove the first and last charecters of a string
pub fn fnl_remove(data: impl ToString) -> String {
    let data = data.to_string();
    let mut chars = data.chars();
    chars.next();
    chars.next_back();
    chars.as_str().to_string()
}
/// converts a string to camelCase.
pub fn to_camelcase<T: AsRef<str>>(s: T) -> String {
    fn _camelcase<'a>(mut chars: Chars<'a>) -> String {
        chars
            .clone()
            .enumerate()
            .fold(String::new(), |mut acc, (i, c)| {
                match i {
                    0 => acc.push(c.to_lowercase().next().unwrap()),
                    _ => {
                        if c == '_' {
                            // Skip the underscore and convert the next character to uppercase
                            if let Some(next) = chars.next() {
                                acc.push(next.to_uppercase().next().unwrap());
                            }
                        } else {
                            acc.push(c);
                        }
                    }
                }
                acc
            })
    }
    _camelcase(s.as_ref().chars())
}
/// converts a string to PascalCase.
pub fn to_pascalcase<T: AsRef<str>>(s: T) -> String {
    fn _pascalcase<'a>(chars: Chars<'a>) -> String {
        chars.enumerate().fold(String::new(), |mut acc, (i, c)| {
            match i {
                0 => acc.push(c.to_uppercase().next().unwrap()),
                _ => {
                    if c.is_uppercase() {
                        acc.push(c.to_lowercase().next().unwrap());
                    } else {
                        acc.push(c);
                    }
                }
            }
            acc
        })
    }
    
    _pascalcase(s.as_ref().chars())
}
/// converts a string to kebab-case.
pub fn to_kebabcase<T: AsRef<str>>(s: T) -> String {
    fn _kebabcase<'a>(chars: Chars<'a>) -> String {
        chars
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
    _kebabcase(s.as_ref().chars())
}
/// converts a string to snake_case.
pub fn to_snakecase<T: AsRef<str>>(s: T) -> String {
    fn _snakecase<'a>(chars: Chars<'a>) -> String {
        chars
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
    _snakecase(s.as_ref().chars())
}
/// converts a string to SCREAMING_SNAKE_CASE.
pub fn to_screaming_snakecase<T: AsRef<str>>(s: T) -> String {
    fn _screaming_snakecase<'a>(chars: Chars<'a>) -> String {
        chars.fold(String::new(), |mut acc, c| {
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
    _screaming_snakecase(s.as_ref().chars())
}
