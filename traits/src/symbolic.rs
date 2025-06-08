/*
    Appellation: symbolic <module>
    Contrib: @FL03
*/
/// [`Displayable`] simply combines the two major traits [`core::fmt::Debug`] and
/// [`core::fmt::Display`] into a single trait. This is useful for types that need to be
/// displayed in a human-readable format, such as in debugging or logging.
///
/// **Note:** This trait is sealed, meaning it cannot be implemented outside of this crate,
/// however, it is automatically implemented for any type that implements both
/// [`core::fmt::Debug`] & [`core::fmt::Display`].
pub trait Displayable: core::fmt::Debug + core::fmt::Display {
    private!();
}

/// [`Symbolic`] denotes a type consisting of some single or set of value(s) that can are
/// considered displayable. Essentially, this trait is a wrapper around the [`Displayable`]
/// trait enabling additional implementations
pub trait Symbolic {
    type Item: Displayable;
}

macro_rules! impl_symbolic {
    (<$tag:ident>: [$($t:ty),* $(,)?]) => {
        $(
            impl_symbolic!(@impl <$tag>: $t );
        )*
    };
    (@impl <Self>: $t:ty ) => {
        impl Symbolic for $t {
            type Item = Self;
        }
    };
    (@impl <$T:ident>: $t:ty ) => {
        impl<$T> Symbolic for $t
        where
            $T: Displayable,
        {
            type Item = $T;
        }
    };
}

/*
 ************* Implementations *************
*/

impl<T> Displayable for T
where
    T: core::fmt::Debug + core::fmt::Display,
{
    seal!();
}

impl Symbolic for &str {
    type Item = Self;
}

impl_symbolic! {
    <Self>: [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, char, bool]
}

#[cfg(feature = "alloc")]
impl_symbolic! {
    <Self>: [
        alloc::string::String,
    ]
}

#[cfg(feature = "alloc")]
impl_symbolic! {
    <T>: [
        alloc::boxed::Box<T>,
        alloc::rc::Rc<T>,
        alloc::sync::Arc<T>,
        alloc::vec::Vec<T>,
    ]
}
