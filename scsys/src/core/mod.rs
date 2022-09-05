/*
    Appellation: core <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{configs::*, contexts::*, messages::*, primitives::*, states::*, utils::*};

mod configs;
mod contexts;
mod messages;
mod primitives;
mod states;

mod utils {
    
}

#[cfg(test)]
mod tests {
    use super::collect_config_files;

    #[test]
    fn test_collect_config_files() {
        let actual = collect_config_files("**/*.config.*", false);
        let expected = actual.clone().len();
        assert_eq!(actual.len(), expected)
    }
}
