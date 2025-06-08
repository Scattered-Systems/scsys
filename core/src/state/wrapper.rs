/*
    appellation: wrapper <module>
    authors: @FL03
*/

mod impl_ops;
mod impl_state;

/// [`State`] is a generic type wrapper materializing the [`RawState`] trait.
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, transparent)
)]
#[repr(transparent)]
pub struct State<Q = usize>(pub Q);

/// generic implementation of the [`State`] type available for all types `Q`
impl<Q> State<Q> {
    /// returns a new instance of [`State`] with the default value
    pub fn new() -> Self
    where
        Q: Default,
    {
        State(Q::default())
    }
    /// returns a new instance of [`State`] with the given value
    pub fn from_value(state: Q) -> Self {
        State(state)
    }
    /// generate a new instance of the state with a random value
    #[cfg(feature = "rand")]
    pub fn random() -> Self
    where
        rand_distr::StandardUniform: rand_distr::Distribution<Q>,
    {
        use rand::Rng;
        let mut rng = rand::rng();
        Self::from_value(rng.random())
    }
    #[cfg(feature = "rand")]
    pub fn random_with<R, Dist>(rng: &mut R, distr: Dist) -> Self
    where
        Dist: rand_distr::Distribution<Q>,
        R: rand::RngCore,
    {
        use rand::Rng;
        // sample a value using the given rng configured with the given distribution
        let value = rng.sample(distr);
        // return a new instance of the state with the sampled value
        State(value)
    }
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &Q {
        &self.0
    }
    /// returns a mutable reference to the inner value
    pub const fn get_mut(&mut self) -> &mut Q {
        &mut self.0
    }
    /// consumes the current instance to return the inner value
    #[inline]
    pub fn value(self) -> Q {
        self.0
    }
    #[deprecated(
        since = "0.2.9",
        note = "use `value` instead, this will be removed in the next major release"
    )]
    pub fn into_inner(self) -> Q {
        self.0
    }
    /// [`replace`](core::mem::replace) the inner state with the given value and return the
    /// previous
    pub const fn replace(&mut self, state: Q) -> Q {
        core::mem::replace(self.get_mut(), state)
    }
    /// mutate the inner value and return a mutable reference to the wrapper for chaining
    pub fn set(&mut self, state: Q) -> &mut Self {
        *self.get_mut() = state;
        self
    }
    /// [`swap`](core::mem::swap) the inner value with that of another state instance of the
    /// same type `Q`
    pub const fn swap(&mut self, state: &mut State<Q>) {
        core::mem::swap(self.get_mut(), state.get_mut())
    }
    /// takes the inner value and replaces it with the default value
    pub fn take(&mut self) -> Q
    where
        Q: Default,
    {
        core::mem::take(self.get_mut())
    }
    /// consumes the current instance to replace it with another.
    pub fn with<U>(self, id: U) -> State<U> {
        State(id)
    }
    /// apply a function onto the inner value and return a new instance with the result
    pub fn map<U, F>(self, f: F) -> State<U>
    where
        F: FnOnce(Q) -> U,
    {
        State(f(self.0))
    }
    /// updates the inner value using the given function and returns a mutable reference to the
    /// current instance for chaining
    pub fn map_inplace<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Q),
    {
        f(self.get_mut());
        self
    }
    /// returns a new instance containing a reference to the inner value
    pub const fn view(&self) -> State<&Q> {
        State(self.get())
    }
    /// returns a new instance containing a mutable reference to the inner value
    pub const fn view_mut(&mut self) -> State<&mut Q> {
        State(self.get_mut())
    }
}
