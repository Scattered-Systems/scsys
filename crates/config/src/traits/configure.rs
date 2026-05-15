/*
    Appellation: configure <module>
    Contrib: @FL03
*/
/// The [`RawConfiguration`] trait defines the base representation format for all
/// configuration schemas.
pub trait RawConfiguration {}

/// [`Configuration`] is used to establish a common interface for all configuration types.
pub trait Configuration: RawConfiguration {}

/// [`Configurable`] trait for types that can be configured with another type, denoted by
/// [Configurable::Config].
pub trait Configurable {
    type Config;

    fn config(&self) -> &Self::Config;
}
