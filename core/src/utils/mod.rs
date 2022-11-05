/*
    Appellation: utils <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{configure::*, fs::*, misc::*, time::*};

pub(crate) mod configure;
pub(crate) mod fs;
pub(crate) mod time;

pub(crate) mod misc {
    use std::{str::FromStr, string::ToString};

    pub fn is_float<T: ToString>(data: &T) -> bool {
        match f64::from_str(&data.to_string()) {
            Err(_) => false,
            Ok(_) => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_float() {
        let data = vec!["1", "-10", "ifjuka87"];
        assert!(is_float(&data[0]));
        assert!(is_float(&data[1]));
        assert!(is_float(&data[2]) == false)
    }
}
