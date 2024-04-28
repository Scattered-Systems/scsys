/*
    Appellation: macros <module>
    Creator: FL03 <jo3mccain@icloud.com>
*/

macro_rules! enum_from_primitive {
    ($name:ident($($k:literal: $v:ident),*)<$t:ty>) => {
        enum_from_primitive!(@impl $name($($k: $v),*)<$t>);
    };
    (@impl $name:ident($($k:literal: $v:ident),*)<$t:ty>) => {
        impl From<$t> for $name {
            fn from(d: $t) -> Self {
                match d as usize % Self::COUNT {
                    $(
                        $k => $name::$v,
                    )*
                    _ => $name::default(),
                }
            }
        }

        impl From<$name> for $t {
            fn from(d: $name) -> Self {
                d as $t
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {
        $sub
    };
}
