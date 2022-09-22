/*
    Appellation: apps <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{configs::*, contexts::*, modes::ApplicationMode, states::*};

mod configs;
mod contexts;
mod states;

pub(crate) mod modes {
    use serde::{Deserialize, Serialize};
    use strum::{EnumString, EnumVariantNames};

    #[derive(
    Clone, Debug, Hash, PartialEq, Deserialize, Serialize, EnumString, EnumVariantNames,
    )]
    #[strum(serialize_all = "snake_case")]
    pub enum ApplicationMode {
        Development,
        Staging,
        Production,
        Custom(String),
    }

    impl Default for ApplicationMode {
        fn default() -> Self {
            Self::try_from("development").expect("Failed")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::ApplicationMode;

        #[test]
        fn test_default_application_mode() {
            let a = ApplicationMode::default();
            let b = ApplicationMode::Development;
            assert_eq!(a, b)
        }
    }
}
