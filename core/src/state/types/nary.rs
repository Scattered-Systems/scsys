/*
    appellation: nary <module>
    authors: @FL03
*/
use crate::state::{NStateKind, RawStateKind};

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
