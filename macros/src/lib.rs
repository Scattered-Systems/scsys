/*
    Appellation: scsys-macros <library>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::utils::*;

mod utils;

#[macro_export]
macro_rules! network_addr {
    ( $( ($x:expr, $y:expr) )*, ) => {
        (
            let mut tmp = Vec::new();

            $( tmp.push(($x)))*

            tmp
        )
    }
}


#[cfg(test)]
mod tests {
    use super::dict;

    #[test]
    fn test_dict() {
        let a = dict!(("a", "b"));
        let b = std::collections::BTreeMap::from([("a", "b")]);

        assert_eq!(a, b)

    }

    #[test]
    fn test_dframe() {
        let a = "a".to_string();
        let b = a.clone();

        assert_eq!(a, b)
    }
}