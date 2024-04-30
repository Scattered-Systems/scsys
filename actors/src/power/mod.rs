/*
    Appellation: power <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Power
//!
//!
pub use self::state::Power;

pub(crate) mod state;

#[cfg(feature = "tokio-ext")]
pub mod shutdown;

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[test]
    fn test_power() {
        let on = {
            let tmp = Power::from_str("on");
            assert!(tmp.is_ok());
            tmp.unwrap()
        };

        assert_eq!(on, Power::On);
    }
}
