/*
    Appellation: container <module>
    Contrib: @FL03
*/
use super::GetMut;
/// [`RawContainer`] defines a standard interface for all _containers_ that are used to store
/// other entities.
///
/// ## **Safety**
///
/// This trait is marked as `unsafe` because it is expected that the implementer will
/// ensure that the `Item` type is valid for the container type. For example, if the container
/// is a `Vec<T>`, then the `Item` type must be `T`. If the implementer does not ensure this,
/// then it is possible to create a container that is not valid for the `Item` type.
pub unsafe trait RawContainer {
    type Item;
}

pub trait KeyedContainer<T>: Container<T>
where
    Self::Cont<T>: GetMut<T, Key = Self::Key>,
{
    type Key;
}

/// The [`Container`] trait is a higher-level abstraction over [`RawContainer`].
pub trait Container<T> {
    type Cont<V>: RawContainer<Item = V> + ?Sized;
}

/*
 ************* Implementations *************
*/
unsafe impl<T> RawContainer for [T] {
    type Item = T;
}

unsafe impl<T> RawContainer for &[T] {
    type Item = T;
}

unsafe impl<T> RawContainer for &mut [T] {
    type Item = T;
}

impl<T> Container<T> for [T] {
    type Cont<V> = [V];
}

macro_rules! impl_container {
    ($($($t:ident)::*<$T:ident>),* $(,)?) => {
        $(
            impl_container!(@impl $($t)::*<$T>);
        )*
    };
    (@impl $($t:ident)::*<$T:ident>) => {
        impl_container!(@raw $($t)::*<$T>);
        impl_container!(@cont $($t)::*<$T>);
    };
    (@raw $($t:ident)::*<$lt:lifetime, $T:ident>) => {
        unsafe impl<$T> RawContainer for $($t)::*<$lt, $T> {
            type Item = $T;
        }
    };
    (@raw $($t:ident)::*<$T:ident>) => {
        unsafe impl<$T> RawContainer for $($t)::*<$T> {
            type Item = $T;
        }
    };
    (@cont $($t:ident)::*<$T:ident>) => {
        impl<$T> Container<$T> for $($t)::*<$T> {
            type Cont<DType_> = $($t)::*<DType_>;
        }
    };
}

#[cfg(feature = "alloc")]
impl_container! {
    alloc::vec::Vec<T>,
    alloc::boxed::Box<T>,
    alloc::rc::Rc<T>,
    alloc::sync::Arc<T>,
    alloc::collections::BTreeSet<T>,
    alloc::collections::LinkedList<T>,
}

#[cfg(feature = "std")]
impl_container! {
    std::collections::HashSet<V>,
}
