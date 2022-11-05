/*
    Appellation: providers <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[doc(inline)]
pub use self::{networks::*, provider::*, storage::*};

pub(crate) mod networks;
pub(crate) mod provider;
pub(crate) mod storage;

#[cfg(test)]
mod tests {
    use super::S3Configuration;

    #[test]
    fn test_s3_setup() {
        let a = S3Configuration::from_env(None, None, None, None);
        let b = a.clone();

        assert_eq!(a, b)
    }
}
