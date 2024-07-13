/*
    Appellation: traits <states>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::borrow::Borrow;

pub trait StateKind {}

pub trait State<Q>
where
    Q: StateKind,
{
    type Kind: Borrow<Q>;
}

pub trait StateData {
    type Item: ?Sized;
}

/// [Stateful] describes a stateful object
pub trait Stateful<Q>
where
    Q: StateKind,
{
    type State: State<Q>;

    /// [Stateful::state] is used to get the state of the object
    fn state(&self) -> &Self::State;
    /// [Stateful::update_state] is used to update the state of the object
    fn update_state(&mut self, state: Self::State);
}

/*
 ******** implementations ********
*/

impl<Q, S> State<Q> for S
where
    Q: StateKind,
    S: Borrow<Q>,
{
    type Kind = S;
}
