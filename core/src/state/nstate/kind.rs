/*
    Appellation: kind <module>
    Contrib: @FL03
*/
use super::NState;

/// a private, base represenation of a state kind
pub trait RawStateKind: 'static + Send + Sync + core::fmt::Debug + core::fmt::Display {
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

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Nary<const N: usize>;

unsafe impl<const N: usize> Send for Nary<N> {}

unsafe impl<const N: usize> Sync for Nary<N> {}

impl<const N: usize> core::fmt::Display for Nary<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Nary({})", N)
    }
}

impl<const N: usize> RawStateKind for Nary<N> {
    seal!();
}

impl<const N: usize> AsRef<str> for Nary<N> {
    fn as_ref(&self) -> &str {
        stringify!(Nary)
    }
}

impl<const N: usize> NStateKind for Nary<N> {
    const RANK: usize = N;
}

impl<T> StateKind for T
where
    T: RawStateKind + AsRef<str>,
{
    fn kind(&self) -> &str {
        self.as_ref()
    }
}

macro_rules! impl_state_kind {
    (@kind $n:literal) => {
        paste::paste! {
            #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
            #[cfg_attr(
                feature = "serde", 
                derive(serde_derive::Deserialize, serde_derive::Serialize)
            )]
            pub enum [<State $n>] {}

            unsafe impl Send for [<State $n>] {}

            unsafe impl Sync for [<State $n>] {}

            impl ::core::fmt::Display for [<State $n>] {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    write!(f, "State({})", $n)
                }
            }

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
