/*
    Appellation: kind <module>
    Contrib: @FL03
*/

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

            impl AsRef<str> for [<State $n>] {
                fn as_ref(&self) -> &str {
                    stringify!([<State $n>])
                }
            }

            impl $crate::state::RawStateKind for [<State $n>] {
                seal!();
            }

            impl $crate::state::NStateKind for [<State $n>] {
                const RANK: usize = $n;
            }
        }
    };
    (@state $name:ident($n:literal)) => {
        paste::paste! {
            pub type [<$name State>]<Q> = $crate::state::StateBase<Q, [<State $n>]>;
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
