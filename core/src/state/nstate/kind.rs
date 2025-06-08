/*
    Appellation: kind <module>
    Contrib: @FL03
*/
use super::NState;

/// a private, base represenation of a state kind
pub trait RawStateKind {
    private!();
}
///
///
/// **note:** this trait is auto implemented for types that implement [`AsRef<str>`].
pub trait StateKind: RawStateKind {
    fn kind(&self) -> &str;
}
/// a particular kind of state that is defined by some _rank_
pub trait NStateKind: StateKind {
    /// the "rank" of the state speaks to the total number of dimensions (or states) allowed
    const RANK: usize;
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Nary<const N: usize> {}
impl<T> RawStateKind for T
where
    T: AsRef<str>,
{
    seal!();
}

impl<T> StateKind for T
where
    T: AsRef<str>,
{
    fn kind(&self) -> &str {
        self.as_ref()
    }
}

macro_rules! impl_state_kind {
    (@kind $n:literal) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[cfg_attr(feature = "serde", derive(serde_derive::Deserialize, serde_derive::Serialize))]
            pub enum [<State $n>] {}

            impl RawStateKind for [<State $n>] {
                seal!();
            }

            impl StateKind for [<State $n>] {
                fn kind(&self) -> &str {
                    stringify!([<State $n>])
                }
            }

            impl NStateKind for [<State $n>] {
                const RANK: usize = $n;
            }
        }
    };
    (@state $name:ident($n:literal)) => {
        paste::paste! {
            pub type [<$name State>]<Q> = NState<Q, [<State $n>]>;
        }
    };
    ($($name:ident($n:literal)),* $(,)?) => {
        $(
            impl_state_kind!(@kind $n);
            impl_state_kind!(@state $name($n));
        )*
    };
}

impl_state_kind! {
    Unary(1),
    Binary(2),
    Ternary(3),
    Quaternary(4),
    Quinary(5),
    Senary(6),
    Septenary(7),
    Octonary(8),
    Nonary(9),
    Denary(10),
}
