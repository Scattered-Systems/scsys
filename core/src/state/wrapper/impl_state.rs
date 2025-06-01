/*
    Appellation: impl_state <module>
    Contrib: @FL03
*/
use crate::state::State;
use core::mem::MaybeUninit;

impl<'a, T> State<&'a T> {}

impl<'a, T> State<&'a mut T> {}

impl<T> State<*const T> {}

impl<T> State<*mut T> {}

impl<T> State<MaybeUninit<T>> {
    /// returns a new state with an [`uninitialized`](MaybeUninit::uninit) inner state
    pub fn uninit() -> State<MaybeUninit<T>> {
        State(MaybeUninit::uninit())
    }
    /// returns a new state with an [`initialized`](MaybeUninit::new) inner state
    pub fn init(value: T) -> State<MaybeUninit<T>> {
        State(MaybeUninit::new(value))
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
    /// returns true if the inner state is [`None`](Option::None)
    pub fn is_none(&self) -> bool {
        self.get().is_none()
    }
    /// returns true if the inner state is [`Some`](Option::Some)
    pub fn is_some(&self) -> bool {
        self.get().is_some()
    }
}

impl<Q> AsRef<Q> for State<Q> {
    fn as_ref(&self) -> &Q {
        &self.0
    }
}

impl<Q> AsMut<Q> for State<Q> {
    fn as_mut(&mut self) -> &mut Q {
        &mut self.0
    }
}

impl<Q> core::borrow::Borrow<Q> for State<Q> {
    fn borrow(&self) -> &Q {
        &self.0
    }
}

impl<Q> core::borrow::BorrowMut<Q> for State<Q> {
    fn borrow_mut(&mut self) -> &mut Q {
        &mut self.0
    }
}

impl<Q> core::ops::Deref for State<Q> {
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Q> core::ops::DerefMut for State<Q> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<Q> From<Q> for State<Q> {
    fn from(from: Q) -> Self {
        State(from)
    }
}

impl<Q> PartialEq<Q> for State<Q>
where
    Q: PartialEq,
{
    fn eq(&self, other: &Q) -> bool {
        self.get() == other
    }
}

impl<Q> PartialOrd<Q> for State<Q>
where
    Q: PartialOrd,
{
    fn partial_cmp(&self, other: &Q) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

fmt_wrapper! {
    State<Q>(Binary, Debug, Display, LowerExp, UpperExp, LowerHex, UpperHex, Octal, Pointer)
}
