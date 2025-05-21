/*
    Appellation: concat <module>
    Contrib: @FL03
*/

/// this trait defines a common method for concatenating two entities into some new type.
pub trait Concat<Rhs = Self> {
    type Output;
    /// Concatenate two slices into a new vector.
    fn concat(&self, other: Rhs) -> Self::Output;
}
