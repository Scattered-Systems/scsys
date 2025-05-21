/*
    Appellation: get <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[macro_export]
macro_rules! get {
    ($($call:ident$(.$field:ident)?<$out:ty>),* $(,)?) => {
        $($crate::getter!(@impl $call$(.$field)?<$out>);)*
    };
    ($via:ident::<[$($call:ident$(.$field:ident)?<$out:ty>),* $(,)?]>) => {
        $($crate::getter!(@impl $via::$call$(.$field)?<$out>);)*
    };
    ($($call:ident$(.$field:ident)?),* $(,)? => $out:ty) => {
        $($crate::getter!(@impl $call$(.$field)?<$out>);)*
    };
    ($via:ident::<[$($call:ident$(.$field:ident)?),* $(,)?]> => $out:ty) => {
        $crate::getter!($via::<[$($call$(.$field)?<$out>),*]>);
    };
    (@impl $call:ident<&$out:ty>) => {
        $crate::getter!(@get $call: &$out);
        $crate::getter!(@get_mut $call: $out);
    };
    (@impl $call:ident<$out:ty>) => {
        $crate::getter!(@get $call: $out);
        $crate::getter!(@get_mut $call: $out);
    };
    (@impl $via:ident::$call:ident<$out:ty>) => {
        $crate::getter!(@impl $via::$call.$call<$out>);
    };
    (@impl $via:ident::$call:ident.$field:ident<$out:ty>) => {
        /// Returns an immutable reference to
        pub const fn $call(&self) -> &$out {
            &self.$via.$field
        }

        paste::paste! {
            pub fn [< $call _mut>](&mut self) -> &mut $out {
                &mut self.$via.$field
            }
        }
    };
    (@get $name:ident: &$T:ty) => {
        pub const fn $name(&self) -> &$T {
            &self.$name
        }
    };
    (@get $name:ident: $T:ty) => {
        pub const fn $name(&self) -> $T {
            self.$name
        }
    };
    (@get_mut $name:ident: $T:ty) => {
        paste::paste! {

            pub fn [<$name _mut>] (&mut self) -> &mut $T {
                &mut self.$name
            }
        }
    };
}
