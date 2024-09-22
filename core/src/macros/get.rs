/*
    Appellation: get <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[macro_export]
macro_rules! getter {
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

    (@impl $call:ident<$out:ty>) => {
        $crate::getter!(@impl $call.$call<$out>);
    };
    (@impl $via:ident::$call:ident<$out:ty>) => {
        $crate::getter!(@impl $via::$call.$call<$out>);
    };
    (@impl $call:ident.$field:ident<$out:ty>) => {
        pub const fn $call(&self) -> &$out {
            &self.$field
        }
        paste::paste! {
            pub fn [< $call _mut>](&mut self) -> &mut $out {
                &mut self.$field
            }
        }
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
}

macro_rules! impl_getters {
    (@impl $call:ident<$out:ty>) => {
        $crate::getter!(@impl $call.$call<$out>);
    };
    (@impl $via:ident::$call:ident<$out:ty>) => {
        $crate::getter!(@impl $via::$call.$call<$out>);
    };
    (@impl $call:ident.$field:ident<$out:ty>) => {
        pub const fn $call(&self) -> &$out {
            &self.$field
        }
        paste::paste! {
            pub fn [< $call _mut>](&mut self) -> &mut $out {
                &mut self.$field
            }
        }
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
}
