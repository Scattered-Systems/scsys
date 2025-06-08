/*
    appellation: state_repr <module>
    authors: @FL03
*/
use super::RawState;

/// The [`StateRepr`] trait defines the base type for stateful items, allowing for a generic
/// item type to be associated with the state.
pub trait StateRepr {
    type Item: RawState;

    private!();
}

/*
 ************* Implementations *************
*/
use crate::state::{NState, State};

impl<Q> StateRepr for State<Q>
where
    Q: RawState,
{
    type Item = Q;

    seal!();
}

impl<Q, K> StateRepr for NState<Q, K>
where
    Q: RawState,
{
    type Item = Q;

    seal!();
}
