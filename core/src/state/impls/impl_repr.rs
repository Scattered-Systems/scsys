/*
    Appellation: impl_repr <module>
    Contrib: @FL03
*/
use crate::state::State;

impl<'a, T> State<&'a T> {

}

impl<'a, T> State<&'a mut T> {

}

impl<T> State<*const T> {}

impl<T> State<*mut T> {}

impl<T> State<core::mem::MaybeUninit<T>> {
    /// returns a new instance containing a reference to the inner value
    pub fn unit() -> State<core::mem::MaybeUninit<T>> {
        State(core::mem::MaybeUninit::uninit())
    }
}

impl<T> State<Option<T>> {
    /// returns a new state with a [`None`](Option::None) inner state
    pub fn none() -> State<Option<T>> {
        State(None)
    }
    /// returns a new instance with some inner state
    pub fn some(value: T) -> State<Option<T>> {
        State(Some(value))
    }

    pub fn is_none(&self) -> bool {
        self.get().is_none()
    }
}
