/*
    appellation: stateful <module>
    authors: @FL03
*/
use super::{RawState, StateRepr};

/// [`Stateful`] is a trait establishing a common interface for all types that are state-aware.
pub trait Stateful<Q>
where
    Q: RawState,
{
    type State: StateRepr<Item = Q>;

    fn state(&self) -> &Self::State;

    fn state_mut(&mut self) -> &mut Self::State;

    fn set_state(&mut self, state: Self::State) -> &mut Self {
        *self.state_mut() = state;
        self
    }
    /// [`replace`](core::mem::replace) the current state with the given value and return the
    /// old state.
    fn replace_state(&mut self, state: Self::State) -> Self::State {
        core::mem::replace(self.state_mut(), state)
    }
    /// [`swap`](core::mem::swap) the current state with another of the same type.
    fn swap_state(&mut self, state: &mut Self::State) {
        core::mem::swap(self.state_mut(), state)
    }

    fn take_state(&mut self) -> Self::State
    where
        Self::State: Default,
    {
        core::mem::take(self.state_mut())
    }
}
