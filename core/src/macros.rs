/*
    Appellation: macros <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[allow(unused_macros)]
macro_rules! impl_fmt {
    ($name:ty: $($t:ident($($rest:tt)*)),*) => {
        $(impl_fmt!($name: $t($($rest)*));)*
    };
    (@impl $name:ty: $t:ident($($rest:tt)*)) => {
        impl core::fmt::$t for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, $($rest)*)
            }
        }
    };
}
