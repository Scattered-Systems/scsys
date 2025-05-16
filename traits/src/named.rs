/*
    Appellation: named <module>
    Contrib: @FL03
*/
/// Interface for nameable data-structures
pub trait Named {
    fn name(&self) -> &str;
}

pub trait Appellation {
    fn appellation(&self) -> &str;
}
