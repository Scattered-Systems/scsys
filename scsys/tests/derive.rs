/*
    Appellation: derive <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:  ... Summary ...
*/
#[cfg(feature = "derive")]
#[cfg(test)]
mod tests {
    use scsys::prelude::*;
    use scsys::Hashable;
    use scsys::{Named, Temporal, Timestamp};

    #[derive(Default, Hashable, Temporal)]
    pub struct TestStruct {
        timestamp: Timestamp,
    }

    impl std::fmt::Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.timestamp)
        }
    }

    #[derive(Default)]
    struct Pancakes;

    #[test]
    fn test_hashable_derive() {
        let _a = TestStruct::default();
        // let hash = a.hash();
        assert!(true)
    }
}
