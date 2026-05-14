/*
    Appellation: named <module>
    Contrib: @FL03
*/
/// Interface for nameable data-structures
pub trait Named {
    fn name(&self) -> &str;
}
/// [`Appellation`] defines a common interface for establishing a solid name for
/// various entities.
pub trait Appellation {
    fn appellation(&self) -> &str;
}
