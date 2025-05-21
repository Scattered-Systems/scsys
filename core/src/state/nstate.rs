/*
    Appellation: state <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use core::marker::PhantomData;

/// A type alias for a [Nary] state with a default value of 4.
pub type NState<T, const N: usize = 4> = KState<Nary<N>, T>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Nary<const N: usize> {}

macro_rules! impl_state_kind {
    (@kind $n:literal) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde_derive::Deserialize, serde_derive::Serialize))]
            pub enum [<State $n>] {}
        }
    };
    (@state $name:ident($n:literal)) => {
        paste::paste! {
            pub type [<$name State>]<T> = KState<[<State $n>], T>;
        }
    };
    ($($name:ident($n:literal)),* $(,)?) => {
        $(
            impl_state_kind!(@kind $n);
            impl_state_kind!(@state $name($n));
        )*
    };
}

impl_state_kind!(Unary(1), Binary(2), Ternary(3));

/// [State] is an abstract object that allows a particular _kind_ of state to be associated
/// with some data.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default)
)]
pub struct KState<K, V> {
    pub(crate) data: V,
    pub(crate) _state: PhantomData<K>,
}

impl<K, V> KState<K, V> {
    pub fn new(data: V) -> Self {
        Self {
            data,
            _state: PhantomData::<K>,
        }
    }

    pub fn data(&self) -> &V {
        &self.data
    }

    pub fn is_state<R: 'static>(&self) -> bool
    where
        K: 'static,
    {
        use core::any::TypeId;
        TypeId::of::<PhantomData<K>>() == TypeId::of::<PhantomData<R>>()
    }
}
