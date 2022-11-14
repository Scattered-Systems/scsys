/*
    Appellation: specs <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{addressable::*, configurable::*, contextual::*, justifiable::*, misc::*,};

pub(crate) mod addressable;
pub(crate) mod configurable;
pub(crate) mod contextual;
pub(crate) mod justifiable;

pub(crate) mod misc {
    use crate::{components::identities::Appellation, core::BoxResult};

    pub trait Catalyst<S: std::convert::Into<T>, T> {
        fn catalyst(&self, data: &S) -> T;
    }

    pub trait Transformation<S> {
        fn data(&self) -> Vec<S>;
        fn transform<T>(&self, catalyst: fn(&S) -> T) -> BoxResult<Vec<T>> {
            let res = self.data().iter().map(catalyst).collect::<Vec<_>>();
            Ok(res)
        }
    }
    pub trait ActorSpec {
        fn appellation(&self) -> Appellation;
        fn justification(&self) -> serde_json::Value;
    }

    pub trait Named {
        fn name() -> String;
    }
}
