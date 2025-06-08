/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::RawStateKind;
use core::marker::PhantomData;

/// [`StateBase`] is an abstract object that allows a particular _kind_ of state to be
/// associated with some generic state `Q`
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default)
)]
pub struct StateBase<Q, K> {
    pub(crate) state: Q,
    pub(crate) _kind: PhantomData<K>,
}

impl<K, Q> StateBase<Q, K>
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
