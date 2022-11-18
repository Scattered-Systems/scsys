/*
    Appellation: derive <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[cfg(feature = "derive")]
#[cfg(test)]
mod tests {
    use scsys::{
        prelude::{Hashable, Temporal, Timestamp, H256},
        Named,
    };

    #[derive(Default, Hashable, Named, Temporal)]
    pub struct TestStruct {
        timestamp: Timestamp,
    }

    impl std::fmt::Display for TestStruct {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.timestamp)
        }
    }

    #[derive(Named)]
    struct Pancakes;

    #[test]
    fn test_simple_derive() {
        let a = Pancakes::name();
        assert_eq!(a, String::from("Pancakes"))
    }

    #[test]
    fn test_hashable_derive() {
        let a = TestStruct::default();
        // let hash = a.hash();
        assert_eq!(TestStruct::name(), String::from("TestStruct"));
    }
}
