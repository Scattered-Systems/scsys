/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use scsys::id::AtomicId;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(C)]
pub struct StateBase<S, T> {
    id: AtomicId,
    data: T,
    state: S,
}

impl<Q, T> StateBase<Q, T> {
    pub fn new(data: T, state: Q) -> Self {
        Self {
            id: AtomicId::new(),
            data,
            state,
        }
    }

    pub const fn id(&self) -> AtomicId {
        self.id
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn state(&self) -> &Q {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut Q {
        &mut self.state
    }
}

impl<Q, T> core::borrow::Borrow<Q> for StateBase<Q, T> {
    fn borrow(&self) -> &Q {
        &self.state
    }
}
