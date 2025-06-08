/*
    Appellation: container <module>
    Contrib: @FL03
*/
use super::GetMut;

/// [`RawContainer`] defines a standard interface for all _containers_ that are used to store
/// other entities.
pub trait RawContainer {
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
impl<T> RawContainer for [T] {
    type Item = T;
}

impl<T> RawContainer for &[T] {
    type Item = T;
}

impl<T> RawContainer for &mut [T] {
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
        impl<$T> RawContainer for $($t)::*<$lt, $T> {
            type Item = $T;
        }
    };
    (@raw $($t:ident)::*<$T:ident>) => {
        impl<$T> RawContainer for $($t)::*<$T> {
            type Item = $T;
        }
    };
    (@cont $($t:ident)::*<$T:ident>) => {
        impl<$T> Container<$T> for $($t)::*<$T> {
            type Cont<DType_> = $($t)::*<DType_>;
        }
    };
}

impl_container! {
    Option<T>
}

#[cfg(feature = "alloc")]
impl_container! {
    alloc::vec::Vec<T>,
    alloc::boxed::Box<T>,
    alloc::rc::Rc<T>,
    alloc::rc::Weak<T>,
    alloc::sync::Arc<T>,
    alloc::collections::BinaryHeap<T>,
    alloc::collections::BTreeSet<T>,
    alloc::collections::LinkedList<T>,
    alloc::collections::VecDeque<T>,
}

#[cfg(feature = "std")]
impl_container! {
    std::cell::Cell<T>,
    std::cell::OnceCell<T>,
    std::cell::RefCell<T>,
    std::sync::Mutex<T>,
    std::sync::RwLock<T>,
    std::sync::LazyLock<T>,
    std::collections::HashSet<V>,
}
