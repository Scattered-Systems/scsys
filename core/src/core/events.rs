/*
    Appellation: events <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

/// Encapsulates the availible events for the ecosystem
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    EnumString,
    EnumVariantNames,
    Eq,
    Hash,
    PartialEq,
    Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Event {
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
    #[default]
    Event,
    Hashing,
    Parsing,
    Passing,
    Quitting,
    Syncing,
}

