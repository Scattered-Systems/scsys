/*
    Appellation: impl_state <module>
    Contrib: @FL03
*/
use crate::state::{RawState, State};
use core::mem::MaybeUninit;

impl<'a, Q> State<&'a Q>
where
    Q: RawState,
{
    /// returns a new state with a cloned inner value
    pub fn cloned(&self) -> State<Q>
    where
        Q: Clone,
    {
        State(self.0.clone())
    }
    /// returns a new state with the inner value copied
    pub fn copied(&self) -> State<Q>
    where
        Q: Copy,
    {
        State(*self.0)
    }
}

impl<'a, Q> State<&'a mut Q>
where
    Q: RawState,
{
    /// returns a new state with a cloned inner value
    pub fn cloned(&self) -> State<Q>
    where
        Q: Clone,
    {
        State(self.0.clone())
    }
    /// returns a new state with the inner value copied
    pub fn copied(&self) -> State<Q>
    where
        Q: Copy,
    {
        State(*self.0)
    }
}

impl<Q> State<*const Q> where Q: RawState {}

impl<Q> State<*mut Q> where Q: RawState {}

impl<Q> State<MaybeUninit<Q>>
where
    Q: RawState,
{
    /// returns a new state with an [`uninitialized`](MaybeUninit::uninit) inner state
    pub fn uninit() -> State<MaybeUninit<Q>> {
        State(MaybeUninit::uninit())
    }
    /// returns a new state with an [`initialized`](MaybeUninit::new) inner state
    pub fn init(value: Q) -> State<MaybeUninit<Q>> {
        State(MaybeUninit::new(value))
    }
}

impl<Q> State<Option<Q>>
where
    Q: RawState,
{
    /// returns a new state with a [`None`](Option::None) inner state
    pub fn none() -> State<Option<Q>> {
        State(None)
    }
    /// returns a new instance with some inner state
    pub fn some(value: Q) -> State<Option<Q>> {
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

impl<Q> AsRef<Q> for State<Q>
where
    Q: RawState,
{
    fn as_ref(&self) -> &Q {
        self.get()
    }
}

impl<Q> AsMut<Q> for State<Q>
where
    Q: RawState,
{
    fn as_mut(&mut self) -> &mut Q {
        self.get_mut()
    }
}

impl<Q> core::borrow::Borrow<Q> for State<Q>
where
    Q: RawState,
{
    fn borrow(&self) -> &Q {
        self.get()
    }
}

impl<Q> core::borrow::BorrowMut<Q> for State<Q>
where
    Q: RawState,
{
    fn borrow_mut(&mut self) -> &mut Q {
        self.get_mut()
    }
}

impl<Q> core::ops::Deref for State<Q>
where
    Q: RawState,
{
    type Target = Q;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<Q> core::ops::DerefMut for State<Q>
where
    Q: RawState,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<Q> From<Q> for State<Q>
where
    Q: RawState,
{
    fn from(from: Q) -> Self {
        State(from)
    }
}

impl<Q> PartialEq<Q> for State<Q>
where
    Q: RawState + PartialEq,
{
    fn eq(&self, other: &Q) -> bool {
        self.get() == other
    }
}

impl<'a, Q> PartialEq<&'a Q> for State<Q>
where
    Q: RawState + PartialEq,
{
    fn eq(&self, other: &&'a Q) -> bool {
        self.get() == *other
    }
}

impl<'a, Q> PartialEq<&'a mut Q> for State<Q>
where
    Q: RawState + PartialEq,
{
    fn eq(&self, other: &&'a mut Q) -> bool {
        *self.get() == **other
    }
}

impl<Q> PartialOrd<Q> for State<Q>
where
    Q: RawState + PartialOrd,
{
    fn partial_cmp(&self, other: &Q) -> Option<core::cmp::Ordering> {
        self.get().partial_cmp(other)
    }
}

fmt_wrapper! {
    State<Q>(Binary, Debug, Display, LowerExp, UpperExp, LowerHex, UpperHex, Octal, Pointer)
}
