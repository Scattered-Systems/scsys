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
    pub(crate) state: Q,
    pub(crate) _kind: PhantomData<K>,
}

impl<K, Q> NState<Q, K>
where
    K: RawStateKind,
{
    pub fn new(state: Q) -> Self {
        Self {
            state,
            _kind: PhantomData::<K>,
        }
    }
    /// returns a reference to the state
    pub const fn get(&self) -> &Q {
        &self.state
    }
    /// returns a mutable reference to the state
    pub const fn get_mut(&mut self) -> &mut Q {
        &mut self.state
    }
    /// returns true if the state is of the specified kind
    pub fn is_kind<R>(&self) -> bool
    where
        R: 'static,
    {
        use core::any::TypeId;
        TypeId::of::<K>() == TypeId::of::<R>()
    }
}
