/*
    Appellation: atomic <mod>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
/// A generic identifier
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, transparent)
)]
#[repr(transparent)]
pub struct State<Q = usize>(pub Q);

impl<Q> State<Q> {
    /// Create a new identifier with the default value
    pub fn new() -> Self
    where
        Q: Default,
    {
        Self(Q::default())
    }
    /// create a new identifier from the given value
    pub fn from_value(id: Q) -> Self {
        Self(id)
    }
    #[cfg(feature = "rand")]
    pub fn random() -> Self
    where
        rand_distr::StandardUniform: rand_distr::Distribution<Q>,
    {
        use rand::Rng;
        let mut rng = rand::rng();
        Self::from_value(rng.random())
    }
    /// returns an immutable reference to the inner value
    pub const fn get(&self) -> &Q {
        &self.0
    }
    /// returns a mutable reference to the inner value
    pub fn get_mut(&mut self) -> &mut Q {
        &mut self.0
    }
    /// consumes the current instance to return the inner value
    pub fn into_inner(self) -> Q {
        self.0
    }
    /// use the [`replace`](core::mem::replace) method to update and return the inner value
    pub fn replace(&mut self, id: Q) -> Q {
        core::mem::replace(self.get_mut(), id)
    }
    /// mutate the inner value and return a mutable reference to the wrapper for chaining
    pub fn set(&mut self, id: Q) -> &mut Self {
        *self.get_mut() = id;
        self
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

    pub fn to_owned(self) -> State<Q>
    where
        Q: Clone,
    {
        State(self.0.clone())
    }
    /// returns a new instance containing a reference to the inner value
    pub const fn view(&self) -> State<&Q> {
        State(&self.0)
    }
    /// returns a new instance containing a mutable reference to the inner value
    pub const fn view_mut(&mut self) -> State<&mut Q> {
        State(&mut self.0)
    }
}

impl<Q> super::RawState for State<Q> {
    type Item = Q;

    seal!();
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

fmt_wrapper! {
    State<Q> {
        Binary("{:b}"),
        Debug("{:?}"),
        Display("{}"),
        LowerExp("{:e}"),
        LowerHex("{:x}"),
        Octal("{:o}"),
        UpperExp("{:E}"),
        UpperHex("{:X}")
    }
}
