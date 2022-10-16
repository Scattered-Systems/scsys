/*
    Appellation: actor <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use crate::components::identities::Appellation;

pub trait ActorSpec {
    fn appellation(&self) -> Appellation;
    fn justification(&self) -> serde_json::Value;
}
