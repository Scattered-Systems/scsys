/*
    Appellation: wrapper <module>
    Contrib: @FL03
*/

///
///
/// ### Example
///
/// ```rust
/// use scsys_core::fmt_wrapper;
///
/// pub struct Sample<T>(pub T);
///
/// fmt_wrapper!(Sample<Q>(Binary, Debug, Display, LowerHex, UpperHex, LowerExp, UpperExp, Pointer));
/// ```
#[macro_export]
macro_rules! fmt_wrapper {
    ($s:ident<$T:ident> ($($trait:ident),* $(,)?)) => {
        $(
            $crate::fmt_wrapper!(@impl $s::<$T>::$trait);
        )*
    };
    ($s:ident<$T:ident>.$field:ident {$($trait:ident),* $(,)?}) => {
        $(
            $crate::fmt_wrapper!(@impl $s::<$T>::$trait.$field);
        )*
    };

    (@impl $s:ident::<$T:ident>::$trait:ident) => {
        $crate::fmt_wrapper!(@impl #[field(0)] $s::<$T>::$trait);
    };
    (@impl #[field($field:tt)] $s:ident::<$T:ident>::$trait:ident) => {
        impl<$T> ::core::fmt::$trait for $s<$T>
        where
        $T: ::core::fmt::$trait
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::$trait::fmt(
                    &self.$field,
                    f,
                )
            }
        }
    };
}

#[macro_export]
macro_rules! wrapper {
    ($($S:ident($vis:vis $T:ident) $(where $($rest:tt)*)?;),* $(,)?) => {
        $(
            $crate::wrapper!(@impl $S($vis $T) $(where $($rest)*)?;);
        )*
    };
    (@impl
        #[derive($($derive:ident),*)]
        $S:ident($vis:vis $T:ident) $(where $($rest:tt)*)?;
    ) => {
        #[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd, $($derive),*)]
        #[cfg_attr(
            feature = "serde",
            derive(serde::Deserialize, serde::Serialize),
            serde(default, transparent),
        )]
        #[repr(transparent)]
        pub struct $S<$T>($vis $T) $(where $($rest)*)?;

        impl<$T> $S<$T> {
            /// returns a new instance initialized with the default value
            pub fn new() -> Self
            where
                $T: Default,
            {
                Self($T::default())
            }
            /// returns a new instance with the given value
            pub fn from_value(value: $T) -> Self {
                Self(value)
            }
            /// returns an immutable reference to the inner value
            pub const fn get(&self) -> &$T {
                &self.0
            }
            /// returns a mutable reference to the inner value
            pub const fn get_mut(&mut self) -> &mut $T {
                &mut self.0
            }
            /// consumes the current instance to return the inner value
            pub fn into_inner(self) -> $T {
                self.0
            }
            /// applies the given function to the inner value and returns a new instance with
            /// the result
            pub fn map<R, F>(self, f: F) -> $S<R>
            where
                F: FnOnce($T) -> R,
            {
                $S(f(self.0))
            }
            /// uses the [`replace`](core::mem::replace) method to update and return the inner value
            pub fn replace(&mut self, value: $T) -> $T {
                core::mem::replace(self.get_mut(), value)
            }
            /// update the innerstate before returing a mutable reference to the wrapper
            pub fn set(&mut self, value: $T) -> &mut Self {
                *self.get_mut() = value;
                self
            }
            /// uses the [`take`](core::mem::take) method to replace the inner value with the default
            /// value to return its previous value
            pub fn take(&mut self) -> $T
            where
                $T: Default,
            {
                core::mem::take(self.get_mut())
            }
            /// consumes the current instance to create another with the given value
            pub fn with(self, value: $T) -> Self {
                Self(value)
            }
            /// captures a referenced value in a new instance
            pub fn view(&self) -> $S<&$T> {
                $S(self.get())
            }
            /// captures a mutable reference to the inner value
            pub fn view_mut(&mut self) -> $S<&mut $T> {
                $S(self.get_mut())
            }
        }

        impl<$T> AsRef<$T> for $S<$T> {
            fn as_ref(&self) -> &$T {
                self.get()
            }
        }

        impl<$T> AsMut<$T> for $S<$T> {
            fn as_mut(&mut self) -> &mut $T {
                self.get_mut()
            }
        }

        impl<$T> ::core::borrow::Borrow<$T> for $S<$T> {
            fn borrow(&self) -> &$T {
                self.get()
            }
        }

        impl<$T> ::core::borrow::BorrowMut<$T> for $S<$T> {
            fn borrow_mut(&mut self) -> &mut $T {
                self.get_mut()
            }
        }

        impl<$T> ::core::ops::Deref for $S<$T> {
            type Target = $T;

            fn deref(&self) -> &Self::Target {
                self.get()
            }
        }

        impl<$T> ::core::ops::DerefMut for $S<$T> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.get_mut()
            }
        }

        impl<$T> From<$T> for $S<$T> {
            fn from(value: $T) -> Self {
                Self(value)
            }
        }
    };
}
