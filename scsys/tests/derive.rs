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
    use scsys::Timestamp;

    #[derive(Default, Hashable)]
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
        let a = TestStruct::default();
        let _hash = a.hash();
        assert!(true)
    }
}
