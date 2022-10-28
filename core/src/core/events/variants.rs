/*
    Appellation: variants <events>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

/// Encapsulates the availible events for the ecosystem
#[derive(
    Clone, Copy, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Events<T: std::string::ToString + std::default::Default = String> {
    Initializing,
    Aggregating,
    Attempting,
    Collecting,
    Constructing,
    Connecting,
    Counting,
    Deleting,
    Destroying,
    Diverging,
    Equating,
    GenericEvent(T),
    Hashing,
    Parsing,
    Passing,
    Quitting,
    Syncing,
}

impl<T: std::string::ToString + std::default::Default> Default for Events<T> {
    fn default() -> Self {
        Self::GenericEvent(T::default())
    }
}
