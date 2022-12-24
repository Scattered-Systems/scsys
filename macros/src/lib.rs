/*
    Appellation: scsys-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::utils::*;

pub(crate) mod utils;

#[macro_export]
macro_rules! create_method {
    ($vis:vis $name:ident) => {
        $vis fn $name(&self) {}
    };
}

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
