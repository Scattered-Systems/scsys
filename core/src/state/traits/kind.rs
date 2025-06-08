/*
    appellation: kinds <module>
    authors: @FL03
*/
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

/*
 ************* Implementations *************
*/

impl<T> StateKind for T
where
    T: RawStateKind + AsRef<str>,
{
    fn kind(&self) -> &str {
        self.as_ref()
    }
}
