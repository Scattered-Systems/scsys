/*
    Appellation: specs <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{addressable::*, misc::*};

pub(crate) mod addressable;

pub(crate) mod misc {
    use crate::components::identities::Appellation;

    pub trait ActorSpec<T> {
        fn appellation(&self) -> Appellation<T>;
        fn justification(&self) -> serde_json::Value;
    }

    pub trait Named {
        fn name() -> String;
    }
}
