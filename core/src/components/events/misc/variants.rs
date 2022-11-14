/*
    Appellation: variants <events>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::events::{Event, Eventful};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

/// Encapsulates the availible events for the ecosystem
#[derive(
    Clone, Copy, Debug, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
#[strum(serialize_all = "snake_case")]
pub enum Events<T: Eventful = Event> {
    GenericEvent(T),
    Idle,
}

impl<T: Eventful> Default for Events<T> {
    fn default() -> Self {
        Self::Idle
    }
}

impl<T: Eventful> std::fmt::Display for Events<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
