/*
    Appellation: states <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{specs::*, state::*};

pub(crate) mod state;

pub(crate) mod specs {
    use crate::messages::Message;
    use std::sync::Arc;

    pub trait StatePack: Default + ToString {
        fn by_ref(&self) -> &Self {
            self
        }
        fn by_ref_mut(&mut self) -> &mut Self {
            self
        }
    }

    pub trait Stateful<S: StatePack>: Clone + Default {
        type Data;
        fn by_ref(&self) -> Self {
            self.clone()
        }
        fn by_ref_mut(&mut self) -> Self {
            self.clone()
        }
        fn by_arc(self: Arc<Self>) -> Arc<Self> {
            self
        }
        fn message(self) -> Message<Self::Data>;
        fn state(self) -> S;
        fn timestamp(self) -> i64;
    }

    pub trait StatefulExt<S: StatePack>: Stateful<S> {
        fn now() -> i64 {
            chrono::Utc::now().timestamp()
        }
        fn update_state(&mut self, msg: Option<Message<Self::Data>>, state: S) -> &Self;
    }
}
