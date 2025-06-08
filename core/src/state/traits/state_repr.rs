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

macro_rules! impl_state_repr {
    (@impl $($t:ident)::*<$T:ident>) => {        
        impl<$T> $crate::state::RawState for $($t)::*<$T>
        where
            $T: $crate::state::RawState,
        {
            seal!();
        }

        impl<$T> $crate::state::StateRepr for $($t)::*<$T>
        where
            $T: $crate::state::RawState,
        {
            type Item = $T;

            seal!();
        }
    };
    (
        $(
            $($t:ident)::*<$T:ident>
        ),* $(,)?
    ) => {
        $(
            impl_state_repr!(@impl $($t)::*<$T>);
        )*
    };
}

impl_state_repr! {
    Option<Q>,
    core::mem::MaybeUninit<Q>,
    core::marker::PhantomData<Q>,
}

#[cfg(feature = "alloc")]
impl_state_repr! {
    alloc::sync::Arc<Q>,
    alloc::boxed::Box<Q>,
    alloc::rc::Rc<Q>,
    alloc::vec::Vec<Q>,
}

#[cfg(feature = "std")]
impl_state_repr! {
    std::cell::Cell<Q>,
    std::sync::Mutex<Q>,
}
