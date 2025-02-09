/*
    Appellation: binary <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::State;

/// A type alias for a [Nary] state with a default value of 4.
pub type NState<T, const N: usize = 4> = State<Nary<N>, T>;

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
            pub type [<$name State>]<T> = State<[<State $n>], T>;
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
