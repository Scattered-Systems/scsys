/*
    Appellation: derive <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
#[cfg(feature = "derive")]
#[cfg(test)]
mod tests {
    use scsys::Named;
    #[cfg(feature = "crypto")]
    use scsys::prelude::Hashable;
    use scsys::prelude::Timestamp;

    #[derive(Default, Hashable)]
    pub struct TestStruct {
        timestamp: Timestamp
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
        // assert!(hash.len() > 0);
    }
}
