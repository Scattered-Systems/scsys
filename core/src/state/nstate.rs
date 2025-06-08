/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::kind::*;

mod kind;

use core::marker::PhantomData;

/// A type alias for a [Nary] state with a default value of 4.
pub type NaryState<T, const N: usize = 4> = NState<T, Nary<N>>;

/// [State] is an abstract object that allows a particular _kind_ of state to be associated
/// with some data.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default)
)]
pub struct NState<Q, K> {
    pub(crate) data: Q,
    pub(crate) _kind: PhantomData<K>,
}

impl<K, Q> NState<Q, K> {
    pub fn new(data: Q) -> Self {
        Self {
            data,
            _kind: PhantomData::<K>,
        }
    }

    pub const fn data(&self) -> &Q {
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
