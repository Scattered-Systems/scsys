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

    pub fn is_float<T: ToString>(data: T) -> bool {
        let input = data.to_string();
        if let Err(_) = f64::from_str(&input) {
            false
        } else {
            true
        }
    }
}
