/*
    Appellation: derive <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:  ... Summary ...
*/

#[cfg(test)]
#[cfg(feature = "derive")]
mod tests {
    use scsys::prelude::{Name, SerdeDisplay, Timestamp};
    use serde::{Deserialize, Serialize};

    #[derive(
        Clone,
        Debug,
        Default,
        Deserialize,
        Eq,
        Hash,
        Name,
        Ord,
        PartialEq,
        PartialOrd,
        SerdeDisplay,
        Serialize,
    )]
    pub struct TestStruct {
        timestamp: Timestamp,
    }

    #[test]
    fn test_serde_display() {
        let a = TestStruct::default();
        assert_eq!(a.name(), "TestStruct");
        let _string = a.to_string();
        assert!(true)
    }
}
