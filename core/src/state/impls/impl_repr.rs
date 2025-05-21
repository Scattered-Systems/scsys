/*
    Appellation: impl_repr <module>
    Contrib: @FL03
*/
use crate::state::State;

impl<'a, T> State<&'a T> {
    /// returns a new instance containing a clone of the inner value
    pub fn cloned(&self) -> State<T>
    where
        T: Clone,
    {
        State(self.0.clone())
    }
    /// returns a new instance containing a copy of the inner value
    pub fn copied(&self) -> State<T>
    where
        T: Copy,
    {
        State(*self.0)
    }
}

impl<'a, T> State<&'a mut T> {
    /// returns a new instance containing a clone of the inner value
    pub fn cloned(&self) -> State<T>
    where
        T: Clone,
    {
        State(self.0.clone())
    }
    /// returns a new instance containing a copy of the inner value
    pub fn copied(&self) -> State<T>
    where
        T: Copy,
    {
        State(*self.0)
    }
}

impl<T> State<*const T> {}

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
