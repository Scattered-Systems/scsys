/*
    Appellation: providers <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{cache::Cache, database::Database, ethereum::Web3Provider};

mod cache;
mod database;
mod ethereum;

pub(crate) mod variants {
    use super::{Cache, Database, Web3Provider};
    use serde::{Deserialize, Serialize};
    use strum::{EnumString, EnumVariantNames};

    #[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
    pub enum Provider {
        Cache(Cache),
        Database(Database),
        Web3(Web3Provider),
    }

    impl Default for Provider {
        fn default() -> Self {
            Self::Database(Database::default())
        }
    }
}
