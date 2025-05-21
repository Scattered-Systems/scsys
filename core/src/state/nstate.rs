/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::kind::*;

mod kind;

use core::marker::PhantomData;

/// A type alias for a [Nary] state with a default value of 4.
pub type NaryState<T, const N: usize = 4> = NState<Nary<N>, T>;

/// [State] is an abstract object that allows a particular _kind_ of state to be associated
/// with some data.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default)
)]
pub struct NState<K, V> {
    pub(crate) data: V,
    pub(crate) _state: PhantomData<K>,
}

impl<K, V> NState<K, V> {
    pub fn new(data: V) -> Self {
        Self {
            data,
            _state: PhantomData::<K>,
        }
    }

    pub fn data(&self) -> &V {
        &self.data
    }

    pub fn is_state<R: 'static>(&self) -> bool
    where
        K: 'static,
    {
        use core::any::TypeId;
        TypeId::of::<PhantomData<K>>() == TypeId::of::<PhantomData<R>>()
    }
}
