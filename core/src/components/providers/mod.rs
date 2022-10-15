/*
    Appellation: providers <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{cache::Cache, databases::*, ethereum::Web3Provider};

mod cache;
mod databases;
mod ethereum;

pub(crate) mod variants {
    use super::{Cache, Databases, Web3Provider};
    use serde::{Deserialize, Serialize};
    use strum::{EnumString, EnumVariantNames};

    #[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub struct Providers {
        pub cache: Option<Cache>,
        pub database: Option<Databases>,
        pub ethereum: Option<Web3Provider>,
    }
}
