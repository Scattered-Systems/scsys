/// [IntoInner] is typically used for basic structures that wrap a single value.
pub trait IntoInner {
    type Inner;

    fn into_inner(self) -> Self::Inner;
}

pub trait Wrapper {
    type Inner;

    fn into_inner(self) -> Self::Inner;

    fn as_inner(&self) -> &Self::Inner;

    fn as_inner_mut(&mut self) -> &mut Self::Inner;
}

pub trait WrapperExt<U> {
    type Item<V>;

    fn map<V, F>(self, f: F) -> Self::Item<V>
    where
        F: FnOnce(U) -> V;
}
