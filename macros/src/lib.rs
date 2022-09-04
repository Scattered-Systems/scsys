/*
    Appellation: scsys-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{actors::*, components::*, core::*, data::*};


mod actors;
mod components;
mod core;
mod data;



#[macro_export]
macro_rules! timestamp {
    ( $( ($x:expr) )*, ) => {
        (
            let mut tmp = Vec::new();

            $( tmp.push(($x, $y)))*

            tmp
        )
    }
}

#[macro_export]
macro_rules! dframe {
    ( $( ($x:expr, $y:expr) )*, ) => {
        (
            let mut tmp = Vec::new();

            $( tmp.push(($x, $y)))*

            tmp
        )
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_dframe() {
        let a = "a".to_string();
        let b = a.clone();

        assert_eq!(a, b)
    }
}