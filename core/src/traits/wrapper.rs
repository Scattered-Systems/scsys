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
