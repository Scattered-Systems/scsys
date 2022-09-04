/*
    Appellation: data <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[macro_export]
macro_rules! dict {
    ( $( ($x:expr, $y:expr) ),* ) => {
        {
            let mut tmp = std::collections::BTreeMap::new();
            $(
                tmp.insert($x, $y);
            )*
            tmp
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_dict_macro() {
        let a = dict![("hello", vec!["world"])];
        let b = a.clone();
        assert_eq!(a, b)
    }
}