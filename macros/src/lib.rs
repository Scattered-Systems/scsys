/*
    Appellation: scsys-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
extern crate scsys_core;

pub mod utils;

#[macro_export]
macro_rules! join {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp = String::new();
            $(
                tmp.push_str($x);
            )*
            tmp
        }
    };
}

#[macro_export]
macro_rules! extend_path {
    ($(
        $x:expr;
        [ $( $y:expr ),* ]
    );*) => {
        vec![
            $($(
            format!("{}/{}", $x, $y)
            ),*),*
        ]
    }
}

#[macro_export]
macro_rules! rstr {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp = Vec::new();
            $(
                tmp.push($crate::utils::gen_random_string($x));
            )*
            tmp
        }
    };
}
