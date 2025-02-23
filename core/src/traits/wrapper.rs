/*
    Appellation: wrapper <module>
    Contrib: @FL03
*/

/// [IntoInner] is typically used for basic structures that wrap a single value.
pub trait IntoInner {
    type Inner;

    fn into_inner(self) -> Self::Inner;
}

/// Interface for nameable data-structures
pub trait Name {
    fn name(&self) -> &str;
}

pub trait Wrapper {
    type Inner;

    fn into_inner(self) -> Self::Inner;

    fn as_inner(&self) -> &Self::Inner;

    fn as_inner_mut(&mut self) -> &mut Self::Inner;

}

pub trait Mapper {
    type Item;
    type Cont<T>: Mapper<Item = T>;

    fn map<U, F>(self, f: F) -> Self::Cont<U> where F: FnOnce(Self::Item) -> U;
}