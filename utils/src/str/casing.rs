/*
    Appellation: casing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(feature = "alloc")]
#[doc(inline)]
pub use self::utils::*;

#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantArray,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize,),
    serde(rename_all = "snake_case")
)]
#[strum(serialize_all = "snake_case")]
pub enum CaseType {
    CamelCase,
    KebabCase,
    PascalCase,
    #[default]
    SnakeCase,
}

#[cfg(feature = "alloc")]
impl CaseType {
    /// Converts a string to the specified case.
    pub fn convert(&self, s: &str) -> alloc::string::String {
        match self {
            Self::CamelCase => utils::to_camelcase(s),
            Self::KebabCase => utils::to_kebabcase(s),
            Self::PascalCase => utils::to_pascalcase(s),
            Self::SnakeCase => utils::to_snakecase(s),
        }
    }
}

#[cfg(feature = "alloc")]
mod utils {
    use alloc::string::String;
    use core::str::Chars;

    /// Converts a string to snake_case.
    pub fn to_snakecase(s: &str) -> String {
        _snakecase(s.chars())
    }

    /// Converts a string to camelCase.
    pub fn to_camelcase(s: &str) -> String {
        _camelcase(s.chars())
    }

    /// Converts a string to PascalCase.
    pub fn to_pascalcase(s: &str) -> String {
        _pascalcase(s.chars())
    }

    /// Converts a string to kebab-case.
    pub fn to_kebabcase(s: &str) -> String {
        _kebabcase(s.chars())
    }

    /// Converts a string to SCREAMING_SNAKE_CASE.
    pub fn to_screaming_snakecase(s: &str) -> String {
        _screaming_snakecase(s.chars())
    }

    /// Converts a string to snake_case.
    fn _snakecase<'a>(chars: Chars<'a>) -> String {
        chars.fold(String::new(), |mut acc, c| {
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
    fn _camelcase<'a>(mut chars: Chars<'a>) -> String {
        // let first = chars.next().unwrap();
        // let rest = chars.collect::<String>();
        // format!("{}{}", first.to_lowercase(), rest)
        chars.clone().enumerate().fold(String::new(), |mut acc, (i, c)| {
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

    /// Converts a string to PascalCase.
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

    /// Converts a string to kebab-case.
    fn _kebabcase<'a>(chars: Chars<'a>) -> String {
        chars.fold(String::new(), |mut acc, c| {
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
}
