/*
    Appellation: scsys-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub(crate) mod utils;

#[macro_export]
macro_rules! shared {
    ( $arg:expr ) => {
        std::sync::Arc::new(std::sync::Mutex::new($arg))
    };
    ( $( $arg:expr ),* ) => {
        {
            let mut res = Vec::new();
            $(
                res.push(std::sync::Arc::new(std::sync::Mutex::new($arg)));
            )*
            res
        }
    };
}

#[macro_export]
macro_rules! create_method {
    ($vis:vis $name:ident, $arg:ident, $argty: ty) => {
        $vis fn $name($arg: $argty) {}
    };
}

#[macro_export]
macro_rules! string {
    ( $x:expr ) => {
        {
            $x.to_string()
        }
    };
    ( $x:ident ) => {
        {
            $x.to_string()
        }
    };
    ( $( $x:expr ),* ) => {
        {
            let mut res = Vec::new();
            $(
                res.push($x.to_string());
            )*
            res
        }
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
    ( $x:expr ) => {
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

#[cfg(feature = "gen")]
#[macro_export]
macro_rules! rstr {
    ( $( $x:expr ),* ) => {
        {
            let mut tmp = Vec::new();
            $(
                tmp.push($crate::utils::gen::gen_random_string($x));
            )*
            tmp
        }
    };
}
