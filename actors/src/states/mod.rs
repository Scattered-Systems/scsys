/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// # States
pub use self::state::*;

pub(crate) mod state;

use std::ops::MulAssign;
use std::sync::{Arc, Mutex};

/// [AsyncStateful] describes an async stateful object
pub trait AsyncStateful<S: StateSpec> {
    fn state(&self) -> Arc<Mutex<S>>;
    fn update_state(&mut self, state: Arc<Mutex<S>>);
}

/// [Stateful] describes a stateful object
pub trait Stateful<S: StateSpec> {
    /// [Stateful::state] is used to get the state of the object
    fn state(&self) -> S;
    /// [Stateful::update_state] is used to update the state of the object
    fn update_state(&mut self, state: S);
}

impl<Q> Stateful<Q> for Q
where
    Q: StateSpec,
{
    fn state(&self) -> Q {
        *self
    }
    fn update_state(&mut self, state: Q) {
        *self = state;
    }
}

/// [StateSpec] is used by [Stateful] to describe a specific state
pub trait StateSpec: Copy + Default + Eq + Ord + ToString {}

pub trait StateSpecExt: MulAssign + StateSpec {}

impl<T> StateSpec for T where T: Copy + Default + Eq + Ord + ToString + MulAssign {}
